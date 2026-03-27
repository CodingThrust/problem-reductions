//! Cost functions for reduction path optimization.

use crate::rules::registry::ReductionOverhead;
use crate::types::ProblemSize;

/// User-defined cost function for path optimization.
pub trait PathCostFn {
    /// Compute cost of taking an edge given current problem size.
    fn edge_cost(&self, overhead: &ReductionOverhead, current_size: &ProblemSize) -> f64;
}

/// Minimize a single output field.
pub struct Minimize(pub &'static str);

impl PathCostFn for Minimize {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        overhead.evaluate_output_size(size).get(self.0).unwrap_or(0) as f64
    }
}

/// Minimize number of reduction steps.
pub struct MinimizeSteps;

impl PathCostFn for MinimizeSteps {
    fn edge_cost(&self, _overhead: &ReductionOverhead, _size: &ProblemSize) -> f64 {
        1.0
    }
}

/// Minimize total output size (sum of all output field values).
///
/// Prefers reduction paths that produce smaller intermediate and final problems.
/// Breaks ties that `MinimizeSteps` cannot resolve (e.g., two 2-step paths
/// where one produces 144 ILP variables and the other 1,332).
pub struct MinimizeOutputSize;

impl PathCostFn for MinimizeOutputSize {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
        output.total() as f64
    }
}

/// Minimize steps first, then use output size as tiebreaker.
///
/// Each edge has a primary cost of `STEP_WEIGHT` (ensuring fewer-step paths
/// always win) plus a small overhead-based cost that breaks ties between
/// equal-step paths.
pub struct MinimizeStepsThenOverhead;

impl PathCostFn for MinimizeStepsThenOverhead {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        // Use a large step weight to ensure step count dominates.
        // The overhead tiebreaker is normalized to [0, 1) by using log1p,
        // so it never outweighs a single step difference.
        const STEP_WEIGHT: f64 = 1e9;
        let output = overhead.evaluate_output_size(size);
        let overhead_tiebreaker = (1.0 + output.total() as f64).ln();
        STEP_WEIGHT + overhead_tiebreaker
    }
}

/// Custom cost function from closure.
pub struct CustomCost<F>(pub F);

impl<F: Fn(&ReductionOverhead, &ProblemSize) -> f64> PathCostFn for CustomCost<F> {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        (self.0)(overhead, size)
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/cost.rs"]
mod tests;
