//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp")]
pub mod ilp;

pub use brute_force::{BruteForce, BruteForceFloat};

#[cfg(feature = "ilp")]
pub use ilp::{ILPSolver, ToILP};

use crate::traits::Problem;
use crate::types::SolutionSize;

/// Trait for problem solvers.
pub trait Solver {
    /// Find the best solution(s) for a problem.
    ///
    /// Returns all configurations that achieve the optimal objective value.
    fn find_best<P: Problem>(&self, problem: &P) -> Vec<Vec<usize>>;

    /// Find the best solution(s) along with their solution sizes.
    fn find_best_with_size<P: Problem>(
        &self,
        problem: &P,
    ) -> Vec<(Vec<usize>, SolutionSize<P::Size>)>;
}
