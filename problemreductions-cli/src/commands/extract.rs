use crate::cli::ExtractArgs;
use crate::dispatch::{extract_bundle_value, read_input, BundleReplay, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};

/// Apply the reduction rules' configuration or aggregate-value mapping.
pub fn extract(args: &ExtractArgs, out: &OutputConfig) -> Result<()> {
    let json: serde_json::Value =
        serde_json::from_str(&read_input(&args.input)?).context("Input is not valid JSON")?;
    if !(json.get("source").is_some() && json.get("target").is_some() && json.get("path").is_some())
    {
        anyhow::bail!(
            "Input is not a reduction bundle.\n\
             Extraction requires a bundle produced by `pred reduce`.\n\
             Got a plain problem file; did you mean `pred evaluate`?"
        );
    }
    let bundle: ReductionBundle =
        serde_json::from_value(json).context("Failed to parse reduction bundle")?;
    if let Some(value) = &args.value {
        let value = serde_json::from_str(value).context("Target value is not valid JSON")?;
        let source_value = extract_bundle_value(&bundle, value)?;
        return out.emit(
            || format!("Problem: {}\nValue: {source_value}", bundle.source.problem_type),
            || Ok(serde_json::json!({"problem": bundle.source.problem_type, "value": source_value})),
        );
    }
    if let Some(config) = &args.config {
        let solution = serde_json::from_str(config).context("Target config is not valid JSON")?;
        let replay = BundleReplay::prepare(&bundle)?;
        let target_evaluation = replay
            .target
            .evaluate_witness_dyn(&solution)?
            .context("target witness is infeasible")?;
        let (source_solution, source_evaluation) = replay.extract(&solution)?;
        out.emit(
            || {
                format!(
                    "Problem: {}\nSolution: {source_solution}\nEvaluation: {source_evaluation}",
                    replay.source_name
                )
            },
            || {
                Ok(serde_json::json!({
                    "problem": replay.source_name,
                    "reduced_to": replay.target_name,
                    "solution": source_solution,
                    "evaluation": source_evaluation,
                    "intermediate": {
                        "problem": replay.target_name,
                        "solution": solution,
                        "evaluation": target_evaluation,
                    },
                }))
            },
        )?;
    }
    Ok(())
}
