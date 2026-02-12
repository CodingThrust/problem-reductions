//! Brute force solver that enumerates all configurations.

use crate::config::DimsIterator;
use crate::solvers::Solver;
use crate::traits::{OptimizationProblem, Problem};

/// A brute force solver that enumerates all possible configurations.
///
/// This solver is exponential in the number of variables but guarantees
/// finding all optimal solutions.
#[derive(Debug, Clone)]
pub struct BruteForce {
    /// Absolute tolerance for comparing objective values.
    pub atol: f64,
    /// Relative tolerance for comparing objective values.
    pub rtol: f64,
}

impl Default for BruteForce {
    fn default() -> Self {
        Self {
            atol: 1e-10,
            rtol: 1e-10,
        }
    }
}

impl BruteForce {
    /// Create a new brute force solver with default tolerances.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a brute force solver with custom tolerances.
    pub fn with_tolerance(atol: f64, rtol: f64) -> Self {
        Self { atol, rtol }
    }

    /// Check if two floating point values are approximately equal.
    fn approx_equal(&self, a: f64, b: f64) -> bool {
        let diff = (a - b).abs();
        diff <= self.atol || diff <= self.rtol * b.abs().max(a.abs())
    }

    /// Internal: find all optimal solutions.
    fn find_all_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: OptimizationProblem,
        P::Metric: Clone,
    {
        let dims = problem.dims();
        if dims.is_empty() {
            return vec![];
        }

        let iter = DimsIterator::new(dims);
        let mut best_solutions: Vec<Vec<usize>> = vec![];
        let mut best_metric: Option<P::Metric> = None;

        for config in iter {
            let metric = problem.evaluate(&config);

            let dominated = match &best_metric {
                None => false,
                Some(current_best) => problem.is_better(current_best, &metric),
            };

            if dominated {
                continue;
            }

            let dominates = match &best_metric {
                None => true,
                Some(current_best) => problem.is_better(&metric, current_best),
            };

            if dominates {
                best_metric = Some(metric);
                best_solutions.clear();
                best_solutions.push(config);
            } else if best_metric.is_some() {
                // Equal quality - add to solutions
                best_solutions.push(config);
            }
        }

        best_solutions
    }

    /// Find all satisfying solutions (internal, used for testing).
    pub(crate) fn find_all_satisfying<P: Problem<Metric = bool>>(
        &self,
        problem: &P,
    ) -> Vec<Vec<usize>> {
        let dims = problem.dims();
        if dims.is_empty() {
            return vec![];
        }
        DimsIterator::new(dims)
            .filter(|config| problem.evaluate(config))
            .collect()
    }
}

impl Solver for BruteForce {
    fn find_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: OptimizationProblem,
        P::Metric: Clone,
    {
        self.find_all_best(problem)
    }

    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>> {
        let dims = problem.dims();
        if dims.is_empty() {
            return None;
        }
        DimsIterator::new(dims).find(|config| problem.evaluate(config))
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
