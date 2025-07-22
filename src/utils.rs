//! Utility functions for Bevy AI

use crate::error::{BevyAIError, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Code analysis utilities
pub mod code_analysis {
    use super::*;
    use syn::{File, Item, ItemFn, ItemStruct, ItemEnum, ItemImpl};
    
    /// Analyze Rust code structure
    pub struct CodeAnalyzer;
    
    /// Code structure information
    #[derive(Debug, Clone)]
    pub struct CodeStructure {
        pub functions: Vec<FunctionInfo>,
        pub structs: Vec<StructInfo>,
        pub enums: Vec<EnumInfo>,
        pub impls: Vec<ImplInfo>,
        pub imports: Vec<String>,
        pub features_used: Vec<String>,
    }
    
    #[derive(Debug, Clone)]
    pub struct FunctionInfo {
        pub name: String,
        pub is_public: bool,
        pub is_async: bool,
        pub parameters: Vec<String>,
        pub return_type: Option<String>,
        pub line_number: usize,
    }
    
    #[derive(Debug, Clone)]
    pub struct StructInfo {
        pub name: String,
        pub is_public: bool,
        pub fields: Vec<String>,
        pub derives: Vec<String>,
        pub line_number: usize,
    }
    
    #[derive(Debug, Clone)]
    pub struct EnumInfo {
        pub name: String,
        pub is_public: bool,
        pub variants: Vec<String>,
        pub derives: Vec<String>,
        pub line_number: usize,
    }
    
    #[derive(Debug, Clone)]
    pub struct ImplInfo {
        pub target: String,
        pub trait_name: Option<String>,
        pub methods: Vec<String>,
        pub line_number: usize,
    }
    
    impl CodeAnalyzer {
        /// Analyze a Rust source file
        pub fn analyze_file<P: AsRef<Path>>(path: P) -> Result<CodeStructure> {
            let content = fs::read_to_string(&path)?;
            Self::analyze_code(&content)
        }
        
        /// Analyze Rust code from a string
        pub fn analyze_code(code: &str) -> Result<CodeStructure> {
            let syntax_tree: File = syn::parse_str(code)
                .map_err(|e| BevyAIError::CodeParsing(format!("Failed to parse Rust code: {}", e)))?;
            
            let mut structure = CodeStructure {
                functions: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
                impls: Vec::new(),
                imports: Vec::new(),
                features_used: Vec::new(),
            };
            
            for item in syntax_tree.items {
                match item {
                    Item::Fn(func) => {
                        structure.functions.push(Self::analyze_function(&func));
                    }
                    Item::Struct(struct_item) => {
                        structure.structs.push(Self::analyze_struct(&struct_item));
                    }
                    Item::Enum(enum_item) => {
                        structure.enums.push(Self::analyze_enum(&enum_item));
                    }
                    Item::Impl(impl_item) => {
                        structure.impls.push(Self::analyze_impl(&impl_item));
                    }
                    Item::Use(use_item) => {
                        structure.imports.push(quote::quote!(#use_item).to_string());
                    }
                    _ => {}
                }
            }
            
            // Analyze Bevy features being used
            structure.features_used = Self::detect_bevy_features(code);
            
            Ok(structure)
        }
        
        fn analyze_function(func: &ItemFn) -> FunctionInfo {
            FunctionInfo {
                name: func.sig.ident.to_string(),
                is_public: matches!(func.vis, syn::Visibility::Public(_)),
                is_async: func.sig.asyncness.is_some(),
                parameters: func.sig.inputs.iter()
                    .map(|param| quote::quote!(#param).to_string())
                    .collect(),
                return_type: Self::extract_return_type(func.sig.output.clone()),
                line_number: 0, // TODO: Extract line numbers
            }
        }
        
        fn analyze_struct(struct_item: &ItemStruct) -> StructInfo {
            StructInfo {
                name: struct_item.ident.to_string(),
                is_public: matches!(struct_item.vis, syn::Visibility::Public(_)),
                fields: struct_item.fields.iter()
                    .map(|field| {
                        field.ident.as_ref()
                            .map(|i| i.to_string())
                            .unwrap_or_else(|| "unnamed".to_string())
                    })
                    .collect(),
                derives: struct_item.attrs.iter()
                    .filter_map(|attr| {
                        if attr.path().is_ident("derive") {
                            Some(quote::quote!(#attr).to_string())
                        } else {
                            None
                        }
                    })
                    .collect(),
                line_number: 0,
            }
        }
        
        fn analyze_enum(enum_item: &ItemEnum) -> EnumInfo {
            EnumInfo {
                name: enum_item.ident.to_string(),
                is_public: matches!(enum_item.vis, syn::Visibility::Public(_)),
                variants: enum_item.variants.iter()
                    .map(|variant| variant.ident.to_string())
                    .collect(),
                derives: enum_item.attrs.iter()
                    .filter_map(|attr| {
                        if attr.path().is_ident("derive") {
                            Some(quote::quote!(#attr).to_string())
                        } else {
                            None
                        }
                    })
                    .collect(),
                line_number: 0,
            }
        }
        
        fn analyze_impl(impl_item: &ItemImpl) -> ImplInfo {
            ImplInfo {
                target: quote::quote!(#impl_item.self_ty).to_string(),
                trait_name: impl_item.trait_.as_ref()
                    .map(|(_, path, _)| quote::quote!(#path).to_string()),
                methods: impl_item.items.iter()
                    .filter_map(|item| {
                        if let syn::ImplItem::Fn(method) = item {
                            Some(method.sig.ident.to_string())
                        } else {
                            None
                        }
                    })
                    .collect(),
                line_number: 0,
            }
        }
        
        fn detect_bevy_features(code: &str) -> Vec<String> {
            let mut features = Vec::new();
            
            let feature_patterns = [
                ("2D Rendering", vec!["Camera2dBundle", "Sprite", "SpriteBundle"]),
                ("3D Rendering", vec!["Camera3dBundle", "PbrBundle", "Mesh"]),
                ("UI", vec!["ButtonBundle", "TextBundle", "NodeBundle"]),
                ("Audio", vec!["AudioBundle", "AudioSource"]),
                ("Physics", vec!["RigidBody", "Collider", "Velocity"]),
                ("Animation", vec!["AnimationPlayer", "AnimationClip"]),
                ("Networking", vec!["NetworkEvent", "Connection"]),
                ("Input", vec!["ButtonInput", "KeyCode", "MouseButton"]),
                ("ECS", vec!["Component", "Resource", "System"]),
                ("Transform", vec!["Transform", "GlobalTransform"]),
                ("Time", vec!["Time", "Timer"]),
                ("Events", vec!["EventReader", "EventWriter"]),
                ("Assets", vec!["Assets", "Handle", "AssetServer"]),
                ("Scene", vec!["Scene", "SceneBundle"]),
                ("Lighting", vec!["DirectionalLight", "PointLight", "SpotLight"]),
            ];
            
            for (feature_name, patterns) in &feature_patterns {
                for pattern in patterns {
                    if code.contains(pattern) {
                        features.push(feature_name.to_string());
                        break;
                    }
                }
            }
            
            features
        }
    
        // Helper function to extract return type
        fn extract_return_type(return_type: syn::ReturnType) -> Option<String> {
            match return_type {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(quote::quote!(#ty).to_string()),
            }
        }
    }
}

/// File system utilities
pub mod fs_utils {
    use super::*;
    
    /// Find all Rust files in a directory
    pub fn find_rust_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
        let mut rust_files = Vec::new();
        
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                rust_files.push(entry.path().to_path_buf());
            }
        }
        
        Ok(rust_files)
    }
    
    /// Get file extension
    pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }
    
    /// Create a backup of a file
    pub fn backup_file<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
        let path = path.as_ref();
        let backup_path = path.with_extension(
            format!("{}.backup.{}", 
                path.extension().unwrap_or_default().to_string_lossy(),
                chrono::Utc::now().timestamp()
            )
        );
        
        fs::copy(path, &backup_path)?;
        Ok(backup_path)
    }
    
    /// Calculate MD5 hash of a file
    pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
        let content = fs::read(path)?;
        Ok(format!("{:x}", md5::compute(&content)))
    }
    
    /// Count lines in a file
    pub fn count_lines<P: AsRef<Path>>(path: P) -> Result<usize> {
        let content = fs::read_to_string(path)?;
        Ok(content.lines().count())
    }
}

/// Build system utilities
pub mod build_utils {
    use super::*;
    
    /// Check if Rust toolchain is available
    pub fn check_rust_toolchain() -> Result<String> {
        let output = Command::new("rustc")
            .arg("--version")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(BevyAIError::build_system("Rust toolchain not found".to_string()))
        }
    }
    
    /// Check if Cargo is available
    pub fn check_cargo() -> Result<String> {
        let output = Command::new("cargo")
            .arg("--version")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(BevyAIError::build_system("Cargo not found".to_string()))
        }
    }
    
    /// Run cargo check
    pub fn cargo_check<P: AsRef<Path>>(project_path: P) -> Result<String> {
        let output = Command::new("cargo")
            .arg("check")
            .current_dir(project_path)
            .output()?;
        
        Ok(format!("{}\n{}", 
           String::from_utf8_lossy(&output.stdout),
           String::from_utf8_lossy(&output.stderr)))
    }
    
    /// Run cargo clippy
    pub fn cargo_clippy<P: AsRef<Path>>(project_path: P) -> Result<String> {
        let output = Command::new("cargo")
            .args(&["clippy", "--", "-D", "warnings"])
            .current_dir(project_path)
            .output()?;
        
        Ok(format!("{}\n{}", 
           String::from_utf8_lossy(&output.stdout),
           String::from_utf8_lossy(&output.stderr)))
    }
    
    /// Run cargo fmt
    pub fn cargo_fmt<P: AsRef<Path>>(project_path: P) -> Result<String> {
        let output = Command::new("cargo")
            .arg("fmt")
            .current_dir(project_path)
            .output()?;
        
        if output.status.success() {
            Ok("Code formatted successfully".to_string())
        } else {
            Err(BevyAIError::build_system(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
}

/// Text processing utilities
pub mod text_utils {
    use super::*;
    
    /// Extract code blocks from markdown text
    pub fn extract_code_blocks(text: &str) -> Vec<(Option<String>, String)> {
        let mut code_blocks = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            if lines[i].starts_with("```") {
                let language = if lines[i].len() > 3 {
                    Some(lines[i][3..].trim().to_string())
                } else {
                    None
                };
                
                let mut code = String::new();
                i += 1;
                
                while i < lines.len() && !lines[i].starts_with("```") {
                    code.push_str(lines[i]);
                    code.push('\n');
                    i += 1;
                }
                
                code_blocks.push((language, code.trim().to_string()));
            }
            i += 1;
        }
        
        code_blocks
    }
    
    /// Clean up AI-generated code
    pub fn clean_ai_code(code: &str) -> String {
        // Remove common AI artifacts
        let mut cleaned = code.to_string();
        
        // Remove explanatory comments that start with "Note:" or "TODO:"
        cleaned = cleaned.lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("// Note:") && 
                !trimmed.starts_with("// TODO: Add") &&
                !trimmed.starts_with("// TODO: Implement")
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        // Normalize whitespace
        cleaned = cleaned.trim().to_string();
        
        cleaned
    }
    
    /// Format Rust code using rustfmt
    pub fn format_rust_code(code: &str) -> Result<String> {
        use std::io::Write;
        use std::process::Stdio;
        
        let mut child = Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(code.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            // If rustfmt fails, return original code
            Ok(code.to_string())
        }
    }
    
    /// Convert snake_case to PascalCase
    pub fn snake_to_pascal(input: &str) -> String {
        input.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect()
    }
    
    /// Convert PascalCase to snake_case
    pub fn pascal_to_snake(input: &str) -> String {
        let mut result = String::new();
        let mut prev_was_upper = false;
        
        for (i, c) in input.chars().enumerate() {
            if c.is_uppercase() && i > 0 && !prev_was_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
            prev_was_upper = c.is_uppercase();
        }
        
        result
    }
}

/// Configuration utilities
pub mod config_utils {
    use super::*;
    use crate::config::AIConfig;
    
    /// Validate AI configuration
    pub fn validate_config(config: &AIConfig) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        if config.openai.is_none() && config.anthropic.is_none() && config.google.is_none() {
            warnings.push("No AI providers configured".to_string());
        }
        
        if config.generation.temperature < 0.0 || config.generation.temperature > 2.0 {
            warnings.push("Temperature should be between 0.0 and 2.0".to_string());
        }
        
        if config.generation.max_tokens < 100 {
            warnings.push("Max tokens is very low, may result in incomplete responses".to_string());
        }
        
        Ok(warnings)
    }
    
    /// Get default Bevy version
    pub fn get_latest_bevy_version() -> Result<String> {
        // In a real implementation, this would query crates.io API
        Ok("0.12".to_string())
    }
    
    /// Get recommended dependencies for a game type
    pub fn get_recommended_dependencies(game_type: &str) -> Vec<String> {
        match game_type.to_lowercase().as_str() {
            "platformer" | "2d" => vec![
                "bevy".to_string(),
            ],
            "fps" | "3d" | "shooter" => vec![
                "bevy".to_string(),
                "bevy_rapier3d".to_string(),
            ],
            "physics" => vec![
                "bevy".to_string(),
                "bevy_rapier2d".to_string(),
                "bevy_rapier3d".to_string(),
            ],
            "ui" | "menu" => vec![
                "bevy".to_string(),
                "bevy_egui".to_string(),
            ],
            _ => vec![
                "bevy".to_string(),
            ],
        }
    }
}
