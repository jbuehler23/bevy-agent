//! AI agent implementation for Bevy AI

use crate::config::{AIConfig, ModelType};
use crate::error::{BevyAIError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

/// Main AI agent for Bevy game development
pub struct BevyAIAgent {
    client: Client,
    config: AIConfig,
}

/// Request builder for AI operations
pub struct AIRequest {
    agent: BevyAIAgent,
    prompt: String,
    model: Option<ModelType>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    context: Vec<String>,
    system_prompt: Option<String>,
}

/// Response from AI models
#[derive(Debug, Clone)]
pub struct AIResponse {
    /// The generated content from the AI model
    pub content: String,
    /// The model that generated this response
    pub model: ModelType,
    /// Number of tokens used in the request (if available)
    pub tokens_used: Option<u32>,
    /// Reason why the generation finished (if available)
    pub finish_reason: Option<String>,
    /// Unique identifier for this conversation
    pub conversation_id: uuid::Uuid,
}

/// OpenAI API structures
#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIUsage {
    total_tokens: u32,
}

/// Anthropic API structures
#[derive(Serialize, Deserialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    usage: Option<AnthropicUsage>,
    stop_reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

/// Google API structures
#[derive(Serialize, Deserialize)]
struct GoogleRequest {
    contents: Vec<GoogleContent>,
    generation_config: GoogleGenerationConfig,
}

#[derive(Serialize, Deserialize)]
struct GoogleContent {
    parts: Vec<GooglePart>,
}

#[derive(Serialize, Deserialize)]
struct GooglePart {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct GoogleGenerationConfig {
    temperature: Option<f32>,
    max_output_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct GoogleResponse {
    candidates: Vec<GoogleCandidate>,
    usage_metadata: Option<GoogleUsage>,
}

#[derive(Serialize, Deserialize)]
struct GoogleCandidate {
    content: GoogleContent,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GoogleUsage {
    total_token_count: u32,
}

impl BevyAIAgent {
    /// Create a new AI agent with the given configuration
    pub async fn new(config: AIConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .user_agent(crate::USER_AGENT)
            .build()?;

        Ok(Self { client, config })
    }

    /// Create a new AI request
    pub fn request<S: Into<String>>(&self, prompt: S) -> AIRequest {
        AIRequest {
            agent: self.clone(),
            prompt: prompt.into(),
            model: None,
            temperature: None,
            max_tokens: None,
            context: Vec::new(),
            system_prompt: None,
        }
    }

    /// Generate a complete Bevy game from description
    pub fn generate_game<S: Into<String>>(&self, description: S) -> AIRequest {
        let system_prompt = crate::ai::prompts::GAME_GENERATION_PROMPT;

        self.request(description)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Add a feature to existing game code
    pub fn add_feature<S: Into<String>>(
        &self,
        feature_description: S,
        existing_code: S,
    ) -> AIRequest {
        let system_prompt = crate::ai::prompts::FEATURE_ADDITION_PROMPT;
        let prompt = format!(
            "Add this feature: {}\n\nExisting code:\n```rust\n{}\n```",
            feature_description.into(),
            existing_code.into()
        );

        self.request(prompt)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Improve existing code
    pub fn improve_code<S: Into<String>>(&self, aspect: S, code: S) -> AIRequest {
        let system_prompt = crate::ai::prompts::CODE_IMPROVEMENT_PROMPT;
        let prompt = format!(
            "Improve the {} of this code:\n\n```rust\n{}\n```",
            aspect.into(),
            code.into()
        );

        self.request(prompt)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Explain existing code
    pub fn explain_code<S: Into<String>>(&self, code: S) -> AIRequest {
        let system_prompt = crate::ai::prompts::CODE_EXPLANATION_PROMPT;
        let prompt = format!("Explain this Bevy code:\n\n```rust\n{}\n```", code.into());

        self.request(prompt)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Debug code issues
    pub fn debug_code<S: Into<String>>(&self, code: S, error_message: S) -> AIRequest {
        let system_prompt = crate::ai::prompts::CODE_DEBUGGING_PROMPT;
        let prompt = format!(
            "Debug this Bevy code:\n\n```rust\n{}\n```\n\nError: {}",
            code.into(),
            error_message.into()
        );

        self.request(prompt)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Optimize code for performance
    pub fn optimize_performance<S: Into<String>>(&self, code: S) -> AIRequest {
        let system_prompt = crate::ai::prompts::PERFORMANCE_OPTIMIZATION_PROMPT;

        self.request(code)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Generate unit tests for code
    pub fn generate_tests<S: Into<String>>(&self, code: S) -> AIRequest {
        let system_prompt = crate::ai::prompts::TEST_GENERATION_PROMPT;

        self.request(code)
            .with_system_prompt(system_prompt)
            .with_model(self.config.default_model.clone())
    }

    /// Extract Rust code from AI response
    pub fn extract_code(&self, response: &str) -> String {
        // Look for rust code blocks
        if let Some(start) = response.find("```rust") {
            let code_start = start + 7;
            if let Some(end) = response[code_start..].find("```") {
                return response[code_start..code_start + end].trim().to_string();
            }
        }

        // Look for any code blocks
        if let Some(start) = response.find("```") {
            let code_start = start + 3;
            if let Some(newline) = response[code_start..].find('\n') {
                let actual_start = code_start + newline + 1;
                if let Some(end) = response[actual_start..].find("```") {
                    return response[actual_start..actual_start + end]
                        .trim()
                        .to_string();
                }
            }
        }

        // If no code blocks found, return the entire response
        response.trim().to_string()
    }

    /// Execute OpenAI API call
    async fn call_openai(&self, request: &OpenAIRequest, model: &ModelType) -> Result<AIResponse> {
        let api_key = self.config.get_api_key(model)?;

        let base_url = self
            .config
            .openai
            .as_ref()
            .and_then(|c| c.base_url.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("https://api.openai.com");

        let url = format!("{base_url}/v1/chat/completions");

        debug!("Making OpenAI API call to: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(BevyAIError::ai_api(format!(
                "OpenAI API error ({status}): {error_text}"
            )));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let choice = openai_response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| BevyAIError::ai_api("No response from OpenAI"))?;

        Ok(AIResponse {
            content: choice.message.content,
            model: model.clone(),
            tokens_used: openai_response.usage.map(|u| u.total_tokens),
            finish_reason: choice.finish_reason,
            conversation_id: uuid::Uuid::new_v4(),
        })
    }

    /// Execute Anthropic API call
    async fn call_anthropic(
        &self,
        request: &AnthropicRequest,
        model: &ModelType,
    ) -> Result<AIResponse> {
        let api_key = self.config.get_api_key(model)?;

        let base_url = self
            .config
            .anthropic
            .as_ref()
            .and_then(|c| c.base_url.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("https://api.anthropic.com");

        let url = format!("{base_url}/v1/messages");

        debug!("Making Anthropic API call to: {}", url);

        let response = self
            .client
            .post(&url)
            .header("x-api-key", api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(BevyAIError::ai_api(format!(
                "Anthropic API error ({status}): {error_text}"
            )));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        let content = anthropic_response
            .content
            .into_iter()
            .next()
            .ok_or_else(|| BevyAIError::ai_api("No response from Anthropic"))?;

        let tokens_used = anthropic_response
            .usage
            .map(|u| u.input_tokens + u.output_tokens);

        Ok(AIResponse {
            content: content.text,
            model: model.clone(),
            tokens_used,
            finish_reason: anthropic_response.stop_reason,
            conversation_id: uuid::Uuid::new_v4(),
        })
    }

    /// Execute Google API call
    async fn call_google(&self, request: &GoogleRequest, model: &ModelType) -> Result<AIResponse> {
        let api_key = self.config.get_api_key(model)?;

        let base_url = self
            .config
            .google
            .as_ref()
            .and_then(|c| c.base_url.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("https://generativelanguage.googleapis.com");

        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            base_url,
            model.as_str(),
            api_key
        );

        debug!("Making Google API call to: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(BevyAIError::ai_api(format!(
                "Google API error ({status}): {error_text}"
            )));
        }

        let google_response: GoogleResponse = response.json().await?;

        let candidate = google_response
            .candidates
            .into_iter()
            .next()
            .ok_or_else(|| BevyAIError::ai_api("No response from Google"))?;

        let content = candidate
            .content
            .parts
            .into_iter()
            .next()
            .ok_or_else(|| BevyAIError::ai_api("No content in Google response"))?;

        Ok(AIResponse {
            content: content.text,
            model: model.clone(),
            tokens_used: google_response.usage_metadata.map(|u| u.total_token_count),
            finish_reason: candidate.finish_reason,
            conversation_id: uuid::Uuid::new_v4(),
        })
    }
}

impl Clone for BevyAIAgent {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
        }
    }
}

impl AIRequest {
    /// Set the AI model to use
    pub fn with_model(mut self, model: ModelType) -> Self {
        self.model = Some(model);
        self
    }

    /// Set the temperature for generation
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set maximum tokens to generate
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Add context to the request
    pub fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        self.context.push(context.into());
        self
    }

    /// Set system prompt
    pub fn with_system_prompt<S: Into<String>>(mut self, system_prompt: S) -> Self {
        self.system_prompt = Some(system_prompt.into());
        self
    }

    /// Execute the AI request
    pub async fn execute(self) -> Result<AIResponse> {
        let model = self
            .model
            .unwrap_or_else(|| self.agent.config.default_model.clone());
        let temperature = self
            .temperature
            .unwrap_or(self.agent.config.generation.temperature);
        let max_tokens = self
            .max_tokens
            .unwrap_or(self.agent.config.generation.max_tokens);

        info!("Executing AI request with model: {}", model);

        match model.provider() {
            "openai" => {
                let mut messages = Vec::new();

                if let Some(system_prompt) = &self.system_prompt {
                    messages.push(OpenAIMessage {
                        role: "system".to_string(),
                        content: system_prompt.clone(),
                    });
                }

                for context in &self.context {
                    messages.push(OpenAIMessage {
                        role: "assistant".to_string(),
                        content: context.clone(),
                    });
                }

                messages.push(OpenAIMessage {
                    role: "user".to_string(),
                    content: self.prompt,
                });

                let request = OpenAIRequest {
                    model: model.as_str().to_string(),
                    messages,
                    max_tokens: Some(max_tokens),
                    temperature: Some(temperature),
                    stream: false,
                };

                self.agent.call_openai(&request, &model).await
            }
            "anthropic" => {
                let mut messages = Vec::new();

                if let Some(system_prompt) = &self.system_prompt {
                    let combined_prompt =
                        format!("{}\n\nHuman: {}\n\nAssistant:", system_prompt, self.prompt);
                    messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: combined_prompt,
                    });
                } else {
                    messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: self.prompt,
                    });
                }

                let request = AnthropicRequest {
                    model: model.as_str().to_string(),
                    messages,
                    max_tokens,
                    temperature: Some(temperature),
                };

                self.agent.call_anthropic(&request, &model).await
            }
            "google" => {
                let mut prompt = self.prompt;
                if let Some(system_prompt) = &self.system_prompt {
                    prompt = format!("{system_prompt}\n\n{prompt}");
                }

                let request = GoogleRequest {
                    contents: vec![GoogleContent {
                        parts: vec![GooglePart { text: prompt }],
                    }],
                    generation_config: GoogleGenerationConfig {
                        temperature: Some(temperature),
                        max_output_tokens: Some(max_tokens),
                    },
                };

                self.agent.call_google(&request, &model).await
            }
            provider => Err(BevyAIError::unsupported_model(provider)),
        }
    }
}
