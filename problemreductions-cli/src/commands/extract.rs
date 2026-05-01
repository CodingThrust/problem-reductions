use crate::dispatch::{read_input, BundleReplay, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use std::path::Path;

/// Extract a source-space configuration from a target-space configuration and a reduction bundle.
///
/// This lets external solvers (that solved the bundle's target problem on their own)
/// recover a solution in the original source problem space without having to
/// re-solve through `pred solve`.
pub fn extract(input: &Path, config_str: &str, out: &OutputConfig) -> Result<()> {
    let content = read_input(input)?;
    let json: serde_json::Value =
        serde_json::from_str(&content).context("Input is not valid JSON")?;

    if !(json.get("source").is_some() && json.get("target").is_some() && json.get("path").is_some())
    {
        anyhow::bail!(
            "Input is not a reduction bundle.\n\
             `pred extract` requires a bundle produced by `pred reduce`.\n\
             Got a plain problem file; did you mean `pred evaluate`?"
        );
    }

    let bundle: ReductionBundle =
        serde_json::from_value(json).context("Failed to parse reduction bundle")?;

    // An empty --config means an empty target configuration (zero-variable target problem).
    let target_config: Vec<usize> = if config_str.trim().is_empty() {
        Vec::new()
    } else {
        config_str
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Invalid config value '{}': {}", s.trim(), e))
            })
            .collect::<Result<Vec<_>>>()?
    };

    let replay = BundleReplay::prepare(&bundle)?;

    let target_dims = replay.target.dims_dyn();
    if target_config.len() != target_dims.len() {
        anyhow::bail!(
            "Target config has {} values but target problem {} has {} variables",
            target_config.len(),
            replay.target_name,
            target_dims.len()
        );
    }
    for (i, (val, dim)) in target_config.iter().zip(target_dims.iter()).enumerate() {
        if *val >= *dim {
            anyhow::bail!(
                "Target config value {} at position {} is out of range: variable {} has {} possible values (0..{})",
                val, i, i, dim, dim.saturating_sub(1)
            );
        }
    }
    let target_eval = replay.target.evaluate_dyn(&target_config);

    let (source_config, source_eval) = replay.extract(&target_config);

    let text = format!(
        "Problem: {}\nSolver: external (via {})\nSolution: {:?}\nEvaluation: {}",
        replay.source_name, replay.target_name, source_config, source_eval,
    );

    // Schema aligned with `pred solve` on a bundle: `problem`, `reduced_to`, `solution`,
    // `evaluation`, `intermediate { problem, solution, evaluation }`. `solver` is "external"
    // to signal that pred did not run a solver — the target config came from outside.
    let json = serde_json::json!({
        "problem": replay.source_name,
        "solver": "external",
        "reduced_to": replay.target_name,
        "solution": source_config,
        "evaluation": source_eval,
        "intermediate": {
            "problem": replay.target_name,
            "solution": target_config,
            "evaluation": target_eval,
        },
    });

    out.emit_with_default_name("pred_extract.json", &text, &json)
}
