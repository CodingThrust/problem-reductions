//! Brute force solver that enumerates all configurations.

use crate::config::DimsIterator;
use crate::solvers::Solver;
use crate::traits::{OptimizationProblem, Problem};

/// A brute force solver that enumerates all possible configurations.
///
/// This solver is exponential in the number of variables but guarantees
/// finding all optimal solutions.
#[derive(Debug, Clone, Default)]
pub struct BruteForce;

impl BruteForce {
    /// Create a new brute force solver.
    pub fn new() -> Self {
        Self
    }

    /// Find all optimal solutions for an optimization problem.
    ///
    /// Returns all configurations that achieve the optimal metric value.
    /// Returns empty vec if all configurations are invalid.
    pub fn find_all_best<P: OptimizationProblem>(&self, problem: &P) -> Vec<Vec<usize>> {
        let iter = DimsIterator::new(problem.dims());
        let direction = problem.direction();
        let mut best_solutions: Vec<Vec<usize>> = vec![];
        let mut best_metric: Option<crate::types::SolutionSize<P::Value>> = None;

        for config in iter {
            let metric = problem.evaluate(&config);

            // Skip infeasible solutions
            if !metric.is_valid() {
                continue;
            }

            let dominated = match &best_metric {
                None => false,
                Some(current_best) => current_best.is_better(&metric, direction),
            };

            if dominated {
                continue;
            }

            let dominates = match &best_metric {
                None => true,
                Some(current_best) => metric.is_better(current_best, direction),
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

    /// Find all satisfying solutions for constraint satisfaction problems.
    ///
    /// Returns all configurations where `problem.evaluate(config)` returns `true`.
    pub fn find_all_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Vec<Vec<usize>> {
        DimsIterator::new(problem.dims())
            .filter(|config| problem.evaluate(config))
            .collect()
    }
}

impl Solver for BruteForce {
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>> {
        self.find_all_best(problem).into_iter().next()
    }

    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>> {
        DimsIterator::new(problem.dims()).find(|config| problem.evaluate(config))
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
