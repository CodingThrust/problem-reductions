use crate::dispatch::{read_input, BundleReplay, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::solvers::{SolveOutcome, SolverExecution};
use serde::Deserialize;
use std::path::Path;

const RESULT_SHAPE: &str = r#"Target result must be {"status": "optimal"|"feasible", "solution": [...]} or {"status": "infeasible"}; a bare configuration is no longer accepted, wrap it as {"status": "feasible", "solution": [...]}"#;

/// Status envelope of a target result; rules validate the typed solution.
#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
#[allow(dead_code)]
enum ResultEnvelope {
    Optimal { solution: serde::de::IgnoredAny },
    Feasible { solution: serde::de::IgnoredAny },
    Infeasible,
}

impl ResultEnvelope {
    /// The unverified external claim, if any, that a source `infeasible` result rests on.
    fn infeasibility_note(&self) -> Option<&'static str> {
        match self {
            Self::Optimal { .. } => Some(
                r#"note: source infeasibility rests on the external solver's optimality claim; pass "status":"feasible" unless optimality was proven"#,
            ),
            Self::Infeasible => Some(
                "note: source infeasibility rests on the external solver's infeasibility claim; pred did not verify it",
            ),
            Self::Feasible { .. } => None,
        }
    }
}

/// Recover the source result from an external solver's explicit target result.
pub fn extract(input: &Path, result_path: &Path, out: &OutputConfig) -> Result<()> {
    let bundle: ReductionBundle = serde_json::from_str(&read_input(input)?)
        .context("pred extract requires a reduction bundle produced by pred reduce")?;
    let target: serde_json::Value =
        serde_json::from_str(&read_input(result_path)?).context("Target result is not JSON")?;
    let claimed = ResultEnvelope::deserialize(&target).context(RESULT_SHAPE)?;
    let replay = BundleReplay::prepare(&bundle)?;
    let result = replay.recover_result(target, SolverExecution::External)?;
    out.emit(
        || {
            let mut text = format!(
                "Problem: {}\nSolver: external (via {})",
                result.source_name, result.target_name
            );
            super::solve::append_outcome_text(&mut text, &result.source_outcome);
            text
        },
        || Ok(result.to_json()),
    )?;
    // stdout stays the machine-readable result; the caveat goes to stderr.
    if matches!(result.source_outcome, SolveOutcome::Infeasible) {
        if let Some(note) = claimed.infeasibility_note() {
            out.info(note);
        }
    }
    Ok(())
}
