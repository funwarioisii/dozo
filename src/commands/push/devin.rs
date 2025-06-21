use anyhow::Result;
use std::path::Path;

pub fn generate_files(_config_path: &Path, _force: bool) -> Result<Vec<String>> {
    anyhow::bail!("❌ Devin push is not yet implemented. Currently only pull is supported.\n\nTo pull knowledge from Devin:\n  dozo pull --from devin\n\nNote: Requires DEVIN_API_KEY environment variable to be set.")
}
