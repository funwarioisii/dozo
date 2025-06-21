use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Test fixture for creating temporary test environments
pub struct TestEnv {
    pub temp_dir: TempDir,
    pub config_dir: std::path::PathBuf,
    old_dir: std::path::PathBuf,
}

impl TestEnv {
    /// Creates a new test environment with temporary directory
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).unwrap();

        let old_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        Self {
            temp_dir,
            config_dir,
            old_dir,
        }
    }

    /// Creates a test file with the given path and content
    pub fn create_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        create_test_file(&self.config_dir, path, content)
    }

    /// Creates a test file in the temp directory root
    pub fn create_temp_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        create_test_file(self.temp_dir.path(), path, content)
    }

    /// Gets the temp directory path
    pub fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Creates typical configuration files for testing
    pub fn setup_typical_config(&self) -> anyhow::Result<()> {
        self.create_file("intro.md", "# Introduction\nProject introduction")?;
        self.create_file("rules.md", "# Rules\nCoding rules and guidelines")?;
        self.create_file(
            "frontend/react-rules.md",
            "# React Rules\nReact-specific guidelines",
        )?;
        self.create_file(
            "backend/api-rules.md",
            "# API Rules\nAPI development guidelines",
        )?;
        self.create_file("commands/deploy.md", "# Deploy\nDeployment commands")?;
        self.create_file("commands/test.md", "# Test\nTesting commands")?;
        Ok(())
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        // Restore original directory
        let _ = std::env::set_current_dir(&self.old_dir);
    }
}

/// Creates a test file with the given path and content
pub fn create_test_file(dir: &Path, name: &str, content: &str) -> anyhow::Result<()> {
    let file_path = dir.join(name);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(file_path, content)?;
    Ok(())
}

/// Asserts that a file exists and contains expected content
pub fn assert_file_contains(path: &Path, expected: &str) {
    assert!(path.exists(), "File should exist: {}", path.display());
    let content = fs::read_to_string(path).unwrap();
    assert!(
        content.contains(expected),
        "File {} should contain '{}' but content is: {}",
        path.display(),
        expected,
        content
    );
}

/// Asserts that a file exists with exact content
pub fn assert_file_content(path: &Path, expected: &str) {
    assert!(path.exists(), "File should exist: {}", path.display());
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(
        content,
        expected,
        "File {} should have exact content",
        path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_setup() {
        let env = TestEnv::new();
        assert!(env.temp_path().exists());
        assert!(env.config_dir.exists());
    }

    #[test]
    fn test_create_file() {
        let env = TestEnv::new();
        env.create_file("test.md", "# Test").unwrap();
        assert!(env.config_dir.join("test.md").exists());
    }

    #[test]
    fn test_setup_typical_config() {
        let env = TestEnv::new();
        env.setup_typical_config().unwrap();

        assert!(env.config_dir.join("intro.md").exists());
        assert!(env.config_dir.join("rules.md").exists());
        assert!(env.config_dir.join("frontend/react-rules.md").exists());
        assert!(env.config_dir.join("commands/deploy.md").exists());
    }

    #[test]
    fn test_helper_functions() {
        let env = TestEnv::new();
        env.create_temp_file("test.txt", "test content").unwrap();

        assert_file_contains(&env.temp_path().join("test.txt"), "test");
        assert_file_content(&env.temp_path().join("test.txt"), "test content");
    }
}
