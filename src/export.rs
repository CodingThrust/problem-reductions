//! JSON export schema for example payloads.

use crate::expr::Expr;
use crate::rules::registry::ReductionOverhead;
use crate::rules::ReductionGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const EXAMPLES_DIR_ENV: &str = "PROBLEMREDUCTIONS_EXAMPLES_DIR";

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

/// One output field mapped to an expression.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverheadEntry {
    pub field: String,
    #[serde(skip_deserializing, default = "default_expr")]
    pub expr: Expr,
    pub formula: String,
}

fn default_expr() -> Expr {
    Expr::Const(0.0)
}

/// Legacy top-level reduction structure kept for migration compatibility.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReductionData {
    pub source: ProblemSide,
    pub target: ProblemSide,
    pub overhead: Vec<OverheadEntry>,
}

/// One source↔target solution pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolutionPair {
    pub source_config: Vec<usize>,
    pub target_config: Vec<usize>,
}

/// Legacy runtime results kept for migration compatibility.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResultData {
    pub solutions: Vec<SolutionPair>,
}

/// A complete rule example: reduction + solutions in one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RuleExample {
    pub source: ProblemSide,
    pub target: ProblemSide,
    pub overhead: Vec<OverheadEntry>,
    pub solutions: Vec<SolutionPair>,
}

/// A complete model example: instance + evaluations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelExample {
    pub problem: String,
    pub variant: BTreeMap<String, String>,
    pub instance: serde_json::Value,
    pub samples: Vec<SampleEval>,
    pub optimal: Vec<SampleEval>,
}

impl ModelExample {
    /// Extract the structural identity of this model example.
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
    pub version: u32,
    pub rules: Vec<RuleExample>,
}

/// Canonical exported database of model examples.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelDb {
    pub version: u32,
    pub models: Vec<ModelExample>,
}

pub const EXAMPLE_DB_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SampleEval {
    pub config: Vec<usize>,
    pub metric: serde_json::Value,
}

/// Convert a `ReductionOverhead` to JSON-serializable entries.
pub fn overhead_to_json(overhead: &ReductionOverhead) -> Vec<OverheadEntry> {
    overhead
        .output_size
        .iter()
        .map(|(field, expr)| OverheadEntry {
            field: field.to_string(),
            formula: expr.to_string(),
            expr: expr.clone(),
        })
        .collect()
}

/// Look up `ReductionOverhead` for a direct reduction using `ReductionGraph::find_best_entry`.
pub fn lookup_overhead(
    source_name: &str,
    source_variant: &BTreeMap<String, String>,
    target_name: &str,
    _target_variant: &BTreeMap<String, String>,
) -> Option<ReductionOverhead> {
    let graph = ReductionGraph::new();
    let matched = graph.find_best_entry(source_name, target_name, source_variant)?;
    Some(matched.overhead)
}

/// Convert `Problem::variant()` output to a stable `BTreeMap`.
pub fn variant_to_map(variant: Vec<(&str, &str)>) -> BTreeMap<String, String> {
    variant
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Default output directory for generated example JSON.
pub fn examples_output_dir() -> PathBuf {
    if let Some(dir) = env::var_os(EXAMPLES_DIR_ENV) {
        PathBuf::from(dir)
    } else {
        PathBuf::from("docs/paper/examples/generated")
    }
}

fn write_json_file<T: Serialize>(dir: &Path, name: &str, payload: &T) {
    fs::create_dir_all(dir).expect("Failed to create examples directory");
    let path = dir.join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(payload).expect("Failed to serialize example");
    fs::write(&path, json).expect("Failed to write example JSON");
    println!("Exported: {}", path.display());
}

/// Write a merged rule example JSON file.
pub fn write_rule_example_to(dir: &Path, name: &str, example: &RuleExample) {
    write_json_file(dir, name, example);
}

/// Write a merged rule example JSON file to the configured output directory.
pub fn write_rule_example(name: &str, example: &RuleExample) {
    write_rule_example_to(&examples_output_dir(), name, example);
}

/// Write a model example JSON file to a target directory.
pub fn write_model_example_to(dir: &Path, name: &str, example: &ModelExample) {
    write_json_file(dir, name, example);
}

/// Write a model example JSON file to the configured output directory.
pub fn write_model_example(name: &str, example: &ModelExample) {
    write_model_example_to(&examples_output_dir(), name, example);
}

/// Write the canonical rule database to `rules.json`.
pub fn write_rule_db_to(dir: &Path, db: &RuleDb) {
    write_json_file(dir, "rules", db);
}

/// Write the canonical model database to `models.json`.
pub fn write_model_db_to(dir: &Path, db: &ModelDb) {
    write_json_file(dir, "models", db);
}

/// Compatibility helper used by the legacy reduction example files.
pub fn write_example(name: &str, reduction: &ReductionData, results: &ResultData) {
    let example = RuleExample {
        source: reduction.source.clone(),
        target: reduction.target.clone(),
        overhead: reduction.overhead.clone(),
        solutions: results.solutions.clone(),
    };
    write_rule_example(name, &example);
}

#[cfg(test)]
#[path = "unit_tests/export.rs"]
mod tests;
