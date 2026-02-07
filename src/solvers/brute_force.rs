//! Brute force solver that enumerates all configurations.

use crate::config::ConfigIterator;
use crate::solvers::Solver;
use crate::traits::Problem;
use crate::types::SolutionSize;

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
    /// If true, only return valid solutions.
    pub valid_only: bool,
}

impl Default for BruteForce {
    fn default() -> Self {
        Self {
            atol: 1e-10,
            rtol: 1e-10,
            valid_only: true,
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
        Self {
            atol,
            rtol,
            valid_only: true,
        }
    }

    /// Set whether to only return valid solutions.
    pub fn valid_only(mut self, valid_only: bool) -> Self {
        self.valid_only = valid_only;
        self
    }

    /// Check if two floating point values are approximately equal.
    fn approx_equal(&self, a: f64, b: f64) -> bool {
        let diff = (a - b).abs();
        diff <= self.atol || diff <= self.rtol * b.abs().max(a.abs())
    }
}

impl Solver for BruteForce {
    fn find_best<P: Problem>(&self, problem: &P) -> Vec<Vec<usize>> {
        self.find_best_with_size(problem)
            .into_iter()
            .map(|(config, _)| config)
            .collect()
    }

    fn find_best_with_size<P: Problem>(
        &self,
        problem: &P,
    ) -> Vec<(Vec<usize>, SolutionSize<P::Size>)> {
        let num_variables = problem.num_variables();
        let num_flavors = problem.num_flavors();

        if num_variables == 0 {
            return vec![];
        }

        let iter = ConfigIterator::new(num_variables, num_flavors);
        let energy_mode = problem.energy_mode();

        let mut best_solutions: Vec<(Vec<usize>, SolutionSize<P::Size>)> = vec![];
        let mut best_size: Option<P::Size> = None;

        for config in iter {
            let solution = problem.solution_size(&config);

            // Skip invalid solutions if valid_only is true
            if self.valid_only && !solution.is_valid {
                continue;
            }

            let is_new_best = match &best_size {
                None => true,
                Some(current_best) => energy_mode.is_better(&solution.size, current_best),
            };

            if is_new_best {
                best_size = Some(solution.size.clone());
                best_solutions.clear();
                best_solutions.push((config, solution));
            } else if let Some(current_best) = &best_size {
                // Check if equal to best (for collecting all optimal solutions)
                if self.is_equal_size(&solution.size, current_best) {
                    best_solutions.push((config, solution));
                }
            }
        }

        best_solutions
    }
}

impl BruteForce {
    /// Check if two sizes are equal (with tolerance for floating point).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn is_equal_size<T: PartialOrd + Clone>(&self, a: &T, b: &T) -> bool {
        // For exact types, use exact comparison via partial_cmp
        // This works for integers and handles incomparable values correctly
        matches!(a.partial_cmp(b), Some(std::cmp::Ordering::Equal))
    }
}

/// Extension trait for floating point comparisons in brute force solver.
pub trait BruteForceFloat {
    /// Find best solutions with floating point tolerance.
    fn find_best_float<P: Problem<Size = f64>>(
        &self,
        problem: &P,
    ) -> Vec<(Vec<usize>, SolutionSize<f64>)>;
}

impl BruteForceFloat for BruteForce {
    fn find_best_float<P: Problem<Size = f64>>(
        &self,
        problem: &P,
    ) -> Vec<(Vec<usize>, SolutionSize<f64>)> {
        let num_variables = problem.num_variables();
        let num_flavors = problem.num_flavors();

        if num_variables == 0 {
            return vec![];
        }

        let iter = ConfigIterator::new(num_variables, num_flavors);
        let energy_mode = problem.energy_mode();

        let mut best_solutions: Vec<(Vec<usize>, SolutionSize<f64>)> = vec![];
        let mut best_size: Option<f64> = None;

        for config in iter {
            let solution = problem.solution_size(&config);

            if self.valid_only && !solution.is_valid {
                continue;
            }

            let is_new_best = match &best_size {
                None => true,
                Some(current_best) => energy_mode.is_better(&solution.size, current_best),
            };

            if is_new_best {
                best_size = Some(solution.size);
                best_solutions.clear();
                best_solutions.push((config, solution));
            } else if let Some(current_best) = &best_size {
                if self.approx_equal(solution.size, *current_best) {
                    best_solutions.push((config, solution));
                }
            }
        }

        best_solutions
    }
}

#[cfg(test)]
#[path = "../tests_unit/solvers/brute_force.rs"]
mod tests;
