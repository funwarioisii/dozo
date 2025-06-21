use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dozo")]
#[command(about = "A unified CLI tool for managing coding agent configurations")]
#[command(version = "0.1.0")]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Push {
        #[arg(long, default_value = "all")]
        target: String,

        #[arg(short, long)]
        force: bool,
    },

    Pull {
        #[arg(long)]
        from: String,

        #[arg(long)]
        merge: bool,
    },

    Status,
}

impl Cli {
    pub fn config_dir(&self) -> String {
        self.config
            .clone()
            .unwrap_or_else(|| ".agentic-coding".to_string())
    }
}

pub const AVAILABLE_TOOLS: &[&str] = &["cursor", "claude", "devin", "all"];
pub fn validate_tool_name(tool: &str) -> Result<(), String> {
    if AVAILABLE_TOOLS.contains(&tool) {
        Ok(())
    } else {
        Err(format!(
            "Unknown tool '{}'. Available tools: {}",
            tool,
            AVAILABLE_TOOLS.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tool_name() {
        assert!(validate_tool_name("cursor").is_ok());
        assert!(validate_tool_name("claude").is_ok());
        assert!(validate_tool_name("devin").is_ok());
        assert!(validate_tool_name("all").is_ok());
        assert!(validate_tool_name("invalid").is_err());
    }

    #[test]
    fn test_cli_parsing() {
        use clap::Parser;

        let cli = Cli::try_parse_from(["dozo", "status"]).unwrap();
        assert!(matches!(cli.command, Commands::Status { .. }));
        let cli = Cli::try_parse_from(["dozo", "--verbose", "push", "--target", "cursor"]).unwrap();
        assert!(cli.verbose);
        if let Commands::Push { target, .. } = cli.command {
            assert_eq!(target, "cursor");
        } else {
            panic!("Expected Push command");
        }
    }
}
