pub mod claude;
pub mod cursor;
pub mod devin;

use crate::cli::validate_tool_name;
use crate::commands::utils::find_all_files;
use anyhow::{Context, Result};
use std::path::Path;

pub async fn pull_command(config_dir: &str, from: &str, merge: bool, verbose: bool) -> Result<()> {
    validate_tool_name(from).map_err(|e| anyhow::anyhow!(e))?;

    println!("🔄 Pulling configuration from {}...", from);

    let target_dir = Path::new(config_dir);
    if !merge && target_dir.exists() {
        let file_count = find_all_files(target_dir)?.len();
        if file_count > 0 {
            anyhow::bail!(
                "Configuration directory '{}' already contains {} files. Use --merge to merge with existing configuration.",
                config_dir, file_count
            );
        }
    }

    std::fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create directory {}", target_dir.display()))?;

    match from {
        "cursor" => cursor::pull_from(target_dir, verbose)?,
        "claude" => claude::pull_from(target_dir, verbose)?,
        "devin" => devin::pull_from(target_dir, verbose).await?,
        _ => anyhow::bail!("Unsupported tool: {}", from),
    }

    Ok(())
}
