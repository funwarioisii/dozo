mod common;

use clap::Parser;
use dozo::cli::{Cli, Commands};

#[test]
fn test_cli_parsing() {
    // Test push command
    let cli = Cli::try_parse_from(["dozo", "push", "--target", "cursor"]).unwrap();
    assert!(matches!(cli.command, Commands::Push { .. }));
    assert!(!cli.verbose);

    // Test custom config directory
    let cli = Cli::try_parse_from(["dozo", "--config", "custom-config", "push"]).unwrap();
    assert_eq!(cli.config_dir(), "custom-config");

    // Test default config directory
    let cli = Cli::try_parse_from(["dozo", "pull", "--from", "devin"]).unwrap();
    assert_eq!(cli.config_dir(), ".agentic-coding");
}
