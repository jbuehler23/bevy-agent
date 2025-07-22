//! Error types and utilities for Bevy AI

use thiserror::Error;

/// Result type alias for Bevy AI operations
pub type Result<T> = std::result::Result<T, BevyAIError>;

/// Main error type for Bevy AI operations
#[derive(Error, Debug)]
pub enum BevyAIError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("File traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
    
    #[error("Template rendering error: {0}")]
    Template(#[from] handlebars::RenderError),
    
    #[error("Template creation error: {0}")]
    TemplateCreation(#[from] handlebars::TemplateError),
    
    #[error("Code parsing error: {0}")]
    CodeParsing(String),
    
    #[error("AI API error: {message}")]
    AIApi { message: String },
    
    #[error("API key not configured for {provider}")]
    MissingApiKey { provider: String },
    
    #[error("Unsupported AI model: {model}")]
    UnsupportedModel { model: String },
    
    #[error("Project not found at path: {path}")]
    ProjectNotFound { path: String },
    
    #[error("Invalid project structure: {reason}")]
    InvalidProject { reason: String },
    
    #[error("Feature generation failed: {reason}")]
    FeatureGeneration { reason: String },
    
    #[error("Code optimization failed: {reason}")]
    CodeOptimization { reason: String },
    
    #[error("File operation failed: {operation} on {path}")]
    FileOperation { operation: String, path: String },
    
    #[error("Template not found: {name}")]
    TemplateNotFound { name: String },
    
    #[error("Dependency resolution failed: {dependency}")]
    DependencyResolution { dependency: String },
    
    #[error("Build system error: {message}")]
    BuildSystem { message: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
}

impl BevyAIError {
    /// Create a new AI API error
    pub fn ai_api<S: Into<String>>(message: S) -> Self {
        Self::AIApi {
            message: message.into(),
        }
    }
    
    /// Create a new missing API key error
    pub fn missing_api_key<S: Into<String>>(provider: S) -> Self {
        Self::MissingApiKey {
            provider: provider.into(),
        }
    }
    
    /// Create a new unsupported model error
    pub fn unsupported_model<S: Into<String>>(model: S) -> Self {
        Self::UnsupportedModel {
            model: model.into(),
        }
    }
    
    /// Create a new project not found error
    pub fn project_not_found<S: Into<String>>(path: S) -> Self {
        Self::ProjectNotFound {
            path: path.into(),
        }
    }
    
    /// Create a new invalid project error
    pub fn invalid_project<S: Into<String>>(reason: S) -> Self {
        Self::InvalidProject {
            reason: reason.into(),
        }
    }
    
    /// Create a new feature generation error
    pub fn feature_generation<S: Into<String>>(reason: S) -> Self {
        Self::FeatureGeneration {
            reason: reason.into(),
        }
    }
    
    /// Create a new code optimization error
    pub fn code_optimization<S: Into<String>>(reason: S) -> Self {
        Self::CodeOptimization {
            reason: reason.into(),
        }
    }
    
    /// Create a new file operation error
    pub fn file_operation<S: Into<String>>(operation: S, path: S) -> Self {
        Self::FileOperation {
            operation: operation.into(),
            path: path.into(),
        }
    }
    
    /// Create a new template not found error
    pub fn template_not_found<S: Into<String>>(name: S) -> Self {
        Self::TemplateNotFound {
            name: name.into(),
        }
    }
    
    /// Create a new dependency resolution error
    pub fn dependency_resolution<S: Into<String>>(dependency: S) -> Self {
        Self::DependencyResolution {
            dependency: dependency.into(),
        }
    }
    
    /// Create a new build system error
    pub fn build_system<S: Into<String>>(message: S) -> Self {
        Self::BuildSystem {
            message: message.into(),
        }
    }
    
    /// Create a new validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}
