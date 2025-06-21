mod common;

use clap::Parser;
use dozo::cli::{Cli, Commands};

#[test]
fn test_cli_parsing() {
    // Test basic status command
    let cli = Cli::try_parse_from(["dozo", "status"]).unwrap();
    assert!(matches!(cli.command, Commands::Status));
    assert!(!cli.verbose);

    // Test custom config directory
    let cli = Cli::try_parse_from(["dozo", "--config", "custom-config", "status"]).unwrap();
    assert_eq!(cli.config_dir(), "custom-config");

    // Test default config directory
    let cli = Cli::try_parse_from(["dozo", "status"]).unwrap();
    assert_eq!(cli.config_dir(), ".agentic-coding");
}
