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

    /// Internal: find all optimal solutions.
    fn find_all_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: OptimizationProblem,
        P::Metric: Clone,
    {
        let iter = DimsIterator::new(problem.dims());
        let mut best_solutions: Vec<Vec<usize>> = vec![];
        let mut best_metric: Option<P::Metric> = None;

        for config in iter {
            let metric = problem.evaluate(&config);

            // Skip infeasible solutions
            if !problem.is_feasible(&metric) {
                continue;
            }

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

    /// Find all satisfying solutions for constraint satisfaction problems.
    ///
    /// Returns all configurations where `problem.evaluate(config)` returns `true`.
    pub fn find_all_satisfying<P: Problem<Metric = bool>>(
        &self,
        problem: &P,
    ) -> Vec<Vec<usize>> {
        DimsIterator::new(problem.dims())
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
        DimsIterator::new(problem.dims()).find(|config| problem.evaluate(config))
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
