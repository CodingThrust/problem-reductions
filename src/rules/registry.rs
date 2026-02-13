//! Automatic reduction registration via inventory.

use crate::polynomial::Polynomial;
use crate::types::ProblemSize;

/// Overhead specification for a reduction.
#[derive(Clone, Debug, Default)]
pub struct ReductionOverhead {
    /// Output size as polynomials of input size variables.
    /// Each entry is (output_field_name, polynomial).
    pub output_size: Vec<(&'static str, Polynomial)>,
}

impl ReductionOverhead {
    pub fn new(output_size: Vec<(&'static str, Polynomial)>) -> Self {
        Self { output_size }
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
/// Uses function pointer to lazily create the overhead (avoids static allocation issues).
pub struct ReductionEntry {
    /// Base name of source problem (e.g., "MaximumIndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "MinimumVertexCover").
    pub target_name: &'static str,
    /// Variant attributes for source problem as key-value pairs.
    /// Common keys: "graph" (graph type), "weight" (weight type).
    pub source_variant: &'static [(&'static str, &'static str)],
    /// Variant attributes for target problem as key-value pairs.
    pub target_variant: &'static [(&'static str, &'static str)],
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

    /// Check if this reduction involves only the base (unweighted) variants.
    pub fn is_base_reduction(&self) -> bool {
        let source_unweighted = self
            .source_variant
            .iter()
            .find(|(k, _)| *k == "weight")
            .map(|(_, v)| *v == "Unweighted")
            .unwrap_or(true);
        let target_unweighted = self
            .target_variant
            .iter()
            .find(|(k, _)| *k == "weight")
            .map(|(_, v)| *v == "Unweighted")
            .unwrap_or(true);
        source_unweighted && target_unweighted
    }
}

impl std::fmt::Debug for ReductionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReductionEntry")
            .field("source_name", &self.source_name)
            .field("target_name", &self.target_name)
            .field("source_variant", &self.source_variant)
            .field("target_variant", &self.target_variant)
            .field("overhead", &self.overhead())
            .field("module_path", &self.module_path)
            .finish()
    }
}

inventory::collect!(ReductionEntry);

/// A registered concrete problem variant (for JSON export nodes).
/// Variants registered here appear as nodes even without explicit reduction rules.
pub struct ConcreteVariantEntry {
    pub name: &'static str,
    pub variant: &'static [(&'static str, &'static str)],
}

inventory::collect!(ConcreteVariantEntry);

#[cfg(test)]
#[path = "../unit_tests/rules/registry.rs"]
mod tests;
