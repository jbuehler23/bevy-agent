//! Example showing project management features

use bevy_ai::{Project, BevyAIAgent, AIConfig};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for this example
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path().join("example-game");
    
    // Initialize configuration
    let config = AIConfig::load_or_create()?;
    let agent = BevyAIAgent::new(config).await?;
    
    // Create a new project
    println!("🚀 Creating new project...");
    let mut project = Project::init(
        project_path.clone(),
        "Example Game",
        "A demonstration of Bevy AI project management",
        agent
    ).await?;
    
    // Generate initial game
    println!("🤖 Generating initial game code...");
    let response = project.generate_game(
        "A 2D puzzle game with grid-based mechanics"
    ).await?;
    
    println!("✅ Initial game generated!");
    
    // Add features
    println!("🔧 Adding inventory system...");
    project.add_feature("inventory system with item management").await?;
    
    println!("🔧 Adding save/load functionality...");
    project.add_feature("save and load game state to JSON files").await?;
    
    // Show project statistics
    let stats = project.manager().stats().await?;
    println!("📊 Project Statistics:");
    println!("  Lines of Code: {}", stats.lines_of_code);
    println!("  AI Conversations: {}", stats.conversations);
    println!("  Generated Files: {}", stats.generated_files);
    println!("  Features: {}", stats.features);
    
    // Show project info
    if let Some(config) = project.manager().config() {
        println!("📋 Project Info:");
        println!("  Name: {}", config.metadata.name);
        println!("  Description: {}", config.metadata.description);
        println!("  Features: {:?}", config.metadata.features);
        println!("  Conversations: {}", config.conversations.len());
    }
    
    println!("🎉 Example completed! Project created at: {}", project_path.display());
    
    Ok(())
}
