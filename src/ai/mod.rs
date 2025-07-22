//! AI integration and model handling for Bevy AI
//! 
//! This module provides the core AI functionality including:
//! 
//! - [`BevyAIAgent`]: Main interface for AI interactions
//! - [`AIRequest`]: Builder pattern for constructing AI requests  
//! - [`AIResponse`]: Response handling and parsing
//! - [`ModelType`]: Enumeration of supported AI models
//! 
//! # Example
//! 
//! ```rust,no_run
//! use bevy_ai::ai::{BevyAIAgent, ModelType};
//! use bevy_ai::config::AIConfig;
//! 
//! # async fn example() -> bevy_ai::Result<()> {
//! let config = AIConfig::default();
//! let agent = BevyAIAgent::new(config).await?;
//! 
//! let response = agent
//!     .request("Create a simple Bevy game")
//!     .with_max_tokens(1000)
//!     .execute()
//!     .await?;
//! 
//! println!("Generated code: {}", response.content);
//! # Ok(())
//! # }
//! ```

pub mod agent;
pub mod models;
pub mod prompts;

pub use agent::{BevyAIAgent, AIRequest, AIResponse};
pub use models::ModelType;
