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
//! 2. Configuration files (`~/.config/bevy-agent/config.toml`)
//! 3. Project-specific config files (`./bevy-agent.toml`)
//! 4. Default values
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy_agent::config::{AIConfig, OpenAIConfig};
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
    /// OpenAI API key
    pub api_key: String,
    /// Optional organization ID
    pub organization: Option<String>,
    /// Optional custom base URL
    pub base_url: Option<String>,
}

/// Anthropic API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    /// Anthropic API key
    pub api_key: String,
    /// Optional custom base URL
    pub base_url: Option<String>,
}

/// Google API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    /// Google API key
    pub api_key: String,
    /// Optional custom base URL
    pub base_url: Option<String>,
}

/// Available AI models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelType {
    /// OpenAI GPT-4 model
    #[serde(rename = "gpt-4")]
    GPT4,
    /// OpenAI GPT-4 Turbo model
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
    /// OpenAI GPT-3.5 Turbo model
    #[serde(rename = "gpt-3.5-turbo")]
    GPT35Turbo,
    /// Anthropic Claude 3 Opus model
    #[serde(rename = "claude-3-opus")]
    Claude3Opus,
    /// Anthropic Claude 3 Sonnet model
    #[serde(rename = "claude-3-sonnet")]
    Claude3Sonnet,
    /// Anthropic Claude 3 Haiku model
    #[serde(rename = "claude-3-haiku")]
    Claude3Haiku,
    /// Google Gemini Pro model
    #[serde(rename = "gemini-pro")]
    GeminiPro,
    /// Google Gemini Pro Vision model
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
            ModelType::Claude3Opus | ModelType::Claude3Sonnet | ModelType::Claude3Haiku => {
                "anthropic"
            }
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

/// Project configuration stored in .bevy-agent.json
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
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project version
    pub version: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Bevy engine version used
    pub bevy_version: String,
    /// Enabled features
    pub features: Vec<String>,
    /// Project tags
    pub tags: Vec<String>,
}

/// Conversation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    /// Unique identifier for this conversation
    pub id: uuid::Uuid,
    /// The user's request/prompt
    pub request: String,
    /// The AI's response
    pub response: String,
    /// The AI model that was used
    pub model_used: ModelType,
    /// When this conversation occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Number of tokens used (if available)
    pub tokens_used: Option<u32>,
    /// Cost of the API call (if available)
    pub cost: Option<f64>,
    /// List of files that were modified in this conversation
    pub files_modified: Vec<String>,
}

/// Generated file tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// Path to the generated file
    pub path: String,
    /// What generated this file (e.g., "AI", "template")
    pub generator: String,
    /// The AI model that generated this file
    pub model: ModelType,
    /// When this file was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// File checksum for integrity verification
    pub checksum: String,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Name of the dependency
    pub name: String,
    /// Version requirement for the dependency
    pub version: String,
    /// Features to enable for this dependency
    pub features: Vec<String>,
    /// Reason why this dependency was added
    pub reason: String,
    /// Which AI model added this dependency
    pub added_by: ModelType,
    /// When this dependency was added
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Custom template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTemplate {
    /// Name of the template
    pub name: String,
    /// Description of what this template does
    pub description: String,
    /// Path to the template file
    pub template_path: String,
    /// Variables that can be customized in this template
    pub variables: Vec<TemplateVariable>,
    /// When this template was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Name of the variable
    pub name: String,
    /// Description of what this variable is for
    pub description: String,
    /// Default value for this variable (if any)
    pub default_value: Option<String>,
    /// Whether this variable is required
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

        if let Ok(model) = env::var("bevy_agent_DEFAULT_MODEL") {
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
            .ok_or_else(|| {
                BevyAIError::Config(config::ConfigError::Message(
                    "Could not find home directory".to_string(),
                ))
            })?
            .join(".bevy-agent-config.json"))
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
            "openai" => self
                .openai
                .as_ref()
                .map(|c| c.api_key.clone())
                .ok_or_else(|| BevyAIError::missing_api_key("OpenAI")),
            "anthropic" => self
                .anthropic
                .as_ref()
                .map(|c| c.api_key.clone())
                .ok_or_else(|| BevyAIError::missing_api_key("Anthropic")),
            "google" => self
                .google
                .as_ref()
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

        all_models
            .into_iter()
            .filter(|model| self.is_model_available(model))
            .collect()
    }
}
