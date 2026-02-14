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
        overhead
            .evaluate_output_size(size)
            .expect("overhead evaluation failed")
            .get(self.0)
            .unwrap_or(0) as f64
    }
}

/// Minimize weighted sum of output fields.
pub struct MinimizeWeighted(pub Vec<(&'static str, f64)>);

impl PathCostFn for MinimizeWeighted {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead
            .evaluate_output_size(size)
            .expect("overhead evaluation failed");
        self.0
            .iter()
            .map(|(field, weight)| weight * output.get(field).unwrap_or(0) as f64)
            .sum()
    }
}

/// Minimize the maximum of specified fields.
pub struct MinimizeMax(pub Vec<&'static str>);

impl PathCostFn for MinimizeMax {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead
            .evaluate_output_size(size)
            .expect("overhead evaluation failed");
        self.0
            .iter()
            .map(|field| output.get(field).unwrap_or(0) as f64)
            .fold(0.0, f64::max)
    }
}

/// Lexicographic: minimize first field, break ties with subsequent.
pub struct MinimizeLexicographic(pub Vec<&'static str>);

impl PathCostFn for MinimizeLexicographic {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead
            .evaluate_output_size(size)
            .expect("overhead evaluation failed");
        let mut cost = 0.0;
        let mut scale = 1.0;
        for field in &self.0 {
            cost += scale * output.get(field).unwrap_or(0) as f64;
            scale *= 1e-10;
        }
        cost
    }
}

/// Minimize number of reduction steps.
pub struct MinimizeSteps;

impl PathCostFn for MinimizeSteps {
    fn edge_cost(&self, _overhead: &ReductionOverhead, _size: &ProblemSize) -> f64 {
        1.0
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
