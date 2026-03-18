//! JSON export schema for example payloads.

use crate::rules::registry::ReductionOverhead;
use crate::rules::ReductionGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// One side (source or target) of a reduction.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProblemSide {
    /// Problem name matching `Problem::NAME` (e.g., `"MaximumIndependentSet"`).
    pub problem: String,
    /// Variant attributes (e.g., `{"graph": "SimpleGraph", "weight": "One"}`).
    pub variant: BTreeMap<String, String>,
    /// Problem-specific instance data (edges, matrix, clauses, etc.).
    pub instance: serde_json::Value,
}

impl ProblemSide {
    /// Build a serializable problem side from a typed problem.
    pub fn from_problem<P>(problem: &P) -> Self
    where
        P: Problem + Serialize,
    {
        Self {
            problem: P::NAME.to_string(),
            variant: variant_to_map(P::variant()),
            instance: serde_json::to_value(problem).expect("Failed to serialize problem instance"),
        }
    }

    /// Extract the structural identity of this problem side.
    pub fn problem_ref(&self) -> ProblemRef {
        ProblemRef {
            name: self.problem.clone(),
            variant: self.variant.clone(),
        }
    }
}

/// Canonical structural identity for a problem node in the reduction graph.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProblemRef {
    pub name: String,
    pub variant: BTreeMap<String, String>,
}

/// One source↔target solution pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolutionPair {
    pub source_config: Vec<usize>,
    pub target_config: Vec<usize>,
}

/// A complete rule example: reduction + solutions in one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RuleExample {
    pub source: ProblemSide,
    pub target: ProblemSide,
    pub solutions: Vec<SolutionPair>,
}

/// A complete model example: instance + known optimal solution.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelExample {
    pub problem: String,
    pub variant: BTreeMap<String, String>,
    pub instance: serde_json::Value,
    pub optimal_config: Vec<usize>,
    pub optimal_value: serde_json::Value,
}

impl ModelExample {
    pub fn new(
        problem: &str,
        variant: BTreeMap<String, String>,
        instance: serde_json::Value,
        optimal_config: Vec<usize>,
        optimal_value: serde_json::Value,
    ) -> Self {
        Self {
            problem: problem.to_string(),
            variant,
            instance,
            optimal_config,
            optimal_value,
        }
    }

    pub fn problem_ref(&self) -> ProblemRef {
        ProblemRef {
            name: self.problem.clone(),
            variant: self.variant.clone(),
        }
    }
}

/// Canonical exported database of rule examples.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RuleDb {
    pub rules: Vec<RuleExample>,
}

/// Canonical exported database of model examples.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelDb {
    pub models: Vec<ModelExample>,
}

/// Canonical exported database of model and rule examples.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExampleDb {
    pub models: Vec<ModelExample>,
    pub rules: Vec<RuleExample>,
}

/// Look up `ReductionOverhead` for a direct reduction using `ReductionGraph::find_best_entry`.
pub fn lookup_overhead(
    source_name: &str,
    source_variant: &BTreeMap<String, String>,
    target_name: &str,
    target_variant: &BTreeMap<String, String>,
) -> Option<ReductionOverhead> {
    let graph = ReductionGraph::new();
    let matched =
        graph.find_best_entry(source_name, source_variant, target_name, target_variant)?;
    Some(matched.overhead)
}

/// Convert `Problem::variant()` output to a stable `BTreeMap`.
///
/// Normalizes empty `"graph"` values to `"SimpleGraph"` for consistency
/// with the reduction graph convention.
pub fn variant_to_map(variant: Vec<(&str, &str)>) -> BTreeMap<String, String> {
    variant
        .into_iter()
        .map(|(k, v)| {
            let value = if k == "graph" && v.is_empty() {
                "SimpleGraph".to_string()
            } else {
                v.to_string()
            };
            (k.to_string(), value)
        })
        .collect()
}

fn write_json_file<T: Serialize>(dir: &Path, name: &str, payload: &T) {
    fs::create_dir_all(dir).expect("Failed to create examples directory");
    let path = dir.join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(payload).expect("Failed to serialize example");
    fs::write(&path, json).expect("Failed to write example JSON");
    println!("Exported: {}", path.display());
}

fn render_compact_array<T: Serialize>(items: &[T]) -> String {
    if items.is_empty() {
        "[]".to_string()
    } else {
        let rows = items
            .iter()
            .map(|item| {
                format!(
                    "    {}",
                    serde_json::to_string(item).expect("Failed to serialize example entry")
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        format!("[\n{rows}\n  ]")
    }
}

fn write_example_db_file(dir: &Path, db: &ExampleDb) {
    fs::create_dir_all(dir).expect("Failed to create examples directory");
    let path = dir.join("examples.json");
    let json = format!(
        "{{\n  \"models\": {},\n  \"rules\": {}\n}}\n",
        render_compact_array(&db.models),
        render_compact_array(&db.rules)
    );
    fs::write(&path, json).expect("Failed to write example JSON");
    println!("Exported: {}", path.display());
}

/// Write a merged rule example JSON file.
pub fn write_rule_example_to(dir: &Path, name: &str, example: &RuleExample) {
    write_json_file(dir, name, example);
}

/// Write a model example JSON file to a target directory.
pub fn write_model_example_to(dir: &Path, name: &str, example: &ModelExample) {
    write_json_file(dir, name, example);
}

/// Write the canonical rule database as a wrapped JSON object.
pub fn write_rule_db_to(dir: &Path, db: &RuleDb) {
    write_json_file(dir, "rules", db);
}

/// Write the canonical model database as a wrapped JSON object.
pub fn write_model_db_to(dir: &Path, db: &ModelDb) {
    write_json_file(dir, "models", db);
}

/// Write the canonical example database as a wrapped JSON object.
pub fn write_example_db_to(dir: &Path, db: &ExampleDb) {
    write_example_db_file(dir, db);
}

#[cfg(test)]
#[path = "unit_tests/export.rs"]
mod tests;
