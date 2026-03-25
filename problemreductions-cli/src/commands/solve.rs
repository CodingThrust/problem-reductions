use crate::dispatch::{load_problem, read_input, ProblemJson, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::rules::ReductionGraph;
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

fn solve_result_text(problem: &str, solver: &str, result: &crate::dispatch::SolveResult) -> String {
    let mut text = format!("Problem: {}\nSolver: {}", problem, solver);
    if let Some(config) = &result.config {
        text.push_str(&format!("\nSolution: {:?}", config));
    }
    text.push_str(&format!("\nEvaluation: {}", result.evaluation));
    text
}

fn solve_result_json(
    problem: &str,
    solver: &str,
    result: &crate::dispatch::SolveResult,
) -> serde_json::Value {
    let mut json = serde_json::json!({
        "problem": problem,
        "solver": solver,
        "evaluation": result.evaluation,
    });
    if let Some(config) = &result.config {
        json["solution"] = serde_json::json!(config);
    }
    json
}

fn plain_problem_output(
    problem: &str,
    solver: &str,
    result: &crate::dispatch::SolveResult,
) -> (String, serde_json::Value) {
    (
        solve_result_text(problem, solver, result),
        solve_result_json(problem, solver, result),
    )
}

pub fn solve(input: &Path, solver_name: &str, timeout: u64, out: &OutputConfig) -> Result<()> {
    if solver_name != "brute-force" && solver_name != "ilp" && solver_name != "customized" {
        anyhow::bail!(
            "Unknown solver: {}. Available solvers: brute-force, ilp, customized",
            solver_name
        );
    }

    let parsed = parse_input(input)?;

    if timeout > 0 {
        let solver_name = solver_name.to_string();
        let out = out.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = match parsed {
                SolveInput::Problem(pj) => {
                    solve_problem(&pj.problem_type, &pj.variant, pj.data, &solver_name, &out)
                }
                SolveInput::Bundle(b) => solve_bundle(b, &solver_name, &out),
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
                solve_problem(&pj.problem_type, &pj.variant, pj.data, solver_name, out)
            }
            SolveInput::Bundle(b) => solve_bundle(b, solver_name, out),
        }
    }
}

/// Solve a plain problem file directly.
fn solve_problem(
    problem_type: &str,
    variant: &std::collections::BTreeMap<String, String>,
    data: serde_json::Value,
    solver_name: &str,
    out: &OutputConfig,
) -> Result<()> {
    let problem = load_problem(problem_type, variant, data)?;
    let name = problem.problem_name();

    match solver_name {
        "brute-force" => {
            let result = problem.solve_brute_force();
            let (text, json) = plain_problem_output(name, "brute-force", &result);
            let result = out.emit_with_default_name("", &text, &json);
            if out.output.is_none() && crate::output::stderr_is_tty() {
                out.info("\nHint: use -o to save full solution details as JSON.");
            }
            result
        }
        "ilp" => {
            let result = problem.solve_with_ilp().map_err(add_ilp_solver_hint)?;
            let solver_desc = if name == "ILP" {
                "ilp".to_string()
            } else {
                "ilp (via ILP)".to_string()
            };
            let result = crate::dispatch::SolveResult {
                config: Some(result.config),
                evaluation: result.evaluation,
            };
            let text = solve_result_text(name, &solver_desc, &result);
            let mut json = solve_result_json(name, "ilp", &result);
            if name != "ILP" {
                json["reduced_to"] = serde_json::json!("ILP");
            }
            let result = out.emit_with_default_name("", &text, &json);
            if out.output.is_none() && crate::output::stderr_is_tty() {
                out.info("\nHint: use -o to save full solution details as JSON.");
            }
            result
        }
        "customized" => {
            let result = problem
                .solve_with_customized()
                .map_err(add_customized_solver_hint)?;
            let result = crate::dispatch::SolveResult {
                config: Some(result.config),
                evaluation: result.evaluation,
            };
            let (text, json) = plain_problem_output(name, "customized", &result);
            let result = out.emit_with_default_name("", &text, &json);
            if out.output.is_none() && crate::output::stderr_is_tty() {
                out.info("\nHint: use -o to save full solution details as JSON.");
            }
            result
        }
        _ => unreachable!(),
    }
}

/// Solve a reduction bundle: solve the target problem, then map the solution back.
fn solve_bundle(bundle: ReductionBundle, solver_name: &str, out: &OutputConfig) -> Result<()> {
    // 1. Load the target problem from the bundle
    let target = load_problem(
        &bundle.target.problem_type,
        &bundle.target.variant,
        bundle.target.data.clone(),
    )?;
    let target_name = target.problem_name();

    // 2. Solve the target problem
    let target_result = match solver_name {
        "brute-force" => target.solve_brute_force_witness().ok_or_else(|| {
            anyhow::anyhow!(
                "Bundle solving requires a witness-capable target problem and witness-capable reduction path; {} only supports aggregate-value solving.",
                target_name
            )
        })?,
        "ilp" => target.solve_with_ilp().map_err(add_ilp_solver_hint)?,
        "customized" => target
            .solve_with_customized()
            .map_err(add_customized_solver_hint)?,
        _ => unreachable!(),
    };

    // 3. Load source problem and re-execute the reduction chain to get extract_solution
    let source = load_problem(
        &bundle.source.problem_type,
        &bundle.source.variant,
        bundle.source.data.clone(),
    )?;
    let source_name = source.problem_name();

    let graph = ReductionGraph::new();

    // Reconstruct the ReductionPath from the bundle's path steps
    let reduction_path = problemreductions::rules::ReductionPath {
        steps: bundle
            .path
            .iter()
            .map(|s| problemreductions::rules::ReductionStep {
                name: s.name.clone(),
                variant: s.variant.clone(),
            })
            .collect(),
    };

    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .ok_or_else(|| anyhow::anyhow!(
            "Bundle solving requires a witness-capable reduction path; this bundle cannot recover a source solution."
        ))?;

    // 4. Extract solution back to source problem space
    let source_config = chain.extract_solution(&target_result.config);
    let source_eval = source.evaluate_dyn(&source_config);

    let solver_desc = format!("{} (via {})", solver_name, target_name);
    let text = format!(
        "Problem: {}\nSolver: {}\nSolution: {:?}\nEvaluation: {}",
        source_name, solver_desc, source_config, source_eval,
    );

    let json = serde_json::json!({
        "problem": source_name,
        "solver": solver_name,
        "reduced_to": target_name,
        "solution": source_config,
        "evaluation": source_eval,
        "intermediate": {
            "problem": target_name,
            "solution": target_result.config,
            "evaluation": target_result.evaluation,
        },
    });

    let result = out.emit_with_default_name("", &text, &json);
    if out.output.is_none() && crate::output::stderr_is_tty() {
        out.info("\nHint: use -o to save full solution details (including intermediate results) as JSON.");
    }
    result
}

fn add_customized_solver_hint(err: anyhow::Error) -> anyhow::Error {
    let message = err.to_string();
    if message.contains("unsupported by customized solver") {
        anyhow::anyhow!(
            "{message}\n\nHint: the customized solver only supports select problems (FD-based models, PartialFeedbackEdgeSet, RootedTreeArrangement).\nTry `--solver brute-force` or `--solver ilp` instead."
        )
    } else {
        err
    }
}

fn add_ilp_solver_hint(err: anyhow::Error) -> anyhow::Error {
    let message = err.to_string();
    if (message.starts_with("No reduction path from ") && message.ends_with(" to ILP"))
        || message.contains("witness-capable")
    {
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
    use crate::dispatch::SolveResult;
    use crate::output::OutputConfig;
    use crate::test_support::aggregate_bundle;

    #[test]
    fn test_solve_value_only_problem_omits_solution() {
        let result = SolveResult {
            config: None,
            evaluation: "Sum(56)".to_string(),
        };
        let (text, json) =
            plain_problem_output("CliTestAggregateValueSource", "brute-force", &result);
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

        let err = solve_bundle(bundle, "brute-force", &out).unwrap_err();
        assert!(
            err.to_string().contains("witness"),
            "unexpected error: {err}"
        );
    }
}
