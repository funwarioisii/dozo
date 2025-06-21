use crate::commands::utils::copy_hierarchy_with_extension_change;
use anyhow::{Context, Result};
use std::path::Path;

pub fn pull_from(target_dir: &Path, verbose: bool) -> Result<()> {
    let mut pulled_files = Vec::new();
    let mut has_content = false;

    let cursor_rules_dir = Path::new(".cursor/rules");
    if cursor_rules_dir.exists() {
        let copied_files =
            copy_hierarchy_with_extension_change(cursor_rules_dir, target_dir, "mdc", "md")?;
        pulled_files.extend(copied_files);
        has_content = true;

        if verbose {
            println!("   Found .cursor/rules/ directory (modern format)");
        }
    }

    let cursorrules_file = Path::new(".cursorrules");
    if cursorrules_file.exists() {
        let target_file = target_dir.join("cursorrules.md");
        std::fs::copy(cursorrules_file, &target_file).with_context(|| {
            format!(
                "Failed to copy {} to {}",
                cursorrules_file.display(),
                target_file.display()
            )
        })?;

        pulled_files.push(target_file.display().to_string());
        has_content = true;

        if verbose {
            println!("   Found .cursorrules file (legacy format)");
        }
    }

    if !has_content {
        anyhow::bail!(
            "No Cursor configuration found. Expected .cursor/rules/ directory or .cursorrules file."
        );
    }

    println!("✅ Pulled {} files from Cursor", pulled_files.len());
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
    fn test_pull_from_no_cursor_config() {
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
            .contains("No Cursor configuration found"));
    }
}
