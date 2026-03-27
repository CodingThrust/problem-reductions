use crate::dispatch::{load_problem, read_input, ProblemJson};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use std::path::Path;

pub fn export(input: &Path, format: &str, out: &OutputConfig) -> Result<()> {
    if format != "lp" {
        anyhow::bail!("Unknown format: {}. Available formats: lp", format);
    }

    let content = read_input(input)?;
    let pj: ProblemJson = serde_json::from_str(&content).context("Failed to parse problem JSON")?;

    let problem = load_problem(&pj.problem_type, &pj.variant, pj.data)?;
    let lp_string = problem.export_lp().ok_or_else(|| {
        anyhow::anyhow!(
            "Problem type {} does not support LP-format export. \
             Use `pred reduce` to reduce it to ILP first, then export.",
            pj.problem_type
        )
    })?;

    let json = serde_json::json!({
        "format": format,
        "problem": pj.problem_type,
        "content": &lp_string,
    });
    out.emit_with_default_name("", &lp_string, &json)
}
