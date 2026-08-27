use crate::dispatch::{
    load_problem, read_input, solve_result_json, solver_request, BundleReplay, ProblemJson,
    ReductionBundle,
};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::solvers::{SolveOutcome, SolveResult, SolverExecution, SolverRequest};
use std::path::Path;
use std::time::Duration;

/// Input can be either a problem JSON or a reduction bundle JSON.
enum SolveInput {
    /// A plain problem file (from `pred create`).
    Problem(ProblemJson),
    /// A reduction bundle (from `pred reduce`) with source, target, and path.
    Bundle(ReductionBundle),
}

fn parse_input(path: &Path) -> Result<SolveInput> {
    let content = read_input(path)?;
    let json: serde_json::Value = serde_json::from_str(&content).context("Failed to parse JSON")?;

    // Reduction bundles have "source", "target", and "path" fields
    if json.get("source").is_some() && json.get("target").is_some() && json.get("path").is_some() {
        let bundle: ReductionBundle =
            serde_json::from_value(json).context("Failed to parse reduction bundle")?;
        Ok(SolveInput::Bundle(bundle))
    } else {
        let problem: ProblemJson =
            serde_json::from_value(json).context("Failed to parse problem JSON")?;
        Ok(SolveInput::Problem(problem))
    }
}

fn solver_text(solver: &SolverExecution) -> String {
    match solver {
        SolverExecution::Customized { implementation } => format!("customized ({implementation})"),
        SolverExecution::Ilp { reduction_path } => {
            format!("ilp ({})", reduction_path.join(" -> "))
        }
        SolverExecution::BruteForce => "brute-force".to_string(),
    }
}

fn solve_result_text(problem: &str, result: &SolveResult) -> String {
    let mut text = format!(
        "Problem: {}\nSolver: {}",
        problem,
        solver_text(&result.solver)
    );
    append_outcome_text(&mut text, &result.outcome);
    text
}

fn append_outcome_text(text: &mut String, outcome: &SolveOutcome) {
    match outcome {
        SolveOutcome::Optimal {
            solution,
            evaluation,
        } => {
            text.push_str("\nStatus: optimal");
            text.push_str(&format!("\nSolution: {:?}", solution));
            text.push_str(&format!("\nEvaluation: {evaluation}"));
        }
        SolveOutcome::Infeasible => text.push_str("\nStatus: infeasible"),
    }
}

#[cfg(test)]
fn plain_problem_output(problem: &str, result: &SolveResult) -> (String, serde_json::Value) {
    (
        solve_result_text(problem, result),
        solve_result_json(problem, result),
    )
}

pub fn solve(
    input: &Path,
    solver_name: Option<&str>,
    timeout: i64,
    out: &OutputConfig,
) -> Result<()> {
    let request = solver_request(solver_name)?;

    let parsed = parse_input(input)?;

    let timeout_seconds =
        u64::try_from(timeout).map_err(|_| anyhow::anyhow!("timeout must be a nonnegative i64"))?;

    if timeout_seconds > 0 {
        let out = out.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = match parsed {
                SolveInput::Problem(pj) => {
                    solve_problem(&pj.problem_type, &pj.variant, pj.data, request, &out)
                }
                SolveInput::Bundle(b) => solve_bundle(b, request, &out),
            };
            tx.send(result).ok();
        });
        match rx.recv_timeout(Duration::from_secs(timeout_seconds)) {
            Ok(result) => result,
            Err(_) => anyhow::bail!("Solve timed out after {} seconds", timeout_seconds),
        }
    } else {
        match parsed {
            SolveInput::Problem(pj) => {
                solve_problem(&pj.problem_type, &pj.variant, pj.data, request, out)
            }
            SolveInput::Bundle(b) => solve_bundle(b, request, out),
        }
    }
}

/// Solve a plain problem file directly.
fn solve_problem(
    problem_type: &str,
    variant: &std::collections::BTreeMap<String, String>,
    data: serde_json::Value,
    request: SolverRequest,
    out: &OutputConfig,
) -> Result<()> {
    let problem = load_problem(problem_type, variant, data)?;
    let name = problem.problem_name();
    let result = problem.solve(request).map_err(add_solver_hint)?;
    let emitted = out.emit(
        || solve_result_text(name, &result),
        || Ok(solve_result_json(name, &result)),
    );
    if out.output.is_none() && crate::output::stderr_is_tty() {
        out.info("\nHint: use -o to save full solution details as JSON.");
    }
    emitted
}

/// Solve a reduction bundle: solve the target problem, then map the solution back.
fn solve_bundle(bundle: ReductionBundle, request: SolverRequest, out: &OutputConfig) -> Result<()> {
    let replay = BundleReplay::prepare(&bundle)?;
    let result = replay.solve(request).map_err(add_solver_hint)?;

    let emitted = out.emit(
        || {
            let solver_desc = format!(
                "{} (via {})",
                solver_text(&result.solver),
                result.target_name
            );
            let mut text = format!("Problem: {}\nSolver: {}", result.source_name, solver_desc);
            append_outcome_text(&mut text, &result.source_outcome);
            text
        },
        || Ok(result.to_json()),
    );
    if out.output.is_none() && crate::output::stderr_is_tty() {
        out.info("\nHint: use -o to save full solution details (including intermediate results) as JSON.");
    }
    emitted
}

fn add_solver_hint(err: anyhow::Error) -> anyhow::Error {
    let missing_capability = err
        .downcast_ref::<problemreductions::solvers::SolveError>()
        .is_some_and(|error| {
            matches!(
                error,
                problemreductions::solvers::SolveError::MissingIlpCapability(_)
                    | problemreductions::solvers::SolveError::MissingCustomizedCapability(_)
            )
        });
    if missing_capability {
        let message = err.to_string();
        anyhow::anyhow!(
            "{message}\n\nHint: try `--solver brute-force` for direct exhaustive search on small instances."
        )
    } else {
        err
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::OutputConfig;
    use crate::test_support::aggregate_bundle;

    #[test]
    fn test_solve_result_contains_solution() {
        let result = SolveResult {
            solver: SolverExecution::BruteForce,
            outcome: SolveOutcome::Optimal {
                solution: serde_json::json!([true, true, true]),
                evaluation: "Max(14)".to_string(),
            },
        };
        let (text, json) = plain_problem_output("CliTestAggregateValueSource", &result);
        assert!(text.contains("Evaluation: Max(14)"), "{text}");
        assert!(text.contains("Solution:"), "{text}");
        assert_eq!(json["solution"], serde_json::json!([true, true, true]));
        assert_eq!(json["status"], "optimal");
    }

    #[test]
    fn test_solve_bundle_rejects_aggregate_only_path() {
        let bundle = aggregate_bundle();
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = solve_bundle(bundle, SolverRequest::BruteForce, &out).unwrap_err();
        assert!(
            err.to_string().contains("witness"),
            "unexpected error: {err}"
        );
    }
}
