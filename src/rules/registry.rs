//! Automatic reduction registration via inventory.

use crate::polynomial::Polynomial;
use crate::types::ProblemSize;

/// Overhead specification for a reduction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReductionOverhead {
    /// Output size as polynomials of input size variables.
    /// Each entry is (output_field_name, polynomial).
    pub output_size: Vec<(&'static str, Polynomial)>,
}

impl ReductionOverhead {
    pub fn new(output_size: Vec<(&'static str, Polynomial)>) -> Self {
        Self { output_size }
    }

    /// Identity overhead: each output field equals the same-named input field.
    /// Used by variant cast reductions where problem size doesn't change.
    pub fn identity(fields: &[&'static str]) -> Self {
        Self {
            output_size: fields.iter().map(|&f| (f, Polynomial::var(f))).collect(),
        }
    }

    /// Evaluate output size given input size.
    ///
    /// Uses `round()` for the f64 to usize conversion because polynomial coefficients
    /// are typically integers (1, 2, 3, 7, 21, etc.) and any fractional results come
    /// from floating-point arithmetic imprecision, not intentional fractions.
    /// For problem sizes, rounding to nearest integer is the most intuitive behavior.
    pub fn evaluate_output_size(&self, input: &ProblemSize) -> ProblemSize {
        let fields: Vec<_> = self
            .output_size
            .iter()
            .map(|(name, poly)| (*name, poly.evaluate(input).round() as usize))
            .collect();
        ProblemSize::new(fields)
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
