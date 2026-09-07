use crate::dispatch::{load_problem, read_input, ProblemJson};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use std::path::Path;

pub fn evaluate(input: &Path, config_str: &str, out: &OutputConfig) -> Result<()> {
    let content = read_input(input)?;
    let json: serde_json::Value =
        serde_json::from_str(&content).context("Input is not valid JSON")?;

    if json.get("source").is_some() && json.get("target").is_some() && json.get("path").is_some() {
        anyhow::bail!(
            "Input is a reduction bundle, not a problem instance.\n\
             `pred evaluate` only works on problem files (from `pred create`).\n\
             To solve a bundle, use: pred solve <bundle>"
        );
    }

    let problem_json: ProblemJson =
        serde_json::from_value(json).context("Failed to parse problem JSON")?;

    let problem = load_problem(
        &problem_json.problem_type,
        &problem_json.variant,
        problem_json.data,
    )?;

    let config: serde_json::Value =
        serde_json::from_str(config_str).context("Config is not valid JSON")?;

    let result = problem.evaluate_dyn(&config)?;

    out.emit(
        || result.to_string(),
        || {
            Ok(serde_json::json!({
                "problem": problem.problem_name(),
                "config": config,
                "result": result,
            }))
        },
    )
}
