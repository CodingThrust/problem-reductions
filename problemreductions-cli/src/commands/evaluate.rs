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

    let config: Vec<usize> = config_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Invalid config value '{}': {}", s.trim(), e))
        })
        .collect::<Result<Vec<_>>>()?;

    let dims = problem.dims_dyn();
    if config.len() != dims.len() {
        anyhow::bail!(
            "Config has {} values but problem has {} variables",
            config.len(),
            dims.len()
        );
    }

    for (i, (val, dim)) in config.iter().zip(dims.iter()).enumerate() {
        if *val >= *dim {
            anyhow::bail!(
                "Config value {} at position {} is out of range: variable {} has {} possible values (0..{})",
                val, i, i, dim, dim.saturating_sub(1)
            );
        }
    }

    let result = problem.evaluate_dyn(&config);

    let text = result.to_string();
    let json = serde_json::json!({
        "problem": problem.problem_name(),
        "config": config,
        "result": result,
    });

    out.emit_with_default_name("pred_evaluate.json", &text, &json)
}
