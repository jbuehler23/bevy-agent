//! Project management and utilities for Bevy AI

use crate::config::{ProjectConfig, ProjectMetadata, ConversationEntry, GeneratedFile, Dependency};
use crate::error::{BevyAIError, Result};
use crate::ai::{BevyAIAgent, AIResponse};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use std::collections::HashMap;

/// Project manager for Bevy AI projects
pub struct ProjectManager {
    project_path: PathBuf,
    config: Option<ProjectConfig>,
}

/// Represents a Bevy AI project
pub struct Project {
    manager: ProjectManager,
    agent: BevyAIAgent,
}

/// Cargo.toml management
pub struct CargoManager {
    // This struct is currently not implemented
    // TODO: Add cargo management functionality
}

/// Dependency information
#[derive(Debug, Clone)]
pub struct DependencyInfo {
    /// The name of the dependency
    pub name: String,
    /// The version requirement of the dependency
    pub version: String,
    /// List of features to enable for this dependency
    pub features: Vec<String>,
    /// Whether this dependency is optional
    pub optional: bool,
    /// Whether to enable default features for this dependency
    pub default_features: bool,
}

impl ProjectManager {
    /// Create a new project manager for the given path
    pub fn new<P: AsRef<Path>>(project_path: P) -> Self {
        Self {
            project_path: project_path.as_ref().to_path_buf(),
            config: None,
        }
    }
    
    /// Initialize a new Bevy AI project
    pub async fn init(&mut self, name: &str, description: &str) -> Result<()> {
        // Create project directory structure
        self.create_directory_structure().await?;
        
        // Initialize Cargo.toml
        self.create_cargo_toml(name).await?;
        
        // Create basic main.rs
        self.create_main_rs().await?;
        
        // Create .bevy-agent.json config
        let metadata = ProjectMetadata {
            name: name.to_string(),
            description: description.to_string(),
            version: "0.1.0".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            bevy_version: "0.12".to_string(),
            features: Vec::new(),
            tags: Vec::new(),
        };
        
        let config = ProjectConfig {
            metadata,
            conversations: Vec::new(),
            generated_files: Vec::new(),
            dependencies: Vec::new(),
            templates: Vec::new(),
        };
        
        self.save_config(&config).await?;
        self.config = Some(config);
        
        Ok(())
    }
    
    /// Load existing project configuration
    pub async fn load(&mut self) -> Result<()> {
        let config_path = self.project_path.join(".bevy-agent.json");
        
        if !config_path.exists() {
            return Err(BevyAIError::project_not_found(self.project_path.display().to_string()));
        }
        
        let content = fs::read_to_string(&config_path)?;
        let config: ProjectConfig = serde_json::from_str(&content)?;
        
        self.config = Some(config);
        Ok(())
    }
    
    /// Save project configuration
    pub async fn save_config(&self, config: &ProjectConfig) -> Result<()> {
        let config_path = self.project_path.join(".bevy-agent.json");
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    /// Get project configuration
    pub fn config(&self) -> Option<&ProjectConfig> {
        self.config.as_ref()
    }
    
    /// Get mutable project configuration
    pub fn config_mut(&mut self) -> Option<&mut ProjectConfig> {
        self.config.as_mut()
    }
    
    /// Add a conversation entry
    pub async fn add_conversation(&mut self, entry: ConversationEntry) -> Result<()> {
        if let Some(config) = &mut self.config {
            config.conversations.push(entry);
            config.metadata.updated_at = chrono::Utc::now();
            let config_clone = config.clone();
            self.save_config(&config_clone).await?;
        }
        Ok(())
    }
    
    /// Add a generated file entry
    pub async fn add_generated_file(&mut self, file: GeneratedFile) -> Result<()> {
        if let Some(config) = &mut self.config {
            config.generated_files.push(file);
            config.metadata.updated_at = chrono::Utc::now();
            let config_clone = config.clone();
            self.save_config(&config_clone).await?;
        }
        Ok(())
    }
    
    /// Add a dependency
    pub async fn add_dependency(&mut self, dependency: Dependency) -> Result<()> {
        if let Some(config) = &mut self.config {
            // Check if dependency already exists
            if !config.dependencies.iter().any(|d| d.name == dependency.name) {
                config.dependencies.push(dependency.clone());
                config.metadata.updated_at = chrono::Utc::now();
                let config_clone = config.clone();
                self.save_config(&config_clone).await?;
                
                // Update Cargo.toml
                self.update_cargo_dependencies(&[dependency]).await?;
            }
        }
        Ok(())
    }
    
    /// Create directory structure
    async fn create_directory_structure(&self) -> Result<()> {
        let dirs = [
            "src",
            "assets",
            "assets/textures",
            "assets/sounds",
            "assets/models",
            "assets/fonts",
            "examples",
            "tests",
            "benches",
        ];
        
        for dir in &dirs {
            let dir_path = self.project_path.join(dir);
            fs::create_dir_all(&dir_path)?;
        }
        
        Ok(())
    }
    
    /// Create basic Cargo.toml
    async fn create_cargo_toml(&self, name: &str) -> Result<()> {
        let cargo_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "A Bevy game created with Bevy AI"

[dependencies]
bevy = {{ version = "0.12", features = ["default"] }}

[dev-dependencies]
bevy = {{ version = "0.12", features = ["default", "dynamic_linking"] }}

[[example]]
name = "game"
path = "examples/game.rs"

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
"#, name);
        
        let cargo_path = self.project_path.join("Cargo.toml");
        fs::write(&cargo_path, cargo_content)?;
        
        Ok(())
    }
    
    /// Create basic main.rs
    async fn create_main_rs(&self) -> Result<()> {
        let main_content = r#"use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            // Add your systems here
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}
"#;
        
        let main_path = self.project_path.join("src/main.rs");
        fs::write(&main_path, main_content)?;
        
        Ok(())
    }
    
    /// Update Cargo.toml with new dependencies
    async fn update_cargo_dependencies(&self, dependencies: &[Dependency]) -> Result<()> {
        let cargo_path = self.project_path.join("Cargo.toml");
        let mut cargo_content = fs::read_to_string(&cargo_path)?;
        
        for dep in dependencies {
            let dep_line = if dep.features.is_empty() {
                format!("{} = \"{}\"", dep.name, dep.version)
            } else {
                format!("{} = {{ version = \"{}\", features = {:?} }}", 
                       dep.name, dep.version, dep.features)
            };
            
            // Simple append to dependencies section
            if let Some(deps_start) = cargo_content.find("[dependencies]") {
                let insert_pos = cargo_content[deps_start..].find('\n').unwrap() + deps_start + 1;
                cargo_content.insert_str(insert_pos, &format!("{}\n", dep_line));
            }
        }
        
        fs::write(&cargo_path, cargo_content)?;
        Ok(())
    }
    
    /// Analyze existing code for dependencies
    pub async fn analyze_dependencies(&self) -> Result<Vec<DependencyInfo>> {
        let mut dependencies = Vec::new();
        let src_path = self.project_path.join("src");
        
        if !src_path.exists() {
            return Ok(dependencies);
        }
        
        // Walk through all Rust files
        for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(entry.path())?;
                dependencies.extend(self.extract_dependencies_from_code(&content));
            }
        }
        
        Ok(dependencies)
    }
    
    /// Extract dependencies from Rust code
    fn extract_dependencies_from_code(&self, code: &str) -> Vec<DependencyInfo> {
        let mut dependencies = HashMap::new();
        
        // Common Bevy-related dependencies based on use statements
        let dependency_patterns = [
            ("bevy_rapier2d", vec!["rapier", "physics"]),
            ("bevy_rapier3d", vec!["rapier", "physics"]),
            ("rand", vec!["rand"]),
            ("serde", vec!["serde"]),
            ("noise", vec!["noise"]),
            ("image", vec!["image"]),
            ("nalgebra", vec!["nalgebra", "na"]),
            ("glam", vec!["glam"]),
            ("winit", vec!["winit"]),
            ("wgpu", vec!["wgpu"]),
        ];
        
        for (dep_name, patterns) in &dependency_patterns {
            for pattern in patterns {
                if code.contains(pattern) {
                    dependencies.insert(dep_name.to_string(), DependencyInfo {
                        name: dep_name.to_string(),
                        version: "*".to_string(),
                        features: Vec::new(),
                        optional: false,
                        default_features: true,
                    });
                    break;
                }
            }
        }
        
        dependencies.into_values().collect()
    }
    
    /// Build the project
    pub async fn build(&self) -> Result<String> {
        let output = Command::new("cargo")
            .arg("build")
            .current_dir(&self.project_path)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(BevyAIError::build_system(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
    
    /// Run the project
    pub async fn run(&self) -> Result<String> {
        let output = Command::new("cargo")
            .arg("run")
            .current_dir(&self.project_path)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(BevyAIError::build_system(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
    
    /// Check code with clippy
    pub async fn check(&self) -> Result<String> {
        let output = Command::new("cargo")
            .args(&["clippy", "--", "-D", "warnings"])
            .current_dir(&self.project_path)
            .output()?;
        
        Ok(format!("{}\n{}", 
           String::from_utf8_lossy(&output.stdout),
           String::from_utf8_lossy(&output.stderr)))
    }
    
    /// Format code
    pub async fn format(&self) -> Result<String> {
        let output = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.project_path)
            .output()?;
        
        if output.status.success() {
            Ok("Code formatted successfully".to_string())
        } else {
            Err(BevyAIError::build_system(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
    
    /// Get project statistics
    pub async fn stats(&self) -> Result<ProjectStats> {
        let mut stats = ProjectStats::default();
        
        if let Some(config) = &self.config {
            stats.conversations = config.conversations.len();
            stats.generated_files = config.generated_files.len();
            stats.dependencies = config.dependencies.len();
            stats.features = config.metadata.features.len();
        }
        
        // Count lines of code
        let src_path = self.project_path.join("src");
        if src_path.exists() {
            for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
                if entry.path().extension().map_or(false, |ext| ext == "rs") {
                    let content = fs::read_to_string(entry.path())?;
                    stats.lines_of_code += content.lines().count();
                    stats.rust_files += 1;
                }
            }
        }
        
        Ok(stats)
    }
}

impl Project {
    /// Create a new project with AI agent
    pub async fn new(project_path: PathBuf, agent: BevyAIAgent) -> Result<Self> {
        let mut manager = ProjectManager::new(project_path);
        manager.load().await?;
        
        Ok(Self { manager, agent })
    }
    
    /// Initialize a new project
    pub async fn init(
        project_path: PathBuf, 
        name: &str, 
        description: &str, 
        agent: BevyAIAgent
    ) -> Result<Self> {
        let mut manager = ProjectManager::new(&project_path);
        manager.init(name, description).await?;
        
        Ok(Self { manager, agent })
    }
    
    /// Generate game code with AI
    pub async fn generate_game(&mut self, description: &str) -> Result<AIResponse> {
        let response = self.agent
            .generate_game(description)
            .execute()
            .await?;
        
        // Save conversation
        let conversation = ConversationEntry {
            id: response.conversation_id,
            request: description.to_string(),
            response: response.content.clone(),
            model_used: response.model.clone(),
            timestamp: chrono::Utc::now(),
            tokens_used: response.tokens_used,
            cost: None, // TODO: Calculate cost based on model and tokens
            files_modified: vec!["src/main.rs".to_string()],
        };
        
        self.manager.add_conversation(conversation).await?;
        
        // Write generated code
        let code = self.agent.extract_code(&response.content);
        let main_path = self.manager.project_path.join("src/main.rs");
        fs::write(&main_path, &code)?;
        
        // Track generated file
        let generated_file = GeneratedFile {
            path: "src/main.rs".to_string(),
            generator: "generate_game".to_string(),
            model: response.model.clone(),
            created_at: chrono::Utc::now(),
            checksum: format!("{:x}", md5::compute(&code)),
        };
        
        self.manager.add_generated_file(generated_file).await?;
        
        Ok(response)
    }
    
    /// Add feature with AI
    pub async fn add_feature(&mut self, feature_description: &str) -> Result<AIResponse> {
        let main_path = self.manager.project_path.join("src/main.rs");
        let existing_code = fs::read_to_string(&main_path)?;
        
        let response = self.agent
            .add_feature(feature_description, &existing_code)
            .execute()
            .await?;
        
        // Save conversation
        let conversation = ConversationEntry {
            id: response.conversation_id,
            request: format!("Add feature: {}", feature_description),
            response: response.content.clone(),
            model_used: response.model.clone(),
            timestamp: chrono::Utc::now(),
            tokens_used: response.tokens_used,
            cost: None,
            files_modified: vec!["src/main.rs".to_string()],
        };
        
        self.manager.add_conversation(conversation).await?;
        
        // Write updated code
        let code = self.agent.extract_code(&response.content);
        fs::write(&main_path, &code)?;
        
        // Update project features
        if let Some(config) = self.manager.config_mut() {
            config.metadata.features.push(feature_description.to_string());
            config.metadata.updated_at = chrono::Utc::now();
            let config_clone = config.clone();
            self.manager.save_config(&config_clone).await?;
        }
        
        Ok(response)
    }
    
    /// Get project manager
    pub fn manager(&self) -> &ProjectManager {
        &self.manager
    }
    
    /// Get mutable project manager
    pub fn manager_mut(&mut self) -> &mut ProjectManager {
        &mut self.manager
    }
    
    /// Get AI agent
    pub fn agent(&self) -> &BevyAIAgent {
        &self.agent
    }
}

/// Project statistics
#[derive(Debug, Default)]
pub struct ProjectStats {
    /// Number of AI conversations in the project
    pub conversations: usize,
    /// Number of files generated by AI
    pub generated_files: usize,
    /// Number of dependencies in the project
    pub dependencies: usize,
    /// Number of features added to the project
    pub features: usize,
    /// Total lines of code in the project
    pub lines_of_code: usize,
    /// Number of Rust source files in the project
    pub rust_files: usize,
}
