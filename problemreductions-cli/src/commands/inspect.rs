use crate::dispatch::{load_problem, read_input, ProblemJson, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::Result;
use problemreductions::rules::ReductionGraph;
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

    let variant_str = if variant.is_empty() {
        String::new()
    } else {
        let pairs: Vec<String> = variant.iter().map(|(k, v)| format!("{k}={v}")).collect();
        format!(" {{{}}}", pairs.join(", "))
    };

    let mut text = format!("Type: {}{}\n", name, variant_str);

    // Size info
    let size_names = problem.problem_size_names_dyn();
    let size_values = problem.problem_size_values_dyn();
    if !size_names.is_empty() {
        let sizes: Vec<String> = size_names
            .iter()
            .zip(size_values.iter())
            .map(|(n, v)| format!("{} {}", v, n))
            .collect();
        text.push_str(&format!("Size: {}\n", sizes.join(", ")));
    }
    text.push_str(&format!("Variables: {}\n", problem.num_variables_dyn()));

    // Solvers
    text.push_str("Solvers: ilp (default), brute-force\n");

    // Reductions
    let outgoing = graph.outgoing_reductions(name);
    let targets = targets_deduped(&outgoing);
    if !targets.is_empty() {
        text.push_str(&format!("Reduces to: {}\n", targets.join(", ")));
    }

    let json_val = serde_json::json!({
        "kind": "problem",
        "type": name,
        "variant": variant,
        "size": size_names.iter().zip(size_values.iter())
            .map(|(n, v)| serde_json::json!({"field": n, "value": v}))
            .collect::<Vec<_>>(),
        "num_variables": problem.num_variables_dyn(),
        "solvers": ["ilp", "brute-force"],
        "reduces_to": targets,
    });

    out.emit_with_default_name("", &text, &json_val)
}

fn inspect_bundle(bundle: &ReductionBundle, out: &OutputConfig) -> Result<()> {
    let mut text = String::from("Kind: Reduction Bundle\n");
    text.push_str(&format!("Source: {}\n", bundle.source.problem_type));
    text.push_str(&format!("Target: {}\n", bundle.target.problem_type));
    text.push_str(&format!("Steps: {}\n", bundle.path.len().saturating_sub(1)));

    let path_str: Vec<&str> = bundle.path.iter().map(|s| s.name.as_str()).collect();
    text.push_str(&format!("Path: {}\n", path_str.join(" -> ")));

    let json_val = serde_json::json!({
        "kind": "bundle",
        "source": bundle.source.problem_type,
        "target": bundle.target.problem_type,
        "steps": bundle.path.len().saturating_sub(1),
        "path": path_str,
    });

    out.emit_with_default_name("", &text, &json_val)
}

fn targets_deduped(outgoing: &[problemreductions::rules::ReductionEdgeInfo]) -> Vec<String> {
    let mut targets: Vec<String> = outgoing.iter().map(|e| e.target_name.to_string()).collect();
    targets.sort();
    targets.dedup();
    targets
}
