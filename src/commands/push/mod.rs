pub mod claude;
pub mod cursor;
pub mod devin;

use crate::cli::validate_tool_name;
use crate::commands::utils::ensure_config_exists;
use anyhow::Result;

pub async fn push_command(
    config_dir: &str,
    target: &str,
    force: bool,
    verbose: bool,
) -> Result<()> {
    validate_tool_name(target).map_err(|e| anyhow::anyhow!(e))?;
    ensure_config_exists(config_dir)?;

    println!("🚀 Pushing configuration to {}...", target);

    let tools_to_process = if target == "all" {
        vec!["cursor", "claude", "devin"]
    } else {
        vec![target]
    };

    let mut success_count = 0;
    let mut error_count = 0;

    for tool in tools_to_process {
        match generate_tool_files(tool, config_dir, force) {
            Ok(file_paths) => {
                print_push_success(tool, &file_paths, verbose);
                success_count += 1;
            }
            Err(e) => {
                eprintln!("❌ Failed to generate {} configuration: {}", tool, e);
                error_count += 1;
            }
        }
    }

    print_push_summary(success_count, error_count);
    Ok(())
}

fn generate_tool_files(tool: &str, config_dir: &str, force: bool) -> Result<Vec<String>> {
    let config_path = std::path::Path::new(config_dir);

    match tool {
        "cursor" => cursor::generate_files(config_path, force),
        "claude" => claude::generate_files(config_path, force),
        "devin" => devin::generate_files(config_path, force),
        _ => anyhow::bail!("Unknown tool: {}", tool),
    }
}

fn print_push_success(tool: &str, file_paths: &[String], verbose: bool) {
    if verbose {
        println!("✅ Generated {} configuration:", tool);
        for path in file_paths {
            println!("   - {}", path);
        }
    } else {
        println!(
            "✅ Generated {} configuration ({} files)",
            tool,
            file_paths.len()
        );
    }
}

fn print_push_summary(success_count: usize, error_count: usize) {
    if error_count == 0 {
        println!("🎉 Successfully pushed to {} tool(s)!", success_count);
    } else {
        println!("⚠️  Push completed with {} error(s)", error_count);
    }
}
