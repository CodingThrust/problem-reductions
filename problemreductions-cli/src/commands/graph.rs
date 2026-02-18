use crate::output::OutputConfig;
use crate::problem_name::parse_problem_spec;
use anyhow::Result;
use problemreductions::rules::ReductionGraph;

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
