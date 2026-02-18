//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp-solver")]
pub mod ilp;

pub use brute_force::BruteForce;

#[cfg(feature = "ilp-solver")]
pub use ilp::ILPSolver;

use crate::traits::{OptimizationProblem, Problem};

/// Trait for problem solvers.
pub trait Solver {
    /// Find one optimal solution for an optimization problem.
    ///
    /// Returns a single configuration that achieves the optimal metric value,
    /// or `None` if no feasible configuration exists.
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;

    /// Find any satisfying solution for a satisfaction problem (Metric = bool).
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
