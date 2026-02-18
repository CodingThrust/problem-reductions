use anyhow::Context;
use std::path::PathBuf;

/// Output configuration derived from CLI flags.
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Output file path. When set, output is saved as JSON.
    pub output: Option<PathBuf>,
}

impl OutputConfig {
    /// Emit output: if `-o` is set, save as JSON; otherwise print human text.
    pub fn emit_with_default_name(
        &self,
        _default_name: &str,
        human_text: &str,
        json_value: &serde_json::Value,
    ) -> anyhow::Result<()> {
        if let Some(ref path) = self.output {
            let content =
                serde_json::to_string_pretty(json_value).context("Failed to serialize JSON")?;
            std::fs::write(path, &content)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            eprintln!("Wrote {}", path.display());
        } else {
            println!("{human_text}");
        }
        Ok(())
    }
}
