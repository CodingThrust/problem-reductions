//! Explicit variant registration via inventory.

use std::any::Any;
use std::collections::BTreeMap;

use crate::registry::dyn_problem::{DynProblem, SolveValueFn, SolveWitnessFn};

/// A registered problem variant entry.
///
/// Submitted by [`declare_variants!`] for each concrete problem type.
/// The reduction graph uses these entries to build nodes with complexity metadata.
pub struct VariantEntry {
    /// Problem name (from `Problem::NAME`).
    pub name: &'static str,
    /// Function returning variant key-value pairs (from `Problem::variant()`).
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Worst-case time complexity expression (e.g., `"2^num_vertices"`).
    pub complexity: &'static str,
    /// Compiled complexity evaluation function.
    /// Takes a `&dyn Any` (must be `&ProblemType`), calls getter methods directly,
    /// and returns the estimated worst-case time as f64.
    pub complexity_eval_fn: fn(&dyn Any) -> f64,
    /// Whether this entry is the declared default variant for its problem.
    pub is_default: bool,
    /// Variant-level aliases (e.g., `&["3SAT"]` for `KSatisfiability<K3>`).
    ///
    /// Unlike problem-level aliases (on `ProblemSchemaEntry`), these resolve to a
    /// specific reduction-graph node, not just to a canonical problem name. The CLI
    /// resolver tries variant-level aliases first and falls back to problem-level.
    pub aliases: &'static [&'static str],
    /// Factory: deserialize JSON into a boxed dynamic problem.
    pub factory: fn(serde_json::Value) -> Result<Box<dyn DynProblem>, serde_json::Error>,
    /// Serialize: downcast `&dyn Any` and serialize to JSON.
    pub serialize_fn: fn(&dyn Any) -> Option<serde_json::Value>,
    /// Solve value: downcast `&dyn Any` and brute-force solve to an aggregate string.
    pub solve_value_fn: SolveValueFn,
    /// Solve witness: downcast `&dyn Any` and brute-force recover a witness when available.
    pub solve_witness_fn: SolveWitnessFn,
}

impl VariantEntry {
    /// Get the variant by calling the function.
    pub fn variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.variant_fn)()
    }

    /// Get the variant as a `BTreeMap<String, String>`.
    pub fn variant_map(&self) -> BTreeMap<String, String> {
        self.variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}

/// Find a variant entry by exact problem name and exact variant map.
///
/// No alias resolution or default fallback. Both `name` and `variant` must match exactly.
pub fn find_variant_entry(
    name: &str,
    variant: &BTreeMap<String, String>,
) -> Option<&'static VariantEntry> {
    inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == name && entry.variant_map() == *variant)
}

/// Find a variant entry by a variant-level alias (case-insensitive).
///
/// A variant-level alias points at a specific reduction-graph node (e.g., `"3SAT"` →
/// `KSatisfiability` with variant `{k: "K3"}`), unlike problem-level aliases which
/// resolve only to a canonical problem name.
///
/// Returns the matched entry along with its variant map. The first match in registration
/// order wins — duplicate variant-level aliases across problems are a declaration bug.
pub fn find_variant_by_alias(
    input: &str,
) -> Option<(&'static VariantEntry, BTreeMap<String, String>)> {
    let lower = input.to_lowercase();
    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.aliases.iter().any(|a| a.to_lowercase() == lower))?;
    Some((entry, entry.variant_map()))
}

/// Validate all variant-level aliases registered in inventory.
///
/// This is intended for explicit test-time or startup invocation. It rejects
/// duplicate variant-level aliases, aliases that collide with canonical
/// problem names or problem-level aliases, and empty aliases for manually
/// constructed [`VariantEntry`] values that bypass `declare_variants!`.
pub fn validate_variant_aliases() -> Result<(), Vec<String>> {
    let mut problem_names: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for problem in super::problem_type::problem_types() {
        problem_names
            .entry(problem.canonical_name.to_lowercase())
            .or_default()
            .push(format!(
                "canonical problem name `{}`",
                problem.canonical_name
            ));

        for alias in problem.aliases {
            problem_names
                .entry(alias.to_lowercase())
                .or_default()
                .push(format!(
                    "problem-level alias `{alias}` for `{}`",
                    problem.canonical_name
                ));
        }
    }

    let entries: Vec<_> = inventory::iter::<VariantEntry>()
        .map(|e| (variant_label(e), e.aliases))
        .collect();

    validate_aliases_inner(&problem_names, &entries)
}

/// Core validation logic, separated for testability with mock data.
///
/// - `problem_names`: lowercase key → list of human-readable sources (canonical names + problem-level aliases).
/// - `entries`: `(variant_label, aliases_slice)` per variant entry.
pub fn validate_aliases_inner(
    problem_names: &BTreeMap<String, Vec<String>>,
    entries: &[(String, &[&str])],
) -> Result<(), Vec<String>> {
    let mut conflicts = Vec::new();
    let mut variant_aliases: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    for (target, aliases) in entries {
        for alias in *aliases {
            if alias.trim().is_empty() {
                conflicts.push(format!(
                    "variant-level alias on {target} is empty or whitespace-only"
                ));
                continue;
            }

            let lower = alias.to_lowercase();
            if let Some(collisions) = problem_names.get(&lower) {
                for collision in collisions {
                    conflicts.push(format!(
                        "variant-level alias `{alias}` on {target} conflicts with {collision}"
                    ));
                }
            }

            variant_aliases
                .entry(lower)
                .or_default()
                .push((alias.to_string(), target.clone()));
        }
    }

    for (lower, registrations) in variant_aliases {
        if registrations.len() > 1 {
            let details = registrations
                .iter()
                .map(|(alias, target)| format!("`{alias}` on {target}"))
                .collect::<Vec<_>>()
                .join("; ");
            conflicts.push(format!(
                "duplicate variant-level alias `{lower}` (case-insensitive): {details}"
            ));
        }
    }

    if conflicts.is_empty() {
        Ok(())
    } else {
        conflicts.sort();
        Err(conflicts)
    }
}

pub fn variant_label(entry: &VariantEntry) -> String {
    let variant = entry.variant();
    if variant.is_empty() {
        return entry.name.to_string();
    }

    let parts = variant
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{} {{{parts}}}", entry.name)
}

impl std::fmt::Debug for VariantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantEntry")
            .field("name", &self.name)
            .field("variant", &self.variant())
            .field("complexity", &self.complexity)
            .finish()
    }
}

inventory::collect!(VariantEntry);

#[cfg(test)]
#[path = "../unit_tests/registry/variant.rs"]
mod tests;
