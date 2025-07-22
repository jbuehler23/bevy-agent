//! Example using game templates instead of AI generation

use bevy_ai::game_templates::{TemplateManager, TemplateContext};
use bevy_ai::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎮 Exploring built-in game templates\n");
    
    // Create template manager
    let manager = TemplateManager::new()?;
    
    // List all available templates
    println!("📋 Available templates:");
    for template_name in manager.available_templates() {
        println!("  • {}", template_name);
    }
    println!();
    
    // Generate different game types
    generate_game_from_template(&manager, "platformer_2d", "MyPlatformer").await?;
    generate_game_from_template(&manager, "fps_3d", "MyFPSGame").await?;
    generate_game_from_template(&manager, "puzzle_game", "MyPuzzleGame").await?;
    
    println!("🎉 All templates generated successfully!");
    
    Ok(())
}

async fn generate_game_from_template(
    manager: &TemplateManager,
    template_name: &str,
    game_name: &str,
) -> Result<()> {
    println!("🏗️  Generating {} from {} template...", game_name, template_name);
    
    // Create context for template
    let template_context = TemplateContext::new(
        game_name.to_string(), 
        format!("A {} game created from template", template_name)
    );
    let generated_content = manager.generate(template_name, &template_context)?;
    
    println!("✅ {} template generated successfully!", game_name);
    println!("   Content length: {} characters", generated_content.len());
    println!("   Preview: {}\n", 
        generated_content.chars().take(200).collect::<String>()
            .replace('\n', " ")
            .trim());
    
    Ok(())
}
