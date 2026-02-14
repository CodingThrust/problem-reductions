//! Automatic reduction registration via inventory.

use crate::expr::{EvalError, Expr, Func};
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use std::any::Any;
use std::collections::HashSet;

/// Overhead specification for a reduction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReductionOverhead {
    /// Output size as symbolic expressions of input size variables.
    /// Each entry is (output_field_name, expression).
    pub output_size: Vec<(&'static str, Expr)>,
}

impl ReductionOverhead {
    pub fn new(specs: Vec<(&'static str, &'static str)>) -> Self {
        Self {
            output_size: specs
                .into_iter()
                .map(|(field, expr_str)| {
                    let expr = Expr::parse(expr_str).unwrap_or_else(|e| {
                        panic!("invalid overhead expression for '{field}': {e}")
                    });
                    (field, expr)
                })
                .collect(),
        }
    }

    /// Identity overhead: each output field equals the same-named input field.
    /// Used by variant cast reductions where problem size doesn't change.
    pub fn identity(fields: &[&'static str]) -> Self {
        Self {
            output_size: fields
                .iter()
                .map(|&f| {
                    let expr = Expr::parse(f)
                        .unwrap_or_else(|e| panic!("invalid identity field name '{f}': {e}"));
                    (f, expr)
                })
                .collect(),
        }
    }

    /// Evaluate output size given input size.
    ///
    /// Uses `round()` for the f64 to usize conversion because expression coefficients
    /// are typically integers (1, 2, 3, 7, 21, etc.) and any fractional results come
    /// from floating-point arithmetic imprecision, not intentional fractions.
    pub fn evaluate_output_size(&self, input: &ProblemSize) -> Result<ProblemSize, EvalError> {
        let mut fields = Vec::new();
        for (name, expr) in &self.output_size {
            let val = expr.evaluate(input)?;
            let rounded = val.round();
            if !rounded.is_finite() || rounded < 0.0 || rounded > usize::MAX as f64 {
                return Err(EvalError::Domain {
                    func: Func::Floor,
                    detail: format!("overhead for '{name}' produced out-of-range value: {val}")
                        .into(),
                });
            }
            fields.push((*name, rounded as usize));
        }
        Ok(ProblemSize::new(fields))
    }

    /// Collect all input variable names referenced by the overhead expressions.
    pub fn input_variable_names(&self) -> HashSet<&'static str> {
        self.output_size
            .iter()
            .flat_map(|(_, expr)| expr.variable_names())
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
    /// Type-level problem size field names for the source problem.
    pub source_size_names_fn: fn() -> &'static [&'static str],
    /// Type-level problem size field names for the target problem.
    pub target_size_names_fn: fn() -> &'static [&'static str],
    /// Type-erased reduction executor.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls `ReduceTo::reduce_to()`,
    /// and returns the result as a boxed `DynReductionResult`.
    pub reduce_fn: fn(&dyn Any) -> Box<dyn DynReductionResult>,
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

#[cfg(test)]
#[path = "../unit_tests/rules/registry.rs"]
mod tests;
