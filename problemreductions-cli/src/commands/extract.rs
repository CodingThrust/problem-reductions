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

    let target_config: serde_json::Value =
        serde_json::from_str(config_str).context("Target config is not valid JSON")?;

    let replay = BundleReplay::prepare(&bundle)?;

    let target_eval = replay.target.evaluate_dyn(&target_config)?;

    let (source_config, source_eval) = replay.extract(&target_config)?;

    out.emit(
        || {
            format!(
                "Problem: {}\nSolver: external (via {})\nSolution: {:?}\nEvaluation: {}",
                replay.source_name, replay.target_name, source_config, source_eval,
            )
        },
        || {
            // Schema aligned with `pred solve` on a bundle. `solver` is "external"
            // because pred did not run the solver that produced the target config.
            Ok(serde_json::json!({
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
            }))
        },
    )
}
