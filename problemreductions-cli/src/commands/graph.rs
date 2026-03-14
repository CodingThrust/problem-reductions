use crate::output::OutputConfig;
use crate::problem_name::{aliases_for, parse_problem_spec, resolve_problem_ref};
use anyhow::{Context, Result};
use problemreductions::registry::collect_schemas;
use problemreductions::rules::{Minimize, MinimizeSteps, ReductionGraph, TraversalDirection};
use problemreductions::types::ProblemSize;
use problemreductions::{big_o_normal_form, Expr};
use std::collections::BTreeMap;

pub fn list(out: &OutputConfig) -> Result<()> {
    use crate::output::{format_table, Align};

    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    // Collect data: one row per variant, grouped by problem type.
    struct VariantRow {
        /// Full problem/variant name (e.g., "MIS/SimpleGraph/i32")
        display: String,
        /// Aliases (shown only on first variant of each problem)
        aliases: String,
        /// Whether this variant is the default
        is_default: bool,
        /// Number of outgoing reductions from this variant
        rules: usize,
        /// Best-known complexity
        complexity: String,
    }

    let mut rows_data: Vec<VariantRow> = Vec::new();
    for name in &types {
        let variants = graph.variants_for(name);
        let default_variant = graph.default_variant_for(name);
        let aliases = aliases_for(name);
        let alias_str = if aliases.is_empty() {
            String::new()
        } else {
            aliases.join(", ")
        };

        for (i, v) in variants.iter().enumerate() {
            let slash = variant_to_full_slash(v);
            let display = if slash.is_empty() {
                name.to_string()
            } else {
                format!("{name}{slash}")
            };
            let is_default = default_variant.as_ref() == Some(v);
            let rules = graph.outgoing_reductions(name).len();
            let complexity = graph
                .variant_complexity(name, v)
                .map(|c| big_o_of(&Expr::parse(c)))
                .unwrap_or_default();
            rows_data.push(VariantRow {
                display,
                aliases: if i == 0 {
                    alias_str.clone()
                } else {
                    String::new()
                },
                is_default,
                rules: if i == 0 { rules } else { 0 },
                complexity,
            });
        }
    }

    let summary = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    let columns: Vec<(&str, Align, usize)> = vec![
        ("Problem", Align::Left, 7),
        ("Aliases", Align::Left, 7),
        ("Rules", Align::Right, 5),
        ("Complexity", Align::Left, 10),
    ];

    let rows: Vec<Vec<String>> = rows_data
        .iter()
        .map(|r| {
            let label = if r.is_default {
                format!("{} *", r.display)
            } else {
                r.display.clone()
            };
            vec![
                label,
                r.aliases.clone(),
                if r.rules > 0 {
                    r.rules.to_string()
                } else {
                    String::new()
                },
                r.complexity.clone(),
            ]
        })
        .collect();

    let color_fns: Vec<Option<crate::output::CellFormatter>> =
        vec![Some(crate::output::fmt_problem_name), None, None, None];

    let mut text = String::new();
    text.push_str(&crate::output::fmt_section(&summary));
    text.push('\n');
    text.push_str(&format_table(&columns, &rows, &color_fns));
    text.push_str("\n* = default variant\n");
    text.push_str("Use `pred show <problem>` to see reductions and fields.\n");

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "variants": rows_data.iter().map(|r| {
            serde_json::json!({
                "name": r.display,
                "aliases": r.aliases,
                "default": r.is_default,
                "rules": r.rules,
                "complexity": r.complexity,
            })
        }).collect::<Vec<_>>(),
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}

pub fn list_rules(out: &OutputConfig) -> Result<()> {
    use crate::output::{format_table, Align};

    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    struct RuleRow {
        source: String,
        target: String,
        overhead: String,
    }

    let mut rows_data: Vec<RuleRow> = Vec::new();
    for name in &types {
        for edge in graph.outgoing_reductions(name) {
            let source_slash = variant_to_full_slash(&edge.source_variant);
            let target_slash = variant_to_full_slash(&edge.target_variant);
            let oh_parts = fmt_overhead_parts(&edge.overhead.output_size);
            rows_data.push(RuleRow {
                source: format!("{}{}", edge.source_name, source_slash),
                target: format!("{}{}", edge.target_name, target_slash),
                overhead: oh_parts.join(", "),
            });
        }
    }

    let summary = format!("Registered reduction rules: {}\n", rows_data.len());

    let columns: Vec<(&str, Align, usize)> = vec![
        ("Source", Align::Left, 6),
        ("Target", Align::Left, 6),
        ("Overhead", Align::Left, 8),
    ];

    let rows: Vec<Vec<String>> = rows_data
        .iter()
        .map(|r| vec![r.source.clone(), r.target.clone(), r.overhead.clone()])
        .collect();

    let color_fns: Vec<Option<crate::output::CellFormatter>> = vec![
        Some(crate::output::fmt_problem_name),
        Some(crate::output::fmt_problem_name),
        None,
    ];

    let mut text = String::new();
    text.push_str(&crate::output::fmt_section(&summary));
    text.push('\n');
    text.push_str(&format_table(&columns, &rows, &color_fns));
    text.push_str("\nUse `pred show <problem>` for details on a specific problem.\n");

    let json = serde_json::json!({
        "num_rules": rows_data.len(),
        "rules": rows_data.iter().map(|r| {
            serde_json::json!({
                "source": r.source,
                "target": r.target,
                "overhead": r.overhead,
            })
        }).collect::<Vec<_>>(),
    });

    out.emit_with_default_name("pred_rules_list.json", &text, &json)
}

pub fn show(problem: &str, out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();
    let resolved = resolve_problem_ref(problem, &graph)?;
    let name = &resolved.name;
    let variant = &resolved.variant;

    let default_variant = graph.default_variant_for(name);
    let is_default = default_variant.as_ref() == Some(variant);

    let slash = variant_to_full_slash(variant);
    let header = format!("{name}{slash}");
    let mut text = format!("{}\n", crate::output::fmt_problem_name(&header));

    // Show description from schema
    let schemas = collect_schemas();
    let schema = schemas.iter().find(|s| s.name == *name);
    if let Some(s) = schema {
        if !s.description.is_empty() {
            text.push_str(&format!("  {}\n", s.description));
        }
    }

    // Show variant info
    if let Some(c) = graph.variant_complexity(name, variant) {
        text.push_str(&format!(
            "  Best Known Complexity: {}\n",
            big_o_of(&Expr::parse(c))
        ));
    }

    // Show fields from schema
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
    let size_fields = graph.size_field_names(name);
    if !size_fields.is_empty() {
        text.push_str(&format!(
            "\n{}\n",
            crate::output::fmt_section(&format!("Size fields ({}):", size_fields.len()))
        ));
        for f in &size_fields {
            text.push_str(&format!("  {f}\n"));
        }
    }

    // Show reductions filtered to this specific variant
    let outgoing: Vec<_> = graph
        .outgoing_reductions(name)
        .into_iter()
        .filter(|e| &e.source_variant == variant)
        .collect();
    let incoming: Vec<_> = graph
        .incoming_reductions(name)
        .into_iter()
        .filter(|e| &e.target_variant == variant)
        .collect();

    text.push_str(&format!(
        "\n{}\n",
        crate::output::fmt_section(&format!("Outgoing reductions ({}):", outgoing.len()))
    ));
    for e in &outgoing {
        text.push_str(&format!(
            "  {} {}",
            crate::output::fmt_outgoing("\u{2192}"),
            fmt_node(&graph, e.target_name, &e.target_variant),
        ));
        let oh_parts = fmt_overhead_parts(&e.overhead.output_size);
        if !oh_parts.is_empty() {
            text.push_str(&format!("  ({})", oh_parts.join(", ")));
        }
        text.push('\n');
    }

    text.push_str(&format!(
        "\n{}\n",
        crate::output::fmt_section(&format!("Incoming reductions ({}):", incoming.len()))
    ));
    for e in &incoming {
        text.push_str(&format!(
            "  {} {}",
            fmt_node(&graph, e.source_name, &e.source_variant),
            crate::output::fmt_outgoing("\u{2192}"),
        ));
        let oh_parts = fmt_overhead_parts(&e.overhead.output_size);
        if !oh_parts.is_empty() {
            text.push_str(&format!("  ({})", oh_parts.join(", ")));
        }
        text.push('\n');
    }

    let edge_to_json = |e: &problemreductions::rules::ReductionEdgeInfo| {
        serde_json::json!({
            "source": {"name": e.source_name, "variant": e.source_variant},
            "target": {"name": e.target_name, "variant": e.target_variant},
            "overhead": overhead_to_json(&e.overhead.output_size),
        })
    };

    let complexity = graph.variant_complexity(name, variant).unwrap_or("");
    let mut json = serde_json::json!({
        "name": name,
        "variant": variant,
        "default": is_default,
        "complexity": complexity,
        "big_o": if complexity.is_empty() {
            String::new()
        } else {
            big_o_of(&Expr::parse(complexity))
        },
        "size_fields": size_fields,
        "reduces_to": outgoing.iter().map(&edge_to_json).collect::<Vec<_>>(),
        "reduces_from": incoming.iter().map(&edge_to_json).collect::<Vec<_>>(),
    });
    if let Some(s) = schema {
        if let (Some(obj), Ok(schema_val)) = (json.as_object_mut(), serde_json::to_value(s)) {
            obj.insert("schema".to_string(), schema_val);
        }
    }

    let default_name = format!("pred_show_{}.json", name);
    out.emit_with_default_name(&default_name, &text, &json)
}

/// Format an expression as Big O notation using asymptotic normalization.
/// Falls back to wrapping the original expression if normalization fails.
fn big_o_of(expr: &Expr) -> String {
    match big_o_normal_form(expr) {
        Ok(norm) => format!("O({})", norm),
        Err(_) => format!("O({})", expr),
    }
}

/// Format overhead fields as `field = O(...)` strings.
fn fmt_overhead_parts(output_size: &[(&'static str, Expr)]) -> Vec<String> {
    output_size
        .iter()
        .map(|(field, poly)| format!("{field} = {}", big_o_of(poly)))
        .collect()
}

/// Convert overhead fields to JSON entries with Big O notation.
fn overhead_to_json(output_size: &[(&'static str, Expr)]) -> Vec<serde_json::Value> {
    output_size
        .iter()
        .map(|(field, poly)| {
            serde_json::json!({
                "field": field,
                "formula": poly.to_string(),
                "big_o": big_o_of(poly),
            })
        })
        .collect()
}

/// Convert a variant BTreeMap to slash notation showing ALL values.
/// E.g., {graph: "SimpleGraph", weight: "i32"} → "/SimpleGraph/i32".
pub(crate) fn variant_to_full_slash(variant: &BTreeMap<String, String>) -> String {
    if variant.is_empty() {
        String::new()
    } else {
        let vals: Vec<&str> = variant.values().map(|v| v.as_str()).collect();
        format!("/{}", vals.join("/"))
    }
}

/// Format a problem node as **bold name/variant** in slash notation.
/// This is the single source of truth for "name/variant" display.
fn fmt_node(_graph: &ReductionGraph, name: &str, variant: &BTreeMap<String, String>) -> String {
    let slash = variant_to_full_slash(variant);
    crate::output::fmt_problem_name(&format!("{name}{slash}"))
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
                parts.push(fmt_node(graph, &step.name, &step.variant));
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
            fmt_node(graph, &from.name, &from.variant),
            crate::output::fmt_outgoing("→"),
            fmt_node(graph, &to.name, &to.variant),
        ));
        let oh = &overheads[i];
        for (field, poly) in &oh.output_size {
            text.push_str(&format!("    {field} = {}\n", big_o_of(poly)));
        }
    }

    // Show composed overall overhead for multi-step paths
    if reduction_path.len() > 1 {
        let composed = graph.compose_path_overhead(reduction_path);
        text.push_str(&format!("\n  {}:\n", crate::output::fmt_section("Overall")));
        for (field, poly) in &composed.output_size {
            text.push_str(&format!("    {field} = {}\n", big_o_of(poly)));
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
                "overhead": overhead_to_json(&oh.output_size),
            })
        })
        .collect();

    let composed = graph.compose_path_overhead(reduction_path);
    let overall = overhead_to_json(&composed.output_size);

    serde_json::json!({
        "steps": reduction_path.len(),
        "path": steps_json,
        "overall_overhead": overall,
    })
}

pub fn path(
    source: &str,
    target: &str,
    cost: &str,
    all: bool,
    max_paths: usize,
    out: &OutputConfig,
) -> Result<()> {
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

    // Resolve source and target to exact variant nodes
    let src_ref = resolve_problem_ref(source, &graph)?;
    let dst_ref = resolve_problem_ref(target, &graph)?;

    if all {
        return path_all(
            &graph,
            &src_ref.name,
            &src_ref.variant,
            &dst_ref.name,
            &dst_ref.variant,
            max_paths,
            out,
        );
    }

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

    let best_path = match cost_choice {
        CostChoice::Steps => graph.find_cheapest_path(
            &src_ref.name,
            &src_ref.variant,
            &dst_ref.name,
            &dst_ref.variant,
            &input_size,
            &MinimizeSteps,
        ),
        CostChoice::Field(f) => graph.find_cheapest_path(
            &src_ref.name,
            &src_ref.variant,
            &dst_ref.name,
            &dst_ref.variant,
            &input_size,
            &Minimize(f),
        ),
    };

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
    max_paths: usize,
    out: &OutputConfig,
) -> Result<()> {
    // Fetch one extra to detect truncation
    let mut all_paths =
        graph.find_paths_up_to(src_name, src_variant, dst_name, dst_variant, max_paths + 1);

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

    let truncated = all_paths.len() > max_paths;
    if truncated {
        all_paths.truncate(max_paths);
    }

    let returned = all_paths.len();
    let mut text = format!(
        "Found {} paths from {} to {}:\n",
        returned, src_name, dst_name
    );
    for (idx, p) in all_paths.iter().enumerate() {
        text.push_str(&format!("\n--- Path {} ---\n", idx + 1));
        text.push_str(&format_path_text(graph, p));
    }
    if truncated {
        text.push_str(&format!(
            "\n(showing {max_paths} of more paths; use --max-paths to increase)\n"
        ));
    }

    let paths_json: Vec<serde_json::Value> = all_paths
        .iter()
        .map(|p| format_path_json(graph, p))
        .collect();

    let json = serde_json::json!({
        "paths": paths_json,
        "truncated": truncated,
        "returned": returned,
        "max_paths": max_paths,
    });

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

        // Write manifest
        let manifest = serde_json::json!({
            "paths": returned,
            "truncated": truncated,
            "max_paths": max_paths,
        });
        let manifest_file = dir.join("manifest.json");
        let manifest_content =
            serde_json::to_string_pretty(&manifest).context("Failed to serialize manifest")?;
        std::fs::write(&manifest_file, &manifest_content)
            .with_context(|| format!("Failed to write {}", manifest_file.display()))?;

        out.info(&format!(
            "Wrote {} path files to {}{}",
            returned,
            dir.display(),
            if truncated {
                " (truncated; use --max-paths to increase)".to_string()
            } else {
                String::new()
            }
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
    let graph = ReductionGraph::new();
    let resolved = resolve_problem_ref(problem, &graph)?;
    let spec_name = resolved.name.clone();
    let variant = resolved.variant;

    let direction = parse_direction(direction_str)?;

    let neighbors = graph.k_neighbors(&spec_name, &variant, max_hops, direction);

    let dir_label = match direction {
        TraversalDirection::Outgoing => "outgoing",
        TraversalDirection::Incoming => "incoming",
        TraversalDirection::Both => "both directions",
    };

    // Build tree structure via BFS with parent tracking
    let tree = graph.k_neighbor_tree(&spec_name, &variant, max_hops, direction);

    let root_label = fmt_node(&graph, &spec_name, &variant);

    let header_label = fmt_node(&graph, &spec_name, &variant);
    let mut text = format!(
        "{} — {}-hop neighbors ({})\n\n",
        header_label, max_hops, dir_label,
    );

    text.push_str(&root_label);
    text.push('\n');
    render_tree(&graph, &tree, &mut text, "");

    text.push_str(&format!(
        "\n{} reachable nodes in {} hops\n",
        neighbors.len(),
        max_hops,
    ));

    let json = serde_json::json!({
        "source": spec_name,
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

    let default_name = format!("pred_{}_{}_{}.json", direction_str, spec_name, max_hops);
    out.emit_with_default_name(&default_name, &text, &json)
}

use problemreductions::rules::NeighborTree;

/// Render a tree with box-drawing characters.
fn render_tree(graph: &ReductionGraph, nodes: &[NeighborTree], text: &mut String, prefix: &str) {
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == nodes.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        text.push_str(&format!(
            "{}{}{}\n",
            crate::output::fmt_dim(prefix),
            crate::output::fmt_dim(connector),
            fmt_node(graph, &node.name, &node.variant),
        ));

        if !node.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, child_prefix);
            render_tree(graph, &node.children, text, &new_prefix);
        }
    }
}
