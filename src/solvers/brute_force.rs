//! Brute force solver that enumerates all configurations.

use crate::config::DimsIterator;
use crate::solvers::{SolveError, Solver};
use crate::traits::Problem;
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

    /// Find one witness configuration when the aggregate value admits them.
    pub fn find_witness<P>(&self, problem: &P) -> Result<Option<Vec<usize>>, SolveError>
    where
        P: Problem,
        P::Value: Aggregate,
    {
        let total = self.solve(problem)?;

        if !P::Value::supports_witnesses() {
            return Ok(None);
        }

        for config in DimsIterator::new(problem.dims()) {
            let value = problem.evaluate(&config)?;
            if P::Value::contributes_to_witnesses(&value, &total) {
                return Ok(Some(config));
            }
        }
        Ok(None)
    }

    /// Find all witness configurations for witness-supporting aggregates.
    pub fn find_all_witnesses<P>(&self, problem: &P) -> Result<Vec<Vec<usize>>, SolveError>
    where
        P: Problem,
        P::Value: Aggregate,
    {
        let total = self.solve(problem)?;

        if !P::Value::supports_witnesses() {
            return Ok(vec![]);
        }

        let mut witnesses = Vec::new();
        for config in DimsIterator::new(problem.dims()) {
            let value = problem.evaluate(&config)?;
            if P::Value::contributes_to_witnesses(&value, &total) {
                witnesses.push(config);
            }
        }
        Ok(witnesses)
    }

    /// Solve a problem and collect all witness configurations.
    pub fn solve_with_witnesses<P>(
        &self,
        problem: &P,
    ) -> Result<(P::Value, Vec<Vec<usize>>), SolveError>
    where
        P: Problem,
        P::Value: Aggregate,
    {
        let total = self.solve(problem)?;

        if !P::Value::supports_witnesses() {
            return Ok((total, vec![]));
        }

        let mut witnesses = Vec::new();
        for config in DimsIterator::new(problem.dims()) {
            let value = problem.evaluate(&config)?;
            if P::Value::contributes_to_witnesses(&value, &total) {
                witnesses.push(config);
            }
        }

        Ok((total, witnesses))
    }
}

impl Solver for BruteForce {
    fn solve<P>(&self, problem: &P) -> Result<P::Value, SolveError>
    where
        P: Problem,
        P::Value: Aggregate,
    {
        let mut total = P::Value::identity();
        for config in DimsIterator::new(problem.dims()) {
            let value = problem.evaluate(&config)?;
            total = total.combine(value)?;
        }
        Ok(total)
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
