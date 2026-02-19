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

/// Whether stderr is connected to a TTY (used to suppress hints in piped output).
pub fn stderr_is_tty() -> bool {
    std::io::stderr().is_terminal()
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

pub fn fmt_outgoing(text: &str) -> String {
    if use_color() {
        format!("{}", text.green())
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

/// Function that transforms cell text for display (e.g., adding color).
pub type CellFormatter = fn(&str) -> String;

/// Column alignment specification for table formatting.
pub enum Align {
    Left,
    Right,
}

/// Format data as an aligned table.
///
/// Each column is defined by a `(header, alignment, width)` tuple.
/// `width` is auto-expanded to fit the header. Rows provide one string per column.
/// An optional `color_fn` can transform cell text for display (widths are computed
/// on the raw text before coloring).
pub fn format_table(
    columns: &[(&str, Align, usize)],
    rows: &[Vec<String>],
    color_fns: &[Option<CellFormatter>],
) -> String {
    // Compute actual column widths (max of header width, specified width, and data width)
    let widths: Vec<usize> = columns
        .iter()
        .enumerate()
        .map(|(i, (header, _, min_w))| {
            let data_max = rows.iter().map(|r| r[i].len()).max().unwrap_or(0);
            data_max.max(*min_w).max(header.len())
        })
        .collect();

    let mut text = String::new();

    // Header
    text.push_str("  ");
    for (i, (header, align, _)) in columns.iter().enumerate() {
        if i > 0 {
            text.push_str("  ");
        }
        match align {
            Align::Left => text.push_str(&format!("{:<w$}", header, w = widths[i])),
            Align::Right => text.push_str(&format!("{:>w$}", header, w = widths[i])),
        }
    }
    text.push('\n');

    // Separator
    text.push_str("  ");
    for (i, _) in columns.iter().enumerate() {
        if i > 0 {
            text.push_str("  ");
        }
        text.push_str(&"─".repeat(widths[i]));
    }
    text.push('\n');

    // Data rows
    for row in rows {
        text.push_str("  ");
        for (i, (_, align, _)) in columns.iter().enumerate() {
            if i > 0 {
                text.push_str("  ");
            }
            let cell = &row[i];
            let padded = match align {
                Align::Left => format!("{:<w$}", cell, w = widths[i]),
                Align::Right => format!("{:>w$}", cell, w = widths[i]),
            };
            if let Some(Some(f)) = color_fns.get(i) {
                // Pad first, then colorize (so ANSI codes don't affect width)
                text.push_str(&f(&padded));
            } else {
                text.push_str(&padded);
            }
        }
        text.push('\n');
    }

    text
}
