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

pub fn solve(input: &Path, solver_name: &str, timeout: u64, out: &OutputConfig) -> Result<()> {
    if solver_name != "brute-force" && solver_name != "ilp" {
        anyhow::bail!(
            "Unknown solver: {}. Available solvers: brute-force, ilp",
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
                SolveInput::Problem(pj) => solve_problem(
                    &pj.problem_type,
                    &pj.variant,
                    pj.data,
                    &solver_name,
                    &out,
                ),
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
            let result = problem.solve_brute_force()?;
            let text = format!(
                "Problem: {}\nSolver: brute-force\nSolution: {:?}\nEvaluation: {}",
                name, result.config, result.evaluation,
            );
            let json = serde_json::json!({
                "problem": name,
                "solver": "brute-force",
                "solution": result.config,
                "evaluation": result.evaluation,
            });
            let result = out.emit_with_default_name("", &text, &json);
            if out.output.is_none() && crate::output::stderr_is_tty() {
                out.info("\nHint: use -o to save full solution details as JSON.");
            }
            result
        }
        "ilp" => {
            let result = problem.solve_with_ilp()?;
            let solver_desc = if name == "ILP" {
                "ilp".to_string()
            } else {
                "ilp (via ILP)".to_string()
            };
            let text = format!(
                "Problem: {}\nSolver: {}\nSolution: {:?}\nEvaluation: {}",
                name, solver_desc, result.config, result.evaluation,
            );
            let mut json = serde_json::json!({
                "problem": name,
                "solver": "ilp",
                "solution": result.config,
                "evaluation": result.evaluation,
            });
            if name != "ILP" {
                json["reduced_to"] = serde_json::json!("ILP");
            }
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
        "brute-force" => target.solve_brute_force()?,
        "ilp" => target.solve_with_ilp()?,
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
        .ok_or_else(|| {
            anyhow::anyhow!("Failed to re-execute reduction chain for solution extraction")
        })?;

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
