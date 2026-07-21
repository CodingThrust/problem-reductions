use crate::dispatch::{
    load_problem, read_input, solve_result_json, solver_request, BundleReplay, ProblemJson,
    ReductionBundle,
};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::solvers::{DeterministicSolveResult, SolverExecution, SolverRequest};
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
        SolverExecution::Native { implementation } => format!("native ({implementation})"),
        SolverExecution::Ilp { reduction_path } => {
            format!("ilp ({})", reduction_path.join(" -> "))
        }
        SolverExecution::BruteForce => "brute-force".to_string(),
    }
}

fn solve_result_text(problem: &str, result: &DeterministicSolveResult) -> String {
    let mut text = format!(
        "Problem: {}\nSolver: {}",
        problem,
        solver_text(&result.solver)
    );
    if let Some(config) = &result.config {
        text.push_str(&format!("\nSolution: {:?}", config));
    }
    text.push_str(&format!("\nEvaluation: {}", result.evaluation));
    text
}

fn plain_problem_output(
    problem: &str,
    result: &DeterministicSolveResult,
) -> (String, serde_json::Value) {
    (
        solve_result_text(problem, result),
        solve_result_json(problem, result),
    )
}

pub fn solve(
    input: &Path,
    solver_name: Option<&str>,
    timeout: u64,
    out: &OutputConfig,
) -> Result<()> {
    let request = solver_request(solver_name)?;

    let parsed = parse_input(input)?;

    if timeout > 0 {
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
        match rx.recv_timeout(Duration::from_secs(timeout)) {
            Ok(result) => result,
            Err(_) => anyhow::bail!("Solve timed out after {} seconds", timeout),
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
    let result = problem
        .solve_deterministically(request)
        .map_err(add_solver_hint)?;
    let (text, json) = plain_problem_output(name, &result);
    let emitted = out.emit_with_default_name("", &text, &json);
    if out.output.is_none() && crate::output::stderr_is_tty() {
        out.info("\nHint: use -o to save full solution details as JSON.");
    }
    emitted
}

/// Solve a reduction bundle: solve the target problem, then map the solution back.
fn solve_bundle(bundle: ReductionBundle, request: SolverRequest, out: &OutputConfig) -> Result<()> {
    let replay = BundleReplay::prepare(&bundle)?;

    let target_result = replay
        .target
        .solve_deterministically(request)
        .map_err(add_solver_hint)?;
    let target_config = target_result.config.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "Bundle solving requires a witness-capable target problem and witness-capable reduction path; {} only supports aggregate-value solving.",
            replay.target_name
        )
    })?;

    let (source_config, source_eval) = replay.extract(target_config);

    let solver_desc = format!(
        "{} (via {})",
        solver_text(&target_result.solver),
        replay.target_name
    );
    let text = format!(
        "Problem: {}\nSolver: {}\nSolution: {:?}\nEvaluation: {}",
        replay.source_name, solver_desc, source_config, source_eval,
    );

    let json = serde_json::json!({
        "problem": replay.source_name,
        "solver": &target_result.solver,
        "solution": source_config,
        "evaluation": source_eval,
        "intermediate": {
            "problem": replay.target_name,
            "solution": target_config,
            "evaluation": target_result.evaluation,
        },
    });

    let result = out.emit_with_default_name("", &text, &json);
    if out.output.is_none() && crate::output::stderr_is_tty() {
        out.info("\nHint: use -o to save full solution details (including intermediate results) as JSON.");
    }
    result
}

fn add_solver_hint(err: anyhow::Error) -> anyhow::Error {
    let message = err.to_string();
    if message.starts_with("No ILP pipeline is registered for ") {
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
    fn test_solve_value_only_problem_omits_solution() {
        let result = DeterministicSolveResult {
            solver: SolverExecution::BruteForce,
            config: None,
            evaluation: "Sum(56)".to_string(),
        };
        let (text, json) = plain_problem_output("CliTestAggregateValueSource", &result);
        assert!(text.contains("Evaluation: Sum(56)"), "{text}");
        assert!(!text.contains("Solution:"), "{text}");
        assert!(json.get("solution").is_none(), "{json}");
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
