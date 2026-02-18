use crate::output::OutputConfig;
use crate::problem_name::{parse_problem_spec, resolve_variant};
use anyhow::Result;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub fn list(out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    let mut text = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    for name in &types {
        text.push_str(&format!("  {name}\n"));
    }

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": types,
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}

pub fn show(problem: &str, show_variants: bool, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let graph = ReductionGraph::new();
    let graph_json: serde_json::Value = serde_json::from_str(&graph.to_json_string()?)?;
    let nodes = graph_json["nodes"].as_array().unwrap();
    let edges = graph_json["edges"].as_array().unwrap();

    // Find all nodes matching this problem name
    let matching_nodes: Vec<(usize, &serde_json::Value)> = nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n["name"].as_str() == Some(&spec.name))
        .collect();

    if matching_nodes.is_empty() {
        anyhow::bail!("Unknown problem: {}", spec.name);
    }

    let mut text = format!("{}\n", spec.name);

    if show_variants {
        text.push_str(&format!("\nVariants ({}):\n", matching_nodes.len()));
        for (_, node) in &matching_nodes {
            let variant = &node["variant"];
            if variant.as_object().map_or(true, |v| v.is_empty()) {
                text.push_str("  (no variants)\n");
            } else {
                let pairs: Vec<String> = variant
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                    .collect();
                text.push_str(&format!("  {{{}}}\n", pairs.join(", ")));
            }
        }
    }

    // Show reductions from/to this problem
    let node_indices: Vec<usize> = matching_nodes.iter().map(|(i, _)| *i).collect();

    let outgoing: Vec<&serde_json::Value> = edges
        .iter()
        .filter(|e| node_indices.contains(&(e["source"].as_u64().unwrap() as usize)))
        .collect();
    let incoming: Vec<&serde_json::Value> = edges
        .iter()
        .filter(|e| node_indices.contains(&(e["target"].as_u64().unwrap() as usize)))
        .collect();

    text.push_str(&format!("\nReduces to ({}):\n", outgoing.len()));
    for edge in &outgoing {
        let target = &nodes[edge["target"].as_u64().unwrap() as usize];
        text.push_str(&format!("  -> {}", target["name"].as_str().unwrap()));
        let variant = &target["variant"];
        if let Some(obj) = variant.as_object() {
            if !obj.is_empty() {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                    .collect();
                text.push_str(&format!(" {{{}}}", pairs.join(", ")));
            }
        }
        text.push('\n');
    }

    text.push_str(&format!("\nReduces from ({}):\n", incoming.len()));
    for edge in &incoming {
        let source = &nodes[edge["source"].as_u64().unwrap() as usize];
        text.push_str(&format!("  <- {}", source["name"].as_str().unwrap()));
        let variant = &source["variant"];
        if let Some(obj) = variant.as_object() {
            if !obj.is_empty() {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                    .collect();
                text.push_str(&format!(" {{{}}}", pairs.join(", ")));
            }
        }
        text.push('\n');
    }

    let json = serde_json::json!({
        "name": spec.name,
        "variants": matching_nodes.iter().map(|(_, n)| &n["variant"]).collect::<Vec<_>>(),
        "reduces_to": outgoing.iter().map(|e| {
            let t = &nodes[e["target"].as_u64().unwrap() as usize];
            serde_json::json!({"name": t["name"], "variant": t["variant"]})
        }).collect::<Vec<_>>(),
        "reduces_from": incoming.iter().map(|e| {
            let s = &nodes[e["source"].as_u64().unwrap() as usize];
            serde_json::json!({"name": s["name"], "variant": s["variant"]})
        }).collect::<Vec<_>>(),
    });

    let default_name = format!("pred_show_{}.json", spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}

fn collect_variants(nodes: &[serde_json::Value], name: &str) -> Vec<BTreeMap<String, String>> {
    nodes
        .iter()
        .filter(|n| n["name"].as_str() == Some(name))
        .map(|n| {
            n["variant"]
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect()
}

pub fn path(source: &str, target: &str, cost: &str, out: &OutputConfig) -> Result<()> {
    let src_spec = parse_problem_spec(source)?;
    let dst_spec = parse_problem_spec(target)?;
    let graph = ReductionGraph::new();
    let graph_json: serde_json::Value = serde_json::from_str(&graph.to_json_string()?)?;
    let nodes = graph_json["nodes"].as_array().unwrap();

    let src_variants = collect_variants(nodes, &src_spec.name);
    let dst_variants = collect_variants(nodes, &dst_spec.name);

    if src_variants.is_empty() {
        anyhow::bail!("Unknown problem: {}", src_spec.name);
    }
    if dst_variants.is_empty() {
        anyhow::bail!("Unknown problem: {}", dst_spec.name);
    }

    let input_size = ProblemSize::new(vec![]);

    let mut best_path: Option<problemreductions::rules::ReductionPath> = None;

    let src_resolved = if src_spec.variant_values.is_empty() {
        src_variants.clone()
    } else {
        vec![resolve_variant(&src_spec, &src_variants)?]
    };
    let dst_resolved = if dst_spec.variant_values.is_empty() {
        dst_variants.clone()
    } else {
        vec![resolve_variant(&dst_spec, &dst_variants)?]
    };

    for sv in &src_resolved {
        for dv in &dst_resolved {
            let found = if cost == "minimize-steps" {
                graph.find_cheapest_path(
                    &src_spec.name,
                    sv,
                    &dst_spec.name,
                    dv,
                    &input_size,
                    &MinimizeSteps,
                )
            } else if cost.starts_with("minimize:") {
                // Without concrete input size, fall back to minimize-steps
                graph.find_cheapest_path(
                    &src_spec.name,
                    sv,
                    &dst_spec.name,
                    dv,
                    &input_size,
                    &MinimizeSteps,
                )
            } else {
                anyhow::bail!(
                    "Unknown cost function: {}. Use 'minimize-steps' or 'minimize:<field>'",
                    cost
                );
            };

            if let Some(p) = found {
                let is_better = best_path.as_ref().map_or(true, |bp| p.len() < bp.len());
                if is_better {
                    best_path = Some(p);
                }
            }
        }
    }

    match best_path {
        Some(ref reduction_path) => {
            let text = format!(
                "Path ({} steps): {}",
                reduction_path.len(),
                reduction_path
            );

            let steps_json: Vec<serde_json::Value> = reduction_path
                .steps
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "name": s.name,
                        "variant": s.variant,
                    })
                })
                .collect();

            let json = serde_json::json!({
                "steps": reduction_path.len(),
                "path": steps_json,
            });

            let default_name =
                format!("pred_path_{}_to_{}.json", src_spec.name, dst_spec.name);
            out.emit_with_default_name(&default_name, &text, &json)
        }
        None => {
            eprintln!("No path found from {} to {}", src_spec.name, dst_spec.name);
            std::process::exit(1);
        }
    }
}

pub fn export(output: &PathBuf) -> Result<()> {
    let graph = ReductionGraph::new();

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    graph
        .to_json_file(output)
        .map_err(|e| anyhow::anyhow!("Failed to export: {}", e))?;

    eprintln!(
        "Exported reduction graph ({} types, {} reductions, {} variant nodes) to {}",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
        output.display()
    );

    Ok(())
}
