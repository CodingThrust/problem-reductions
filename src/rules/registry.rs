//! Automatic reduction registration via inventory.

use crate::polynomial::Polynomial;
use crate::types::ProblemSize;

/// Overhead specification for a reduction.
#[derive(Clone, Debug)]
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
    pub fn evaluate_output_size(&self, input: &ProblemSize) -> ProblemSize {
        let fields: Vec<_> = self.output_size.iter()
            .map(|(name, poly)| (*name, poly.evaluate(input).round() as usize))
            .collect();
        ProblemSize::new(fields)
    }
}

impl Default for ReductionOverhead {
    fn default() -> Self {
        Self { output_size: vec![] }
    }
}

/// A registered reduction entry.
#[derive(Clone, Debug)]
pub struct ReductionEntry {
    /// Base name of source problem (e.g., "IndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "VertexCovering").
    pub target_name: &'static str,
    /// Graph type of source problem (e.g., "SimpleGraph").
    pub source_graph: &'static str,
    /// Graph type of target problem.
    pub target_graph: &'static str,
    /// Overhead information.
    pub overhead: ReductionOverhead,
}

inventory::collect!(ReductionEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly;

    #[test]
    fn test_reduction_overhead_evaluate() {
        let overhead = ReductionOverhead::new(vec![
            ("n", poly!(3 * m)),
            ("m", poly!(m^2)),
        ]);

        let input = ProblemSize::new(vec![("m", 4)]);
        let output = overhead.evaluate_output_size(&input);

        assert_eq!(output.get("n"), Some(12));  // 3 * 4
        assert_eq!(output.get("m"), Some(16));  // 4^2
    }

    #[test]
    fn test_reduction_overhead_default() {
        let overhead = ReductionOverhead::default();
        assert!(overhead.output_size.is_empty());
    }
}
