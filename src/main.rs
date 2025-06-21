mod cli;
mod commands;
mod devin;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_dir = cli.config_dir();

    if cli.verbose {
        println!("🔧 Running in verbose mode");
        println!("📁 Using config directory: {}", config_dir);
    }

    commands::execute_command(cli.command, &config_dir, cli.verbose).await?;

    Ok(())
}
