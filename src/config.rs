//! Configuration management for Bevy AI
//! 
//! This module handles all configuration aspects of the Bevy AI system, including:
//! 
//! - AI provider configurations (OpenAI, Anthropic, Google)
//! - Project-specific settings
//! - Environment variable handling
//! - Configuration file parsing (TOML, JSON)
//! 
//! # Configuration Sources
//! 
//! Configuration can be loaded from multiple sources in order of preference:
//! 
//! 1. Environment variables (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, etc.)
//! 2. Configuration files (`~/.config/bevy-ai/config.toml`)
//! 3. Project-specific config files (`./bevy-ai.toml`)
//! 4. Default values
//! 
//! # Example
//! 
//! ```rust,no_run
//! use bevy_ai::config::{AIConfig, OpenAIConfig};
//! 
//! // Load from environment
//! let config = AIConfig::from_env().unwrap_or_default();
//! 
//! // Or create manually
//! let config = AIConfig {
//!     openai: Some(OpenAIConfig {
//!         api_key: "your-key-here".to_string(),
//!         organization: None,
//!         base_url: None,
//!     }),
//!     ..Default::default()
//! };
//! ```

use crate::error::{BevyAIError, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Main configuration for the Bevy AI system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// OpenAI API configuration
    pub openai: Option<OpenAIConfig>,
    /// Anthropic API configuration
    pub anthropic: Option<AnthropicConfig>,
    /// Google API configuration
    pub google: Option<GoogleConfig>,
    /// Default AI model to use
    pub default_model: ModelType,
    /// Default generation settings
    pub generation: GenerationConfig,
    /// Project settings
    pub project: ProjectSettings,
}

/// OpenAI API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub organization: Option<String>,
    pub base_url: Option<String>,
}

/// Anthropic API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub base_url: Option<String>,
}

/// Google API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    pub api_key: String,
    pub base_url: Option<String>,
}

/// Available AI models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelType {
    #[serde(rename = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
    #[serde(rename = "gpt-3.5-turbo")]
    GPT35Turbo,
    #[serde(rename = "claude-3-opus")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku")]
    Claude3Haiku,
    #[serde(rename = "gemini-pro")]
    GeminiPro,
    #[serde(rename = "gemini-pro-vision")]
    GeminiProVision,
}

impl ModelType {
    /// Get the string representation of the model
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelType::GPT4 => "gpt-4",
            ModelType::GPT4Turbo => "gpt-4-turbo",
            ModelType::GPT35Turbo => "gpt-3.5-turbo",
            ModelType::Claude3Opus => "claude-3-opus",
            ModelType::Claude3Sonnet => "claude-3-sonnet",
            ModelType::Claude3Haiku => "claude-3-haiku",
            ModelType::GeminiPro => "gemini-pro",
            ModelType::GeminiProVision => "gemini-pro-vision",
        }
    }
    
    /// Get the provider for this model
    pub fn provider(&self) -> &'static str {
        match self {
            ModelType::GPT4 | ModelType::GPT4Turbo | ModelType::GPT35Turbo => "openai",
            ModelType::Claude3Opus | ModelType::Claude3Sonnet | ModelType::Claude3Haiku => "anthropic",
            ModelType::GeminiPro | ModelType::GeminiProVision => "google",
        }
    }
    
    /// Check if this model supports vision/image inputs
    pub fn supports_vision(&self) -> bool {
        matches!(self, ModelType::GPT4 | ModelType::GeminiProVision)
    }
    
    /// Get maximum context length for this model
    pub fn max_context_length(&self) -> usize {
        match self {
            ModelType::GPT4 => 8192,
            ModelType::GPT4Turbo => 128000,
            ModelType::GPT35Turbo => 16385,
            ModelType::Claude3Opus => 200000,
            ModelType::Claude3Sonnet => 200000,
            ModelType::Claude3Haiku => 200000,
            ModelType::GeminiPro => 32768,
            ModelType::GeminiProVision => 16384,
        }
    }
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ModelType {
    type Err = BevyAIError;
    
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "gpt-4" => Ok(ModelType::GPT4),
            "gpt-4-turbo" => Ok(ModelType::GPT4Turbo),
            "gpt-3.5-turbo" => Ok(ModelType::GPT35Turbo),
            "claude-3-opus" => Ok(ModelType::Claude3Opus),
            "claude-3-sonnet" => Ok(ModelType::Claude3Sonnet),
            "claude-3-haiku" => Ok(ModelType::Claude3Haiku),
            "gemini-pro" => Ok(ModelType::GeminiPro),
            "gemini-pro-vision" => Ok(ModelType::GeminiProVision),
            _ => Err(BevyAIError::unsupported_model(s)),
        }
    }
}

/// Code generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Temperature for AI generation (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// Whether to include explanatory comments
    pub include_comments: bool,
    /// Whether to generate tests
    pub generate_tests: bool,
    /// Bevy version to target
    pub bevy_version: String,
    /// Rust edition to use
    pub rust_edition: String,
}

/// Project-level settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Whether to track AI conversations
    pub track_conversations: bool,
    /// Whether to auto-format generated code
    pub auto_format: bool,
    /// Whether to auto-detect dependencies
    pub auto_dependencies: bool,
    /// Default project template
    pub default_template: String,
}

/// Project configuration stored in .bevy-ai.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project metadata
    pub metadata: ProjectMetadata,
    /// AI conversation history
    pub conversations: Vec<ConversationEntry>,
    /// Generated files tracking
    pub generated_files: Vec<GeneratedFile>,
    /// Project dependencies
    pub dependencies: Vec<Dependency>,
    /// Custom templates
    pub templates: Vec<CustomTemplate>,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub bevy_version: String,
    pub features: Vec<String>,
    pub tags: Vec<String>,
}

/// Conversation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub id: uuid::Uuid,
    pub request: String,
    pub response: String,
    pub model_used: ModelType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tokens_used: Option<u32>,
    pub cost: Option<f64>,
    pub files_modified: Vec<String>,
}

/// Generated file tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: String,
    pub generator: String,
    pub model: ModelType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub checksum: String,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
    pub reason: String,
    pub added_by: ModelType,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Custom template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTemplate {
    pub name: String,
    pub description: String,
    pub template_path: String,
    pub variables: Vec<TemplateVariable>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            openai: None,
            anthropic: None,
            google: None,
            default_model: ModelType::GPT4,
            generation: GenerationConfig::default(),
            project: ProjectSettings::default(),
        }
    }
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 4000,
            include_comments: true,
            generate_tests: false,
            bevy_version: "0.12".to_string(),
            rust_edition: "2021".to_string(),
        }
    }
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            track_conversations: true,
            auto_format: true,
            auto_dependencies: true,
            default_template: "basic".to_string(),
        }
    }
}

impl AIConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();
        
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            config.openai = Some(OpenAIConfig {
                api_key,
                organization: env::var("OPENAI_ORGANIZATION").ok(),
                base_url: env::var("OPENAI_BASE_URL").ok(),
            });
        }
        
        if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
            config.anthropic = Some(AnthropicConfig {
                api_key,
                base_url: env::var("ANTHROPIC_BASE_URL").ok(),
            });
        }
        
        if let Ok(api_key) = env::var("GOOGLE_API_KEY") {
            config.google = Some(GoogleConfig {
                api_key,
                base_url: env::var("GOOGLE_BASE_URL").ok(),
            });
        }
        
        if let Ok(model) = env::var("BEVY_AI_DEFAULT_MODEL") {
            config.default_model = model.parse()?;
        }
        
        Ok(config)
    }
    
    /// Load configuration from file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
    
    /// Save configuration to file
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// Get the default config file path
    pub fn default_config_path() -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .ok_or_else(|| BevyAIError::Config(config::ConfigError::Message("Could not find home directory".to_string())))?
            .join(".bevy-ai-config.json"))
    }
    
    /// Load configuration from default location or create if not exists
    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::default_config_path()?;
        
        if config_path.exists() {
            Self::from_file(&config_path)
        } else {
            let config = Self::from_env()?;
            config.save_to_file(&config_path)?;
            Ok(config)
        }
    }
    
    /// Get API key for a specific model
    pub fn get_api_key(&self, model: &ModelType) -> Result<String> {
        match model.provider() {
            "openai" => self.openai.as_ref()
                .map(|c| c.api_key.clone())
                .ok_or_else(|| BevyAIError::missing_api_key("OpenAI")),
            "anthropic" => self.anthropic.as_ref()
                .map(|c| c.api_key.clone())
                .ok_or_else(|| BevyAIError::missing_api_key("Anthropic")),
            "google" => self.google.as_ref()
                .map(|c| c.api_key.clone())
                .ok_or_else(|| BevyAIError::missing_api_key("Google")),
            provider => Err(BevyAIError::unsupported_model(provider)),
        }
    }
    
    /// Check if a model is available (has API key configured)
    pub fn is_model_available(&self, model: &ModelType) -> bool {
        self.get_api_key(model).is_ok()
    }
    
    /// Get list of available models
    pub fn available_models(&self) -> Vec<ModelType> {
        let all_models = vec![
            ModelType::GPT4,
            ModelType::GPT4Turbo,
            ModelType::GPT35Turbo,
            ModelType::Claude3Opus,
            ModelType::Claude3Sonnet,
            ModelType::Claude3Haiku,
            ModelType::GeminiPro,
            ModelType::GeminiProVision,
        ];
        
        all_models.into_iter()
            .filter(|model| self.is_model_available(model))
            .collect()
    }
}
