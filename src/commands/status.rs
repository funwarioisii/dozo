use crate::commands::utils::{ensure_config_exists, find_markdown_files, get_project_name};
use anyhow::Result;
use std::path::Path;

pub async fn status_command(config_dir: &str, verbose: bool) -> Result<()> {
    ensure_config_exists(config_dir)?;

    let project_name = get_project_name();
    println!("📊 {} - Synchronization Status", project_name);
    println!();

    for tool in ["cursor", "claude", "devin"] {
        print_tool_status(tool, verbose)?;
        println!();
    }

    if verbose {
        println!("📁 Configuration directory: {}", config_dir);
    }

    Ok(())
}

fn print_tool_status(tool: &str, verbose: bool) -> Result<()> {
    let output_path = get_tool_output_path(tool);
    let exists = check_tool_exists(tool, &output_path);
    let icon = if exists { "✅" } else { "⚠️ " };

    println!("{} {}", icon, tool);

    match tool {
        "cursor" => print_cursor_status(&output_path, exists)?,
        "claude" => print_claude_status(exists)?,
        _ => print_generic_status(&output_path, exists)?,
    }

    if verbose {
        print_verbose_info(tool);
    }

    Ok(())
}

fn get_tool_output_path(tool: &str) -> std::path::PathBuf {
    match tool {
        "cursor" => ".cursor/rules".into(),
        "claude" => "CLAUDE.md".into(),
        "devin" => "devin-knowledge.json".into(),
        _ => format!("{}.config", tool).into(),
    }
}

fn check_tool_exists(tool: &str, output_path: &Path) -> bool {
    match tool {
        "cursor" => output_path.is_dir(),
        "claude" => {
            let main_file = Path::new("CLAUDE.md");
            let commands_dir = Path::new(".claude/commands");
            main_file.exists() || commands_dir.exists()
        }
        _ => output_path.exists(),
    }
}

fn print_cursor_status(output_path: &Path, exists: bool) -> Result<()> {
    if !exists {
        println!("   Directory: {} (not found)", output_path.display());
        return Ok(());
    }

    let file_count = find_markdown_files(output_path).unwrap_or_default().len();
    println!(
        "   Directory: {} ✓ ({} files)",
        output_path.display(),
        file_count
    );
    print_last_modified(output_path)?;
    Ok(())
}

fn print_claude_status(_exists: bool) -> Result<()> {
    let main_file = Path::new("CLAUDE.md");
    let commands_dir = Path::new(".claude/commands");

    if main_file.exists() {
        println!("   Main file: {} ✓", main_file.display());
        print_last_modified(main_file)?;
    } else {
        println!("   Main file: {} (not found)", main_file.display());
    }

    if commands_dir.exists() {
        let command_count = find_markdown_files(commands_dir).unwrap_or_default().len();
        println!(
            "   Commands: {} ✓ ({} files)",
            commands_dir.display(),
            command_count
        );
    }

    Ok(())
}

fn print_generic_status(output_path: &Path, exists: bool) -> Result<()> {
    if exists {
        println!("   File: {} ✓", output_path.display());
        print_last_modified(output_path)?;
    } else {
        println!("   File: {} (not found)", output_path.display());
    }
    Ok(())
}

fn print_last_modified(path: &Path) -> Result<()> {
    if let Ok(metadata) = std::fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = modified.elapsed() {
                println!("   Last modified: {} ago", format_duration(duration));
            }
        }
    }
    Ok(())
}

fn print_verbose_info(tool: &str) {
    match tool {
        "cursor" => println!("   Output: .cursor/rules/ (hierarchy)"),
        "claude" => println!("   Output: CLAUDE.md + .claude/commands/ (if exists)"),
        "devin" => println!("   API: https://app.devin.ai/api"),
        _ => {}
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let seconds = duration.as_secs();

    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else if seconds < 86400 {
        format!("{}h", seconds / 3600)
    } else {
        format!("{}d", seconds / 86400)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> Result<()> {
        let file_path = dir.join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, content)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_status_command_success() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).unwrap();

        create_test_file(&config_dir, "test.md", "# Test").unwrap();

        let result = status_command(config_dir.to_str().unwrap(), false).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_status_command_missing_config() {
        let result = status_command("/path/that/does/not/exist", false).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_status_command_verbose() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).unwrap();

        create_test_file(&config_dir, "test.md", "# Test").unwrap();

        let result = status_command(config_dir.to_str().unwrap(), true).await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_tool_output_path() {
        assert_eq!(get_tool_output_path("cursor"), Path::new(".cursor/rules"));
        assert_eq!(get_tool_output_path("claude"), Path::new("CLAUDE.md"));
        assert_eq!(
            get_tool_output_path("devin"),
            Path::new("devin-knowledge.json")
        );
        assert_eq!(get_tool_output_path("unknown"), Path::new("unknown.config"));
    }

    #[test]
    fn test_print_cursor_status() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_dir = temp_dir.path().join(".cursor/rules");
        fs::create_dir_all(&cursor_dir).unwrap();

        create_test_file(&cursor_dir, "test1.md", "# Test 1").unwrap();
        create_test_file(&cursor_dir, "test2.md", "# Test 2").unwrap();

        let result = print_cursor_status(&cursor_dir, true);
        assert!(result.is_ok());

        let non_existing = temp_dir.path().join("non-existing");
        let result = print_cursor_status(&non_existing, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_generic_status() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        fs::write(&test_file, "content").unwrap();
        let result = print_generic_status(&test_file, true);
        assert!(result.is_ok());

        let non_existing = temp_dir.path().join("non-existing.txt");
        let result = print_generic_status(&non_existing, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_duration() {
        use std::time::Duration;

        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m");
        assert_eq!(format_duration(Duration::from_secs(3660)), "1h");
        assert_eq!(format_duration(Duration::from_secs(86401)), "1d");
    }

    #[test]
    fn test_print_verbose_info() {
        print_verbose_info("cursor");
        print_verbose_info("claude");
        print_verbose_info("devin");
        print_verbose_info("unknown");
    }
}
