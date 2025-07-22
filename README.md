# 🧠 Bevy AI - AI-Powered Game Development Assistant

[![Crates.io](https://img.shields.io/crates/v/bevy-ai.svg)](https://crates.io/crates/bevy-ai)
[![Documentation](https://docs.rs/bevy-ai/badge.svg)](https://docs.rs/bevy-ai)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/jbuehler23/bevy-ai/workflows/CI/badge.svg)](https://github.com/jbuehler23/bevy-ai/actions)
[![Downloads](https://img.shields.io/crates/d/bevy-ai.svg)](https://crates.io/crates/bevy-ai)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

Bevy AI is a comprehensive library and CLI tool that leverages cutting-edge AI models (GPT-4, Claude-3, Gemini) to accelerate Bevy game development through natural language code generation, intelligent feature addition, and automated code optimization.

## ✨ Features

### 🧠 AI Model Integration
- **OpenAI**: GPT-4, GPT-4-Turbo, GPT-3.5-turbo
- **Anthropic**: Claude-3-opus, Claude-3-sonnet, Claude-3-haiku
- **Google**: Gemini-pro, Gemini-pro-vision

### 🚀 Dynamic Code Generation
- No more rigid templates - AI generates contextual, unique code
- Understands complex game concepts and mechanics
- Creates complete, working Bevy applications from scratch
- Context-aware feature additions that integrate seamlessly

### 🎮 Game Development Features
- **Natural Language Game Creation**: Describe your game in plain English
- **Intelligent Feature Addition**: Add complex systems with AI assistance
- **Code Analysis & Optimization**: AI-powered code review and improvements
- **Smart Dependency Management**: Automatically detects and manages Cargo dependencies
- **Template System**: Built-in templates for common game types
- **Project Management**: Track AI conversations and generated files

## 🛠️ Installation

### CLI Tool
```bash
cargo install bevy-ai
```

### Library
Add to your `Cargo.toml`:
```toml
[dependencies]
bevy-ai = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Quick Start

### 1. Configure API Keys
```bash
# Configure OpenAI (recommended)
bevy-ai config --openai-key sk-...

# Or use Anthropic
bevy-ai config --anthropic-key ant-...

# Or use Google
bevy-ai config --google-key AIz...

# Set default model
bevy-ai config --default-model gpt-4
```

### 2. Create Your First AI-Generated Game
```bash
# Create a 2D platformer
bevy-ai create "2D platformer with physics and collectibles"

# Create a 3D space shooter
bevy-ai create "3D space shooter with asteroids and power-ups"

# Create a puzzle game
bevy-ai create "puzzle game with grid-based mechanics and level progression"
```

### 3. Add Features to Existing Games
```bash
cd your-game-project
bevy-ai add "inventory system with drag-and-drop UI"
bevy-ai add "magic system with spell combinations and cooldowns"
bevy-ai add "multiplayer support with networking"
```

### 4. Improve and Optimize Code
```bash
# Improve performance
bevy-ai improve performance

# Enhance readability
bevy-ai improve readability

# Add better structure
bevy-ai improve structure
```

## 🎯 Advanced Usage

### Complex Game Creation
```bash
# Roguelike with procedural generation
bevy-ai create "roguelike dungeon crawler with procedural generation, turn-based combat, and character progression"

# Physics-based puzzle game
bevy-ai create "physics-based puzzle game like Portal with teleportation mechanics and environmental puzzles"

# Tower defense with AI
bevy-ai create "tower defense with pathfinding enemies, upgradeable towers, and resource management"
```

### Model Selection
```bash
# Use specific models for different tasks
bevy-ai create "space game" --model gpt-4
bevy-ai add "multiplayer" --model claude-3-opus
bevy-ai improve performance --model gpt-4-turbo
```

### Code Analysis and Debugging
```bash
# Get AI explanations of your codebase
bevy-ai explain

# Debug specific issues
bevy-ai debug "compilation error in movement system"

# Analyze specific files
bevy-ai explain --file src/player.rs
```

### Project Management
```bash
# View project information
bevy-ai project info

# Show project statistics
bevy-ai project stats

# View AI conversation history
bevy-ai project history

# Clean up generated files
bevy-ai project clean
```

### Build Operations
```bash
# Build your game
bevy-ai build build

# Run your game
bevy-ai build run

# Check for errors
bevy-ai build check

# Run clippy lints
bevy-ai build clippy

# Format code
bevy-ai build format
```

### Template System
```bash
# List available templates
bevy-ai template list

# Show template details
bevy-ai template show platformer_2d

# Create game from template
bevy-ai create "my platformer" --template platformer_2d
```

## 📚 Library Usage

### Basic Usage
```rust
use bevy_ai::{BevyAIAgent, AIConfig, ModelType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AIConfig::from_env()?;
    let agent = BevyAIAgent::new(config).await?;
    
    let response = agent
        .generate_game("A simple 2D shooter")
        .with_model(ModelType::GPT4)
        .execute()
        .await?;
    
    println!("Generated code:\n{}", response.content);
    Ok(())
}
```

### Advanced Project Management
```rust
use bevy_ai::{Project, BevyAIAgent, AIConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AIConfig::load_or_create()?;
    let agent = BevyAIAgent::new(config).await?;
    
    // Create a new project
    let mut project = Project::init(
        PathBuf::from("my-game"),
        "My Awesome Game",
        "A revolutionary puzzle platformer",
        agent
    ).await?;
    
    // Generate initial game code
    let response = project.generate_game(
        "2D puzzle platformer with physics-based mechanics"
    ).await?;
    
    // Add features
    project.add_feature("inventory system with crafting").await?;
    project.add_feature("save/load system").await?;
    
    // Build the project
    project.manager().build().await?;
    
    Ok(())
}
```

### Custom AI Requests
```rust
use bevy_ai::{BevyAIAgent, AIConfig, ModelType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AIConfig::from_env()?;
    let agent = BevyAIAgent::new(config).await?;
    
    let response = agent
        .request("Create a component for enemy AI behavior")
        .with_model(ModelType::Claude3Opus)
        .with_temperature(0.8)
        .with_max_tokens(2000)
        .with_context("This is for a 2D tower defense game")
        .execute()
        .await?;
    
    println!("AI Response: {}", response.content);
    Ok(())
}
```

## 🎮 Real-World Examples

### Metroidvania Game
```bash
bevy-ai create "metroidvania with ability-gated progression, interconnected world, and backtracking mechanics"
```
Generates:
- Interconnected world systems
- Power-up mechanics
- Ability-gated progression logic
- Map and exploration systems

### Dialogue System
```bash
bevy-ai add "dialogue system with branching conversations, character portraits, and voice acting support"
```
Creates:
- NPC interaction systems
- Dialogue tree management
- UI components for conversations
- Audio integration hooks

### A* Pathfinding
```bash
bevy-ai add "A* pathfinding for enemy AI with dynamic obstacle avoidance"
```
Implements:
- Pathfinding algorithms
- Navigation mesh generation
- Dynamic obstacle detection
- AI behavior integration

## 🔧 Configuration

### Configuration File
Bevy AI stores configuration in `~/.bevy-ai-config.json`:

```json
{
  "openai": {
    "api_key": "sk-...",
    "organization": null,
    "base_url": null
  },
  "anthropic": {
    "api_key": "ant-...",
    "base_url": null
  },
  "google": {
    "api_key": "AIz...",
    "base_url": null
  },
  "default_model": "gpt-4",
  "generation": {
    "temperature": 0.7,
    "max_tokens": 4000,
    "include_comments": true,
    "generate_tests": false,
    "bevy_version": "0.12",
    "rust_edition": "2021"
  },
  "project": {
    "track_conversations": true,
    "auto_format": true,
    "auto_dependencies": true,
    "default_template": "basic"
  }
}
```

### Environment Variables
You can also configure using environment variables:
```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="ant-..."
export GOOGLE_API_KEY="AIz..."
export BEVY_AI_DEFAULT_MODEL="gpt-4"
```

### Project Configuration
Each Bevy AI project includes a `.bevy-ai.json` file that tracks:
- Project metadata
- AI conversation history
- Generated files
- Dependencies
- Custom templates

## 🔮 Key Advantages Over Templates

- **Unlimited Creativity**: Not constrained by predefined templates
- **Context-Aware**: Understands your existing codebase
- **Learning**: Benefits from the latest AI model improvements
- **Explanatory**: Can explain and teach as it generates code
- **Adaptive**: Handles edge cases and unique requirements

## 🏗️ Architecture

### Core Components

1. **AI Integration Layer** (`src/ai/`)
   - Multi-provider AI client (OpenAI, Anthropic, Google)
   - Model-specific prompt optimization
   - Response parsing and code extraction

2. **Project Management** (`src/project.rs`)
   - Project lifecycle management
   - Conversation history tracking
   - File generation monitoring

3. **Template System** (`src/game_templates.rs`)
   - Built-in game templates
   - Custom template support
   - Context-aware code generation

4. **CLI Interface** (`src/cli.rs`)
   - Command-line interface
   - User interaction handling
   - Progress reporting

5. **Utilities** (`src/utils.rs`)
   - Code analysis tools
   - Build system integration
   - File system operations

### Supported AI Models

| Provider | Models | Context Length | Strengths |
|----------|--------|----------------|-----------|
| OpenAI | GPT-4, GPT-4-Turbo, GPT-3.5-turbo | 8K-128K | Code generation, debugging |
| Anthropic | Claude-3-opus, Claude-3-sonnet, Claude-3-haiku | 200K | Long context, reasoning |
| Google | Gemini-pro, Gemini-pro-vision | 32K | Multimodal, fast inference |

### Built-in Templates

- **Basic Game**: Simple 3D scene with camera and lighting
- **2D Platformer**: Physics-based platformer with player movement
- **3D FPS**: First-person shooter with mouse look and movement
- **Puzzle Game**: Grid-based puzzle mechanics
- **Strategy Game**: RTS-style unit management and resources

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
git clone https://github.com/jbuehler23/bevy-ai.git
cd bevy-ai
cargo build
cargo test
```

### Running Examples
```bash
# Set up API keys
export OPENAI_API_KEY="your-key-here"

# Run CLI examples
cargo run -- create "simple platformer"
cargo run -- add "particle effects"
cargo run -- improve performance
```

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- The [Bevy](https://bevyengine.org/) team for creating an amazing game engine
- OpenAI, Anthropic, and Google for providing powerful AI models
- The Rust community for excellent tooling and libraries

## 🔗 Links

- [Documentation](https://docs.rs/bevy-ai)
- [Crates.io](https://crates.io/crates/bevy-ai)
- [Examples](examples/)
- [Issue Tracker](https://github.com/jbuehler23/bevy-ai/issues)
- [Discussions](https://github.com/jbuehler23/bevy-ai/discussions)

## 📊 Status

This project is actively developed and maintained. We aim to support the latest versions of Bevy and provide cutting-edge AI integration for game development.

Current Status:
- ✅ Core AI integration
- ✅ Multi-provider support
- ✅ Project management
- ✅ Template system
- ✅ CLI interface
- 🚧 Web interface (planned)
- 🚧 IDE plugins (planned)
- 🚧 Asset generation (planned)