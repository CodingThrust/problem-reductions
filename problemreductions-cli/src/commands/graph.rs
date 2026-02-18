use crate::output::OutputConfig;
use crate::problem_name::{aliases_for, parse_problem_spec, resolve_variant};
use anyhow::{Context, Result};
use problemreductions::registry::collect_schemas;
use problemreductions::rules::{Minimize, MinimizeSteps, ReductionGraph};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;
use std::path::Path;

pub fn list(out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    // Collect data for each problem
    struct Row {
        name: String,
        aliases: Vec<&'static str>,
        num_variants: usize,
        num_reduces_to: usize,
    }
    let rows: Vec<Row> = types
        .iter()
        .map(|name| {
            let aliases = aliases_for(name);
            let num_variants = graph.variants_for(name).len();
            let num_reduces_to = graph.outgoing_reductions(name).len();
            Row {
                name: name.to_string(),
                aliases,
                num_variants,
                num_reduces_to,
            }
        })
        .collect();

    // Compute column widths
    let name_width = rows.iter().map(|r| r.name.len()).max().unwrap_or(7).max(7);
    let alias_width = rows
        .iter()
        .map(|r| {
            if r.aliases.is_empty() {
                0
            } else {
                r.aliases.join(", ").len()
            }
        })
        .max()
        .unwrap_or(7)
        .max(7);

    let summary = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    let mut text = String::new();
    text.push_str(&crate::output::fmt_section(&summary));
    text.push_str(&format!(
        "\n  {:<name_w$}  {:<alias_w$}  {:>8}  {:>10}\n",
        "Problem",
        "Aliases",
        "Variants",
        "Reduces to",
        name_w = name_width,
        alias_w = alias_width,
    ));
    text.push_str(&format!(
        "  {:<name_w$}  {:<alias_w$}  {:>8}  {:>10}\n",
        "─".repeat(name_width),
        "─".repeat(alias_width),
        "────────",
        "──────────",
        name_w = name_width,
        alias_w = alias_width,
    ));

    for row in &rows {
        let alias_str = if row.aliases.is_empty() {
            String::new()
        } else {
            row.aliases.join(", ")
        };
        // Refined approach: pad first, then colorize
        let padded_name = format!("{:<name_w$}", row.name, name_w = name_width);
        let colored_name = crate::output::fmt_problem_name(&padded_name);
        let padded_alias = format!("{:<alias_w$}", alias_str, alias_w = alias_width);
        let colored_alias = crate::output::fmt_dim(&padded_alias);
        text.push_str(&format!(
            "  {}  {}  {:>8}  {:>10}\n",
            colored_name,
            colored_alias,
            row.num_variants,
            row.num_reduces_to,
        ));
    }

    text.push_str(&format!(
        "\nUse `pred show <problem>` to see variants, reductions, and fields.\n"
    ));

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": rows.iter().map(|r| {
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
        anyhow::bail!("Unknown problem: {}", spec.name);
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
    text.push_str(&format!("\n{}\n", crate::output::fmt_section(&format!("Variants ({}):", variants.len()))));
    for v in &variants {
        text.push_str(&format!("  {}\n", format_variant(v)));
    }

    // Show fields from schema (right after variants)
    if let Some(s) = schema {
        text.push_str(&format!("\n{}\n", crate::output::fmt_section(&format!("Fields ({}):", s.fields.len()))));
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
        text.push_str(&format!("\n{}\n", crate::output::fmt_section(&format!("Size fields ({}):", size_fields.len()))));
        for f in size_fields {
            text.push_str(&format!("  {f}\n"));
        }
    }

    // Show reductions from/to this problem
    let outgoing = graph.outgoing_reductions(&spec.name);
    let incoming = graph.incoming_reductions(&spec.name);

    text.push_str(&format!("\n{}\n", crate::output::fmt_section(&format!("Reduces to ({}):", outgoing.len()))));
    for e in &outgoing {
        text.push_str(&format!(
            "  {} {} {} {} {}\n",
            e.source_name,
            format_variant(&e.source_variant),
            crate::output::fmt_outgoing("\u{2192}"),
            crate::output::fmt_problem_name(e.target_name),
            format_variant(&e.target_variant),
        ));
    }

    text.push_str(&format!("\n{}\n", crate::output::fmt_section(&format!("Reduces from ({}):", incoming.len()))));
    for e in &incoming {
        text.push_str(&format!(
            "  {} {} {} {} {}\n",
            crate::output::fmt_problem_name(e.source_name),
            format_variant(&e.source_variant),
            crate::output::fmt_incoming("\u{2192}"),
            e.target_name,
            format_variant(&e.target_variant),
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
                    &src_spec.name, sv, &dst_spec.name, dv, &input_size, &MinimizeSteps,
                ),
                CostChoice::Field(f) => graph.find_cheapest_path(
                    &src_spec.name, sv, &dst_spec.name, dv, &input_size, &Minimize(f),
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
