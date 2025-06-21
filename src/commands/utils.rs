use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn ensure_config_exists(config_dir: &str) -> Result<()> {
    let config_path = Path::new(config_dir);
    if !config_path.exists() {
        anyhow::bail!("Configuration directory '{}' not found", config_dir);
    }
    Ok(())
}

pub fn get_project_name() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|dir| dir.file_name()?.to_str().map(String::from))
        .unwrap_or_else(|| "Project".to_string())
}

pub fn find_markdown_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            files.push(path.to_path_buf());
        }
    }

    files.sort();
    Ok(files)
}

pub fn find_all_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        files.push(entry.path().to_path_buf());
    }

    Ok(files)
}

pub fn copy_hierarchy(
    source_dir: &Path,
    target_dir: &Path,
    change_extension: Option<&str>,
) -> Result<Vec<String>> {
    let mut created_files = Vec::new();

    std::fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create directory {}", target_dir.display()))?;

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let source_path = entry.path();

        if source_path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let relative_path = source_path.strip_prefix(source_dir).with_context(|| {
            format!("Failed to get relative path for {}", source_path.display())
        })?;

        let mut target_path = target_dir.join(relative_path);
        if let Some(new_ext) = change_extension {
            target_path.set_extension(new_ext);
        }

        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        let content = std::fs::read_to_string(source_path)
            .with_context(|| format!("Failed to read {}", source_path.display()))?;

        std::fs::write(&target_path, content)
            .with_context(|| format!("Failed to write {}", target_path.display()))?;

        created_files.push(target_path.display().to_string());
    }

    Ok(created_files)
}

pub fn copy_hierarchy_with_extension_change(
    source_dir: &Path,
    target_dir: &Path,
    from_ext: &str,
    to_ext: &str,
) -> Result<Vec<String>> {
    let mut created_files = Vec::new();

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let source_path = entry.path();

        if source_path.extension().and_then(|ext| ext.to_str()) != Some(from_ext) {
            continue;
        }

        let relative_path = source_path.strip_prefix(source_dir).with_context(|| {
            format!("Failed to get relative path for {}", source_path.display())
        })?;

        let mut target_path = target_dir.join(relative_path);
        target_path.set_extension(to_ext);

        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        let content = std::fs::read_to_string(source_path)
            .with_context(|| format!("Failed to read {}", source_path.display()))?;

        std::fs::write(&target_path, content)
            .with_context(|| format!("Failed to write {}", target_path.display()))?;

        created_files.push(target_path.display().to_string());
    }

    Ok(created_files)
}

pub fn read_and_combine_markdown_files(config_dir: &str) -> Result<String> {
    let config_path = Path::new(config_dir);
    let mut combined = String::new();

    let mut files = find_markdown_files(config_path)?
        .into_iter()
        .filter(|path| {
            // commands ディレクトリのみ除外（devin, cursor, claude は含める）
            !path.components().any(|c| c.as_os_str() == "commands")
        })
        .collect::<Vec<_>>();

    files.sort();

    for file_path in files {
        let content = std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;

        let relative_path = file_path
            .strip_prefix(config_path)
            .with_context(|| format!("Failed to get relative path for {}", file_path.display()))?;

        let section_title = relative_path
            .with_extension("")
            .to_string_lossy()
            .replace("/", " / ")
            .replace("-", " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        combined.push_str(&format!("\n## {}\n\n{}\n", section_title, content));
    }

    Ok(combined)
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

    #[test]
    fn test_ensure_config_exists_with_existing_dir() {
        let temp_dir = TempDir::new().unwrap();
        let result = ensure_config_exists(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_ensure_config_exists_with_missing_dir() {
        let result = ensure_config_exists("/path/that/does/not/exist");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_get_project_name() {
        let name = get_project_name();
        assert!(!name.is_empty());
        assert!(name.len() > 0);
    }

    #[test]
    fn test_find_markdown_files() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        create_test_file(dir_path, "test1.md", "# Test 1").unwrap();
        create_test_file(dir_path, "test2.md", "# Test 2").unwrap();
        create_test_file(dir_path, "test.txt", "Not markdown").unwrap();
        create_test_file(dir_path, "subdir/test3.md", "# Test 3").unwrap();

        let files = find_markdown_files(dir_path).unwrap();
        assert_eq!(files.len(), 3);

        let file_names: Vec<_> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(file_names.contains(&"test1.md"));
        assert!(file_names.contains(&"test2.md"));
        assert!(file_names.contains(&"test3.md"));
    }

    #[test]
    fn test_find_all_files() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        create_test_file(dir_path, "test.md", "markdown").unwrap();
        create_test_file(dir_path, "test.txt", "text").unwrap();
        create_test_file(dir_path, "subdir/test.json", "{}").unwrap();

        let files = find_all_files(dir_path).unwrap();
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn test_copy_hierarchy() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let target_dir = temp_dir.path().join("target");

        fs::create_dir_all(&source_dir).unwrap();
        create_test_file(&source_dir, "test1.md", "# Test 1").unwrap();
        create_test_file(&source_dir, "subdir/test2.md", "# Test 2").unwrap();
        create_test_file(&source_dir, "test.txt", "Not copied").unwrap();

        let result = copy_hierarchy(&source_dir, &target_dir, None).unwrap();
        assert_eq!(result.len(), 2);

        assert!(target_dir.join("test1.md").exists());
        assert!(target_dir.join("subdir/test2.md").exists());
        assert!(!target_dir.join("test.txt").exists());
    }

    #[test]
    fn test_copy_hierarchy_with_extension_change() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let target_dir = temp_dir.path().join("target");

        fs::create_dir_all(&source_dir).unwrap();
        create_test_file(&source_dir, "test1.md", "# Test 1").unwrap();
        create_test_file(&source_dir, "subdir/test2.md", "# Test 2").unwrap();

        let result = copy_hierarchy(&source_dir, &target_dir, Some("mdc")).unwrap();
        assert_eq!(result.len(), 2);

        assert!(target_dir.join("test1.mdc").exists());
        assert!(target_dir.join("subdir/test2.mdc").exists());
        assert!(!target_dir.join("test1.md").exists());
    }

    #[test]
    fn test_copy_hierarchy_with_extension_change_function() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let target_dir = temp_dir.path().join("target");

        fs::create_dir_all(&source_dir).unwrap();
        create_test_file(&source_dir, "test1.mdc", "# Test 1").unwrap();
        create_test_file(&source_dir, "test2.mdc", "# Test 2").unwrap();
        create_test_file(&source_dir, "test.txt", "Not copied").unwrap();

        let result =
            copy_hierarchy_with_extension_change(&source_dir, &target_dir, "mdc", "md").unwrap();
        assert_eq!(result.len(), 2);

        assert!(target_dir.join("test1.md").exists());
        assert!(target_dir.join("test2.md").exists());
        assert!(!target_dir.join("test.txt").exists());
    }

    #[test]
    fn test_read_and_combine_markdown_files() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        create_test_file(config_dir, "intro.md", "Introduction content").unwrap();
        create_test_file(config_dir, "rules.md", "Rules content").unwrap();
        create_test_file(config_dir, "commands/deploy.md", "Deploy command").unwrap();

        let result = read_and_combine_markdown_files(config_dir.to_str().unwrap()).unwrap();

        assert!(result.contains("Introduction content"));
        assert!(result.contains("Rules content"));
        assert!(!result.contains("Deploy command"));

        assert!(result.contains("## Intro"));
        assert!(result.contains("## Rules"));
    }

    #[test]
    fn test_read_and_combine_markdown_files_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        let result = read_and_combine_markdown_files(config_dir.to_str().unwrap()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_read_and_combine_markdown_files_with_subdirs() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        create_test_file(config_dir, "frontend/react-rules.md", "React content").unwrap();
        create_test_file(config_dir, "backend/api-rules.md", "API content").unwrap();

        let result = read_and_combine_markdown_files(config_dir.to_str().unwrap()).unwrap();

        assert!(result.contains("React content"));
        assert!(result.contains("API content"));
        assert!(result.contains("## Backend / Api Rules"));
        assert!(result.contains("## Frontend / React Rules"));
    }

    #[test]
    fn test_read_and_combine_markdown_files_includes_tool_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path();

        // 手動作成ファイル
        create_test_file(config_dir, "general.md", "General rules").unwrap();

        // ツールディレクトリのファイル（含まれるべき）
        create_test_file(config_dir, "devin/knowledge1.md", "Devin knowledge 1").unwrap();
        create_test_file(config_dir, "cursor/rules1.md", "Cursor rules").unwrap();
        create_test_file(config_dir, "claude/claude-rules.md", "Claude rules").unwrap();

        // commandsディレクトリのファイル（除外されるべき）
        create_test_file(config_dir, "commands/deploy.md", "Deploy command").unwrap();

        let result = read_and_combine_markdown_files(config_dir.to_str().unwrap()).unwrap();

        // 含まれるべきもの
        assert!(result.contains("General rules"));
        assert!(result.contains("Devin knowledge 1"));
        assert!(result.contains("Cursor rules"));
        assert!(result.contains("Claude rules"));

        // 除外されるべきもの
        assert!(!result.contains("Deploy command"));

        // セクションタイトルの確認
        assert!(result.contains("## General"));
        assert!(result.contains("## Devin / Knowledge1"));
        assert!(result.contains("## Cursor / Rules1"));
        assert!(result.contains("## Claude / Claude Rules"));
    }
}
