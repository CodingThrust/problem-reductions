use crate::output::OutputConfig;
use crate::problem_name::{aliases_for, parse_problem_spec, resolve_variant};
use anyhow::{Context, Result};
use problemreductions::registry::collect_schemas;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;
use std::path::Path;

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
        let aliases = aliases_for(name);
        if aliases.is_empty() {
            text.push_str(&format!("  {name}\n"));
        } else {
            text.push_str(&format!("  {name} ({})\n", aliases.join(", ")));
        }
    }

    text.push_str("\nUse `pred show <problem>` to see variants, reductions, and fields.\n");

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": types.iter().map(|name| {
            let aliases = aliases_for(name);
            serde_json::json!({ "name": name, "aliases": aliases })
        }).collect::<Vec<_>>(),
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}

pub fn show(problem: &str, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let graph = ReductionGraph::new();

    let variants = graph.variants_for(&spec.name);
    if variants.is_empty() {
        anyhow::bail!("Unknown problem: {}", spec.name);
    }

    let mut text = format!("{}\n", spec.name);

    // Show description from schema
    let schemas = collect_schemas();
    let schema = schemas.iter().find(|s| s.name == spec.name);
    if let Some(s) = schema {
        if !s.description.is_empty() {
            text.push_str(&format!("  {}\n", s.description));
        }
    }

    // Show variants
    text.push_str(&format!("\nVariants ({}):\n", variants.len()));
    for v in &variants {
        text.push_str(&format!("  {}\n", format_variant(v)));
    }

    // Show fields from schema (right after variants)
    if let Some(s) = schema {
        text.push_str(&format!("\nFields ({}):\n", s.fields.len()));
        for field in &s.fields {
            text.push_str(&format!("  {} ({})", field.name, field.type_name));
            if !field.description.is_empty() {
                text.push_str(&format!(" -- {}", field.description));
            }
            text.push('\n');
        }
    }

    // Show reductions from/to this problem
    let outgoing = graph.outgoing_reductions(&spec.name);
    let incoming = graph.incoming_reductions(&spec.name);

    text.push_str(&format!("\nReduces to ({}):\n", outgoing.len()));
    for e in &outgoing {
        text.push_str(&format!(
            "  {} {} -> {} {}\n",
            e.source_name,
            format_variant(&e.source_variant),
            e.target_name,
            format_variant(&e.target_variant),
        ));
    }

    text.push_str(&format!("\nReduces from ({}):\n", incoming.len()));
    for e in &incoming {
        text.push_str(&format!(
            "  {} {} -> {} {}\n",
            e.source_name,
            format_variant(&e.source_variant),
            e.target_name,
            format_variant(&e.target_variant),
        ));
    }

    let mut json = serde_json::json!({
        "name": spec.name,
        "variants": variants,
        "reduces_to": outgoing.iter().map(|e| {
            serde_json::json!({"source": {"name": e.source_name, "variant": e.source_variant}, "target": {"name": e.target_name, "variant": e.target_variant}})
        }).collect::<Vec<_>>(),
        "reduces_from": incoming.iter().map(|e| {
            serde_json::json!({"source": {"name": e.source_name, "variant": e.source_variant}, "target": {"name": e.target_name, "variant": e.target_variant}})
        }).collect::<Vec<_>>(),
    });
    if let Some(s) = schema {
        if let (Some(obj), Ok(schema_val)) = (json.as_object_mut(), serde_json::to_value(s)) {
            obj.insert("schema".to_string(), schema_val);
        }
    }

    let default_name = format!("pred_show_{}.json", spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}

fn format_variant(v: &BTreeMap<String, String>) -> String {
    if v.is_empty() {
        "(default)".to_string()
    } else {
        let pairs: Vec<String> = v.iter().map(|(k, val)| format!("{k}={val}")).collect();
        format!("{{{}}}", pairs.join(", "))
    }
}

fn format_path_text(
    graph: &ReductionGraph,
    reduction_path: &problemreductions::rules::ReductionPath,
) -> String {
    let mut text = format!(
        "Path ({} steps): {}\n",
        reduction_path.len(),
        reduction_path
    );

    let overheads = graph.path_overheads(reduction_path);
    let steps = &reduction_path.steps;
    for i in 0..steps.len().saturating_sub(1) {
        let from = &steps[i];
        let to = &steps[i + 1];
        text.push_str(&format!("\n  Step {}: {} → {}\n", i + 1, from, to));
        let oh = &overheads[i];
        for (field, poly) in &oh.output_size {
            text.push_str(&format!("    {field} = {poly}\n"));
        }
    }

    text
}

fn format_path_json(
    graph: &ReductionGraph,
    reduction_path: &problemreductions::rules::ReductionPath,
) -> serde_json::Value {
    let overheads = graph.path_overheads(reduction_path);
    let steps_json: Vec<serde_json::Value> = reduction_path
        .steps
        .windows(2)
        .zip(overheads.iter())
        .enumerate()
        .map(|(i, (pair, oh))| {
            serde_json::json!({
                "from": {"name": pair[0].name, "variant": pair[0].variant},
                "to": {"name": pair[1].name, "variant": pair[1].variant},
                "step": i + 1,
                "overhead": oh.output_size.iter().map(|(field, poly)| {
                    serde_json::json!({"field": field, "formula": poly.to_string()})
                }).collect::<Vec<_>>(),
            })
        })
        .collect();

    serde_json::json!({
        "steps": reduction_path.len(),
        "path": steps_json,
    })
}

pub fn path(source: &str, target: &str, cost: &str, all: bool, out: &OutputConfig) -> Result<()> {
    let src_spec = parse_problem_spec(source)?;
    let dst_spec = parse_problem_spec(target)?;
    let graph = ReductionGraph::new();

    let src_variants = graph.variants_for(&src_spec.name);
    let dst_variants = graph.variants_for(&dst_spec.name);

    if src_variants.is_empty() {
        anyhow::bail!(
            "Unknown source problem: {}\n\n\
             Usage: pred path <SOURCE> <TARGET>\n\
             Example: pred path MIS QUBO\n\n\
             Run `pred list` to see all available problems.",
            src_spec.name
        );
    }
    if dst_variants.is_empty() {
        anyhow::bail!(
            "Unknown target problem: {}\n\n\
             Usage: pred path <SOURCE> <TARGET>\n\
             Example: pred path MIS QUBO\n\n\
             Run `pred list` to see all available problems.",
            dst_spec.name
        );
    }

    if all {
        // --all uses only the specified variant or the first (default) one
        let sv = if src_spec.variant_values.is_empty() {
            src_variants[0].clone()
        } else {
            resolve_variant(&src_spec, &src_variants)?
        };
        let dv = if dst_spec.variant_values.is_empty() {
            dst_variants[0].clone()
        } else {
            resolve_variant(&dst_spec, &dst_variants)?
        };
        return path_all(&graph, &src_spec.name, &sv, &dst_spec.name, &dv, out);
    }

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

    let input_size = ProblemSize::new(vec![]);
    let mut best_path: Option<problemreductions::rules::ReductionPath> = None;

    for sv in &src_resolved {
        for dv in &dst_resolved {
            if cost != "minimize-steps" && !cost.starts_with("minimize:") {
                anyhow::bail!(
                    "Unknown cost function: {}. Use 'minimize-steps' or 'minimize:<field>'",
                    cost
                );
            }
            // TODO: use field-specific cost when concrete input size is available
            let found = graph.find_cheapest_path(
                &src_spec.name,
                sv,
                &dst_spec.name,
                dv,
                &input_size,
                &MinimizeSteps,
            );

            if let Some(p) = found {
                let is_better = best_path.as_ref().is_none_or(|bp| p.len() < bp.len());
                if is_better {
                    best_path = Some(p);
                }
            }
        }
    }

    match best_path {
        Some(ref reduction_path) => {
            let text = format_path_text(&graph, reduction_path);
            if let Some(ref path) = out.output {
                let json = format_path_json(&graph, reduction_path);
                let content =
                    serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?;
                std::fs::write(path, &content)
                    .with_context(|| format!("Failed to write {}", path.display()))?;
                eprintln!("Wrote {}", path.display());
            } else {
                println!("{text}");
            }
            Ok(())
        }
        None => {
            anyhow::bail!(
                "No reduction path from {} to {}\n\n\
                 Usage: pred path <SOURCE> <TARGET>\n\
                 Example: pred path MIS QUBO\n\n\
                 Run `pred show {}` and `pred show {}` to check available reductions.",
                src_spec.name,
                dst_spec.name,
                src_spec.name,
                dst_spec.name,
            );
        }
    }
}

fn path_all(
    graph: &ReductionGraph,
    src_name: &str,
    src_variant: &BTreeMap<String, String>,
    dst_name: &str,
    dst_variant: &BTreeMap<String, String>,
    out: &OutputConfig,
) -> Result<()> {
    let mut all_paths = graph.find_all_paths(src_name, src_variant, dst_name, dst_variant);

    if all_paths.is_empty() {
        anyhow::bail!(
            "No reduction path from {} to {}\n\n\
             Usage: pred path <SOURCE> <TARGET> --all\n\
             Example: pred path MIS QUBO --all\n\n\
             Run `pred show {}` and `pred show {}` to check available reductions.",
            src_name,
            dst_name,
            src_name,
            dst_name,
        );
    }

    // Sort by path length (shortest first)
    all_paths.sort_by_key(|p| p.len());

    let mut text = format!("Found {} paths from {} to {}:\n", all_paths.len(), src_name, dst_name);
    for (idx, p) in all_paths.iter().enumerate() {
        text.push_str(&format!("\n--- Path {} ---\n", idx + 1));
        text.push_str(&format_path_text(graph, p));
    }

    if let Some(ref dir) = out.output {
        // -o specifies the output folder; save each path as a separate JSON file
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory {}", dir.display()))?;

        for (idx, p) in all_paths.iter().enumerate() {
            let json = format_path_json(graph, p);
            let file = dir.join(format!("path_{}.json", idx + 1));
            let content =
                serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?;
            std::fs::write(&file, &content)
                .with_context(|| format!("Failed to write {}", file.display()))?;
        }
        eprintln!(
            "Wrote {} path files to {}",
            all_paths.len(),
            dir.display()
        );
    } else {
        println!("{text}");
    }

    Ok(())
}

pub fn export(output: &Path) -> Result<()> {
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
