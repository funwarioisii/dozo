use crate::commands::utils::copy_hierarchy;
use anyhow::{Context, Result};
use std::path::Path;

pub fn pull_from(target_dir: &Path, verbose: bool) -> Result<()> {
    let mut pulled_files = Vec::new();
    let mut has_content = false;

    let claude_file = Path::new("CLAUDE.md");
    if claude_file.exists() {
        let target_file = target_dir.join("CLAUDE.md");
        std::fs::copy(claude_file, &target_file).with_context(|| {
            format!(
                "Failed to copy {} to {}",
                claude_file.display(),
                target_file.display()
            )
        })?;

        pulled_files.push(target_file.display().to_string());
        has_content = true;
    }

    let commands_source = Path::new(".claude/commands");
    if commands_source.exists() {
        let commands_target = target_dir.join("commands");
        let mut command_files = copy_hierarchy(commands_source, &commands_target, None)?;
        pulled_files.append(&mut command_files);
        has_content = true;
    }

    if !has_content {
        anyhow::bail!(
            "No Claude configuration found. Expected CLAUDE.md or .claude/commands directory."
        );
    }

    println!("✅ Pulled {} files from Claude", pulled_files.len());
    if verbose {
        for file in &pulled_files {
            println!("   - {}", file);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_pull_from_no_claude_config() {
        let temp_dir = TempDir::new().unwrap();
        let target_dir = temp_dir.path().join("target");
        fs::create_dir_all(&target_dir).unwrap();

        let old_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = pull_from(&target_dir, false);

        std::env::set_current_dir(old_dir).unwrap();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No Claude configuration found"));
    }
}
