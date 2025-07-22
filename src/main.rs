use bevy_agent::cli::{Cli, CliHandler};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut handler = match CliHandler::new(cli.config.clone()).await {
        Ok(handler) => handler,
        Err(e) => {
            eprintln!("Error initializing Bevy AI Agent: {e}");
            eprintln!("Run 'bevy-agent config --openai-key YOUR_KEY' to configure API access");
            std::process::exit(1);
        }
    };

    if let Err(e) = handler.handle(cli).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
