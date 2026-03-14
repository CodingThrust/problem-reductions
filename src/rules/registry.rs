//! Automatic reduction registration via inventory.

use crate::expr::Expr;
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use std::any::Any;
use std::collections::HashSet;

/// Overhead specification for a reduction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReductionOverhead {
    /// Output size as expressions of input size variables.
    /// Each entry is (output_field_name, expression).
    pub output_size: Vec<(&'static str, Expr)>,
}

impl ReductionOverhead {
    pub fn new(output_size: Vec<(&'static str, Expr)>) -> Self {
        Self { output_size }
    }

    /// Identity overhead: each output field equals the same-named input field.
    /// Used by variant cast reductions where problem size doesn't change.
    pub fn identity(fields: &[&'static str]) -> Self {
        Self {
            output_size: fields.iter().map(|&f| (f, Expr::Var(f))).collect(),
        }
    }

    /// Evaluate output size given input size.
    ///
    /// Uses `round()` for the f64 to usize conversion because expression values
    /// are typically integers and any fractional results come from floating-point
    /// arithmetic imprecision, not intentional fractions.
    pub fn evaluate_output_size(&self, input: &ProblemSize) -> ProblemSize {
        let fields: Vec<_> = self
            .output_size
            .iter()
            .map(|(name, expr)| (*name, expr.eval(input).round() as usize))
            .collect();
        ProblemSize::new(fields)
    }

    /// Collect all input variable names referenced by the overhead expressions.
    pub fn input_variable_names(&self) -> HashSet<&'static str> {
        self.output_size
            .iter()
            .flat_map(|(_, expr)| expr.variables())
            .collect()
    }

    /// Compose two overheads: substitute self's output into `next`'s input.
    ///
    /// Returns a new overhead whose expressions map from self's input variables
    /// directly to `next`'s output variables.
    pub fn compose(&self, next: &ReductionOverhead) -> ReductionOverhead {
        use std::collections::HashMap;

        // Build substitution map: output field name → output expression
        let mapping: HashMap<&str, &Expr> = self
            .output_size
            .iter()
            .map(|(name, expr)| (*name, expr))
            .collect();

        let composed = next
            .output_size
            .iter()
            .map(|(name, expr)| (*name, expr.substitute(&mapping)))
            .collect();

        ReductionOverhead {
            output_size: composed,
        }
    }

    /// Get the expression for a named output field.
    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.output_size
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, e)| e)
    }
}

/// A registered reduction entry for static inventory registration.
/// Uses function pointers to lazily derive variant fields from `Problem::variant()`.
pub struct ReductionEntry {
    /// Stable, unique rule identifier (e.g., `"mvc_to_mis_simplegraph_i32"`).
    pub rule_id: &'static str,
    /// Base name of source problem (e.g., "MaximumIndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "MinimumVertexCover").
    pub target_name: &'static str,
    /// Function to derive source variant attributes from `Problem::variant()`.
    pub source_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Function to derive target variant attributes from `Problem::variant()`.
    pub target_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Function to create overhead information (lazy evaluation for static context).
    pub overhead_fn: fn() -> ReductionOverhead,
    /// Module path where the reduction is defined (from `module_path!()`).
    pub module_path: &'static str,
    /// Type-erased reduction executor.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls `ReduceTo::reduce_to()`,
    /// and returns the result as a boxed `DynReductionResult`.
    pub reduce_fn: fn(&dyn Any) -> Box<dyn DynReductionResult>,
    /// Compiled overhead evaluation function.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls getter methods directly,
    /// and returns the computed target problem size.
    pub overhead_eval_fn: fn(&dyn Any) -> ProblemSize,
}

impl ReductionEntry {
    /// Get the overhead by calling the function.
    pub fn overhead(&self) -> ReductionOverhead {
        (self.overhead_fn)()
    }

    /// Get the source variant by calling the function.
    pub fn source_variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.source_variant_fn)()
    }

    /// Get the target variant by calling the function.
    pub fn target_variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.target_variant_fn)()
    }

    /// Check if this reduction involves only the base (unweighted) variants.
    pub fn is_base_reduction(&self) -> bool {
        let source = self.source_variant();
        let target = self.target_variant();
        let source_unweighted = source
            .iter()
            .find(|(k, _)| *k == "weight")
            .map(|(_, v)| *v == "One")
            .unwrap_or(true);
        let target_unweighted = target
            .iter()
            .find(|(k, _)| *k == "weight")
            .map(|(_, v)| *v == "One")
            .unwrap_or(true);
        source_unweighted && target_unweighted
    }
}

impl std::fmt::Debug for ReductionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReductionEntry")
            .field("source_name", &self.source_name)
            .field("target_name", &self.target_name)
            .field("source_variant", &self.source_variant())
            .field("target_variant", &self.target_variant())
            .field("overhead", &self.overhead())
            .field("module_path", &self.module_path)
            .finish()
    }
}

inventory::collect!(ReductionEntry);

/// Find a reduction entry by its stable rule ID.
pub fn find_reduction_entry_by_rule_id(id: &str) -> Option<&'static ReductionEntry> {
    inventory::iter::<ReductionEntry>()
        .find(|entry| entry.rule_id == id)
}

/// Return all registered reduction entries.
pub fn reduction_entries() -> Vec<&'static ReductionEntry> {
    inventory::iter::<ReductionEntry>().collect()
}

#[cfg(test)]
#[path = "../unit_tests/rules/registry.rs"]
mod tests;
