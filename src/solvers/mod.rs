//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp")]
pub mod ilp;

pub use brute_force::BruteForce;

#[cfg(feature = "ilp")]
pub use ilp::ILPSolver;

use crate::traits::{OptimizationProblem, Problem};

/// Trait for problem solvers.
pub trait Solver {
    /// Find best solution(s) for an optimization problem.
    ///
    /// Returns all configurations that achieve the optimal metric value.
    /// Returns empty vec if all configurations are invalid.
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Vec<Vec<usize>>;

    /// Find any satisfying solution for a satisfaction problem (Metric = bool).
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
