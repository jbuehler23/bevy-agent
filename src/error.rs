//! Error types and utilities for Bevy AI

use thiserror::Error;

/// Result type alias for Bevy AI operations
pub type Result<T> = std::result::Result<T, BevyAIError>;

/// Main error type for Bevy AI operations
#[derive(Error, Debug)]
pub enum BevyAIError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    /// HTTP request errors
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    /// JSON serialization/deserialization errors  
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Input/output errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// File system traversal errors
    #[error("File traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
    
    /// Template rendering errors
    #[error("Template rendering error: {0}")]
    Template(#[from] handlebars::RenderError),
    
    /// Template creation errors
    #[error("Template creation error: {0}")]
    TemplateCreation(#[from] handlebars::TemplateError),
    
    /// Code parsing errors
    #[error("Code parsing error: {0}")]
    CodeParsing(String),
    
    /// AI API communication errors
    #[error("AI API error: {message}")]
    AIApi { 
        /// The error message from the AI API
        message: String 
    },
    
    /// Missing API key errors
    #[error("API key not configured for {provider}")]
    MissingApiKey { 
        /// The AI provider that requires an API key
        provider: String 
    },
    
    /// Unsupported model errors
    #[error("Unsupported AI model: {model}")]
    UnsupportedModel { 
        /// The unsupported model name
        model: String 
    },
    
    /// Project not found errors
    #[error("Project not found at path: {path}")]
    ProjectNotFound { 
        /// The path where the project was expected
        path: String 
    },
    
    /// Invalid project structure errors
    #[error("Invalid project structure: {reason}")]
    InvalidProject { 
        /// The reason why the project is invalid
        reason: String 
    },
    
    /// Feature generation errors
    #[error("Feature generation failed: {reason}")]
    FeatureGeneration { 
        /// The reason why feature generation failed
        reason: String 
    },
    
    /// Code optimization errors
    #[error("Code optimization failed: {reason}")]
    CodeOptimization { 
        /// The reason why code optimization failed
        reason: String 
    },
    
    /// File operation errors
    #[error("File operation failed: {operation} on {path}")]
    FileOperation { 
        /// The operation that failed
        operation: String, 
        /// The path where the operation failed
        path: String 
    },
    
    /// Template not found errors
    #[error("Template not found: {name}")]
    TemplateNotFound { 
        /// The name of the template that wasn't found
        name: String 
    },
    
    /// Dependency resolution errors
    #[error("Dependency resolution failed: {dependency}")]
    DependencyResolution { 
        /// The dependency that couldn't be resolved
        dependency: String 
    },
    
    /// Build system errors
    #[error("Build system error: {message}")]
    BuildSystem { 
        /// The build system error message
        message: String 
    },
    
    /// Validation errors
    #[error("Validation error: {message}")]
    Validation { 
        /// The validation error message
        message: String 
    },
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
