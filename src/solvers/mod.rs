//! Solvers for computational problems.

mod brute_force;
pub(crate) mod customized;
pub mod decision_search;
mod outcome;
mod pipelines;
mod registry;
mod resolver;
pub(crate) use outcome::{
    check_reported_evaluation, downcast_outcome, erase_outcome, outcome_to_json, ErasedOutcome,
};
pub use outcome::{ProblemOutcome, SolveOutcome};

pub mod ilp;

#[doc(hidden)]
pub use brute_force::BruteForceRegistration;
pub use brute_force::{cartesian_dimensions, BruteForce, BruteForceProblem, SolutionAggregate};
pub use registry::{
    brute_force_dimensions, solver_capabilities, CustomizedSolverCapability, ExactProblemKey,
    IlpSolverCapability, RegistryBuildError, SolverCapabilities,
};
pub use resolver::{solve, SolveResult, SolverExecution, SolverRequest};

pub use ilp::{ILPSolveError, ILPSolver};

/// Failure while solving a valid problem instance.
#[derive(Debug, thiserror::Error)]
pub enum SolveError {
    #[error(transparent)]
    Extraction(#[from] crate::rules::ExtractionError),
    #[error("configuration evaluation failed: {0}")]
    Evaluation(#[from] crate::traits::EvaluationError),
    #[error("aggregate combination failed: {0}")]
    Aggregation(#[from] crate::types::AggregationError),
    #[error("no reference-solver registration for {0}")]
    MissingRegistration(String),
    #[error("invalid reference-solver registration: {0}")]
    RegistrationTypeMismatch(String),
    #[error("cannot allocate solver storage: {0}")]
    Allocation(#[from] std::collections::TryReserveError),
    #[error("integer overflow while {0}")]
    IntegerOverflow(String),
    #[error("inexact integer-to-float conversion: {0}")]
    InexactFloatConversion(#[from] crate::types::ExactI64ToF64Error),
    #[error("non-finite floating-point result while {0}")]
    NonFiniteResult(String),
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

impl From<std::num::TryFromIntError> for SolveError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::IntegerOverflow(error.to_string())
    }
}
