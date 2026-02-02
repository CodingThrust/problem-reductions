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
    /// Base name of source problem (e.g., "IndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "VertexCovering").
    pub target_name: &'static str,
    /// Variant attributes for source problem as key-value pairs.
    /// Common keys: "graph" (graph type), "weight" (weight type).
    pub source_variant: &'static [(&'static str, &'static str)],
    /// Variant attributes for target problem as key-value pairs.
    pub target_variant: &'static [(&'static str, &'static str)],
    /// Function to create overhead information (lazy evaluation for static context).
    pub overhead_fn: fn() -> ReductionOverhead,
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
            .finish()
    }
}

inventory::collect!(ReductionEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly;

    #[test]
    fn test_reduction_overhead_evaluate() {
        let overhead = ReductionOverhead::new(vec![("n", poly!(3 * m)), ("m", poly!(m ^ 2))]);

        let input = ProblemSize::new(vec![("m", 4)]);
        let output = overhead.evaluate_output_size(&input);

        assert_eq!(output.get("n"), Some(12)); // 3 * 4
        assert_eq!(output.get("m"), Some(16)); // 4^2
    }

    #[test]
    fn test_reduction_overhead_default() {
        let overhead = ReductionOverhead::default();
        assert!(overhead.output_size.is_empty());
    }

    #[test]
    fn test_reduction_entry_overhead() {
        let entry = ReductionEntry {
            source_name: "TestSource",
            target_name: "TestTarget",
            source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            overhead_fn: || ReductionOverhead::new(vec![("n", poly!(2 * n))]),
        };

        let overhead = entry.overhead();
        let input = ProblemSize::new(vec![("n", 5)]);
        let output = overhead.evaluate_output_size(&input);
        assert_eq!(output.get("n"), Some(10));
    }

    #[test]
    fn test_reduction_entry_debug() {
        let entry = ReductionEntry {
            source_name: "A",
            target_name: "B",
            source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            overhead_fn: || ReductionOverhead::default(),
        };

        let debug_str = format!("{:?}", entry);
        assert!(debug_str.contains("A"));
        assert!(debug_str.contains("B"));
    }

    #[test]
    fn test_is_base_reduction_unweighted() {
        let entry = ReductionEntry {
            source_name: "A",
            target_name: "B",
            source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            overhead_fn: || ReductionOverhead::default(),
        };
        assert!(entry.is_base_reduction());
    }

    #[test]
    fn test_is_base_reduction_weighted() {
        let entry = ReductionEntry {
            source_name: "A",
            target_name: "B",
            source_variant: &[("graph", "SimpleGraph"), ("weight", "i32")],
            target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            overhead_fn: || ReductionOverhead::default(),
        };
        assert!(!entry.is_base_reduction());
    }

    #[test]
    fn test_is_base_reduction_no_weight_key() {
        // If no weight key is present, assume unweighted (base)
        let entry = ReductionEntry {
            source_name: "A",
            target_name: "B",
            source_variant: &[("graph", "SimpleGraph")],
            target_variant: &[("graph", "SimpleGraph")],
            overhead_fn: || ReductionOverhead::default(),
        };
        assert!(entry.is_base_reduction());
    }

    #[test]
    fn test_reduction_entries_registered() {
        let entries: Vec<_> = inventory::iter::<ReductionEntry>().collect();

        // Should have at least some registered reductions
        assert!(entries.len() >= 10);

        // Check specific reductions exist
        assert!(entries
            .iter()
            .any(|e| e.source_name == "IndependentSet" && e.target_name == "VertexCovering"));
    }
}
