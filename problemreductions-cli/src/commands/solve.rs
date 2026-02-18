use crate::dispatch::{load_problem, ProblemJson, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::rules::ReductionGraph;
use std::path::Path;

/// Input can be either a problem JSON or a reduction bundle JSON.
enum SolveInput {
    /// A plain problem file (from `pred create`).
    Problem(ProblemJson),
    /// A reduction bundle (from `pred reduce`) with source, target, and path.
    Bundle(ReductionBundle),
}

fn parse_input(path: &Path) -> Result<SolveInput> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
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

pub fn solve(input: &Path, solver_name: &str, out: &OutputConfig) -> Result<()> {
    if solver_name != "brute-force" && solver_name != "ilp" {
        anyhow::bail!(
            "Unknown solver: {}. Available solvers: brute-force, ilp",
            solver_name
        );
    }

    let parsed = parse_input(input)?;

    match parsed {
        SolveInput::Problem(problem_json) => solve_problem(
            &problem_json.problem_type,
            &problem_json.variant,
            problem_json.data,
            solver_name,
            out,
        ),
        SolveInput::Bundle(bundle) => solve_bundle(bundle, solver_name, out),
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
            out.emit_with_default_name("", &text, &json)
        }
        "ilp" => {
            let result = problem.solve_with_ilp()?;
            let reduced = name != "ILP";
            let text = if reduced {
                format!(
                    "Problem: {} (reduced to ILP)\nSolver: ilp\nSolution: {:?}\nEvaluation: {}",
                    name, result.config, result.evaluation,
                )
            } else {
                format!(
                    "Problem: ILP\nSolver: ilp\nSolution: {:?}\nEvaluation: {}",
                    result.config, result.evaluation,
                )
            };
            let mut json = serde_json::json!({
                "problem": name,
                "solver": "ilp",
                "solution": result.config,
                "evaluation": result.evaluation,
            });
            if reduced {
                json["reduced_to"] = serde_json::json!("ILP");
            }
            out.emit_with_default_name("", &text, &json)
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

    let text = format!(
        "Source: {}\nTarget: {} (solved with {})\nTarget solution: {:?}\nTarget evaluation: {}\nSource solution: {:?}\nSource evaluation: {}",
        source_name,
        target_name,
        solver_name,
        target_result.config,
        target_result.evaluation,
        source_config,
        source_eval,
    );

    let json = serde_json::json!({
        "source": {
            "problem": source_name,
            "solution": source_config,
            "evaluation": source_eval,
        },
        "target": {
            "problem": target_name,
            "solver": solver_name,
            "solution": target_result.config,
            "evaluation": target_result.evaluation,
        },
    });

    out.emit_with_default_name("", &text, &json)
}
