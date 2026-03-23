//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp-solver")]
pub mod ilp;

pub use brute_force::BruteForce;

#[cfg(feature = "ilp-solver")]
pub use ilp::ILPSolver;

use crate::traits::Problem;

/// Trait for problem solvers.
pub trait Solver {
    /// Solve a problem to its aggregate value.
    fn solve<P>(&self, problem: &P) -> P::Value
    where
        P: Problem,
        P::Value: crate::types::Aggregate;
}
