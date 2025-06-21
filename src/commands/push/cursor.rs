use crate::commands::utils::{copy_hierarchy, read_and_combine_markdown_files};
use anyhow::{Context, Result};
use std::path::Path;

/// Generates Cursor configuration files (.cursor/rules/ with .mdc files and .cursorrules)
pub fn generate_files(config_path: &Path, force: bool) -> Result<Vec<String>> {
    let mut generated_files = Vec::new();

    // 1. Generate .cursor/rules/ directory (modern format)
    let target_dir = Path::new(".cursor/rules");
    if target_dir.exists() {
        if !force {
            anyhow::bail!(
                "Directory '{}' already exists. Use --force to overwrite.",
                target_dir.display()
            );
        }
        std::fs::remove_dir_all(target_dir)
            .with_context(|| format!("Failed to remove existing {}", target_dir.display()))?;
    }
    let mut rules_files = copy_hierarchy(config_path, target_dir, Some("mdc"))?;
    generated_files.append(&mut rules_files);

    // 2. Generate .cursorrules file (legacy format)
    let cursorrules_path = Path::new(".cursorrules");
    if cursorrules_path.exists() && !force {
        anyhow::bail!(
            "File '{}' already exists. Use --force to overwrite.",
            cursorrules_path.display()
        );
    }

    // Check if there's a specific cursorrules.md file to use
    let cursorrules_source = config_path.join("cursorrules.md");
    if cursorrules_source.exists() {
        // Use the specific cursorrules.md file
        let content = std::fs::read_to_string(&cursorrules_source)
            .with_context(|| format!("Failed to read {}", cursorrules_source.display()))?;
        std::fs::write(cursorrules_path, content)
            .with_context(|| format!("Failed to write {}", cursorrules_path.display()))?;
    } else {
        // Combine all markdown files into .cursorrules
        let combined_content = read_and_combine_markdown_files(config_path.to_str().unwrap())?;
        if !combined_content.trim().is_empty() {
            std::fs::write(cursorrules_path, combined_content.trim())
                .with_context(|| format!("Failed to write {}", cursorrules_path.display()))?;
        }
    }

    if cursorrules_path.exists() {
        generated_files.push(cursorrules_path.display().to_string());
    }

    Ok(generated_files)
}
