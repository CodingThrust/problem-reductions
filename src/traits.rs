//! Core traits for problem definitions.

use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// The core trait that all problems must implement.
///
/// This trait defines the interface for computational problems that can be
/// solved by enumeration or reduction to other problems.
pub trait Problem: Clone {
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
mod tests {
    use super::*;

    // A simple test problem: select binary variables to maximize sum of weights
    #[derive(Clone)]
    struct SimpleWeightedProblem {
        weights: Vec<i32>,
    }

    impl Problem for SimpleWeightedProblem {
        type Size = i32;

        fn num_variables(&self) -> usize {
            self.weights.len()
        }

        fn num_flavors(&self) -> usize {
            2
        }

        fn problem_size(&self) -> ProblemSize {
            ProblemSize::new(vec![("variables", self.weights.len())])
        }

        fn energy_mode(&self) -> EnergyMode {
            EnergyMode::LargerSizeIsBetter
        }

        fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
            let sum: i32 = config
                .iter()
                .zip(&self.weights)
                .map(|(&c, &w)| if c == 1 { w } else { 0 })
                .sum();
            SolutionSize::valid(sum)
        }
    }

    // A simple CSP for testing
    #[derive(Clone)]
    struct SimpleCsp {
        num_vars: usize,
    }

    impl Problem for SimpleCsp {
        type Size = i32;

        fn num_variables(&self) -> usize {
            self.num_vars
        }

        fn num_flavors(&self) -> usize {
            2
        }

        fn problem_size(&self) -> ProblemSize {
            ProblemSize::new(vec![("variables", self.num_vars)])
        }

        fn energy_mode(&self) -> EnergyMode {
            EnergyMode::LargerSizeIsBetter
        }

        fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
            csp_solution_size(self, config)
        }
    }

    impl ConstraintSatisfactionProblem for SimpleCsp {
        fn constraints(&self) -> Vec<LocalConstraint> {
            // Constraint: at most one variable can be 1
            if self.num_vars >= 2 {
                vec![LocalConstraint::new(
                    2,
                    vec![0, 1],
                    vec![true, true, true, false], // (0,0), (0,1), (1,0) OK; (1,1) invalid
                )]
            } else {
                vec![]
            }
        }

        fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
            // Each variable contributes 1 if selected
            (0..self.num_vars)
                .map(|i| LocalSolutionSize::new(2, vec![i], vec![0, 1]))
                .collect()
        }

        fn weights(&self) -> Vec<Self::Size> {
            vec![1; self.num_vars]
        }

        fn set_weights(&mut self, _weights: Vec<Self::Size>) {}

        fn is_weighted(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_simple_problem() {
        let problem = SimpleWeightedProblem {
            weights: vec![1, 2, 3],
        };

        assert_eq!(problem.num_variables(), 3);
        assert_eq!(problem.num_flavors(), 2);
        assert_eq!(problem.variables(), 0..3);
        assert_eq!(problem.flavors(), vec![0, 1]);

        let sol = problem.solution_size(&[0, 0, 0]);
        assert_eq!(sol.size, 0);
        assert!(sol.is_valid);

        let sol = problem.solution_size(&[1, 1, 1]);
        assert_eq!(sol.size, 6);
        assert!(sol.is_valid);

        let sol = problem.solution_size(&[1, 0, 1]);
        assert_eq!(sol.size, 4);
        assert!(sol.is_valid);
    }

    #[test]
    fn test_valid_config() {
        let problem = SimpleWeightedProblem {
            weights: vec![1, 2, 3],
        };

        assert!(problem.is_valid_config(&[0, 1, 0]));
        assert!(problem.is_valid_config(&[1, 1, 1]));
        assert!(!problem.is_valid_config(&[0, 2, 0])); // invalid flavor
        assert!(!problem.is_valid_config(&[0, 1])); // wrong length
        assert!(!problem.is_valid_config(&[0, 1, 0, 1])); // wrong length
    }

    #[test]
    fn test_batch_evaluation() {
        let problem = SimpleWeightedProblem {
            weights: vec![1, 2, 3],
        };

        let configs = vec![vec![0, 0, 0], vec![1, 1, 1], vec![1, 0, 1]];

        let results = problem.solution_size_multiple(&configs);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].size, 0);
        assert_eq!(results[1].size, 6);
        assert_eq!(results[2].size, 4);
    }

    #[test]
    fn test_csp_solution_size() {
        let problem = SimpleCsp { num_vars: 3 };

        // Test valid configurations
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);

        let sol = problem.solution_size(&[1, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // Test invalid configuration (both 0 and 1 are 1)
        let sol = problem.solution_size(&[1, 1, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2);
    }

    #[test]
    fn test_csp_is_satisfied() {
        let problem = SimpleCsp { num_vars: 3 };

        assert!(problem.is_satisfied(&[0, 0, 0]));
        assert!(problem.is_satisfied(&[1, 0, 0]));
        assert!(problem.is_satisfied(&[0, 1, 0]));
        assert!(!problem.is_satisfied(&[1, 1, 0]));
    }

    #[test]
    fn test_csp_compute_objective() {
        let problem = SimpleCsp { num_vars: 3 };

        assert_eq!(problem.compute_objective(&[0, 0, 0]), 0);
        assert_eq!(problem.compute_objective(&[1, 0, 0]), 1);
        assert_eq!(problem.compute_objective(&[1, 1, 0]), 2);
        assert_eq!(problem.compute_objective(&[1, 1, 1]), 3);
    }
}
