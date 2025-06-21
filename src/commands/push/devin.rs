use crate::commands::utils::{get_project_name, read_and_combine_markdown_files};
use anyhow::{Context, Result};
use std::path::Path;

pub fn generate_files(config_path: &Path, force: bool) -> Result<Vec<String>> {
    let output_path = Path::new("devin-knowledge.json");

    if output_path.exists() && !force {
        anyhow::bail!(
            "File '{}' already exists. Use --force to overwrite.",
            output_path.display()
        );
    }

    let combined_content = read_and_combine_markdown_files(config_path.to_str().unwrap())?;
    let project_name = get_project_name();
    let final_content = serde_json::to_string_pretty(&serde_json::json!({
        "project": project_name,
        "content": combined_content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))?;

    std::fs::write(output_path, final_content)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    Ok(vec![output_path.display().to_string()])
}
