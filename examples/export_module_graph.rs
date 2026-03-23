//! Export module architecture graph for mdBook interactive visualization.
//!
//! Generates `docs/src/static/module-graph.json` from inventory-registered
//! problem schemas and reduction entries — no nightly rustdoc required.
//!
//! Run with: `cargo run --example export_module_graph [output_path]`

use problemreductions::registry::ProblemSchemaEntry;
use problemreductions::rules::ReductionEntry;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// Category for color-coding in the visualization.
fn module_category(module_path: &str) -> &'static str {
    if module_path.contains("models::graph") {
        "model_graph"
    } else if module_path.contains("models::formula") {
        "model_formula"
    } else if module_path.contains("models::set") {
        "model_set"
    } else if module_path.contains("models::algebraic") {
        "model_algebraic"
    } else if module_path.contains("models::misc") {
        "model_misc"
    } else if module_path.contains("rules") {
        "rule"
    } else if module_path.contains("registry") {
        "registry"
    } else if module_path.contains("solvers") {
        "solver"
    } else {
        "core"
    }
}

/// Convert a full module path like "problemreductions::models::graph::foo" to a
/// short display path like "models/graph".
fn module_display_path(module_path: &str) -> String {
    let stripped = module_path
        .strip_prefix("problemreductions::")
        .unwrap_or(module_path);
    // Take up to the second-level module (e.g., "models::graph::foo" -> "models/graph")
    let parts: Vec<&str> = stripped.split("::").collect();
    match parts.len() {
        0 => stripped.replace("::", "/"),
        1 => parts[0].to_string(),
        _ => {
            // For "models::graph::something", keep "models/graph"
            // For "rules::something", keep "rules"
            if parts[0] == "models" && parts.len() >= 2 {
                format!("{}/{}", parts[0], parts[1])
            } else {
                parts[0].to_string()
            }
        }
    }
}

#[derive(Serialize)]
struct ModuleNode {
    name: String,
    category: String,
    doc_path: String,
    items: Vec<ModuleItem>,
}

#[derive(Serialize, Clone)]
struct ModuleItem {
    name: String,
    kind: String,
    doc: String,
}

#[derive(Serialize)]
struct Edge {
    source: String,
    target: String,
}

#[derive(Serialize)]
struct ModuleGraph {
    modules: Vec<ModuleNode>,
    edges: Vec<Edge>,
}

fn main() {
    // Group problems by module display path
    let mut module_items: BTreeMap<String, Vec<ModuleItem>> = BTreeMap::new();
    let mut module_categories: BTreeMap<String, String> = BTreeMap::new();

    for entry in inventory::iter::<ProblemSchemaEntry> {
        let display = module_display_path(entry.module_path);
        let category = module_category(entry.module_path).to_string();
        module_categories.entry(display.clone()).or_insert(category);
        module_items.entry(display).or_default().push(ModuleItem {
            name: entry.display_name.to_string(),
            kind: "struct".to_string(),
            doc: entry.description.to_string(),
        });
    }

    // Add well-known non-model modules with their key items
    type ModuleSpec = (
        &'static str,
        &'static str,
        &'static [(&'static str, &'static str, &'static str)],
    );
    let static_modules: &[ModuleSpec] = &[
        (
            "traits",
            "core",
            &[(
                "Problem",
                "trait",
                "Core trait for all computational problems",
            )],
        ),
        (
            "types",
            "core",
            &[
                ("Aggregate", "trait", "Trait for aggregate value types"),
                ("Max", "struct", "Maximum aggregate wrapper"),
                ("Min", "struct", "Minimum aggregate wrapper"),
                ("Sum", "struct", "Summation aggregate wrapper"),
                ("Or", "struct", "Existential (logical or) aggregate"),
                ("And", "struct", "Universal (logical and) aggregate"),
                ("Extremum", "struct", "Runtime max/min aggregate"),
                ("One", "struct", "Unit weight marker type"),
                ("WeightElement", "trait", "Trait for weight types"),
            ],
        ),
        (
            "variant",
            "core",
            &[
                ("VariantParam", "trait", "Trait for variant parameter types"),
                (
                    "CastToParent",
                    "trait",
                    "Trait for variant cast conversions",
                ),
            ],
        ),
        (
            "topology",
            "core",
            &[
                ("SimpleGraph", "struct", "Simple undirected graph"),
                ("PlanarGraph", "struct", "Planar graph"),
                ("BipartiteGraph", "struct", "Bipartite graph"),
                ("UnitDiskGraph", "struct", "Unit disk graph"),
                ("KingsSubgraph", "struct", "Kings subgraph"),
            ],
        ),
        (
            "rules",
            "rule",
            &[
                ("ReduceTo", "trait", "Trait for witness/config reductions"),
                (
                    "ReductionResult",
                    "trait",
                    "Result of a reduction with solution extraction",
                ),
                (
                    "ReduceToAggregate",
                    "trait",
                    "Trait for aggregate/value reductions",
                ),
                (
                    "AggregateReductionResult",
                    "trait",
                    "Result of a reduction with value extraction",
                ),
            ],
        ),
        (
            "registry",
            "registry",
            &[
                (
                    "ReductionGraph",
                    "struct",
                    "Global graph of all registered reductions",
                ),
                ("ReductionEntry", "struct", "A single registered reduction"),
                ("VariantEntry", "struct", "A registered problem variant"),
            ],
        ),
        (
            "solvers",
            "solver",
            &[
                ("BruteForce", "struct", "Exhaustive search solver"),
                ("ILPSolver", "struct", "Integer linear programming solver"),
                (
                    "Solver",
                    "trait",
                    "Solver trait for aggregate value computation",
                ),
            ],
        ),
        (
            "io",
            "utility",
            &[
                ("to_json", "function", "Serialize a problem to JSON"),
                ("from_json", "function", "Deserialize a problem from JSON"),
            ],
        ),
    ];

    for &(name, category, items) in static_modules {
        module_categories
            .entry(name.to_string())
            .or_insert_with(|| category.to_string());
        let entry = module_items.entry(name.to_string()).or_default();
        // Only add if not already populated from inventory
        if entry.is_empty() {
            for &(item_name, kind, doc) in items {
                entry.push(ModuleItem {
                    name: item_name.to_string(),
                    kind: kind.to_string(),
                    doc: doc.to_string(),
                });
            }
        }
    }

    // Build edges from reduction entries (which module uses which)
    let mut edges: BTreeSet<(String, String)> = BTreeSet::new();

    // Each reduction connects source module -> target module
    for entry in inventory::iter::<ReductionEntry> {
        let src_module = find_problem_module(entry.source_name, &module_items);
        let dst_module = find_problem_module(entry.target_name, &module_items);
        if let (Some(src), Some(dst)) = (src_module, dst_module) {
            if src != dst {
                // rule module depends on both model modules
                edges.insert(("rules".to_string(), src.clone()));
                edges.insert(("rules".to_string(), dst));
            }
        }
    }

    // Add well-known architectural edges
    let known_edges: &[(&str, &str)] = &[
        ("models/graph", "traits"),
        ("models/graph", "types"),
        ("models/graph", "topology"),
        ("models/graph", "variant"),
        ("models/formula", "traits"),
        ("models/formula", "types"),
        ("models/set", "traits"),
        ("models/set", "types"),
        ("models/algebraic", "traits"),
        ("models/algebraic", "types"),
        ("models/misc", "traits"),
        ("models/misc", "types"),
        ("rules", "traits"),
        ("rules", "types"),
        ("registry", "rules"),
        ("registry", "traits"),
        ("registry", "types"),
        ("solvers", "traits"),
        ("solvers", "types"),
        ("io", "traits"),
        ("topology", "variant"),
    ];

    for &(src, dst) in known_edges {
        if module_categories.contains_key(src) && module_categories.contains_key(dst) {
            edges.insert((src.to_string(), dst.to_string()));
        }
    }

    // Assemble output
    let modules: Vec<ModuleNode> = module_categories
        .iter()
        .map(|(name, category)| {
            let items = module_items.get(name).cloned().unwrap_or_default();
            ModuleNode {
                name: name.clone(),
                category: category.clone(),
                doc_path: format!("{name}/index.html"),
                items,
            }
        })
        .collect();

    let edge_list: Vec<Edge> = edges
        .into_iter()
        .map(|(source, target)| Edge { source, target })
        .collect();

    let graph = ModuleGraph {
        modules,
        edges: edge_list,
    };

    let output_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("docs/src/static/module-graph.json"));

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    let json = serde_json::to_string(&graph).expect("Failed to serialize");
    std::fs::write(&output_path, &json).expect("Failed to write file");

    println!(
        "Wrote {} ({} modules, {} edges)",
        output_path.display(),
        graph.modules.len(),
        graph.edges.len()
    );
}

/// Find which module display path a problem belongs to.
fn find_problem_module(
    problem_name: &str,
    module_items: &BTreeMap<String, Vec<ModuleItem>>,
) -> Option<String> {
    for (module, items) in module_items {
        for item in items {
            // Match against display name or the struct name
            if item.name.replace(' ', "") == problem_name {
                return Some(module.clone());
            }
        }
    }
    // Fallback: search inventory directly
    for entry in inventory::iter::<ProblemSchemaEntry> {
        if entry.name == problem_name {
            return Some(module_display_path(entry.module_path));
        }
    }
    None
}
