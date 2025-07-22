//! Command-line interface for Bevy AI

use crate::{
    ai::BevyAIAgent,
    config::{AIConfig, ModelType},
    error::{BevyAIError, Result},
    game_templates::{TemplateManager, TemplateContext},
    project::{Project, ProjectManager},
    utils::{build_utils, config_utils},
};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use tracing::{error, info, warn};

/// Bevy AI - AI-powered Bevy game development assistant
#[derive(Parser)]
#[command(name = "bevy-ai")]
#[command(about = "AI-powered Bevy game prototyping assistant with GPT/Claude integration")]
#[command(version = crate::VERSION)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new game prototype from natural language description
    Create {
        /// Describe the game you want to create
        description: String,
        
        /// AI model to use (gpt-4, claude-3-opus, etc.)
        #[arg(long)]
        model: Option<ModelType>,
        
        /// Project name (defaults to generated name)
        #[arg(long)]
        name: Option<String>,
        
        /// Output directory
        #[arg(long, short)]
        output: Option<PathBuf>,
        
        /// Use a specific template
        #[arg(long)]
        template: Option<String>,
        
        /// Don't create a new directory, use current directory
        #[arg(long)]
        in_place: bool,
    },
    
    /// Add features to existing prototype
    Add {
        /// Describe what to add to the game
        feature: String,
        
        /// AI model to use
        #[arg(long)]
        model: Option<ModelType>,
        
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Refactor or improve existing code
    Improve {
        /// What to improve (performance, readability, features)
        aspect: ImprovementAspect,
        
        /// AI model to use
        #[arg(long)]
        model: Option<ModelType>,
        
        /// Specific file to improve (defaults to main.rs)
        #[arg(long)]
        file: Option<PathBuf>,
        
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Ask AI to explain the current codebase
    Explain {
        /// AI model to use
        #[arg(long)]
        model: Option<ModelType>,
        
        /// Specific file to explain (defaults to main.rs)
        #[arg(long)]
        file: Option<PathBuf>,
        
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Debug code issues with AI assistance
    Debug {
        /// Error message or description of the issue
        error: String,
        
        /// AI model to use
        #[arg(long)]
        model: Option<ModelType>,
        
        /// Specific file with the issue (defaults to main.rs)
        #[arg(long)]
        file: Option<PathBuf>,
        
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Initialize a new Bevy project with AI agent support
    Init {
        /// Project name
        name: String,
        
        /// Project description
        #[arg(long, short)]
        description: Option<String>,
        
        /// Output directory
        #[arg(long, short)]
        output: Option<PathBuf>,
        
        /// Use a specific template
        #[arg(long)]
        template: Option<String>,
    },
    
    /// Configure AI API keys and settings
    Config {
        /// OpenAI API key
        #[arg(long)]
        openai_key: Option<String>,
        
        /// Anthropic API key
        #[arg(long)]
        anthropic_key: Option<String>,
        
        /// Google API key
        #[arg(long)]
        google_key: Option<String>,
        
        /// Default AI model
        #[arg(long)]
        default_model: Option<ModelType>,
        
        /// Show current configuration
        #[arg(long)]
        show: bool,
        
        /// Validate configuration
        #[arg(long)]
        validate: bool,
    },
    
    /// Build operations
    Build {
        #[command(subcommand)]
        operation: BuildOperation,
    },
    
    /// Project management commands
    Project {
        #[command(subcommand)]
        operation: ProjectOperation,
    },
    
    /// Template management commands
    Template {
        #[command(subcommand)]
        operation: TemplateOperation,
    },
}

#[derive(Subcommand)]
pub enum BuildOperation {
    /// Build the current prototype
    Build {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
        
        /// Release build
        #[arg(long)]
        release: bool,
    },
    
    /// Run the current prototype
    Run {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
        
        /// Release build
        #[arg(long)]
        release: bool,
        
        /// Additional arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// Check code for errors
    Check {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Run clippy lints
    Clippy {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Format code
    Format {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Run tests
    Test {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum ProjectOperation {
    /// Show project information
    Info {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Show project statistics
    Stats {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
    
    /// Show conversation history
    History {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
        
        /// Number of recent conversations to show
        #[arg(long, default_value = "10")]
        limit: usize,
    },
    
    /// Export project data
    Export {
        /// Output file path
        output: PathBuf,
        
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
        
        /// Export format
        #[arg(long, default_value = "json")]
        format: ExportFormat,
    },
    
    /// Clean generated files
    Clean {
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
        
        /// Also clean Cargo build artifacts
        #[arg(long)]
        cargo: bool,
    },
}

#[derive(Subcommand)]
pub enum TemplateOperation {
    /// List available templates
    List,
    
    /// Show template details
    Show {
        /// Template name
        name: String,
    },
    
    /// Create a new custom template
    Create {
        /// Template name
        name: String,
        
        /// Template description
        description: String,
        
        /// Template file path
        file: PathBuf,
    },
    
    /// Apply a template to current project
    Apply {
        /// Template name
        name: String,
        
        /// Project directory (defaults to current directory)
        #[arg(long, short)]
        project: Option<PathBuf>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ImprovementAspect {
    Performance,
    Readability,
    Features,
    Structure,
    Testing,
    Documentation,
    Security,
    Efficiency,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExportFormat {
    Json,
    Yaml,
    Toml,
    Markdown,
}

impl std::fmt::Display for ImprovementAspect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImprovementAspect::Performance => write!(f, "performance"),
            ImprovementAspect::Readability => write!(f, "readability"),
            ImprovementAspect::Features => write!(f, "features"),
            ImprovementAspect::Structure => write!(f, "structure"),
            ImprovementAspect::Testing => write!(f, "testing"),
            ImprovementAspect::Documentation => write!(f, "documentation"),
            ImprovementAspect::Security => write!(f, "security"),
            ImprovementAspect::Efficiency => write!(f, "efficiency"),
        }
    }
}

/// Main CLI handler
pub struct CliHandler {
    config: AIConfig,
    agent: Option<BevyAIAgent>,
}

impl CliHandler {
    /// Create a new CLI handler
    pub async fn new(config_path: Option<PathBuf>) -> Result<Self> {
        let config = if let Some(path) = config_path {
            AIConfig::from_file(path)?
        } else {
            AIConfig::load_or_create()?
        };
        
        let agent = BevyAIAgent::new(config.clone()).await.ok();
        
        Ok(Self { config, agent })
    }
    
    /// Handle CLI commands
    pub async fn handle(&mut self, cli: Cli) -> Result<()> {
        // Initialize logging
        if cli.verbose {
            tracing_subscriber::fmt()
                .with_env_filter("bevy_ai=debug")
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_env_filter("bevy_ai=info")
                .init();
        }
        
        match cli.command {
            Commands::Create { description, model, name, output, template, in_place } => {
                self.handle_create(description, model, name, output, template, in_place).await
            }
            Commands::Add { feature, model, project } => {
                self.handle_add(feature, model, project).await
            }
            Commands::Improve { aspect, model, file, project } => {
                self.handle_improve(aspect, model, file, project).await
            }
            Commands::Explain { model, file, project } => {
                self.handle_explain(model, file, project).await
            }
            Commands::Debug { error, model, file, project } => {
                self.handle_debug(error, model, file, project).await
            }
            Commands::Init { name, description, output, template } => {
                self.handle_init(name, description, output, template).await
            }
            Commands::Config { openai_key, anthropic_key, google_key, default_model, show, validate } => {
                self.handle_config(openai_key, anthropic_key, google_key, default_model, show, validate).await
            }
            Commands::Build { operation } => {
                self.handle_build_operation(operation).await
            }
            Commands::Project { operation } => {
                self.handle_project_operation(operation).await
            }
            Commands::Template { operation } => {
                self.handle_template_operation(operation).await
            }
        }
    }
    
    async fn handle_create(
        &mut self,
        description: String,
        model: Option<ModelType>,
        name: Option<String>,
        output: Option<PathBuf>,
        template: Option<String>,
        in_place: bool,
    ) -> Result<()> {
        let agent = self.agent.as_ref()
            .ok_or_else(|| BevyAIError::ai_api("No AI agent available. Please configure API keys.".to_string()))?;
        
        let project_name = name.unwrap_or_else(|| self.generate_project_name(&description));
        let project_path = if in_place {
            std::env::current_dir()?
        } else {
            output.unwrap_or_else(|| std::env::current_dir().unwrap().join(&project_name))
        };
        
        info!("🤖 Creating game '{}' from description: {}", project_name, description);
        
        if let Some(template_name) = template {
            // Use template-based generation
            let template_manager = TemplateManager::new()?;
            let context = TemplateContext::new(project_name.clone(), description.clone());
            let code = template_manager.generate(&template_name, &context)?;
            
            // Create project structure manually
            let mut project_manager = ProjectManager::new(&project_path);
            project_manager.init(&project_name, &description).await?;
            
            // Write generated code
            std::fs::write(project_path.join("src/main.rs"), code)?;
        } else {
            // Use AI generation
            let mut project = Project::init(project_path.clone(), &project_name, &description, agent.clone()).await?;
            let response = project.generate_game(&description).await?;
            
            info!("✅ Game '{}' created successfully!", project_name);
            info!("📁 Project location: {}", project_path.display());
            info!("🚀 To run: cd {} && cargo run", project_path.display());
            
            if let Some(tokens) = response.tokens_used {
                info!("🔢 Tokens used: {}", tokens);
            }
        }
        
        Ok(())
    }
    
    async fn handle_add(&mut self, feature: String, model: Option<ModelType>, project: Option<PathBuf>) -> Result<()> {
        let agent = self.agent.as_ref()
            .ok_or_else(|| BevyAIError::ai_api("No AI agent available. Please configure API keys.".to_string()))?;
        
        let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
        
        info!("🤖 Adding feature: {}", feature);
        
        let mut project = Project::new(project_path, agent.clone()).await?;
        let response = project.add_feature(&feature).await?;
        
        info!("✅ Feature added successfully!");
        
        if let Some(tokens) = response.tokens_used {
            info!("🔢 Tokens used: {}", tokens);
        }
        
        Ok(())
    }
    
    async fn handle_improve(
        &mut self,
        aspect: ImprovementAspect,
        model: Option<ModelType>,
        file: Option<PathBuf>,
        project: Option<PathBuf>,
    ) -> Result<()> {
        let agent = self.agent.as_ref()
            .ok_or_else(|| BevyAIError::ai_api("No AI agent available. Please configure API keys.".to_string()))?;
        
        let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
        let file_path = file.unwrap_or_else(|| project_path.join("src/main.rs"));
        
        if !file_path.exists() {
            return Err(BevyAIError::file_operation("read", &file_path.display().to_string()));
        }
        
        let code = std::fs::read_to_string(&file_path)?;
        
        info!("🤖 Improving {} of {}", aspect, file_path.display());
        
        let response = agent
            .improve_code(aspect.to_string(), code)
            .with_model(model.unwrap_or(self.config.default_model.clone()))
            .execute()
            .await?;
        
        let improved_code = agent.extract_code(&response.content);
        
        // Create backup
        crate::utils::fs_utils::backup_file(&file_path)?;
        
        // Write improved code
        std::fs::write(&file_path, improved_code)?;
        
        info!("✅ Code improved successfully!");
        info!("📝 Backup created for original file");
        
        if let Some(tokens) = response.tokens_used {
            info!("🔢 Tokens used: {}", tokens);
        }
        
        Ok(())
    }
    
    async fn handle_explain(
        &mut self,
        model: Option<ModelType>,
        file: Option<PathBuf>,
        project: Option<PathBuf>,
    ) -> Result<()> {
        let agent = self.agent.as_ref()
            .ok_or_else(|| BevyAIError::ai_api("No AI agent available. Please configure API keys.".to_string()))?;
        
        let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
        let file_path = file.unwrap_or_else(|| project_path.join("src/main.rs"));
        
        if !file_path.exists() {
            return Err(BevyAIError::file_operation("read", &file_path.display().to_string()));
        }
        
        let code = std::fs::read_to_string(&file_path)?;
        
        info!("🤖 Explaining code in {}", file_path.display());
        
        let response = agent
            .explain_code(code)
            .with_model(model.unwrap_or(self.config.default_model.clone()))
            .execute()
            .await?;
        
        println!("\n🤖 AI Code Explanation:\n");
        println!("{}", response.content);
        
        if let Some(tokens) = response.tokens_used {
            info!("🔢 Tokens used: {}", tokens);
        }
        
        Ok(())
    }
    
    async fn handle_debug(
        &mut self,
        error: String,
        model: Option<ModelType>,
        file: Option<PathBuf>,
        project: Option<PathBuf>,
    ) -> Result<()> {
        let agent = self.agent.as_ref()
            .ok_or_else(|| BevyAIError::ai_api("No AI agent available. Please configure API keys.".to_string()))?;
        
        let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
        let file_path = file.unwrap_or_else(|| project_path.join("src/main.rs"));
        
        if !file_path.exists() {
            return Err(BevyAIError::file_operation("read", &file_path.display().to_string()));
        }
        
        let code = std::fs::read_to_string(&file_path)?;
        
        info!("🤖 Debugging issue: {}", error);
        
        let response = agent
            .debug_code(code, error)
            .with_model(model.unwrap_or(self.config.default_model.clone()))
            .execute()
            .await?;
        
        println!("\n🔧 AI Debug Analysis:\n");
        println!("{}", response.content);
        
        // Extract and offer to apply fixed code
        let fixed_code = agent.extract_code(&response.content);
        if fixed_code != response.content {
            println!("\n❓ Apply the suggested fix? [y/N]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if input.trim().to_lowercase() == "y" {
                // Create backup
                crate::utils::fs_utils::backup_file(&file_path)?;
                
                // Write fixed code
                std::fs::write(&file_path, fixed_code)?;
                
                info!("✅ Fix applied successfully!");
                info!("📝 Backup created for original file");
            }
        }
        
        if let Some(tokens) = response.tokens_used {
            info!("🔢 Tokens used: {}", tokens);
        }
        
        Ok(())
    }
    
    async fn handle_init(
        &mut self,
        name: String,
        description: Option<String>,
        output: Option<PathBuf>,
        template: Option<String>,
    ) -> Result<()> {
        let description = description.unwrap_or_else(|| format!("A new Bevy game: {}", name));
        let project_path = output.unwrap_or_else(|| std::env::current_dir().unwrap().join(&name));
        
        info!("🚀 Initializing new Bevy AI project: {}", name);
        
        let mut project_manager = ProjectManager::new(&project_path);
        project_manager.init(&name, &description).await?;
        
        if let Some(template_name) = template {
            let template_manager = TemplateManager::new()?;
            let context = TemplateContext::new(name.clone(), description.clone());
            let code = template_manager.generate(&template_name, &context)?;
            
            std::fs::write(project_path.join("src/main.rs"), code)?;
            info!("📝 Applied template: {}", template_name);
        }
        
        info!("✅ Project '{}' initialized successfully!", name);
        info!("📁 Project location: {}", project_path.display());
        info!("🚀 To get started: cd {} && bevy-ai add \"your first feature\"", project_path.display());
        
        Ok(())
    }
    
    async fn handle_config(
        &mut self,
        openai_key: Option<String>,
        anthropic_key: Option<String>,
        google_key: Option<String>,
        default_model: Option<ModelType>,
        show: bool,
        validate: bool,
    ) -> Result<()> {
        if show {
            println!("🔧 Current Configuration:");
            println!("OpenAI: {}", if self.config.openai.is_some() { "✅ Configured" } else { "❌ Not configured" });
            println!("Anthropic: {}", if self.config.anthropic.is_some() { "✅ Configured" } else { "❌ Not configured" });
            println!("Google: {}", if self.config.google.is_some() { "✅ Configured" } else { "❌ Not configured" });
            println!("Default Model: {}", self.config.default_model);
            println!("Available Models: {:?}", self.config.available_models());
            return Ok(());
        }
        
        if validate {
            let warnings = config_utils::validate_config(&self.config)?;
            if warnings.is_empty() {
                info!("✅ Configuration is valid");
            } else {
                warn!("⚠️ Configuration warnings:");
                for warning in warnings {
                    warn!("  - {}", warning);
                }
            }
            return Ok(());
        }
        
        let mut updated = false;
        
        if let Some(key) = openai_key {
            self.config.openai = Some(crate::config::OpenAIConfig {
                api_key: key,
                organization: None,
                base_url: None,
            });
            info!("✅ OpenAI API key configured");
            updated = true;
        }
        
        if let Some(key) = anthropic_key {
            self.config.anthropic = Some(crate::config::AnthropicConfig {
                api_key: key,
                base_url: None,
            });
            info!("✅ Anthropic API key configured");
            updated = true;
        }
        
        if let Some(key) = google_key {
            self.config.google = Some(crate::config::GoogleConfig {
                api_key: key,
                base_url: None,
            });
            info!("✅ Google API key configured");
            updated = true;
        }
        
        if let Some(model) = default_model {
            self.config.default_model = model;
            info!("✅ Default model set to: {}", self.config.default_model);
            updated = true;
        }
        
        if updated {
            let config_path = AIConfig::default_config_path()?;
            self.config.save_to_file(&config_path)?;
            
            // Recreate agent with new config
            if let Ok(agent) = BevyAIAgent::new(self.config.clone()).await {
                self.agent = Some(agent);
                info!("🔄 AI agent updated with new configuration");
            }
        }
        
        Ok(())
    }
    
    async fn handle_build_operation(&self, operation: BuildOperation) -> Result<()> {
        match operation {
            BuildOperation::Build { project, release } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("🔨 Building project...");
                
                let manager = ProjectManager::new(&project_path);
                let result = manager.build().await?;
                
                info!("✅ Build completed successfully!");
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            BuildOperation::Run { project, release, args } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("🚀 Running project...");
                
                let manager = ProjectManager::new(&project_path);
                let result = manager.run().await?;
                
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            BuildOperation::Check { project } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("🔍 Checking code...");
                
                let result = build_utils::cargo_check(&project_path)?;
                println!("{}", result);
            }
            BuildOperation::Clippy { project } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("📎 Running clippy...");
                
                let result = build_utils::cargo_clippy(&project_path)?;
                println!("{}", result);
            }
            BuildOperation::Format { project } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("🎨 Formatting code...");
                
                let result = build_utils::cargo_fmt(&project_path)?;
                info!("{}", result);
            }
            BuildOperation::Test { project } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("🧪 Running tests...");
                
                // TODO: Implement test running
                info!("Test runner not yet implemented");
            }
        }
        
        Ok(())
    }
    
    async fn handle_project_operation(&self, operation: ProjectOperation) -> Result<()> {
        match operation {
            ProjectOperation::Info { project } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                let mut manager = ProjectManager::new(&project_path);
                manager.load().await?;
                
                if let Some(config) = manager.config() {
                    println!("📋 Project Information:");
                    println!("Name: {}", config.metadata.name);
                    println!("Description: {}", config.metadata.description);
                    println!("Version: {}", config.metadata.version);
                    println!("Created: {}", config.metadata.created_at.format("%Y-%m-%d %H:%M:%S"));
                    println!("Updated: {}", config.metadata.updated_at.format("%Y-%m-%d %H:%M:%S"));
                    println!("Bevy Version: {}", config.metadata.bevy_version);
                    println!("Features: {}", config.metadata.features.join(", "));
                    println!("Conversations: {}", config.conversations.len());
                    println!("Generated Files: {}", config.generated_files.len());
                }
            }
            ProjectOperation::Stats { project } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                let manager = ProjectManager::new(&project_path);
                let stats = manager.stats().await?;
                
                println!("📊 Project Statistics:");
                println!("Lines of Code: {}", stats.lines_of_code);
                println!("Rust Files: {}", stats.rust_files);
                println!("AI Conversations: {}", stats.conversations);
                println!("Generated Files: {}", stats.generated_files);
                println!("Dependencies: {}", stats.dependencies);
                println!("Features: {}", stats.features);
            }
            ProjectOperation::History { project, limit } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                let mut manager = ProjectManager::new(&project_path);
                manager.load().await?;
                
                if let Some(config) = manager.config() {
                    println!("📜 Conversation History (last {}):", limit);
                    for conversation in config.conversations.iter().rev().take(limit) {
                        println!("\n🕐 {} ({})", 
                               conversation.timestamp.format("%Y-%m-%d %H:%M:%S"),
                               conversation.model_used);
                        println!("👤 Request: {}", conversation.request);
                        println!("🤖 Response: {}...", 
                               conversation.response.chars().take(100).collect::<String>());
                        if let Some(tokens) = conversation.tokens_used {
                            println!("🔢 Tokens: {}", tokens);
                        }
                    }
                }
            }
            ProjectOperation::Export { output, project, format } => {
                // TODO: Implement project export
                info!("Project export not yet implemented");
            }
            ProjectOperation::Clean { project, cargo } => {
                let project_path = project.unwrap_or_else(|| std::env::current_dir().unwrap());
                info!("🧹 Cleaning project...");
                
                if cargo {
                    std::process::Command::new("cargo")
                        .arg("clean")
                        .current_dir(&project_path)
                        .output()?;
                    info!("✅ Cargo artifacts cleaned");
                }
                
                info!("✅ Project cleaned");
            }
        }
        
        Ok(())
    }
    
    async fn handle_template_operation(&self, operation: TemplateOperation) -> Result<()> {
        match operation {
            TemplateOperation::List => {
                let manager = TemplateManager::new()?;
                let templates = TemplateManager::builtin_templates();
                
                println!("📝 Available Templates:");
                for template in templates {
                    println!("  {} - {} ({})", 
                           template.name, 
                           template.description,
                           template.category);
                }
            }
            TemplateOperation::Show { name } => {
                let templates = TemplateManager::builtin_templates();
                if let Some(template) = templates.iter().find(|t| t.name == name) {
                    println!("📝 Template: {}", template.name);
                    println!("Description: {}", template.description);
                    println!("Category: {}", template.category);
                    println!("Dependencies: {}", template.dependencies.join(", "));
                    println!("Features: {}", template.features.join(", "));
                } else {
                    error!("Template '{}' not found", name);
                }
            }
            TemplateOperation::Create { name, description, file } => {
                // TODO: Implement custom template creation
                info!("Custom template creation not yet implemented");
            }
            TemplateOperation::Apply { name, project } => {
                // TODO: Implement template application
                info!("Template application not yet implemented");
            }
        }
        
        Ok(())
    }
    
    fn generate_project_name(&self, description: &str) -> String {
        description
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join("_")
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }
}
