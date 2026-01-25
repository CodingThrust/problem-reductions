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
mod tests {
    use super::*;
    use crate::types::{EnergyMode, ProblemSize};

    // Simple maximization problem: maximize sum of selected weights
    #[derive(Clone)]
    struct MaxSumProblem {
        weights: Vec<i32>,
    }

    impl Problem for MaxSumProblem {
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

    // Simple minimization problem: minimize sum of selected weights
    #[derive(Clone)]
    struct MinSumProblem {
        weights: Vec<i32>,
    }

    impl Problem for MinSumProblem {
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
            EnergyMode::SmallerSizeIsBetter
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

    // Problem with validity constraint: select at most one
    #[derive(Clone)]
    struct SelectAtMostOneProblem {
        weights: Vec<i32>,
    }

    impl Problem for SelectAtMostOneProblem {
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
            let selected: usize = config.iter().sum();
            let sum: i32 = config
                .iter()
                .zip(&self.weights)
                .map(|(&c, &w)| if c == 1 { w } else { 0 })
                .sum();
            SolutionSize::new(sum, selected <= 1)
        }
    }

    #[test]
    fn test_brute_force_maximization() {
        let problem = MaxSumProblem {
            weights: vec![1, 2, 3],
        };
        let solver = BruteForce::new();

        let best = solver.find_best(&problem);
        assert_eq!(best.len(), 1);
        assert_eq!(best[0], vec![1, 1, 1]); // Select all for max sum = 6
    }

    #[test]
    fn test_brute_force_minimization() {
        let problem = MinSumProblem {
            weights: vec![1, 2, 3],
        };
        let solver = BruteForce::new();

        let best = solver.find_best(&problem);
        assert_eq!(best.len(), 1);
        assert_eq!(best[0], vec![0, 0, 0]); // Select none for min sum = 0
    }

    #[test]
    fn test_brute_force_with_validity() {
        let problem = SelectAtMostOneProblem {
            weights: vec![1, 5, 3],
        };
        let solver = BruteForce::new();

        let best = solver.find_best(&problem);
        assert_eq!(best.len(), 1);
        assert_eq!(best[0], vec![0, 1, 0]); // Select weight 5 (max single)
    }

    #[test]
    fn test_brute_force_multiple_optimal() {
        let problem = MaxSumProblem {
            weights: vec![1, 1, 1],
        };
        let solver = BruteForce::new();

        let best = solver.find_best(&problem);
        assert_eq!(best.len(), 1);
        assert_eq!(best[0], vec![1, 1, 1]); // All equal, so only one optimal

        // Problem with multiple optimal solutions
        let problem2 = SelectAtMostOneProblem {
            weights: vec![5, 5, 3],
        };
        let best2 = solver.find_best(&problem2);
        assert_eq!(best2.len(), 2); // Both [1,0,0] and [0,1,0] give weight 5
    }

    #[test]
    fn test_brute_force_with_size() {
        let problem = MaxSumProblem {
            weights: vec![1, 2, 3],
        };
        let solver = BruteForce::new();

        let best = solver.find_best_with_size(&problem);
        assert_eq!(best.len(), 1);
        assert_eq!(best[0].0, vec![1, 1, 1]);
        assert_eq!(best[0].1.size, 6);
        assert!(best[0].1.is_valid);
    }

    #[test]
    fn test_brute_force_empty_problem() {
        let problem = MaxSumProblem { weights: vec![] };
        let solver = BruteForce::new();

        let best = solver.find_best(&problem);
        assert!(best.is_empty());
    }

    #[test]
    fn test_brute_force_valid_only_false() {
        let problem = SelectAtMostOneProblem {
            weights: vec![1, 2, 3],
        };
        let solver = BruteForce::new().valid_only(false);

        let best = solver.find_best(&problem);
        // With valid_only=false, the best is selecting all (sum=6) even though invalid
        assert_eq!(best.len(), 1);
        assert_eq!(best[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_brute_force_with_tolerance() {
        let solver = BruteForce::with_tolerance(0.01, 0.01);
        assert_eq!(solver.atol, 0.01);
        assert_eq!(solver.rtol, 0.01);
    }

    // Float problem for testing BruteForceFloat
    #[derive(Clone)]
    struct FloatProblem {
        weights: Vec<f64>,
    }

    impl Problem for FloatProblem {
        type Size = f64;

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
            let sum: f64 = config
                .iter()
                .zip(&self.weights)
                .map(|(&c, &w)| if c == 1 { w } else { 0.0 })
                .sum();
            SolutionSize::valid(sum)
        }
    }

    #[test]
    fn test_brute_force_float() {
        use super::BruteForceFloat;

        let problem = FloatProblem {
            weights: vec![1.0, 2.0, 3.0],
        };
        let solver = BruteForce::new();

        let best = solver.find_best_float(&problem);
        assert_eq!(best.len(), 1);
        assert_eq!(best[0].0, vec![1, 1, 1]);
        assert!((best[0].1.size - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_brute_force_float_tolerance() {
        use super::BruteForceFloat;

        // Problem where multiple solutions have nearly equal values
        #[derive(Clone)]
        struct NearlyEqualProblem;

        impl Problem for NearlyEqualProblem {
            type Size = f64;

            fn num_variables(&self) -> usize {
                2
            }

            fn num_flavors(&self) -> usize {
                2
            }

            fn problem_size(&self) -> ProblemSize {
                ProblemSize::new(vec![("variables", 2)])
            }

            fn energy_mode(&self) -> EnergyMode {
                EnergyMode::LargerSizeIsBetter
            }

            fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
                let size = match (config.first(), config.get(1)) {
                    (Some(1), Some(0)) => 10.0,
                    (Some(0), Some(1)) => 10.0 + 1e-12, // Nearly equal
                    _ => 0.0,
                };
                SolutionSize::valid(size)
            }
        }

        let problem = NearlyEqualProblem;
        let solver = BruteForce::with_tolerance(1e-10, 1e-10);

        let best = solver.find_best_float(&problem);
        // Both should be considered optimal due to tolerance
        assert_eq!(best.len(), 2);
    }

    #[test]
    fn test_brute_force_float_empty() {
        use super::BruteForceFloat;

        let problem = FloatProblem { weights: vec![] };
        let solver = BruteForce::new();

        let best = solver.find_best_float(&problem);
        assert!(best.is_empty());
    }
}
