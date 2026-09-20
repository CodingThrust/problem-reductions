use crate::cli::ExtractArgs;
use crate::dispatch::{extract_bundle_value, read_input, BundleReplay, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::solvers::SolveOutcome;
use serde_json::Value;
use std::path::Path;

#[derive(serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum ExternalResult {
    Feasible { solution: Value },
    Optimal { solution: Value },
    Infeasible {},
    Complete { value: Value },
}

fn load_bundle(input: &Path) -> Result<ReductionBundle> {
    let content = read_input(input)?;
    let json: serde_json::Value =
        serde_json::from_str(&content).context("Input is not valid JSON")?;

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

    Ok(bundle)
}

/// Recover the explicitly stated target result, without upgrading its guarantee.
pub fn extract(args: &ExtractArgs, out: &OutputConfig) -> Result<()> {
    let bundle = load_bundle(&args.input)?;
    let mut json: serde_json::Map<String, Value> =
        serde_json::from_str(&read_input(&args.result)?).context("Invalid target result")?;
    let evaluation = json.remove("evaluation");
    // These fields describe pred output; they do not change the recovery contract.
    for metadata in ["problem", "solver", "reduced_to", "intermediate"] {
        json.remove(metadata);
    }
    let external: ExternalResult =
        serde_json::from_value(Value::Object(json)).context("Invalid target result")?;
    match external {
        ExternalResult::Complete { value } => {
            if evaluation.is_some() {
                anyhow::bail!("complete value results do not have a witness evaluation");
            }
            let source_value = extract_bundle_value(&bundle, value.clone())?;
            out.emit(
                || {
                    format!(
                        "Problem: {}\nStatus: complete\nValue: {source_value}",
                        bundle.source.problem_type
                    )
                },
                || {
                    Ok(serde_json::json!({
                        "problem": bundle.source.problem_type,
                        "status": "complete",
                        "value": source_value,
                        "intermediate": {"status": "complete", "value": value},
                    }))
                },
            )
        }
        ExternalResult::Infeasible {} => {
            if evaluation.is_some() {
                anyhow::bail!("infeasible results do not have a witness evaluation");
            }
            let replay = BundleReplay::prepare(&bundle)?;
            emit_completed(&replay, SolveOutcome::Infeasible, out)
        }
        ExternalResult::Feasible { ref solution } | ExternalResult::Optimal { ref solution } => {
            let replay = BundleReplay::prepare(&bundle)?;
            let actual = replay.target.evaluate_dyn(solution)?;
            if evaluation
                .as_ref()
                .is_some_and(|value| value != &serde_json::json!(actual))
            {
                anyhow::bail!("target evaluation does not match the witness");
            }
            if matches!(external, ExternalResult::Optimal { .. }) {
                return emit_completed(
                    &replay,
                    SolveOutcome::Optimal {
                        solution: solution.clone(),
                        evaluation: actual,
                    },
                    out,
                );
            }
            let (source_solution, source_evaluation) = replay.extract(solution)?;
            out.emit(
                || format!("Problem: {}\nStatus: feasible\nSolution: {source_solution}\nEvaluation: {source_evaluation}", replay.source_name),
                || Ok(serde_json::json!({
                    "problem": replay.source_name,
                    "solver": "external",
                    "reduced_to": replay.target_name,
                    "status": "feasible",
                    "solution": source_solution,
                    "evaluation": source_evaluation,
                    "intermediate": {
                        "problem": replay.target_name,
                        "status": "feasible",
                        "solution": solution,
                        "evaluation": actual,
                    },
                })),
            )
        }
    }
}

fn emit_completed(replay: &BundleReplay, target: SolveOutcome, out: &OutputConfig) -> Result<()> {
    let source = replay.extract_result(&target)?;
    out.emit(
        || {
            let mut text = format!("Problem: {}", replay.source_name);
            super::solve::append_outcome_text(&mut text, &source);
            text
        },
        || {
            let mut json = serde_json::to_value(&source)?;
            json["problem"] = serde_json::json!(replay.source_name);
            json["intermediate"] = serde_json::to_value(&target)?;
            Ok(json)
        },
    )
}
