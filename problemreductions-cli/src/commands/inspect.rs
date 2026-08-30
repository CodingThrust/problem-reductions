use crate::dispatch::{
    load_problem, read_input, solver_capabilities_view, ProblemJson, ReductionBundle,
};
use crate::output::OutputConfig;
use anyhow::Result;
use problemreductions::rules::{ReductionGraph, ReductionMode};
use std::collections::BTreeMap;
use std::path::Path;

pub fn inspect(input: &Path, out: &OutputConfig) -> Result<()> {
    let content = read_input(input)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;

    // Detect if it's a bundle or a problem
    if json.get("source").is_some() && json.get("target").is_some() && json.get("path").is_some() {
        let bundle: ReductionBundle = serde_json::from_value(json)?;
        inspect_bundle(&bundle, out)
    } else {
        let problem_json: ProblemJson = serde_json::from_value(json)?;
        inspect_problem(&problem_json, out)
    }
}

fn inspect_problem(pj: &ProblemJson, out: &OutputConfig) -> Result<()> {
    let problem = load_problem(&pj.problem_type, &pj.variant, pj.data.clone())?;
    let name = problem.problem_name();
    let variant = problem.variant_map();
    let graph = ReductionGraph::new();
    let brute_force_num_variables = problem.brute_force_num_variables()?;

    let parameters = graph.parameter_names(name);
    let solver_view = solver_capabilities_view(&problem)?;
    let targets = executable_reduction_targets(&graph, name, &variant);
    out.emit(
        || {
            let variant_str = if variant.is_empty() {
                String::new()
            } else {
                let pairs: Vec<String> = variant.iter().map(|(k, v)| format!("{k}={v}")).collect();
                format!(" {{{}}}", pairs.join(", "))
            };
            let mut text = format!("Type: {}{}\n", name, variant_str);
            if !parameters.is_empty() {
                text.push_str(&format!("Parameters: {}\n", parameters.join(", ")));
            }
            if let Some(num_variables) = brute_force_num_variables {
                text.push_str(&format!("Brute-force variables: {num_variables}\n"));
            }
            text.push_str(&format!("Default solver: {}\n", solver_view.default_solver));
            text.push_str(&format!("Solvers: {}\n", solver_view.solvers.join(", ")));
            if let Some(customized) = solver_view.capabilities.customized.as_ref() {
                text.push_str(&format!(
                    "Customized implementation: {}\n",
                    customized.implementation
                ));
            }
            if let Some(ilp) = solver_view.capabilities.ilp.as_ref() {
                text.push_str(&format!(
                    "ILP pipeline: {}\n",
                    ilp.reduction_path.join(" -> ")
                ));
            }
            if !targets.is_empty() {
                text.push_str(&format!("Reduces to: {}\n", targets.join(", ")));
            }
            text
        },
        || {
            Ok(serde_json::json!({
                "kind": "problem",
                "type": name,
                "variant": variant,
                "parameters": parameters,
                "brute_force_num_variables": brute_force_num_variables,
                "solvers": solver_view.solvers,
                "default_solver": solver_view.default_solver,
                "solver_capabilities": solver_view.capabilities,
                "reduces_to": targets,
            }))
        },
    )
}

fn inspect_bundle(bundle: &ReductionBundle, out: &OutputConfig) -> Result<()> {
    let path_str: Vec<&str> = bundle.path.iter().map(|s| s.name.as_str()).collect();
    out.emit(
        || {
            let mut text = String::from("Kind: Reduction Bundle\n");
            text.push_str(&format!("Source: {}\n", bundle.source.problem_type));
            text.push_str(&format!("Target: {}\n", bundle.target.problem_type));
            text.push_str(&format!("Steps: {}\n", bundle.path.len().saturating_sub(1)));
            text.push_str(&format!("Path: {}\n", path_str.join(" -> ")));
            text
        },
        || {
            Ok(serde_json::json!({
                "kind": "bundle",
                "source": bundle.source.problem_type,
                "target": bundle.target.problem_type,
                "steps": bundle.path.len().saturating_sub(1),
                "path": path_str,
            }))
        },
    )
}

pub(crate) fn executable_reduction_targets(
    graph: &ReductionGraph,
    name: &str,
    variant: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut targets: Vec<String> = graph
        .outgoing_reductions_from(name, variant, ReductionMode::Witness)
        .into_iter()
        .map(|edge| {
            let default_variant = graph
                .default_variant_for(edge.target_name)
                .unwrap_or_else(|| panic!("default variant not found for {}", edge.target_name));
            if default_variant == edge.target_variant {
                edge.target_name.to_string()
            } else {
                format!(
                    "{}{}",
                    edge.target_name,
                    crate::commands::graph::variant_to_full_slash(&edge.target_variant)
                )
            }
        })
        .collect();
    targets.sort();
    targets.dedup();
    targets
}
