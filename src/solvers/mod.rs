//! Solvers for computational problems.

mod brute_force;
pub mod decision_search;
mod native;
mod pipelines;
mod registry;
mod resolver;

pub mod ilp;

pub use brute_force::BruteForce;
pub use registry::{
    solver_capabilities, ExactProblemKey, IlpSolverCapability, NativeSolverCapability,
    RegistryBuildError, SolverCapabilities,
};
pub use resolver::{
    solve_deterministically, DeterministicSolveError, DeterministicSolveResult, SolverExecution,
    SolverRequest,
};

pub use ilp::{ILPSolveError, ILPSolver};

use crate::traits::Problem;

/// Trait for problem solvers.
pub trait Solver {
    /// Solve a problem to its aggregate value.
    fn solve<P>(&self, problem: &P) -> P::Value
    where
        P: Problem,
        P::Value: crate::types::Aggregate;
}
