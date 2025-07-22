//! Bevy AI Prototype Agent - Enhanced with AI Model Integration
//! 
//! Usage: bevy-ai "create a roguelike dungeon crawler with procedural generation"
//!        bevy-ai "add a magic system with spell combinations"
//!        bevy-ai "create a physics-based puzzle game like Portal"

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;
use tokio;

#[derive(Parser)]
#[command(name = "bevy-ai")]
#[command(about = "AI-powered Bevy game prototyping assistant with GPT/Claude integration")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new game prototype from natural language description
    Create {
        #[arg(help = "Describe the game you want to create")]
        description: String,
        #[arg(long, help = "AI model to use (gpt-4, claude-3, gemini)")]
        model: Option<String>,
    },
    /// Add features to existing prototype
    Add {
        #[arg(help = "Describe what to add to the game")]
        feature: String,
        #[arg(long, help = "AI model to use")]
        model: Option<String>,
    },
    /// Refactor or improve existing code
    Improve {
        #[arg(help = "What to improve (performance, readability, features)")]
        aspect: String,
    },
    /// Ask AI to explain the current codebase
    Explain,
    /// Initialize a new Bevy project with AI agent support
    Init {
        #[arg(help = "Project name")]
        name: String,
    },
    /// Configure AI API keys
    Config {
        #[arg(long)]
        openai_key: Option<String>,
        #[arg(long)]
        anthropic_key: Option<String>,
        #[arg(long)]
        google_key: Option<String>,
    },
    /// Run the current prototype
    Run,
    /// Build the current prototype
    Build,
}

#[derive(Serialize, Deserialize, Clone)]
struct AIConfig {
    openai_api_key: Option<String>,
    anthropic_api_key: Option<String>,
    google_api_key: Option<String>,
    default_model: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    description: String,
    features: Vec<String>,
    ai_generated_files: Vec<String>,
    conversation_history: Vec<ConversationEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ConversationEntry {
    request: String,
    response: String,
    timestamp: String,
    model_used: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Serialize, Deserialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicContent {
    text: String,
}

struct EnhancedAIAgent {
    client: Client,
    config: AIConfig,
}

impl EnhancedAIAgent {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self::load_config()?;
        Ok(EnhancedAIAgent {
            client: Client::new(),
            config,
        })
    }

    fn load_config() -> Result<AIConfig, Box<dyn std::error::Error>> {
        let config_path = dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join(".bevy-ai-config.json");

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            // Create default config
            let config = AIConfig {
                openai_api_key: env::var("OPENAI_API_KEY").ok(),
                anthropic_api_key: env::var("ANTHROPIC_API_KEY").ok(),
                google_api_key: env::var("GOOGLE_API_KEY").ok(),
                default_model: "gpt-4".to_string(),
            };
            
            let content = serde_json::to_string_pretty(&config)?;
            fs::write(&config_path, content)?;
            Ok(config)
        }
    }

    async fn generate_game_code(&self, description: &str, model: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        let model = model.unwrap_or_else(|| self.config.default_model.clone());
        
        let system_prompt = r#"You are an expert Bevy game engine developer. Generate complete, working Rust code for Bevy games based on user descriptions. 

Guidelines:
1. Use Bevy 0.12 syntax and features
2. Include all necessary imports and dependencies
3. Create complete systems, components, and resources
4. Add comments explaining key concepts
5. Use modern Bevy patterns (SystemSets, Commands, Queries)
6. Include basic error handling where appropriate
7. Make code modular and extensible
8. Add placeholder comments for assets that would be needed

Always provide a complete main.rs file that compiles and runs."#;

        let user_prompt = format!("Create a Bevy game: {}\n\nProvide the complete main.rs file with all necessary code.", description);

        match model.as_str() {
            "gpt-4" | "gpt-3.5-turbo" => self.call_openai(&model, system_prompt, &user_prompt).await,
            "claude-3-opus" | "claude-3-sonnet" => self.call_anthropic(&model, system_prompt, &user_prompt).await,
            _ => Err("Unsupported model".into()),
        }
    }

    async fn call_openai(&self, model: &str, system_prompt: &str, user_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = self.config.openai_api_key.as_ref()
            .ok_or("OpenAI API key not configured")?;

        let request = OpenAIRequest {
            model: model.to_string(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            max_tokens: 4000,
            temperature: 0.7,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let openai_response: OpenAIResponse = response.json().await?;
        
        Ok(openai_response.choices
            .into_iter()
            .next()
            .ok_or("No response from OpenAI")?
            .message
            .content)
    }

    async fn call_anthropic(&self, model: &str, system_prompt: &str, user_prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = self.config.anthropic_api_key.as_ref()
            .ok_or("Anthropic API key not configured")?;

        let combined_prompt = format!("{}\n\nHuman: {}\n\nAssistant:", system_prompt, user_prompt);

        let request = AnthropicRequest {
            model: model.to_string(),
            messages: vec![
                AnthropicMessage {
                    role: "user".to_string(),
                    content: combined_prompt,
                },
            ],
            max_tokens: 4000,
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        let anthropic_response: AnthropicResponse = response.json().await?;
        
        Ok(anthropic_response.content
            .into_iter()
            .next()
            .ok_or("No response from Anthropic")?
            .text)
    }

    async fn generate_feature_code(&self, feature_description: &str, existing_code: &str, model: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        let model = model.unwrap_or_else(|| self.config.default_model.clone());
        
        let system_prompt = r#"You are an expert Bevy game engine developer. You will be given existing game code and asked to add new features. 

Guidelines:
1. Analyze the existing code structure
2. Add the requested feature while maintaining code quality
3. Integrate seamlessly with existing systems
4. Add necessary components, systems, and resources
5. Provide clear comments for new functionality
6. Ensure the feature works well with the existing game loop
7. Use proper Bevy patterns and best practices

Return the complete updated main.rs file."#;

        let user_prompt = format!(
            "Add this feature to the existing Bevy game: {}\n\nExisting code:\n```rust\n{}\n```\n\nProvide the complete updated main.rs file with the new feature integrated.",
            feature_description, existing_code
        );

        match model.as_str() {
            "gpt-4" | "gpt-3.5-turbo" => self.call_openai(&model, system_prompt, &user_prompt).await,
            "claude-3-opus" | "claude-3-sonnet" => self.call_anthropic(&model, system_prompt, &user_prompt).await,
            _ => Err("Unsupported model".into()),
        }
    }

    async fn improve_code(&self, aspect: &str, existing_code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = r#"You are an expert Bevy game engine developer and code reviewer. You will be given existing game code and asked to improve specific aspects.

Guidelines:
1. Maintain the same functionality while improving the specified aspect
2. Use modern Bevy best practices
3. Add clear comments explaining improvements
4. Ensure code remains readable and maintainable
5. Follow Rust idioms and conventions
6. Optimize for the requested aspect (performance, readability, etc.)

Return the complete improved main.rs file."#;

        let user_prompt = format!(
            "Improve the {} of this Bevy game code:\n\n```rust\n{}\n```\n\nProvide the complete improved main.rs file.",
            aspect, existing_code
        );

        self.call_openai("gpt-4", system_prompt, &user_prompt).await
    }

    async fn explain_code(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = "You are an expert Bevy game engine teacher. Explain the given Bevy game code in a clear, educational way. Break down the components, systems, and overall architecture.";

        let user_prompt = format!("Explain this Bevy game code:\n\n```rust\n{}\n```", code);

        self.call_openai("gpt-4", system_prompt, &user_prompt).await
    }

    fn generate_cargo_toml(&self, project_name: &str, dependencies: &[&str]) -> String {
        let mut cargo_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = {{ version = "0.12", features = ["default"] }}
"#, project_name);

        for dep in dependencies {
            cargo_content.push_str(&format!("{} = \"*\"\n", dep));
        }

        cargo_content
    }

    fn extract_code_from_response(&self, response: &str) -> String {
        // Extract Rust code blocks from AI response
        if let Some(start) = response.find("```rust") {
            let code_start = start + 7;
            if let Some(end) = response[code_start..].find("```") {
                return response[code_start..code_start + end].trim().to_string();
            }
        }
        
        // If no code blocks found, return the entire response (might be just code)
        response.trim().to_string()
    }

    async fn create_ai_project(&self, name: &str, description: &str, model: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        println!("🤖 Generating game code with AI...");
        
        let ai_response = self.generate_game_code(description, model.clone()).await?;
        let game_code = self.extract_code_from_response(&ai_response);

        // Create project structure
        fs::create_dir_all(&name)?;
        fs::create_dir_all(format!("{}/src", name))?;
        fs::create_dir_all(format!("{}/assets", name))?;

        // Detect dependencies from generated code
        let mut dependencies = vec![];
        if game_code.contains("rapier") {
            dependencies.push("bevy_rapier2d");
        }
        if game_code.contains("rand") {
            dependencies.push("rand");
        }

        // Generate Cargo.toml
        let cargo_toml = self.generate_cargo_toml(name, &dependencies);
        fs::write(format!("{}/Cargo.toml", name), cargo_toml)?;

        // Write the AI-generated code
        fs::write(format!("{}/src/main.rs", name), &game_code)?;

        // Create project config with conversation history
        let config = ProjectConfig {
            name: name.to_string(),
            description: description.to_string(),
            features: vec![],
            ai_generated_files: vec!["src/main.rs".to_string()],
            conversation_history: vec![ConversationEntry {
                request: description.to_string(),
                response: ai_response,
                timestamp: chrono::Utc::now().to_rfc3339(),
                model_used: model.unwrap_or_else(|| self.config.default_model.clone()),
            }],
        };

        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(format!("{}/.bevy-ai.json", name), config_json)?;

        println!("✨ AI-generated project created: {}", name);
        println!("📁 Files generated in ./{}/", name);
        println!("🚀 Run with: cd {} && cargo run", name);

        Ok(())
    }

    async fn add_ai_feature(&self, feature_description: &str, model: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(".bevy-ai.json").exists() {
            println!("❌ No Bevy AI project found in current directory");
            return Ok(());
        }

        println!("🤖 Adding feature with AI...");

        let existing_code = fs::read_to_string("src/main.rs")?;
        let ai_response = self.generate_feature_code(feature_description, &existing_code, model.clone()).await?;
        let updated_code = self.extract_code_from_response(&ai_response);

        // Write updated code
        fs::write("src/main.rs", &updated_code)?;

        // Update project config
        let config_content = fs::read_to_string(".bevy-ai.json")?;
        let mut config: ProjectConfig = serde_json::from_str(&config_content)?;
        
        config.features.push(feature_description.to_string());
        config.conversation_history.push(ConversationEntry {
            request: format!("Add feature: {}", feature_description),
            response: ai_response,
            timestamp: chrono::Utc::now().to_rfc3339(),
            model_used: model.unwrap_or_else(|| self.config.default_model.clone()),
        });

        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(".bevy-ai.json", config_json)?;

        println!("✅ AI added feature: {}", feature_description);
        
        Ok(())
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join(".bevy-ai-config.json");
        
        let content = serde_json::to_string_pretty(&self.config)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { openai_key, anthropic_key, google_key } => {
            let mut agent = EnhancedAIAgent::new().unwrap_or_else(|_| {
                EnhancedAIAgent {
                    client: Client::new(),
                    config: AIConfig {
                        openai_api_key: None,
                        anthropic_api_key: None,
                        google_api_key: None,
                        default_model: "gpt-4".to_string(),
                    },
                }
            });

            if let Some(key) = openai_key {
                agent.config.openai_api_key = Some(key);
                println!("✅ OpenAI API key configured");
            }
            if let Some(key) = anthropic_key {
                agent.config.anthropic_api_key = Some(key);
                println!("✅ Anthropic API key configured");
            }
            if let Some(key) = google_key {
                agent.config.google_api_key = Some(key);
                println!("✅ Google API key configured");
            }

            if let Err(e) = agent.save_config() {
                println!("❌ Error saving config: {}", e);
            }
        }
        _ => {
            let agent = match EnhancedAIAgent::new() {
                Ok(agent) => agent,
                Err(e) => {
                    println!("❌ Error initializing AI agent: {}", e);
                    println!("💡 Run 'bevy-ai config --openai-key YOUR_KEY' to configure API access");
                    return;
                }
            };

            match cli.command {
                Commands::Create { description, model } => {
                    let project_name = description
                        .split_whitespace()
                        .take(3)
                        .collect::<Vec<_>>()
                        .join("_")
                        .to_lowercase();
                    
                    if let Err(e) = agent.create_ai_project(&project_name, &description, model).await {
                        println!("❌ Error creating AI project: {}", e);
                    }
                }
                Commands::Add { feature, model } => {
                    if let Err(e) = agent.add_ai_feature(&feature, model).await {
                        println!("❌ Error adding AI feature: {}", e);
                    }
                }
                Commands::Improve { aspect } => {
                    if !Path::new("src/main.rs").exists() {
                        println!("❌ No main.rs found in current directory");
                        return;
                    }

                    println!("🤖 Improving code with AI...");
                    let existing_code = fs::read_to_string("src/main.rs").unwrap();
                    
                    match agent.improve_code(&aspect, &existing_code).await {
                        Ok(ai_response) => {
                            let improved_code = agent.extract_code_from_response(&ai_response);
                            fs::write("src/main.rs", &improved_code).unwrap();
                            println!("✅ Code improved for: {}", aspect);
                        }
                        Err(e) => println!("❌ Error improving code: {}", e),
                    }
                }
                Commands::Explain => {
                    if !Path::new("src/main.rs").exists() {
                        println!("❌ No main.rs found in current directory");
                        return;
                    }

                    let code = fs::read_to_string("src/main.rs").unwrap();
                    match agent.explain_code(&code).await {
                        Ok(explanation) => {
                            println!("🤖 AI Code Explanation:\n");
                            println!("{}", explanation);
                        }
                        Err(e) => println!("❌ Error explaining code: {}", e),
                    }
                }
                Commands::Init { name } => {
                    if let Err(e) = agent.create_ai_project(&name, "A new Bevy game prototype", None).await {
                        println!("❌ Error initializing project: {}", e);
                    }
                }
                Commands::Run => {
                    println!("🚀 Running prototype...");
                    let output = Command::new("cargo")
                        .args(&["run"])
                        .output()
                        .expect("Failed to run cargo");
                    
                    if !output.status.success() {
                        println!("❌ Build failed:");
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Commands::Build => {
                    println!("🔨 Building prototype...");
                    let output = Command::new("cargo")
                        .args(&["build"])
                        .output()
                        .expect("Failed to run cargo");
                    
                    if output.status.success() {
                        println!("✅ Build successful!");
                    } else {
                        println!("❌ Build failed:");
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                _ => {} // Already handled above
            }
        }
    }
}