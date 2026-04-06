//! Solvers for computational problems.

mod brute_force;
pub mod customized;
pub mod golden_section;

#[cfg(feature = "ilp-solver")]
pub mod ilp;

pub use brute_force::BruteForce;
pub use customized::CustomizedSolver;

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
