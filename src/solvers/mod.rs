//! Solvers for computational problems.

mod brute_force;
mod customized;
pub mod decision_search;
mod pipelines;
mod registry;
mod resolver;

pub mod ilp;

#[doc(hidden)]
pub use brute_force::BruteForceRegistration;
pub use brute_force::{BruteForce, BruteForceProblem};
pub use registry::{
    brute_force_dimensions, solver_capabilities, CustomizedSolverCapability, ExactProblemKey,
    IlpSolverCapability, RegistryBuildError, SolverCapabilities,
};
pub use resolver::{solve, SolveOutcome, SolveResult, SolverExecution, SolverRequest};

pub use ilp::{ILPSolveError, ILPSolver};

/// Failure while solving a valid problem instance.
#[derive(Debug, thiserror::Error)]
pub enum SolveError {
    #[error("configuration evaluation failed: {0}")]
    Evaluation(#[from] crate::traits::EvaluationError),
    #[error("aggregate combination failed: {0}")]
    Aggregation(#[from] crate::types::AggregationError),
    #[error("no reference-solver registration for {0}")]
    MissingRegistration(String),
    #[error("invalid reference-solver registration: {0}")]
    RegistrationTypeMismatch(String),
    #[error("brute-force search space cardinality exceeds usize for dimensions {0:?}")]
    SearchSpaceOverflow(Vec<usize>),
    #[error("solver capability registry is invalid: {0}")]
    InvalidRegistry(&'static RegistryBuildError),
    #[error("No ILP pipeline is registered for {0}")]
    MissingIlpCapability(String),
    #[error("No customized solver is registered for {0}")]
    MissingCustomizedCapability(String),
    #[error("ILP solver failed for {problem}: {source}")]
    IlpSolve {
        problem: String,
        #[source]
        source: ILPSolveError,
    },
}
