use crate::devin::{DevinClient, DevinError, Knowledge};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub async fn pull_from(target_dir: &Path, verbose: bool) -> Result<()> {
    let config_dir = target_dir
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid target directory path"))?;

    pull_from_devin(config_dir, verbose)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to pull from Devin: {}", e))?;

    Ok(())
}

async fn pull_from_devin(config_dir: &str, verbose: bool) -> Result<(), DevinError> {
    if verbose {
        println!("🔄 Pulling knowledge from Devin...");
    }

    let client = DevinClient::new()?;

    let response = client.list_knowledge().await?;

    let total_count = response.knowledge.len();

    if verbose {
        println!("📚 Found {} knowledge items", total_count);
    }

    let current_dir = std::env::current_dir()?;
    let folder_names = get_folder_names(&current_dir);

    if verbose {
        println!("🔍 Filtering for project folders: {:?}", folder_names);
    }

    let relevant_knowledge: Vec<_> = response
        .knowledge
        .into_iter()
        .filter(|knowledge| is_relevant_to_project(knowledge, &folder_names))
        .collect();

    if verbose {
        println!(
            "🎯 Found {} relevant knowledge items for this project",
            relevant_knowledge.len()
        );
    }

    let devin_dir = format!("{}/devin", config_dir);
    fs::create_dir_all(&devin_dir)?;

    let mut saved_files = Vec::new();

    for knowledge in relevant_knowledge {
        let file_path = save_knowledge_to_file(&knowledge, &devin_dir, verbose)?;
        saved_files.push(file_path);
    }

    println!(
        "✅ Pulled {} files from Devin (filtered from {} total)",
        saved_files.len(),
        total_count
    );
    if verbose {
        for file in &saved_files {
            println!("   - {}", file);
        }
    }

    Ok(())
}

fn save_knowledge_to_file(
    knowledge: &Knowledge,
    devin_dir: &str,
    verbose: bool,
) -> Result<String, DevinError> {
    let safe_name = sanitize_filename(&knowledge.name);
    let filename = format!("{}/{}.md", devin_dir, safe_name);

    let content = format!(
        "# {}\n\n<!-- Devin Knowledge ID: {} -->\n<!-- Trigger: {} -->\n<!-- Created: {} -->\n\n{}\n",
        knowledge.name,
        knowledge.id,
        knowledge.trigger_description,
        knowledge.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
        knowledge.body
    );

    fs::write(&filename, content)?;

    if verbose {
        println!("📝 Saved: {}", filename);
    }

    Ok(filename)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn get_folder_names(current_dir: &Path) -> Vec<String> {
    current_dir
        .components()
        .filter_map(|component| {
            if let std::path::Component::Normal(name) = component {
                name.to_str().map(|s| s.to_lowercase())
            } else {
                None
            }
        })
        .collect()
}

fn is_relevant_to_project(knowledge: &Knowledge, folder_names: &[String]) -> bool {
    let trigger_lower = knowledge.trigger_description.to_lowercase();

    let name_lower = knowledge.name.to_lowercase();

    let body_lower = knowledge.body.to_lowercase();

    for folder_name in folder_names {
        if trigger_lower.contains(folder_name)
            || name_lower.contains(folder_name)
            || body_lower.contains(folder_name)
        {
            return true;
        }
    }

    false
}
