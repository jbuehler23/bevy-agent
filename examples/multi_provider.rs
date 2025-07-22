//! Example demonstrating different AI providers

use bevy_agent::{BevyAIAgent, AIConfig, Result};
use bevy_agent::config::{OpenAIConfig, AnthropicConfig, GoogleConfig};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing different AI providers for Bevy game generation\n");
    
    // Test OpenAI GPT-4
    if let Ok(config) = create_openai_config() {
        println!("🤖 Testing OpenAI GPT-4...");
        test_provider(config, "OpenAI GPT-4").await?;
    } else {
        println!("⚠️  OpenAI API key not found in environment");
    }
    
    // Test Anthropic Claude
    if let Ok(config) = create_anthropic_config() {
        println!("🧠 Testing Anthropic Claude...");
        test_provider(config, "Anthropic Claude").await?;
    } else {
        println!("⚠️  Anthropic API key not found in environment");
    }
    
    // Test Google Gemini
    if let Ok(config) = create_google_config() {
        println!("💎 Testing Google Gemini...");
        test_provider(config, "Google Gemini").await?;
    } else {
        println!("⚠️  Google API key not found in environment");
    }
    
    Ok(())
}

fn create_openai_config() -> Result<AIConfig> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| bevy_agent::BevyAIError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "OPENAI_API_KEY not found"
        )))?;
    Ok(AIConfig {
        openai: Some(OpenAIConfig {
            api_key,
            organization: None,
            base_url: None,
        }),
        ..Default::default()
    })
}

fn create_anthropic_config() -> Result<AIConfig> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| bevy_agent::BevyAIError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "ANTHROPIC_API_KEY not found"
        )))?;
    Ok(AIConfig {
        anthropic: Some(AnthropicConfig {
            api_key,
            base_url: None,
        }),
        ..Default::default()
    })
}

fn create_google_config() -> Result<AIConfig> {
    let api_key = std::env::var("GOOGLE_API_KEY")
        .map_err(|_| bevy_agent::BevyAIError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "GOOGLE_API_KEY not found"
        )))?;
    Ok(AIConfig {
        google: Some(GoogleConfig {
            api_key,
            base_url: None,
        }),
        ..Default::default()
    })
}

async fn test_provider(config: AIConfig, provider_name: &str) -> Result<()> {
    let agent = BevyAIAgent::new(config).await?;
    
    let request = agent
        .request("Generate a simple Bevy system that spawns a red cube")
        .with_system_prompt("You are a Bevy game engine expert")
        .with_max_tokens(200);
    
    match request.execute().await {
        Ok(response) => {
            println!("✅ {} response:", provider_name);
            println!("   Length: {} characters", response.content.len());
            println!("   Preview: {}\n", 
                response.content.chars().take(100).collect::<String>()
                    .replace('\n', " "));
        },
        Err(e) => {
            println!("❌ {} failed: {}\n", provider_name, e);
        }
    }
    
    Ok(())
}
