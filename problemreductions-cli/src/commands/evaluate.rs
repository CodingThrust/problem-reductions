use crate::dispatch::{load_problem, ProblemJson};
use crate::output::OutputConfig;
use anyhow::Result;
use std::path::Path;

pub fn evaluate(input: &Path, config_str: &str, out: &OutputConfig) -> Result<()> {
    let content = std::fs::read_to_string(input)?;
    let problem_json: ProblemJson = serde_json::from_str(&content)?;

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

    let result = problem.evaluate_dyn(&config);

    let text = result.to_string();
    let json = serde_json::json!({
        "problem": problem.problem_name(),
        "config": config,
        "result": result,
    });

    out.emit_with_default_name("pred_evaluate.json", &text, &json)
}
