use crate::dispatch::{read_input, BundleReplay, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::solvers::SolverExecution;
use std::path::Path;

/// Recover the source result from an external solver's explicit target result.
pub fn extract(input: &Path, result_path: &Path, out: &OutputConfig) -> Result<()> {
    let bundle: ReductionBundle = serde_json::from_str(&read_input(input)?)
        .context("pred extract requires a reduction bundle produced by pred reduce")?;
    let target: serde_json::Value = serde_json::from_str(&read_input(result_path)?)
        .context("Target result must declare optimal, feasible, or infeasible status")?;
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
    )
}
