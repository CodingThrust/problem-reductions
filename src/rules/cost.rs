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

/// Minimize weighted sum of output fields.
pub struct MinimizeWeighted(pub Vec<(&'static str, f64)>);

impl PathCostFn for MinimizeWeighted {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
        self.0.iter()
            .map(|(field, weight)| weight * output.get(field).unwrap_or(0) as f64)
            .sum()
    }
}

/// Minimize the maximum of specified fields.
pub struct MinimizeMax(pub Vec<&'static str>);

impl PathCostFn for MinimizeMax {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
        self.0.iter()
            .map(|field| output.get(field).unwrap_or(0) as f64)
            .fold(0.0, f64::max)
    }
}

/// Lexicographic: minimize first field, break ties with subsequent.
pub struct MinimizeLexicographic(pub Vec<&'static str>);

impl PathCostFn for MinimizeLexicographic {
    fn edge_cost(&self, overhead: &ReductionOverhead, size: &ProblemSize) -> f64 {
        let output = overhead.evaluate_output_size(size);
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
mod tests {
    use super::*;
    use crate::polynomial::Polynomial;

    fn test_overhead() -> ReductionOverhead {
        ReductionOverhead::new(vec![
            ("n", Polynomial::var("n").scale(2.0)),
            ("m", Polynomial::var("m")),
        ])
    }

    #[test]
    fn test_minimize_single() {
        let cost_fn = Minimize("n");
        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        let overhead = test_overhead();

        assert_eq!(cost_fn.edge_cost(&overhead, &size), 20.0);  // 2 * 10
    }

    #[test]
    fn test_minimize_weighted() {
        let cost_fn = MinimizeWeighted(vec![("n", 1.0), ("m", 2.0)]);
        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        let overhead = test_overhead();

        // output n = 20, output m = 5
        // cost = 1.0 * 20 + 2.0 * 5 = 30
        assert_eq!(cost_fn.edge_cost(&overhead, &size), 30.0);
    }

    #[test]
    fn test_minimize_steps() {
        let cost_fn = MinimizeSteps;
        let size = ProblemSize::new(vec![("n", 100)]);
        let overhead = test_overhead();

        assert_eq!(cost_fn.edge_cost(&overhead, &size), 1.0);
    }

    #[test]
    fn test_minimize_max() {
        let cost_fn = MinimizeMax(vec!["n", "m"]);
        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        let overhead = test_overhead();

        // output n = 20, output m = 5
        // max(20, 5) = 20
        assert_eq!(cost_fn.edge_cost(&overhead, &size), 20.0);
    }

    #[test]
    fn test_minimize_lexicographic() {
        let cost_fn = MinimizeLexicographic(vec!["n", "m"]);
        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        let overhead = test_overhead();

        // output n = 20, output m = 5
        // cost = 20 * 1.0 + 5 * 1e-10 = 20.0000000005
        let cost = cost_fn.edge_cost(&overhead, &size);
        assert!(cost > 20.0 && cost < 20.001);
    }
}
