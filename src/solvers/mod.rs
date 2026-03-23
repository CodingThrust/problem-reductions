//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp-solver")]
pub mod ilp;

pub use brute_force::BruteForce;

#[cfg(feature = "ilp-solver")]
pub use ilp::ILPSolver;

use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};

/// Trait for problem solvers.
pub trait Solver {
    /// Solve a problem to its aggregate value.
    fn solve<P>(&self, problem: &P) -> P::Value
    where
        P: Problem,
        P::Value: crate::types::Aggregate;

    /// Temporary compatibility helper for optimization problems.
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;

    /// Temporary compatibility helper for satisfaction problems.
    fn find_satisfying<P: SatisfactionProblem>(&self, problem: &P) -> Option<Vec<usize>>;
}
