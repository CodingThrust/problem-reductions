//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp-solver")]
pub mod ilp;

pub use brute_force::BruteForce;

#[cfg(feature = "ilp-solver")]
pub use ilp::ILPSolver;

use crate::traits::{ObjectiveProblem, Problem, WitnessProblem};

/// Trait for problem solvers.
pub trait Solver {
    /// Solve a problem to its aggregate value.
    fn solve<P>(&self, problem: &P) -> P::Value
    where
        P: Problem,
        P::Value: crate::types::Aggregate;

    /// Temporary compatibility helper for optimization problems.
    fn find_best<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ObjectiveProblem,
        P::Value: crate::types::Aggregate;

    /// Temporary compatibility helper for satisfaction problems.
    fn find_satisfying<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: WitnessProblem,
        P::Value: crate::types::Aggregate;
}
