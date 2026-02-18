use anyhow::Context;
use std::path::PathBuf;

/// Output configuration derived from CLI flags.
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// If true, output as JSON to a file.
    pub json: bool,
    /// Custom output file path. If None, a default name is used.
    pub output: Option<PathBuf>,
}

impl OutputConfig {
    /// Emit with a custom default filename.
    pub fn emit_with_default_name(
        &self,
        default_name: &str,
        human_text: &str,
        json_value: &serde_json::Value,
    ) -> anyhow::Result<()> {
        if self.json {
            let path = self
                .output
                .clone()
                .unwrap_or_else(|| PathBuf::from(default_name));
            let content =
                serde_json::to_string_pretty(json_value).context("Failed to serialize JSON")?;
            std::fs::write(&path, &content)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            eprintln!("Wrote {}", path.display());
        } else {
            println!("{human_text}");
        }
        Ok(())
    }
}
