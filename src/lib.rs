//! # Bevy AI - AI-Powered Game Development Assistant
//! 
//! Bevy AI is a comprehensive library and CLI tool that leverages cutting-edge AI models
//! (GPT-4, Claude-3, Gemini) to accelerate Bevy game development through natural language
//! code generation, intelligent feature addition, and automated code optimization.
//! 
//! ## Features
//! 
//! - **Natural Language Game Creation**: Describe your game in plain English and get working Bevy code
//! - **Intelligent Feature Addition**: Add complex systems to existing games with AI assistance
//! - **Code Analysis & Optimization**: AI-powered code review and performance improvements
//! - **Multi-Model Support**: Choose from GPT-4, Claude-3, or Gemini for different tasks
//! - **Context-Aware Generation**: Understands existing codebase and maintains consistency
//! - **Smart Dependency Management**: Automatically detects and manages Cargo dependencies
//! - **Built-in Game Templates**: 5 pre-built game templates for rapid prototyping
//! - **Project Management**: Complete project lifecycle management with git integration
//! 
//! ## Installation
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! bevy-ai = "0.1"
//! bevy = "0.14"  # Optional, for Bevy integration features
//! tokio = { version = "1.0", features = ["full"] }  # For async support
//! ```
//! 
//! ## CLI Installation
//! 
//! ```bash
//! # Install the CLI tool
//! cargo install bevy-ai
//! 
//! # Configure API keys
//! bevy-ai config --openai-key sk-...
//! 
//! # Create a new game
//! bevy-ai create "2D platformer with physics and collectibles"
//! 
//! # Add features to existing games
//! bevy-ai add "inventory system with drag-and-drop UI"
//! ```
//! 
//! ## Library Usage
//! 
//! ### Basic AI Agent Usage
//! 
//! ```rust,no_run
//! use bevy_ai::{BevyAIAgent, AIConfig};
//! use bevy_ai::config::OpenAIConfig;
//! 
//! #[tokio::main]
//! async fn main() -> bevy_ai::Result<()> {
//!     // Configure AI provider
//!     let config = AIConfig {
//!         openai: Some(OpenAIConfig {
//!             api_key: "your-openai-key".to_string(),
//!             organization: None,
//!             base_url: None,
//!         }),
//!         ..Default::default()
//!     };
//!     
//!     let agent = BevyAIAgent::new(config).await?;
//!     
//!     // Generate game code
//!     let response = agent
//!         .request("Create a simple 2D shooter game")
//!         .with_system_prompt("You are a Bevy game engine expert")
//!         .with_max_tokens(2000)
//!         .execute()
//!         .await?;
//!     
//!     println!("Generated code:\n{}", response.content);
//!     Ok(())
//! }
//! ```
//! 
//! ### Using Game Templates
//! 
//! ```rust,no_run
//! use bevy_ai::game_templates::{TemplateManager, TemplateContext};
//! 
//! #[tokio::main]
//! async fn main() -> bevy_ai::Result<()> {
//!     let manager = TemplateManager::new()?;
//!     
//!     // List available templates
//!     for template in manager.available_templates() {
//!         println!("Available template: {}", template);
//!     }
//!     
//!     // Generate from template
//!     let context = TemplateContext::new(
//!         "MyGame".to_string(),
//!         "A 2D platformer game".to_string()
//!     );
//!     
//!     let code = manager.generate("platformer_2d", &context)?;
//!     println!("Generated game code:\n{}", code);
//!     Ok(())
//! }
//! ```
//! 
//! ## Available AI Models
//! 
//! - **OpenAI**: GPT-4, GPT-3.5-turbo
//! - **Anthropic**: Claude-3 Opus, Sonnet, Haiku
//! - **Google**: Gemini Pro, Gemini Pro Vision
//! 
//! ## Game Templates
//! 
//! The library includes 5 built-in game templates:
//! 
//! - `basic_game`: Simple 3D scene with camera and lighting
//! - `platformer_2d`: 2D platformer with physics and collectibles
//! - `fps_3d`: 3D first-person shooter with basic enemies
//! - `puzzle_game`: Grid-based puzzle game with level progression
//! - `strategy_game`: Real-time strategy with units and resources
//! 
//! ## Configuration
//! 
//! The library supports multiple configuration methods:
//! 
//! 1. **Environment Variables**: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GOOGLE_API_KEY`
//! 2. **Configuration Files**: TOML, JSON, YAML formats
//! 3. **Programmatic**: Direct configuration in code
//! 
//! ## Error Handling
//! 
//! All functions return `Result<T, BevyAIError>` for comprehensive error handling:
//! 
//! ```rust,no_run
//! use bevy_ai::{BevyAIError, Result};
//! 
//! fn handle_errors() -> Result<()> {
//!     match some_ai_operation() {
//!         Ok(result) => println!("Success: {}", result),
//!         Err(BevyAIError::Http(e)) => eprintln!("Network error: {}", e),
//!         Err(BevyAIError::Json(e)) => eprintln!("JSON error: {}", e),
//!         Err(e) => eprintln!("Other error: {}", e),
//!     }
//!     Ok(())
//! }
//! 
//! # fn some_ai_operation() -> Result<String> { Ok("test".to_string()) }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod ai;
pub mod cli;
pub mod config;
pub mod error;
pub mod game_templates;
pub mod project;
pub mod utils;

// Re-exports for convenience
pub use ai::{BevyAIAgent, ModelType};
pub use config::{AIConfig, ProjectConfig};
pub use error::{BevyAIError, Result};
pub use project::{Project, ProjectManager};

/// Current version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User-Agent string for HTTP requests
pub const USER_AGENT: &str = concat!("bevy-ai/", env!("CARGO_PKG_VERSION"));
