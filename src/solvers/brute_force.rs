//! Brute force solver that enumerates all configurations.

use crate::config::DimsIterator;
use crate::solvers::Solver;
use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};
use crate::types::Aggregate;

/// A brute force solver that enumerates all possible configurations.
///
/// This solver is exponential in the number of variables but guarantees
/// finding the full aggregate value and all witness configurations when the
/// aggregate type supports witnesses.
#[derive(Debug, Clone, Default)]
pub struct BruteForce;

impl BruteForce {
    /// Create a new brute force solver.
    pub fn new() -> Self {
        Self
    }

    /// Temporary compatibility helper for optimization problems.
    pub fn find_all_best<P: OptimizationProblem>(&self, problem: &P) -> Vec<Vec<usize>> {
        let iter = DimsIterator::new(problem.dims());
        let direction = problem.direction();
        let mut best_solutions: Vec<Vec<usize>> = vec![];
        let mut best_metric: Option<
            crate::types::SolutionSize<<P as OptimizationProblem>::Objective>,
        > = None;

        for config in iter {
            let metric = problem.evaluate(&config);

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
                best_solutions.push(config);
            }
        }

        best_solutions
    }

    /// Temporary compatibility helper for optimization problems.
    pub fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>> {
        self.find_all_best(problem).into_iter().next()
    }

    /// Temporary compatibility helper for satisfaction problems.
    pub fn find_all_satisfying<P: SatisfactionProblem>(&self, problem: &P) -> Vec<Vec<usize>> {
        DimsIterator::new(problem.dims())
            .filter(|config| problem.evaluate(config))
            .collect()
    }

    /// Temporary compatibility helper for satisfaction problems.
    pub fn find_satisfying<P: SatisfactionProblem>(&self, problem: &P) -> Option<Vec<usize>> {
        DimsIterator::new(problem.dims()).find(|config| problem.evaluate(config))
    }

    /// Find one witness configuration when the aggregate value admits them.
    pub fn find_witness<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: Problem,
        P::Value: Aggregate,
    {
        self.find_all_witnesses(problem).into_iter().next()
    }

    /// Find all witness configurations for witness-supporting aggregates.
    pub fn find_all_witnesses<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: Problem,
        P::Value: Aggregate,
    {
        let total = self.solve(problem);

        if !P::Value::supports_witnesses() {
            return vec![];
        }

        DimsIterator::new(problem.dims())
            .filter(|config| {
                let value = problem.evaluate(config);
                P::Value::contributes_to_witnesses(&value, &total)
            })
            .collect()
    }

    /// Solve a problem and collect all witness configurations in one passable API.
    pub fn solve_with_witnesses<P>(&self, problem: &P) -> (P::Value, Vec<Vec<usize>>)
    where
        P: Problem,
        P::Value: Aggregate,
    {
        let total = self.solve(problem);

        if !P::Value::supports_witnesses() {
            return (total, vec![]);
        }

        let witnesses = DimsIterator::new(problem.dims())
            .filter(|config| {
                let value = problem.evaluate(config);
                P::Value::contributes_to_witnesses(&value, &total)
            })
            .collect();

        (total, witnesses)
    }
}

impl Solver for BruteForce {
    fn solve<P>(&self, problem: &P) -> P::Value
    where
        P: Problem,
        P::Value: Aggregate,
    {
        DimsIterator::new(problem.dims())
            .map(|config| problem.evaluate(&config))
            .fold(P::Value::identity(), P::Value::combine)
    }

    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>> {
        BruteForce::find_best(self, problem)
    }

    fn find_satisfying<P: SatisfactionProblem>(&self, problem: &P) -> Option<Vec<usize>> {
        BruteForce::find_satisfying(self, problem)
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
