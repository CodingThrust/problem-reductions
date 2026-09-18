use crate::cli::ExtractArgs;
use crate::dispatch::{read_input, BundleReplay, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::rules::ReductionMode;
use problemreductions::solvers::SolveOutcome;

#[derive(serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum ExternalResult {
    Optimal { solution: serde_json::Value },
    Infeasible,
}

/// Recover a candidate, completed exact result, or aggregate through a bundle.
/// `--result` accepts solve-output metadata, but validates any supplied evaluation.
/// The external solver is responsible for proving optimality or infeasibility.
pub fn extract(args: &ExtractArgs, out: &OutputConfig) -> Result<()> {
    let content = read_input(&args.input)?;
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

    if let Some(value) = &args.value {
        let replay = BundleReplay::prepare(&bundle, ReductionMode::Aggregate)?;
        let value = replay.extract_value(
            serde_json::from_str(value).context("Target aggregate is not valid JSON")?,
        )?;
        return out.emit(
            || format!("Problem: {}\nAggregate: {value}", replay.source_name),
            || Ok(serde_json::json!({"problem": replay.source_name, "aggregate": value})),
        );
    }
    let replay = BundleReplay::prepare(&bundle, ReductionMode::Witness)?;
    if let Some(path) = &args.result {
        let json: serde_json::Value =
            serde_json::from_str(&read_input(path)?).context("Invalid completed target result")?;
        if json
            .pointer("/solver/kind")
            .and_then(serde_json::Value::as_str)
            == Some("ilp")
        {
            anyhow::bail!("numerical ILP status is not an exact certificate; recover a candidate with --config")
        }
        let evaluation = json.get("evaluation").cloned();
        let external: ExternalResult =
            serde_json::from_value(json).context("Invalid completed target result")?;
        let target = match external {
            ExternalResult::Optimal { solution } => {
                let actual = replay.target.evaluate_dyn(&solution)?;
                if evaluation
                    .as_ref()
                    .is_some_and(|value| value != &serde_json::json!(actual))
                {
                    anyhow::bail!("target evaluation does not match the witness")
                }
                SolveOutcome::Optimal {
                    solution,
                    evaluation: actual,
                }
            }
            ExternalResult::Infeasible => {
                if evaluation.is_some() {
                    anyhow::bail!("infeasible results do not have a witness evaluation")
                }
                SolveOutcome::Infeasible
            }
        };
        let source = replay.extract_result(&target)?;
        return out.emit(
            || format!("Problem: {}\nResult: {source:?}", replay.source_name),
            || {
                let mut json = serde_json::to_value(&source)?;
                json["problem"] = serde_json::json!(replay.source_name);
                json["intermediate"] = serde_json::to_value(&target)?;
                Ok(json)
            },
        );
    }
    let config_str = args
        .config
        .as_deref()
        .context("provide --config, --result, or --value")?;
    let target_config: serde_json::Value =
        serde_json::from_str(config_str).context("Target config is not valid JSON")?;

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
