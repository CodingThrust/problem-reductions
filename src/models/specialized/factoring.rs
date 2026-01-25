//! Integer Factoring problem implementation.
//!
//! The Factoring problem represents integer factorization as a computational problem.
//! Given a number N, find two factors (a, b) such that a * b = N.

use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

/// The Integer Factoring problem.
///
/// Given a number to factor, find two integers that multiply to give
/// the target number. Variables represent the bits of the two factors.
///
/// # Example
///
/// ```
/// use problemreductions::models::specialized::Factoring;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Factor 6 with 2-bit factors (allowing factors 0-3)
/// let problem = Factoring::new(2, 2, 6);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Should find: 2*3=6 or 3*2=6
/// for sol in &solutions {
///     let (a, b) = problem.read_factors(sol);
///     assert_eq!(a * b, 6);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factoring {
    /// Number of bits for the first factor.
    m: usize,
    /// Number of bits for the second factor.
    n: usize,
    /// The number to factor.
    target: u64,
}

impl Factoring {
    /// Create a new Factoring problem.
    ///
    /// # Arguments
    /// * `m` - Number of bits for the first factor
    /// * `n` - Number of bits for the second factor
    /// * `target` - The number to factor
    pub fn new(m: usize, n: usize, target: u64) -> Self {
        Self { m, n, target }
    }

    /// Get the number of bits for the first factor.
    pub fn m(&self) -> usize {
        self.m
    }

    /// Get the number of bits for the second factor.
    pub fn n(&self) -> usize {
        self.n
    }

    /// Get the target number to factor.
    pub fn target(&self) -> u64 {
        self.target
    }

    /// Read the two factors from a configuration.
    ///
    /// The first `m` bits represent the first factor,
    /// the next `n` bits represent the second factor.
    pub fn read_factors(&self, config: &[usize]) -> (u64, u64) {
        let a = bits_to_int(&config[..self.m]);
        let b = bits_to_int(&config[self.m..self.m + self.n]);
        (a, b)
    }

    /// Check if the configuration is a valid factorization.
    pub fn is_valid_factorization(&self, config: &[usize]) -> bool {
        let (a, b) = self.read_factors(config);
        a * b == self.target
    }
}

/// Convert a bit vector (little-endian) to an integer.
fn bits_to_int(bits: &[usize]) -> u64 {
    bits.iter().enumerate().map(|(i, &b)| (b as u64) << i).sum()
}

/// Convert an integer to a bit vector (little-endian).
#[allow(dead_code)]
fn int_to_bits(n: u64, num_bits: usize) -> Vec<usize> {
    (0..num_bits).map(|i| ((n >> i) & 1) as usize).collect()
}

impl Problem for Factoring {
    type Size = i32;

    fn num_variables(&self) -> usize {
        self.m + self.n
    }

    fn num_flavors(&self) -> usize {
        2 // Binary
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_bits_first", self.m),
            ("num_bits_second", self.n),
            ("target", self.target as usize),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize distance from target
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let (a, b) = self.read_factors(config);
        let product = a * b;

        // Distance from target (0 means exact match)
        let distance = if product > self.target {
            (product - self.target) as i32
        } else {
            (self.target - product) as i32
        };

        let is_valid = product == self.target;
        SolutionSize::new(distance, is_valid)
    }
}

/// Check if the given factors correctly factorize the target.
pub fn is_factoring(target: u64, a: u64, b: u64) -> bool {
    a * b == target
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_factoring_creation() {
        let problem = Factoring::new(3, 3, 15);
        assert_eq!(problem.m(), 3);
        assert_eq!(problem.n(), 3);
        assert_eq!(problem.target(), 15);
        assert_eq!(problem.num_variables(), 6);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_bits_to_int() {
        assert_eq!(bits_to_int(&[0, 0, 0]), 0);
        assert_eq!(bits_to_int(&[1, 0, 0]), 1);
        assert_eq!(bits_to_int(&[0, 1, 0]), 2);
        assert_eq!(bits_to_int(&[1, 1, 0]), 3);
        assert_eq!(bits_to_int(&[0, 0, 1]), 4);
        assert_eq!(bits_to_int(&[1, 1, 1]), 7);
    }

    #[test]
    fn test_int_to_bits() {
        assert_eq!(int_to_bits(0, 3), vec![0, 0, 0]);
        assert_eq!(int_to_bits(1, 3), vec![1, 0, 0]);
        assert_eq!(int_to_bits(2, 3), vec![0, 1, 0]);
        assert_eq!(int_to_bits(3, 3), vec![1, 1, 0]);
        assert_eq!(int_to_bits(7, 3), vec![1, 1, 1]);
    }

    #[test]
    fn test_read_factors() {
        let problem = Factoring::new(2, 2, 6);
        // bits: [a0, a1, b0, b1]
        // a=2 (binary 10), b=3 (binary 11) -> config = [0,1,1,1]
        let (a, b) = problem.read_factors(&[0, 1, 1, 1]);
        assert_eq!(a, 2);
        assert_eq!(b, 3);
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = Factoring::new(2, 2, 6);
        // 2 * 3 = 6
        let sol = problem.solution_size(&[0, 1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0); // Exact match

        // 3 * 2 = 6
        let sol = problem.solution_size(&[1, 1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = Factoring::new(2, 2, 6);
        // 2 * 2 = 4 != 6
        let sol = problem.solution_size(&[0, 1, 0, 1]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2); // Distance from 6

        // 1 * 1 = 1 != 6
        let sol = problem.solution_size(&[1, 0, 1, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 5); // Distance from 6
    }

    #[test]
    fn test_brute_force_factor_6() {
        let problem = Factoring::new(2, 2, 6);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Should find 2*3 and 3*2
        assert!(!solutions.is_empty());
        for sol in &solutions {
            let (a, b) = problem.read_factors(sol);
            assert_eq!(a * b, 6);
        }
    }

    #[test]
    fn test_brute_force_factor_15() {
        let problem = Factoring::new(3, 3, 15);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Should find 3*5, 5*3, 1*15, 15*1
        for sol in &solutions {
            let (a, b) = problem.read_factors(sol);
            assert_eq!(a * b, 15);
        }
    }

    #[test]
    fn test_brute_force_prime() {
        // 7 is prime, only 1*7 and 7*1 work
        let problem = Factoring::new(3, 3, 7);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        let factor_pairs: Vec<_> = solutions.iter().map(|s| problem.read_factors(s)).collect();

        // Should find (1,7) and (7,1)
        assert!(factor_pairs.contains(&(1, 7)) || factor_pairs.contains(&(7, 1)));
    }

    #[test]
    fn test_is_factoring_function() {
        assert!(is_factoring(6, 2, 3));
        assert!(is_factoring(6, 3, 2));
        assert!(is_factoring(15, 3, 5));
        assert!(!is_factoring(6, 2, 2));
    }

    #[test]
    fn test_energy_mode() {
        let problem = Factoring::new(2, 2, 6);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_problem_size() {
        let problem = Factoring::new(3, 4, 12);
        let size = problem.problem_size();
        assert_eq!(size.get("num_bits_first"), Some(3));
        assert_eq!(size.get("num_bits_second"), Some(4));
        assert_eq!(size.get("target"), Some(12));
    }

    #[test]
    fn test_is_valid_factorization() {
        let problem = Factoring::new(2, 2, 6);
        assert!(problem.is_valid_factorization(&[0, 1, 1, 1])); // 2*3=6
        assert!(!problem.is_valid_factorization(&[0, 1, 0, 1])); // 2*2=4
    }

    #[test]
    fn test_factor_one() {
        // Factor 1: only 1*1 works
        let problem = Factoring::new(2, 2, 1);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            let (a, b) = problem.read_factors(sol);
            assert_eq!(a * b, 1);
        }
    }
}
