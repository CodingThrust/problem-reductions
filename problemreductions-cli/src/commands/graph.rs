use crate::dispatch::{load_problem, read_input, ProblemJson};
use crate::output::OutputConfig;
use crate::problem_name::{aliases_for, parse_problem_spec, resolve_problem_ref};
use anyhow::Result;
use problemreductions::registry::collect_schemas;
use problemreductions::registry::ProblemCategory;
use problemreductions::rules::{ExecutedPath, ReductionGraph, ReductionPath, TraversalFlow};
use problemreductions::size::{problem_size_dominates, size_growth_dominates};
use problemreductions::{Expr, Growth};
use std::any::Any;
use std::collections::BTreeMap;
use std::path::Path;

pub fn list(
    query: Option<&str>,
    category: Option<ProblemCategory>,
    all: bool,
    verbose: bool,
    out: &OutputConfig,
) -> Result<()> {
    use crate::output::{format_table, Align};

    let needs_variant_rows = verbose || out.json || out.output.is_some();
    let catalog = problemreductions::registry::problem_types();
    let mut variant_aliases = BTreeMap::<&str, Vec<&str>>::new();
    let mut variant_counts = BTreeMap::<&str, usize>::new();
    let mut aliases_by_variant =
        BTreeMap::<&str, BTreeMap<BTreeMap<String, String>, &'static [&'static str]>>::new();
    for entry in problemreductions::registry::variant_entries() {
        variant_aliases
            .entry(entry.name)
            .or_default()
            .extend(entry.aliases);
        *variant_counts.entry(entry.name).or_default() += 1;
        if needs_variant_rows {
            aliases_by_variant
                .entry(entry.name)
                .or_default()
                .insert(entry.variant_map(), entry.aliases);
        }
    }
    let query = query.map(str::to_lowercase);
    let selected = catalog
        .iter()
        .filter(|problem| {
            category.is_none_or(|wanted| problem.category == wanted)
                && query.as_ref().is_none_or(|needle| {
                    problem.canonical_name.to_lowercase().contains(needle)
                        || problem.display_name.to_lowercase().contains(needle)
                        || problem
                            .aliases
                            .iter()
                            .any(|alias| alias.to_lowercase().contains(needle))
                        || variant_aliases
                            .get(problem.canonical_name)
                            .is_some_and(|aliases| {
                                aliases
                                    .iter()
                                    .any(|alias| alias.to_lowercase().contains(needle))
                            })
                })
        })
        .collect::<Vec<_>>();
    let graph = needs_variant_rows.then(ReductionGraph::new);
    let num_reductions = problemreductions::rules::registry::reduction_entries().len();

    // Collect data: one row per variant, grouped by problem type.
    struct VariantRow {
        /// Full problem/variant name (e.g., "MIS/SimpleGraph/i64")
        display: String,
        /// Aliases (shown only on first variant of each problem)
        aliases: String,
        /// Whether this variant is the default
        is_default: bool,
        /// Number of outgoing reductions from this variant
        rules: usize,
        /// Best-known complexity
        complexity: String,
        category: ProblemCategory,
    }

    let mut rows_data: Vec<VariantRow> = Vec::new();
    if let Some(graph) = &graph {
        for problem in &selected {
            let name = problem.canonical_name;
            let variants = graph.variants_for(name);
            let default_variant = graph.default_variant_for(name);
            let problem_aliases = aliases_for(name);
            let rules = graph.outgoing_reductions(name).len();

            for (i, v) in variants.iter().enumerate() {
                let slash = variant_to_full_slash(v);
                let display = if slash.is_empty() {
                    name.to_string()
                } else {
                    format!("{name}{slash}")
                };
                let is_default = default_variant.as_ref() == Some(v);
                let complexity = graph
                    .variant_complexity(name, v)
                    .map(|c| big_o_of(&Expr::parse(c)))
                    .unwrap_or_default();

                let mut parts: Vec<String> = Vec::new();
                if i == 0 {
                    for alias in &problem_aliases {
                        push_alias_part(&mut parts, alias);
                    }
                }
                if let Some(aliases) = aliases_by_variant
                    .get(name)
                    .and_then(|by_variant| by_variant.get(v))
                {
                    for alias in *aliases {
                        push_alias_part(&mut parts, alias);
                    }
                }

                rows_data.push(VariantRow {
                    display,
                    aliases: parts.join(", "),
                    is_default,
                    rules: if i == 0 { rules } else { 0 },
                    complexity,
                    category: problem.category,
                });
            }
        }
    }

    let mut category_counts = BTreeMap::new();
    for problem in &catalog {
        *category_counts.entry(problem.category).or_insert(0usize) += 1;
    }

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

    let expanded = all || query.is_some() || category.is_some() || verbose;
    let mut text = format!(
        "{}\n\n",
        crate::output::fmt_section(&format!(
            "Registered catalog: {} problem types, {} variant nodes, {} reduction rules",
            catalog.len(),
            variant_counts.values().sum::<usize>(),
            num_reductions,
        ))
    );
    if expanded {
        if selected.is_empty() {
            text.push_str("No matching problem types.\n");
        } else if verbose {
            text.push_str(&format_table(&columns, &rows, &color_fns));
            text.push_str("\n* = default variant\n");
        } else {
            let compact_rows = selected
                .iter()
                .map(|problem| {
                    let mut aliases = problem.aliases.to_vec();
                    if let Some(extra) = variant_aliases.get(problem.canonical_name) {
                        for alias in extra {
                            if !aliases
                                .iter()
                                .any(|known| known.eq_ignore_ascii_case(alias))
                            {
                                aliases.push(alias);
                            }
                        }
                    }
                    vec![
                        problem.canonical_name.to_string(),
                        aliases.join(", "),
                        problem.category.to_string(),
                        variant_counts
                            .get(problem.canonical_name)
                            .copied()
                            .unwrap_or_default()
                            .to_string(),
                    ]
                })
                .collect::<Vec<_>>();
            text.push_str(&format_table(
                &[
                    ("Problem", Align::Left, 7),
                    ("Aliases", Align::Left, 7),
                    ("Category", Align::Left, 8),
                    ("Variants", Align::Right, 8),
                ],
                &compact_rows,
                &[Some(crate::output::fmt_problem_name), None, None, None],
            ));
        }
        text.push_str("\nUse `pred show <problem>` for fields, variants, and reductions.\n");
    } else {
        let category_rows = category_counts
            .iter()
            .map(|(name, count)| vec![name.to_string(), count.to_string()])
            .collect::<Vec<_>>();
        text.push_str(&format_table(
            &[("Category", Align::Left, 8), ("Problems", Align::Right, 8)],
            &category_rows,
            &[None, None],
        ));
        text.push_str(
            "\nSearch with `pred list <query>`, browse a category with `pred list --category <name>`, or use `pred list --all`.\n",
        );
    }

    let json = serde_json::json!({
        "num_types": selected.len(),
        "num_reductions": num_reductions,
        "num_variant_nodes": rows_data.len(),
        "variants": rows_data.iter().map(|r| {
            serde_json::json!({
                "name": r.display,
                "aliases": r.aliases,
                "default": r.is_default,
                "rules": r.rules,
                "complexity": r.complexity,
                "category": r.category,
            })
        }).collect::<Vec<_>>(),
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}

pub fn list_rules(query: Option<&str>, all: bool, verbose: bool, out: &OutputConfig) -> Result<()> {
    use crate::output::{format_table, Align};

    let num_registered = problemreductions::rules::registry::reduction_entries().len();
    let expanded = all || query.is_some() || verbose || out.json || out.output.is_some();
    if !expanded {
        let text = format!(
            "{}\n\nSearch with `pred list --rules <query>` or use `pred list --rules --all`. Add `--verbose` for size contracts.\n",
            crate::output::fmt_section(&format!("Registered reduction rules: {num_registered}"))
        );
        let json = serde_json::json!({ "num_rules": num_registered, "rules": [] });
        return out.emit_with_default_name("pred_rules_list.json", &text, &json);
    }

    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    struct RuleRow {
        source: String,
        target: String,
        size_contract: String,
    }

    let mut rows_data: Vec<RuleRow> = Vec::new();
    for name in &types {
        for edge in graph.outgoing_reductions(name) {
            let source_slash = variant_to_full_slash(&edge.source_variant);
            let target_slash = variant_to_full_slash(&edge.target_variant);
            let size_parts = fmt_size_contract(&edge.size_contract);
            rows_data.push(RuleRow {
                source: format!("{}{}", edge.source_name, source_slash),
                target: format!("{}{}", edge.target_name, target_slash),
                size_contract: size_parts.join(", "),
            });
        }
    }

    let query = query.map(str::to_lowercase);
    let alias_matches = query
        .as_ref()
        .map(|needle| {
            problemreductions::registry::variant_entries()
                .into_iter()
                .filter(|entry| {
                    entry
                        .aliases
                        .iter()
                        .any(|alias| alias.to_lowercase().contains(needle))
                })
                .map(|entry| entry.name.to_lowercase())
                .chain(
                    problemreductions::registry::problem_types()
                        .into_iter()
                        .filter(|problem| {
                            problem
                                .aliases
                                .iter()
                                .any(|alias| alias.to_lowercase().contains(needle))
                        })
                        .map(|problem| problem.canonical_name.to_lowercase()),
                )
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let selected = rows_data
        .iter()
        .filter(|row| {
            query.as_ref().is_none_or(|needle| {
                row.source.to_lowercase().contains(needle)
                    || row.target.to_lowercase().contains(needle)
                    || alias_matches.iter().any(|name| {
                        row.source.to_lowercase().contains(name)
                            || row.target.to_lowercase().contains(name)
                    })
            })
        })
        .collect::<Vec<_>>();

    let columns: Vec<(&str, Align, usize)> = vec![
        ("Source", Align::Left, 6),
        ("Target", Align::Left, 6),
        ("Size change", Align::Left, 8),
    ];

    let rows: Vec<Vec<String>> = selected
        .iter()
        .map(|r| {
            let mut row = vec![r.source.clone(), r.target.clone()];
            if verbose {
                row.push(r.size_contract.clone());
            }
            row
        })
        .collect();

    let color_fns: Vec<Option<crate::output::CellFormatter>> = vec![
        Some(crate::output::fmt_problem_name),
        Some(crate::output::fmt_problem_name),
        None,
    ];

    let mut text = format!(
        "{}\n",
        crate::output::fmt_section(&format!("Registered reduction rules: {}", rows_data.len()))
    );
    if expanded {
        let compact_columns = if verbose {
            columns
        } else {
            vec![("Source", Align::Left, 6), ("Target", Align::Left, 6)]
        };
        let compact_colors: Vec<Option<crate::output::CellFormatter>> = if verbose {
            color_fns
        } else {
            vec![
                Some(crate::output::fmt_problem_name),
                Some(crate::output::fmt_problem_name),
            ]
        };
        text.push('\n');
        text.push_str(&format_table(&compact_columns, &rows, &compact_colors));
        text.push_str("\nUse `pred show <problem>` for details on a specific problem.\n");
    } else {
        text.push_str(
            "\nSearch with `pred list --rules <query>` or use `pred list --rules --all`. Add `--verbose` for size contracts.\n",
        );
    }

    let json = serde_json::json!({
        "num_rules": selected.len(),
        "rules": selected.iter().map(|r| {
            serde_json::json!({
                "source": r.source,
                "target": r.target,
                "size_contract": r.size_contract,
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

    // Show the named fields available in concrete and symbolic size reports.
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
        let size_parts = fmt_size_contract(&e.size_contract);
        if !size_parts.is_empty() {
            text.push_str(&format!("  ({})", size_parts.join(", ")));
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
        let size_parts = fmt_size_contract(&e.size_contract);
        if !size_parts.is_empty() {
            text.push_str(&format!("  ({})", size_parts.join(", ")));
        }
        text.push('\n');
    }

    let edge_to_json = |e: &problemreductions::rules::ReductionEdgeInfo| {
        serde_json::json!({
            "source": {"name": e.source_name, "variant": e.source_variant},
            "target": {"name": e.target_name, "variant": e.target_variant},
            "size_contract": size_contract_to_json(&e.size_contract),
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

/// Format an expression as Big O notation using the growth domain's canonical
/// renderer. Bounded classes render as `O(<expr>)`; a growth the domain cannot
/// bound symbolically (nonlinear exponent / factorial) renders as the honest
/// `O(?)` marker — never the raw unreduced expression.
fn big_o_of(expr: &Expr) -> String {
    Growth::from_expr(expr).to_big_o()
}

enum StrongestContractRelation<'a> {
    Exact(&'a Expr),
    UpperBound(&'a Expr),
    Unavailable(&'a str),
}

fn strongest_contract_fields(
    contract: &problemreductions::rules::ReductionSizeContract,
) -> BTreeMap<&str, StrongestContractRelation<'_>> {
    let mut fields = BTreeMap::new();
    if let Some(transform) = contract.transform() {
        for (field, expression) in transform.expressions() {
            let relation = match transform.relation() {
                problemreductions::size::SizeRelation::Exact => {
                    StrongestContractRelation::Exact(expression)
                }
                problemreductions::size::SizeRelation::UpperBound => {
                    StrongestContractRelation::UpperBound(expression)
                }
            };
            fields.insert(field, relation);
        }
    }
    for unavailable in contract.unavailable() {
        fields
            .entry(unavailable.field)
            .or_insert(StrongestContractRelation::Unavailable(unavailable.reason));
    }
    fields
}

fn fmt_size_contract(
    contract: &Result<
        problemreductions::rules::ReductionSizeContract,
        problemreductions::rules::SizeContractError,
    >,
) -> Vec<String> {
    let contract = match contract {
        Ok(contract) => contract,
        Err(error) => return vec![format!("invalid: {error}")],
    };
    strongest_contract_fields(contract)
        .into_iter()
        .map(|(field, relation)| match relation {
            StrongestContractRelation::Exact(expression) => {
                format!("{field} = {expression}")
            }
            StrongestContractRelation::UpperBound(expression) => {
                format!("{field} <= {expression}")
            }
            StrongestContractRelation::Unavailable(reason) => {
                format!("{field} unavailable: {reason}")
            }
        })
        .collect()
}

fn strongest_size_contract_to_json(
    contract: &Result<
        problemreductions::rules::ReductionSizeContract,
        problemreductions::rules::SizeContractError,
    >,
) -> serde_json::Value {
    match contract {
        Ok(contract) => serde_json::Value::Array(
            strongest_contract_fields(contract)
                .into_iter()
                .map(|(field, relation)| match relation {
                    StrongestContractRelation::Exact(expression) => serde_json::json!({
                        "field": field,
                        "relation": "exact",
                        "formula": expression.to_string(),
                    }),
                    StrongestContractRelation::UpperBound(expression) => serde_json::json!({
                        "field": field,
                        "relation": "upper_bound",
                        "formula": expression.to_string(),
                    }),
                    StrongestContractRelation::Unavailable(reason) => serde_json::json!({
                        "field": field,
                        "relation": "unavailable",
                        "reason": reason,
                    }),
                })
                .collect(),
        ),
        Err(error) => serde_json::json!({"error": error.to_string()}),
    }
}

pub(crate) fn size_contract_to_json(
    contract: &Result<
        problemreductions::rules::ReductionSizeContract,
        problemreductions::rules::SizeContractError,
    >,
) -> serde_json::Value {
    match contract {
        Ok(contract) => serde_json::json!({
            "relation": contract.transform().map(|transform| transform.relation()),
            "fields": contract.transform().map(|transform| transform.expressions().map(|(field, expression)| {
                serde_json::json!({
                    "field": field,
                    "formula": expression.to_string(),
                    "big_o": big_o_of(expression),
                })
            }).collect::<Vec<_>>()).unwrap_or_default(),
            "unavailable": contract.unavailable(),
        }),
        Err(error) => serde_json::json!({"error": error.to_string()}),
    }
}

/// Convert a variant BTreeMap to slash notation showing ALL values.
/// E.g., {graph: "SimpleGraph", weight: "i64"} → "/SimpleGraph/i64".
pub(crate) fn variant_to_full_slash(variant: &BTreeMap<String, String>) -> String {
    if variant.is_empty() {
        String::new()
    } else {
        let vals: Vec<&str> = variant.values().map(|v| v.as_str()).collect();
        format!("/{}", vals.join("/"))
    }
}

/// Build a hint string listing available variants for a problem name.
/// Returns an empty string if there is only one variant (nothing to disambiguate).
pub(crate) fn variant_hint_for(graph: &ReductionGraph, name: &str) -> String {
    let variants = graph.variants_for(name);
    if variants.len() <= 1 {
        return String::new();
    }
    let list: Vec<String> = variants
        .iter()
        .map(|v| format!("{}{}", name, variant_to_full_slash(v)))
        .collect();
    format!(
        "\nTip: try specifying a variant. Available variants for {}:\n  {}\n",
        name,
        list.join(", "),
    )
}

/// Format a problem node as **bold name/variant** in slash notation.
/// This is the single source of truth for "name/variant" display.
fn fmt_node(_graph: &ReductionGraph, name: &str, variant: &BTreeMap<String, String>) -> String {
    let slash = variant_to_full_slash(variant);
    crate::output::fmt_problem_name(&format!("{name}{slash}"))
}

struct ComposedPathSize {
    transform: Result<
        Option<problemreductions::size::SizeTransform>,
        problemreductions::rules::PathSizeError,
    >,
}

enum PreparedSizeRelation {
    Exact(String),
    UpperBound(String),
    Unavailable(String),
}

struct PreparedSizeField {
    field: String,
    relation: PreparedSizeRelation,
}

fn terminal_size_contract(
    graph: &ReductionGraph,
    path: &ReductionPath,
) -> Option<problemreductions::rules::ReductionSizeContract> {
    path.steps
        .windows(2)
        .last()
        .and_then(|pair| {
            graph.find_entry(
                &pair[0].name,
                &pair[0].variant,
                &pair[1].name,
                &pair[1].variant,
            )
        })
        .and_then(|entry| entry.size_contract.ok())
}

fn composed_path_size(graph: &ReductionGraph, path: &ReductionPath) -> ComposedPathSize {
    ComposedPathSize {
        transform: graph.compose_path_size_transform(path),
    }
}

fn prepare_overall_size_fields(
    graph: &ReductionGraph,
    path: &ReductionPath,
) -> Vec<PreparedSizeField> {
    let Some(target) = path.target() else {
        return Vec::new();
    };
    let composed = composed_path_size(graph, path);
    let terminal_contract = terminal_size_contract(graph, path);

    graph
        .size_field_names(target)
        .into_iter()
        .map(|field| {
            let expression = composed
                .transform
                .as_ref()
                .ok()
                .and_then(|transform| transform.as_ref())
                .and_then(|transform| {
                    transform
                        .get(&field)
                        .map(|expression| (transform.relation(), expression))
                });
            let relation = if let Some((relation, expression)) = expression {
                match relation {
                    problemreductions::size::SizeRelation::Exact => {
                        PreparedSizeRelation::Exact(expression.to_string())
                    }
                    problemreductions::size::SizeRelation::UpperBound => {
                        PreparedSizeRelation::UpperBound(expression.to_string())
                    }
                }
            } else if let Some(unavailable) = terminal_contract.as_ref().and_then(|contract| {
                contract
                    .unavailable()
                    .iter()
                    .find(|unavailable| unavailable.field == field)
            }) {
                PreparedSizeRelation::Unavailable(unavailable.reason.to_string())
            } else if terminal_contract
                .as_ref()
                .and_then(|contract| contract.transform())
                .is_some_and(|transform| transform.get(&field).is_some())
            {
                PreparedSizeRelation::Unavailable(match &composed.transform {
                    Err(error) => error.to_string(),
                    Ok(_) => {
                        format!("no composed size relation is available for target field {field}")
                    }
                })
            } else {
                let reason = match &composed.transform {
                    Err(error) => error.to_string(),
                    Ok(_) => {
                        format!("no symbolic size relation is registered for target field {field}")
                    }
                };
                PreparedSizeRelation::Unavailable(reason)
            };
            PreparedSizeField { field, relation }
        })
        .collect()
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
        match graph.find_entry(&from.name, &from.variant, &to.name, &to.variant) {
            Some(entry) => {
                for part in fmt_size_contract(&entry.size_contract) {
                    text.push_str(&format!("    {part}\n"));
                }
            }
            None => text.push_str("    unregistered edge\n"),
        }
    }

    if reduction_path.len() > 1 {
        text.push_str(&format!("\n  {}:\n", crate::output::fmt_section("Overall")));
        for field in prepare_overall_size_fields(graph, reduction_path) {
            match field.relation {
                PreparedSizeRelation::Exact(expression) => {
                    text.push_str(&format!("    {} = {expression}\n", field.field));
                }
                PreparedSizeRelation::UpperBound(expression) => {
                    text.push_str(&format!("    {} <= {expression}\n", field.field));
                }
                PreparedSizeRelation::Unavailable(reason) => {
                    text.push_str(&format!("    {} unavailable: {reason}\n", field.field));
                }
            }
        }
    }

    text
}

pub(crate) fn format_path_json(
    graph: &ReductionGraph,
    reduction_path: &problemreductions::rules::ReductionPath,
) -> serde_json::Value {
    let steps_json: Vec<serde_json::Value> = reduction_path
        .steps
        .windows(2)
        .enumerate()
        .map(|(i, pair)| {
            let size_contract = graph
                .find_entry(
                    &pair[0].name,
                    &pair[0].variant,
                    &pair[1].name,
                    &pair[1].variant,
                )
                .map(|entry| strongest_size_contract_to_json(&entry.size_contract))
                .unwrap_or_else(|| serde_json::json!({"error": "unregistered edge"}));
            serde_json::json!({
                "from": {"name": pair[0].name, "variant": pair[0].variant},
                "to": {"name": pair[1].name, "variant": pair[1].variant},
                "step": i + 1,
                "size_contract": size_contract,
            })
        })
        .collect();

    let fields = prepare_overall_size_fields(graph, reduction_path)
        .into_iter()
        .map(|field| match field.relation {
            PreparedSizeRelation::Exact(expression) => serde_json::json!({
                "field": field.field,
                "relation": "exact",
                "formula": expression,
            }),
            PreparedSizeRelation::UpperBound(expression) => serde_json::json!({
                "field": field.field,
                "relation": "upper_bound",
                "formula": expression,
            }),
            PreparedSizeRelation::Unavailable(reason) => serde_json::json!({
                "field": field.field,
                "relation": "unavailable",
                "reason": reason,
            }),
        })
        .collect::<Vec<_>>();
    let mut overall_object = serde_json::Map::new();
    overall_object.insert("fields".to_string(), serde_json::json!(fields));
    serde_json::json!({
        "steps": reduction_path.len(),
        "path": steps_json,
        "overall_size": overall_object,
    })
}

pub fn path(
    source: &str,
    target: &str,
    limit: usize,
    unfiltered: bool,
    instance: Option<&Path>,
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
    if let Some(instance) = instance {
        let content = read_input(instance)?;
        let problem_json: ProblemJson = serde_json::from_str(&content).map_err(|error| {
            anyhow::anyhow!("Invalid problem JSON in {}: {error}", instance.display())
        })?;
        let loaded = load_problem(
            &problem_json.problem_type,
            &problem_json.variant,
            problem_json.data,
        )?;
        if loaded.problem_name() != src_ref.name || loaded.variant_map() != src_ref.variant {
            anyhow::bail!(
                "Source argument resolves to {}{} but {} contains {}{}",
                src_ref.name,
                variant_to_full_slash(&src_ref.variant),
                instance.display(),
                loaded.problem_name(),
                variant_to_full_slash(&loaded.variant_map()),
            );
        }
        path_concrete(
            &graph,
            &src_ref.name,
            &src_ref.variant,
            &dst_ref.name,
            &dst_ref.variant,
            limit,
            unfiltered,
            loaded.as_any(),
            out,
        )
    } else {
        path_symbolic(
            &graph,
            &src_ref.name,
            &src_ref.variant,
            &dst_ref.name,
            &dst_ref.variant,
            limit,
            unfiltered,
            out,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn path_symbolic(
    graph: &ReductionGraph,
    src_name: &str,
    src_variant: &BTreeMap<String, String>,
    dst_name: &str,
    dst_variant: &BTreeMap<String, String>,
    limit: usize,
    unfiltered: bool,
    out: &OutputConfig,
) -> Result<()> {
    let mut batch = find_path_batch(graph, src_name, src_variant, dst_name, dst_variant, limit)?;

    if batch.paths.is_empty() && !batch.truncated {
        let variant_hint = variant_hint_for(graph, dst_name);
        anyhow::bail!(
            "No reduction path from {} to {}\n\
             {variant_hint}\n\
             Usage: pred path <SOURCE> <TARGET>\n\
             Example: pred path MIS QUBO\n\n\
             Run `pred show {}` and `pred show {}` to check available reductions.",
            src_name,
            dst_name,
            src_name,
            dst_name,
        );
    }
    if !unfiltered {
        let flags = symbolic_pareto_flags(graph, &batch.paths);
        batch.paths = retain_selected(batch.paths, &flags);
    }

    let json_output = out.output.is_some() || out.json;
    let json = if json_output {
        path_batch_json(graph, &batch, None)?
    } else {
        serde_json::Value::Null
    };
    let text = if json_output {
        String::new()
    } else {
        render_paths_text(
            graph,
            &batch.paths,
            src_name,
            dst_name,
            batch.truncated,
            limit,
        )
    };
    out.emit_with_default_name("", &text, &json)
}

pub(crate) struct PathBatch {
    pub(crate) paths: Vec<ReductionPath>,
    pub(crate) truncated: bool,
}

pub(crate) const MAX_PATHS: usize = 999;
pub(crate) const PATH_LIMIT_ERROR: &str = "limit must be an integer from 1 to 999 or 'all'";

pub(crate) fn validate_path_limit(limit: usize) -> std::result::Result<usize, String> {
    (1..=MAX_PATHS)
        .contains(&limit)
        .then_some(limit)
        .ok_or_else(|| PATH_LIMIT_ERROR.to_string())
}

pub(crate) fn parse_path_limit(value: &str) -> std::result::Result<usize, String> {
    if value == "all" {
        return Ok(MAX_PATHS);
    }
    let limit = value
        .parse::<usize>()
        .map_err(|_| PATH_LIMIT_ERROR.to_string())?;
    validate_path_limit(limit)
}

pub(crate) fn find_path_batch(
    graph: &ReductionGraph,
    src_name: &str,
    src_variant: &BTreeMap<String, String>,
    dst_name: &str,
    dst_variant: &BTreeMap<String, String>,
    limit: usize,
) -> Result<PathBatch> {
    validate_path_limit(limit).map_err(anyhow::Error::msg)?;
    let mut paths = graph.find_paths_up_to(src_name, src_variant, dst_name, dst_variant, limit + 1);
    let truncated = paths.len() > limit;
    paths.truncate(limit);
    Ok(PathBatch { paths, truncated })
}

pub(crate) fn path_batch_json(
    graph: &ReductionGraph,
    batch: &PathBatch,
    executed: Option<&[ExecutedPath]>,
) -> Result<serde_json::Value> {
    let paths = match executed {
        Some(executed) => {
            if executed.len() != batch.paths.len() {
                anyhow::bail!(
                    "executed path count {} does not match enumerated path count {}",
                    executed.len(),
                    batch.paths.len()
                );
            }
            executed
                .iter()
                .map(format_concrete_path_json)
                .collect::<Vec<_>>()
        }
        None => batch
            .paths
            .iter()
            .map(|path| format_path_json(graph, path))
            .collect::<Vec<_>>(),
    };
    Ok(serde_json::json!({
        "paths": paths,
        "truncated": batch.truncated,
    }))
}

fn pareto_flags_by<T, C>(
    candidates: &[T],
    cost: impl FnMut(&T) -> Option<C>,
    dominates: impl Fn(&C, &C) -> bool,
) -> Vec<bool> {
    let costs = candidates.iter().map(cost).collect::<Vec<_>>();
    costs
        .iter()
        .enumerate()
        .map(|(index, cost)| {
            !costs.iter().enumerate().any(|(other_index, other)| {
                index != other_index
                    && cost
                        .as_ref()
                        .zip(other.as_ref())
                        .is_some_and(|(cost, other)| dominates(other, cost))
            })
        })
        .collect()
}

pub(crate) fn retain_selected<T>(items: Vec<T>, selected: &[bool]) -> Vec<T> {
    items
        .into_iter()
        .zip(selected)
        .filter_map(|(item, selected)| selected.then_some(item))
        .collect()
}

pub(crate) fn symbolic_pareto_flags(graph: &ReductionGraph, paths: &[ReductionPath]) -> Vec<bool> {
    let target_fields = paths
        .first()
        .and_then(|path| path.steps.last())
        .map(|target| graph.size_field_names(&target.name))
        .unwrap_or_default();
    pareto_flags_by(
        paths,
        |path| {
            let growth = graph
                .compose_path_size_transform(path)
                .ok()
                .flatten()?
                .project_growth();
            target_fields
                .iter()
                .all(|field| growth.get(field).is_some())
                .then_some(growth)
        },
        size_growth_dominates,
    )
}

pub(crate) fn concrete_pareto_flags(executed: &[ExecutedPath]) -> Vec<bool> {
    pareto_flags_by(
        executed,
        |path| {
            let target = path.path.steps.last().expect("path has a target node");
            Some(ReductionGraph::compute_problem_size(
                &target.name,
                &target.variant,
                path.target_problem_any(),
            ))
        },
        problem_size_dominates,
    )
}

fn path_truncation_note(limit: usize) -> &'static str {
    if limit == MAX_PATHS {
        "\n(more paths exist; path search is capped at 999)\n"
    } else {
        "\n(more paths exist; increase --limit, maximum: 999)\n"
    }
}

/// Render the symbolic path listing (header + per-path chains with normalized
/// size contracts). Extracted so it is built only for text output and can be
/// exercised in-process by regression tests without spawning the binary.
fn render_paths_text(
    graph: &ReductionGraph,
    paths: &[ReductionPath],
    src_name: &str,
    dst_name: &str,
    truncated: bool,
    limit: usize,
) -> String {
    let mut text = format!(
        "Found {} paths from {} to {}:\n",
        paths.len(),
        src_name,
        dst_name
    );
    for (idx, p) in paths.iter().enumerate() {
        text.push_str(&format!("\n--- Path {} ---\n", idx + 1));
        text.push_str(&format_path_text(graph, p));
    }
    if truncated {
        text.push_str(path_truncation_note(limit));
    }
    text
}

fn measured_size_json(size: &problemreductions::ProblemSize) -> serde_json::Value {
    serde_json::json!({
        "fields": size.components.iter().map(|(field, value)| {
            serde_json::json!({"field": field, "value": value})
        }).collect::<Vec<_>>()
    })
}

pub(crate) fn format_concrete_path_json(executed: &ExecutedPath) -> serde_json::Value {
    let sizes = executed.target_sizes();
    let steps = executed
        .path
        .steps
        .windows(2)
        .zip(&sizes)
        .enumerate()
        .map(|(index, (pair, size))| {
            serde_json::json!({
                "from": {"name": pair[0].name, "variant": pair[0].variant},
                "to": {"name": pair[1].name, "variant": pair[1].variant},
                "step": index + 1,
                "actual_target_size": measured_size_json(size),
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({
        "steps": executed.path.len(),
        "path": steps,
        "actual_target_size": measured_size_json(sizes.last().expect("path has at least one edge")),
    })
}

fn format_concrete_path_text(graph: &ReductionGraph, executed: &ExecutedPath) -> String {
    let sizes = executed.target_sizes();
    let summary = executed
        .path
        .steps
        .iter()
        .map(|step| fmt_node(graph, &step.name, &step.variant))
        .collect::<Vec<_>>()
        .join(&format!(" {} ", crate::output::fmt_outgoing("→")));
    let mut text = format!("Path ({} steps): {summary}\n", executed.path.len());
    for (index, (pair, size)) in executed.path.steps.windows(2).zip(&sizes).enumerate() {
        text.push_str(&format!(
            "\n  {}: {} {} {}\n",
            crate::output::fmt_section(&format!("Step {}", index + 1)),
            fmt_node(graph, &pair[0].name, &pair[0].variant),
            crate::output::fmt_outgoing("→"),
            fmt_node(graph, &pair[1].name, &pair[1].variant),
        ));
        for (field, value) in &size.components {
            text.push_str(&format!("    {field} = {value}\n"));
        }
    }
    text
}

#[allow(clippy::too_many_arguments)]
fn path_concrete(
    graph: &ReductionGraph,
    src_name: &str,
    src_variant: &BTreeMap<String, String>,
    dst_name: &str,
    dst_variant: &BTreeMap<String, String>,
    limit: usize,
    unfiltered: bool,
    source: &dyn Any,
    out: &OutputConfig,
) -> Result<()> {
    let mut batch = find_path_batch(graph, src_name, src_variant, dst_name, dst_variant, limit)?;
    if batch.paths.is_empty() && !batch.truncated {
        anyhow::bail!("No reduction path from {src_name} to {dst_name}");
    }
    let mut executed = graph.execute_paths(&batch.paths, source)?;
    if !unfiltered {
        let flags = concrete_pareto_flags(&executed);
        batch.paths = retain_selected(batch.paths, &flags);
        executed = retain_selected(executed, &flags);
    }
    let json_output = out.output.is_some() || out.json;
    let json = if json_output {
        path_batch_json(graph, &batch, Some(&executed))?
    } else {
        serde_json::Value::Null
    };
    let text = if json_output {
        String::new()
    } else {
        let mut text = format!(
            "Executed {} paths from {src_name} to {dst_name}:\n",
            batch.paths.len()
        );
        for (index, path) in executed.iter().enumerate() {
            text.push_str(&format!("\n--- Path {} ---\n", index + 1));
            text.push_str(&format_concrete_path_text(graph, path));
        }
        if batch.truncated {
            text.push_str(path_truncation_note(limit));
        }
        text
    };
    out.emit_with_default_name("", &text, &json)
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

fn push_alias_part(parts: &mut Vec<String>, alias: &str) {
    if !parts.iter().any(|part| part.eq_ignore_ascii_case(alias)) {
        parts.push(alias.to_string());
    }
}

fn parse_direction(s: &str) -> Result<TraversalFlow> {
    match s {
        "out" => Ok(TraversalFlow::Outgoing),
        "in" => Ok(TraversalFlow::Incoming),
        "both" => Ok(TraversalFlow::Both),
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
        TraversalFlow::Outgoing => "outgoing",
        TraversalFlow::Incoming => "incoming",
        TraversalFlow::Both => "both directions",
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

#[cfg(test)]
mod tests {
    use super::{pareto_flags_by, push_alias_part};
    use problemreductions::size::problem_size_dominates;
    use problemreductions::ProblemSize;
    use std::cell::Cell;

    #[test]
    fn push_alias_part_deduplicates_case_insensitively_in_order() {
        let mut parts = Vec::new();
        push_alias_part(&mut parts, "KSAT");
        push_alias_part(&mut parts, "3SAT");
        push_alias_part(&mut parts, "ksat");
        push_alias_part(&mut parts, "2SAT");
        push_alias_part(&mut parts, "3sat");

        assert_eq!(parts, vec!["KSAT", "3SAT", "2SAT"]);
    }

    #[test]
    fn nondominated_flags_keep_tradeoffs_and_remove_larger_vectors() {
        let values = vec![
            Some(ProblemSize::new(vec![("x", 2), ("y", 2)])),
            Some(ProblemSize::new(vec![("x", 2), ("y", 3)])),
            Some(ProblemSize::new(vec![("x", 1), ("y", 4)])),
        ];

        let calls = Cell::new(0);
        let flags = pareto_flags_by(
            &values,
            |size| {
                calls.set(calls.get() + 1);
                size.clone()
            },
            problem_size_dominates,
        );

        assert_eq!(flags, vec![true, false, true]);
        assert_eq!(calls.get(), values.len());
    }
}
