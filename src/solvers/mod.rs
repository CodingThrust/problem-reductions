//! Solvers for computational problems.

mod brute_force;
mod customized;
pub mod decision_search;
mod pipelines;
mod registry;
mod resolver;

pub mod ilp;

pub use brute_force::BruteForce;
pub use registry::{
    solver_capabilities, CustomizedSolverCapability, ExactProblemKey, IlpSolverCapability,
    RegistryBuildError, SolverCapabilities,
};
pub use resolver::{
    solve_deterministically, DeterministicSolveError, DeterministicSolveResult, SolveOutcome,
    SolverExecution, SolverRequest,
};

pub use ilp::{ILPSolveError, ILPSolver};

use crate::traits::Problem;

/// Failure while solving a valid problem instance.
#[derive(Debug, thiserror::Error)]
pub enum SolveError {
    #[error("configuration evaluation failed: {0}")]
    Evaluation(#[from] crate::traits::EvaluationError),
    #[error("aggregate combination failed: {0}")]
    Aggregation(#[from] crate::types::AggregationError),
}

/// Trait for problem solvers.
pub trait Solver {
    /// Solve a problem to its aggregate value.
    fn solve<P>(&self, problem: &P) -> Result<P::Value, SolveError>
    where
        P: Problem,
        P::Value: crate::types::Aggregate;
}
