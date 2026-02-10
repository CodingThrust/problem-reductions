//! JSON export schema for reduction examples.
//!
//! Provides a unified serialization format for all reduction example programs.
//! Each example produces two files:
//! - `<name>.json` — reduction structure (source, target, overhead)
//! - `<name>.result.json` — runtime solutions
//!
//! The schema mirrors the internal types: `ReductionOverhead` for polynomials,
//! `Problem::variant()` for problem variants, and `Problem::NAME` for problem names.

use crate::rules::registry::{ReductionEntry, ReductionOverhead};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// One side (source or target) of a reduction.
#[derive(Serialize, Clone, Debug)]
pub struct ProblemSide {
    /// Problem name matching `Problem::NAME` (e.g., `"IndependentSet"`).
    pub problem: String,
    /// Variant attributes (e.g., `{"graph": "SimpleGraph", "weight": "Unweighted"}`).
    pub variant: HashMap<String, String>,
    /// Problem-specific instance data (edges, matrix, clauses, etc.).
    pub instance: serde_json::Value,
}

/// A monomial in JSON: coefficient × Π(variable^exponent).
#[derive(Serialize, Clone, Debug)]
pub struct MonomialJson {
    pub coefficient: f64,
    pub variables: Vec<(String, u8)>,
}

/// One output field mapped to a polynomial.
#[derive(Serialize, Clone, Debug)]
pub struct OverheadEntry {
    pub field: String,
    pub polynomial: Vec<MonomialJson>,
}

/// Top-level reduction structure (written to `<name>.json`).
#[derive(Serialize, Clone, Debug)]
pub struct ReductionData {
    pub source: ProblemSide,
    pub target: ProblemSide,
    pub overhead: Vec<OverheadEntry>,
}

/// One source↔target solution pair.
#[derive(Serialize, Clone, Debug)]
pub struct SolutionPair {
    pub source_config: Vec<usize>,
    pub target_config: Vec<usize>,
}

/// Runtime results (written to `<name>.result.json`).
#[derive(Serialize, Clone, Debug)]
pub struct ResultData {
    pub solutions: Vec<SolutionPair>,
}

/// Convert a `ReductionOverhead` to JSON-serializable entries.
pub fn overhead_to_json(overhead: &ReductionOverhead) -> Vec<OverheadEntry> {
    overhead
        .output_size
        .iter()
        .map(|(field, poly)| OverheadEntry {
            field: field.to_string(),
            polynomial: poly
                .terms
                .iter()
                .map(|m| MonomialJson {
                    coefficient: m.coefficient,
                    variables: m
                        .variables
                        .iter()
                        .map(|(name, exp)| (name.to_string(), *exp))
                        .collect(),
                })
                .collect(),
        })
        .collect()
}

/// Look up `ReductionOverhead` from inventory by source and target problem names.
///
/// Searches all registered `ReductionEntry` items for a matching source/target pair.
/// Returns `None` if no matching reduction is registered (e.g., ILP reductions
/// that don't use the `#[reduction]` macro).
pub fn lookup_overhead(source_name: &str, target_name: &str) -> Option<ReductionOverhead> {
    for entry in inventory::iter::<ReductionEntry> {
        if entry.source_name == source_name && entry.target_name == target_name {
            return Some(entry.overhead());
        }
    }
    None
}

/// Look up overhead, returning an empty overhead if not registered.
pub fn lookup_overhead_or_empty(source_name: &str, target_name: &str) -> ReductionOverhead {
    lookup_overhead(source_name, target_name).unwrap_or_default()
}

/// Convert `Problem::variant()` output to a `HashMap`.
pub fn variant_to_map(variant: Vec<(&str, &str)>) -> HashMap<String, String> {
    variant
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Write both `<name>.json` and `<name>.result.json` to `docs/paper/examples/`.
pub fn write_example(name: &str, reduction: &ReductionData, results: &ResultData) {
    let dir = Path::new("docs/paper/examples");
    fs::create_dir_all(dir).expect("Failed to create examples directory");

    let reduction_path = dir.join(format!("{}.json", name));
    let json = serde_json::to_string_pretty(reduction).expect("Failed to serialize reduction");
    fs::write(&reduction_path, json).expect("Failed to write reduction JSON");
    println!("Exported: {}", reduction_path.display());

    let results_path = dir.join(format!("{}.result.json", name));
    let json = serde_json::to_string_pretty(results).expect("Failed to serialize results");
    fs::write(&results_path, json).expect("Failed to write results JSON");
    println!("Exported: {}", results_path.display());
}
