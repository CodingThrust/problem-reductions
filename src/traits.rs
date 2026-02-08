//! Core traits for problem definitions.

use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// The core trait that all problems must implement.
///
/// This trait defines the interface for computational problems that can be
/// solved by enumeration or reduction to other problems.
pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "IndependentSet").
    const NAME: &'static str;

    /// Returns attributes describing this problem variant.
    /// Each (key, value) pair describes a variant dimension.
    /// Common keys: "graph", "weight"
    fn variant() -> Vec<(&'static str, &'static str)>;

    /// The type used for objective/size values.
    type Size: Clone + PartialOrd + Num + Zero + AddAssign;

    /// Returns the number of variables in the problem.
    fn num_variables(&self) -> usize;

    /// Returns the number of possible values (flavors) for each variable.
    /// For binary problems, this is 2.
    fn num_flavors(&self) -> usize;

    /// Returns metadata about the problem size.
    fn problem_size(&self) -> ProblemSize;

    /// Returns whether larger or smaller objective values are better.
    fn energy_mode(&self) -> EnergyMode;

    /// Evaluate the solution size for a given configuration.
    ///
    /// # Arguments
    /// * `config` - A slice of variable assignments, where each value is in 0..num_flavors.
    ///
    /// # Returns
    /// A `SolutionSize` containing the objective value and validity.
    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size>;

    /// Returns the range of variable indices.
    fn variables(&self) -> std::ops::Range<usize> {
        0..self.num_variables()
    }

    /// Returns the possible flavors as a vector.
    fn flavors(&self) -> Vec<usize> {
        (0..self.num_flavors()).collect()
    }

    /// Check if a configuration is valid for this problem.
    fn is_valid_config(&self, config: &[usize]) -> bool {
        if config.len() != self.num_variables() {
            return false;
        }
        let num_flavors = self.num_flavors();
        config.iter().all(|&v| v < num_flavors)
    }

    /// Evaluate multiple configurations at once (batch evaluation).
    fn solution_size_multiple(&self, configs: &[Vec<usize>]) -> Vec<SolutionSize<Self::Size>> {
        configs.iter().map(|c| self.solution_size(c)).collect()
    }
}

/// Trait for constraint satisfaction problems.
///
/// These problems have explicit constraints that must be satisfied,
/// and objectives that contribute to the solution size.
pub trait ConstraintSatisfactionProblem: Problem {
    /// Returns the hard constraints that must be satisfied.
    fn constraints(&self) -> Vec<LocalConstraint>;

    /// Returns the local objectives that contribute to solution size.
    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>>;

    /// Returns the weights for the problem (e.g., vertex weights).
    fn weights(&self) -> Vec<Self::Size>;

    /// Set new weights for the problem.
    fn set_weights(&mut self, weights: Vec<Self::Size>);

    /// Returns whether the problem has non-uniform weights.
    fn is_weighted(&self) -> bool;

    /// Check if all constraints are satisfied by a configuration.
    fn is_satisfied(&self, config: &[usize]) -> bool {
        self.constraints().iter().all(|c| c.is_satisfied(config))
    }

    /// Compute the total objective value from all local objectives.
    fn compute_objective(&self, config: &[usize]) -> Self::Size {
        let mut total = Self::Size::zero();
        for obj in self.objectives() {
            total += obj.evaluate(config);
        }
        total
    }
}

/// A blanket implementation helper for evaluating CSP solution sizes.
/// This can be used by implementors of ConstraintSatisfactionProblem.
pub fn csp_solution_size<P: ConstraintSatisfactionProblem>(
    problem: &P,
    config: &[usize],
) -> SolutionSize<P::Size> {
    let is_valid = problem.is_satisfied(config);
    let size = problem.compute_objective(config);
    SolutionSize::new(size, is_valid)
}

#[cfg(test)]
#[path = "unit_tests/traits.rs"]
mod tests;
