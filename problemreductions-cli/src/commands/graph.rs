use crate::output::OutputConfig;
use crate::problem_name::{aliases_for, parse_problem_spec, resolve_variant};
use anyhow::{Context, Result};
use problemreductions::registry::collect_schemas;
use problemreductions::rules::{Minimize, MinimizeSteps, ReductionGraph, TraversalDirection};
use problemreductions::types::ProblemSize;
use std::collections::{BTreeMap, HashSet};

pub fn list(out: &OutputConfig) -> Result<()> {
    use crate::output::{format_table, Align};

    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    // Collect data for each problem
    struct RowData {
        name: String,
        aliases: Vec<&'static str>,
        num_variants: usize,
        num_reduces_to: usize,
    }
    let data: Vec<RowData> = types
        .iter()
        .map(|name| {
            let aliases = aliases_for(name);
            let num_variants = graph.variants_for(name).len();
            let num_reduces_to = graph.outgoing_reductions(name).len();
            RowData {
                name: name.to_string(),
                aliases,
                num_variants,
                num_reduces_to,
            }
        })
        .collect();

    let summary = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    let columns: Vec<(&str, Align, usize)> = vec![
        ("Problem", Align::Left, 7),
        ("Aliases", Align::Left, 7),
        ("Variants", Align::Right, 8),
        ("Reduces to", Align::Right, 10),
    ];

    let rows: Vec<Vec<String>> = data
        .iter()
        .map(|r| {
            vec![
                r.name.clone(),
                if r.aliases.is_empty() {
                    String::new()
                } else {
                    r.aliases.join(", ")
                },
                r.num_variants.to_string(),
                r.num_reduces_to.to_string(),
            ]
        })
        .collect();

    let color_fns: Vec<Option<crate::output::CellFormatter>> =
        vec![Some(crate::output::fmt_problem_name), None, None, None];

    let mut text = String::new();
    text.push_str(&crate::output::fmt_section(&summary));
    text.push('\n');
    text.push_str(&format_table(&columns, &rows, &color_fns));
    text.push_str("\nUse `pred show <problem>` to see variants, reductions, and fields.\n");

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": data.iter().map(|r| {
            serde_json::json!({
                "name": r.name,
                "aliases": r.aliases,
                "num_variants": r.num_variants,
                "num_reduces_to": r.num_reduces_to,
            })
        }).collect::<Vec<_>>(),
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}

pub fn show(problem: &str, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let graph = ReductionGraph::new();

    let variants = graph.variants_for(&spec.name);
    if variants.is_empty() {
        anyhow::bail!("{}", crate::problem_name::unknown_problem_error(&spec.name));
    }

    let mut text = format!("{}\n", crate::output::fmt_problem_name(&spec.name));

    // Show description from schema
    let schemas = collect_schemas();
    let schema = schemas.iter().find(|s| s.name == spec.name);
    if let Some(s) = schema {
        if !s.description.is_empty() {
            text.push_str(&format!("  {}\n", s.description));
        }
    }

    // Show variants
    text.push_str(&format!(
        "\n{}\n",
        crate::output::fmt_section(&format!("Variants ({}):", variants.len()))
    ));
    for v in &variants {
        text.push_str(&format!("  {}\n", format_variant(v)));
    }

    // Show fields from schema (right after variants)
    if let Some(s) = schema {
        text.push_str(&format!(
            "\n{}\n",
            crate::output::fmt_section(&format!("Fields ({}):", s.fields.len()))
        ));
        for field in &s.fields {
            text.push_str(&format!("  {} ({})", field.name, field.type_name));
            if !field.description.is_empty() {
                text.push_str(&format!(" -- {}", field.description));
            }
            text.push('\n');
        }
    }

    // Show size fields (used with `pred path --cost minimize:<field>`)
    let size_fields = graph.size_field_names(&spec.name);
    if !size_fields.is_empty() {
        text.push_str(&format!(
            "\n{}\n",
            crate::output::fmt_section(&format!("Size fields ({}):", size_fields.len()))
        ));
        for f in size_fields {
            text.push_str(&format!("  {f}\n"));
        }
    }

    // Show reductions from/to this problem
    let outgoing = graph.outgoing_reductions(&spec.name);
    let incoming = graph.incoming_reductions(&spec.name);

    text.push_str(&format!(
        "\n{}\n",
        crate::output::fmt_section(&format!("Reduces to ({}):", outgoing.len()))
    ));
    for e in &outgoing {
        text.push_str(&format!(
            "  {} {} {}\n",
            fmt_node(e.source_name, &e.source_variant),
            crate::output::fmt_outgoing("\u{2192}"),
            fmt_node(e.target_name, &e.target_variant),
        ));
    }

    text.push_str(&format!(
        "\n{}\n",
        crate::output::fmt_section(&format!("Reduces from ({}):", incoming.len()))
    ));
    for e in &incoming {
        text.push_str(&format!(
            "  {} {} {}\n",
            fmt_node(e.target_name, &e.target_variant),
            crate::output::fmt_outgoing("\u{2190}"),
            fmt_node(e.source_name, &e.source_variant),
        ));
    }

    let mut json = serde_json::json!({
        "name": spec.name,
        "variants": variants,
        "size_fields": size_fields,
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

/// Format a problem node as **bold name** + plain variant.
/// This is the single source of truth for "name {variant}" display.
fn fmt_node(name: &str, variant: &BTreeMap<String, String>) -> String {
    format!(
        "{} {}",
        crate::output::fmt_problem_name(name),
        format_variant(variant),
    )
}

fn format_path_text(
    graph: &ReductionGraph,
    reduction_path: &problemreductions::rules::ReductionPath,
) -> String {
    // Build formatted path header: Name {v} → Name {v} → ...
    let path_summary = {
        let steps = &reduction_path.steps;
        let mut parts = Vec::new();
        let mut prev_name = "";
        for step in steps {
            if step.name != prev_name {
                parts.push(fmt_node(&step.name, &step.variant));
                prev_name = &step.name;
            }
        }
        parts.join(&format!(" {} ", crate::output::fmt_outgoing("→")))
    };
    let mut text = format!("Path ({} steps): {}\n", reduction_path.len(), path_summary);

    let overheads = graph.path_overheads(reduction_path);
    let steps = &reduction_path.steps;
    for i in 0..steps.len().saturating_sub(1) {
        let from = &steps[i];
        let to = &steps[i + 1];
        text.push_str(&format!(
            "\n  {}: {} {} {}\n",
            crate::output::fmt_section(&format!("Step {}", i + 1)),
            fmt_node(&from.name, &from.variant),
            crate::output::fmt_outgoing("→"),
            fmt_node(&to.name, &to.variant),
        ));
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
            "{}\n\nUsage: pred path <SOURCE> <TARGET>\nExample: pred path MIS QUBO",
            crate::problem_name::unknown_problem_error(&src_spec.name)
        );
    }
    if dst_variants.is_empty() {
        anyhow::bail!(
            "{}\n\nUsage: pred path <SOURCE> <TARGET>\nExample: pred path MIS QUBO",
            crate::problem_name::unknown_problem_error(&dst_spec.name)
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

    // Parse cost function once (validate before the search loop)
    enum CostChoice {
        Steps,
        Field(&'static str),
    }
    let cost_choice = if cost == "minimize-steps" {
        CostChoice::Steps
    } else if let Some(field) = cost.strip_prefix("minimize:") {
        // Leak the field name to get &'static str (fine for a CLI that exits immediately)
        CostChoice::Field(Box::leak(field.to_string().into_boxed_str()))
    } else {
        anyhow::bail!(
            "Unknown cost function: {}. Use 'minimize-steps' or 'minimize:<field>'",
            cost
        );
    };

    let mut best_path: Option<problemreductions::rules::ReductionPath> = None;

    for sv in &src_resolved {
        for dv in &dst_resolved {
            let found = match cost_choice {
                CostChoice::Steps => graph.find_cheapest_path(
                    &src_spec.name,
                    sv,
                    &dst_spec.name,
                    dv,
                    &input_size,
                    &MinimizeSteps,
                ),
                CostChoice::Field(f) => graph.find_cheapest_path(
                    &src_spec.name,
                    sv,
                    &dst_spec.name,
                    dv,
                    &input_size,
                    &Minimize(f),
                ),
            };

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
            let json = format_path_json(&graph, reduction_path);
            out.emit_with_default_name("", &text, &json)
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

    let mut text = format!(
        "Found {} paths from {} to {}:\n",
        all_paths.len(),
        src_name,
        dst_name
    );
    for (idx, p) in all_paths.iter().enumerate() {
        text.push_str(&format!("\n--- Path {} ---\n", idx + 1));
        text.push_str(&format_path_text(graph, p));
    }

    let json: serde_json::Value = all_paths
        .iter()
        .map(|p| format_path_json(graph, p))
        .collect::<Vec<_>>()
        .into();

    if let Some(ref dir) = out.output {
        // -o specifies the output folder; save each path as a separate JSON file
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory {}", dir.display()))?;

        for (idx, p) in all_paths.iter().enumerate() {
            let path_json = format_path_json(graph, p);
            let file = dir.join(format!("path_{}.json", idx + 1));
            let content =
                serde_json::to_string_pretty(&path_json).context("Failed to serialize JSON")?;
            std::fs::write(&file, &content)
                .with_context(|| format!("Failed to write {}", file.display()))?;
        }
        out.info(&format!(
            "Wrote {} path files to {}",
            all_paths.len(),
            dir.display()
        ));
    } else if out.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?
        );
    } else {
        println!("{text}");
    }

    Ok(())
}

pub fn export(out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();

    let json_str = graph
        .to_json_string()
        .map_err(|e| anyhow::anyhow!("Failed to export: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&json_str).map_err(|e| anyhow::anyhow!("Failed to parse: {}", e))?;

    let text = format!(
        "Reduction graph: {} types, {} reductions, {} variant nodes\n\
         Use -o to save as JSON.",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    out.emit_with_default_name("reduction_graph.json", &text, &json)
}

fn parse_direction(s: &str) -> Result<TraversalDirection> {
    match s {
        "out" => Ok(TraversalDirection::Outgoing),
        "in" => Ok(TraversalDirection::Incoming),
        "both" => Ok(TraversalDirection::Both),
        _ => anyhow::bail!("Unknown direction: {}. Use 'out', 'in', or 'both'.", s),
    }
}

pub fn neighbors(
    problem: &str,
    max_hops: usize,
    direction_str: &str,
    out: &OutputConfig,
) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let graph = ReductionGraph::new();

    let variants = graph.variants_for(&spec.name);
    if variants.is_empty() {
        anyhow::bail!("{}", crate::problem_name::unknown_problem_error(&spec.name));
    }

    let direction = parse_direction(direction_str)?;

    let variant = if spec.variant_values.is_empty() {
        variants[0].clone()
    } else {
        resolve_variant(&spec, &variants)?
    };

    let neighbors = graph.k_neighbors(&spec.name, &variant, max_hops, direction);

    let dir_label = match direction {
        TraversalDirection::Outgoing => "outgoing",
        TraversalDirection::Incoming => "incoming",
        TraversalDirection::Both => "both directions",
    };

    // Build tree structure via BFS with parent tracking
    let tree = graph.k_neighbor_tree(&spec.name, &variant, max_hops, direction);

    let root_label = fmt_node(&spec.name, &variant);

    let mut text = format!(
        "{} — {}-hop neighbors ({})\n\n",
        crate::output::fmt_problem_name(&spec.name),
        max_hops,
        dir_label,
    );

    text.push_str(&root_label);
    text.push('\n');
    render_tree(&tree, &mut text, "");

    // Count unique problem names
    let unique_names: HashSet<&str> = neighbors.iter().map(|n| n.name).collect();
    text.push_str(&format!(
        "\n{} reachable problems in {} hops\n",
        unique_names.len(),
        max_hops,
    ));

    let json = serde_json::json!({
        "source": spec.name,
        "hops": max_hops,
        "direction": direction_str,
        "neighbors": neighbors.iter().map(|n| {
            serde_json::json!({
                "name": n.name,
                "variant": n.variant,
                "hops": n.hops,
            })
        }).collect::<Vec<_>>(),
    });

    let default_name = format!("pred_{}_{}_{}.json", direction_str, spec.name, max_hops);
    out.emit_with_default_name(&default_name, &text, &json)
}

use problemreductions::rules::NeighborTree;

/// Render a tree with box-drawing characters.
fn render_tree(nodes: &[NeighborTree], text: &mut String, prefix: &str) {
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == nodes.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        text.push_str(&format!(
            "{}{}{}\n",
            crate::output::fmt_dim(prefix),
            crate::output::fmt_dim(connector),
            fmt_node(&node.name, &node.variant),
        ));

        if !node.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, child_prefix);
            render_tree(&node.children, text, &new_prefix);
        }
    }
}
