pub mod pull;
pub mod push;
pub mod status;
pub mod utils;

use crate::cli::Commands;
use anyhow::Result;

pub async fn execute_command(command: Commands, config_dir: &str, verbose: bool) -> Result<()> {
    match command {
        Commands::Push { target, force } => {
            push::push_command(config_dir, &target, force, verbose).await
        }
        Commands::Pull { from, merge } => {
            pull::pull_command(config_dir, &from, merge, verbose).await
        }
        Commands::Status => status::status_command(config_dir, verbose).await,
    }
}
