//! Brute force solver that enumerates all configurations.

use crate::config::DimsIterator;
use crate::solvers::Solver;
use crate::traits::{ObjectiveProblem, Problem, WitnessProblem};
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
    pub fn find_all_best<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: ObjectiveProblem,
        P::Value: Aggregate,
    {
        self.find_all_witnesses(problem)
    }

    /// Temporary compatibility helper for optimization problems.
    pub fn find_best<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ObjectiveProblem,
        P::Value: Aggregate,
    {
        self.find_witness(problem)
    }

    /// Temporary compatibility helper for satisfaction problems.
    pub fn find_all_satisfying<P>(&self, problem: &P) -> Vec<Vec<usize>>
    where
        P: WitnessProblem,
        P::Value: Aggregate,
    {
        self.find_all_witnesses(problem)
    }

    /// Temporary compatibility helper for satisfaction problems.
    pub fn find_satisfying<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: WitnessProblem,
        P::Value: Aggregate,
    {
        self.find_witness(problem)
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

    fn find_best<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ObjectiveProblem,
        P::Value: Aggregate,
    {
        BruteForce::find_witness(self, problem)
    }

    fn find_satisfying<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: WitnessProblem,
        P::Value: Aggregate,
    {
        BruteForce::find_witness(self, problem)
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
