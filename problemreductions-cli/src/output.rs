use anyhow::Context;
use owo_colors::OwoColorize;
use std::io::IsTerminal;
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

/// Whether colored output should be used (TTY + not NO_COLOR).
pub fn use_color() -> bool {
    std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

/// Format a problem name (bold when color is enabled).
pub fn fmt_problem_name(name: &str) -> String {
    if use_color() {
        format!("{}", name.bold())
    } else {
        name.to_string()
    }
}

/// Format a section header (cyan when color is enabled).
pub fn fmt_section(text: &str) -> String {
    if use_color() {
        format!("{}", text.cyan())
    } else {
        text.to_string()
    }
}

/// Format an outgoing arrow (green when color is enabled).
pub fn fmt_arrow_out() -> &'static str {
    // We return static str, so we use ANSI directly for the arrow
    "\u{2192}"
}

pub fn fmt_outgoing(text: &str) -> String {
    if use_color() {
        format!("{}", text.green())
    } else {
        text.to_string()
    }
}

pub fn fmt_incoming(text: &str) -> String {
    if use_color() {
        format!("{}", text.red())
    } else {
        text.to_string()
    }
}

/// Format dim text (for aliases, tree branches).
pub fn fmt_dim(text: &str) -> String {
    if use_color() {
        format!("{}", text.dimmed())
    } else {
        text.to_string()
    }
}
