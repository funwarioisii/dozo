use crate::commands::utils::{copy_hierarchy, get_project_name, read_and_combine_markdown_files};
use anyhow::{Context, Result};
use std::path::Path;

pub fn generate_files(config_path: &Path, force: bool) -> Result<Vec<String>> {
    let mut generated_files = Vec::new();

    let main_file = Path::new("CLAUDE.md");
    if main_file.exists() && !force {
        anyhow::bail!(
            "File '{}' already exists. Use --force to overwrite.",
            main_file.display()
        );
    }

    let combined_content = read_and_combine_markdown_files(config_path.to_str().unwrap())?;
    let project_name = get_project_name();
    let final_content = format!(
        "# {} - Claude Memory\n\n## プロジェクト情報\n- **プロジェクト名**: {}\n\n## コマンド例\n```bash\n# プロジェクトのビルド\nnpm run build\n\n# テストの実行\nnpm test\n\n# 開発サーバーの起動\nnpm run dev\n```{}",
        project_name, project_name, combined_content
    );

    std::fs::write(main_file, final_content)
        .with_context(|| format!("Failed to write to {}", main_file.display()))?;
    generated_files.push(main_file.display().to_string());

    let commands_source = config_path.join("commands");
    if commands_source.exists() {
        let commands_target = Path::new(".claude/commands");

        if commands_target.exists() {
            if !force {
                anyhow::bail!(
                    "Directory '{}' already exists. Use --force to overwrite.",
                    commands_target.display()
                );
            }
            std::fs::remove_dir_all(commands_target).with_context(|| {
                format!("Failed to remove existing {}", commands_target.display())
            })?;
        }

        let mut command_files = copy_hierarchy(&commands_source, commands_target, None)?;
        generated_files.append(&mut command_files);
    }

    Ok(generated_files)
}
