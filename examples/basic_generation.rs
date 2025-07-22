//! Basic example showing how to use Bevy AI as a library

use bevy_agent::{BevyAIAgent, AIConfig, ModelType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration from environment or file
    let config = AIConfig::load_or_create()?;
    
    // Create AI agent
    let agent = BevyAIAgent::new(config).await?;
    
    // Generate a simple game
    println!("🤖 Generating a simple 2D shooter game...");
    
    let response = agent
        .generate_game("A simple 2D shooter with enemies and power-ups")
        .with_model(ModelType::GPT4)
        .execute()
        .await?;
    
    println!("✅ Generated game code!");
    println!("📝 Code:\n{}", response.content);
    
    if let Some(tokens) = response.tokens_used {
        println!("🔢 Tokens used: {}", tokens);
    }
    
    Ok(())
}
