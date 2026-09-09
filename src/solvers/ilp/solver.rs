//! ILP solver implementation using HiGHS.

use super::adapter::{HighsAdapter, IlpBackendError};
use crate::solvers::registry::solver_capability_registry;
use crate::solvers::ExactProblemKey;
use crate::traits::Problem;

/// A failure to produce an ILP solution optimal within backend numerical tolerances.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ILPSolveError {
    /// The constraints have no feasible assignment.
    #[error("the ILP is infeasible")]
    Infeasible,
    /// A target witness did not establish the source decision threshold.
    #[error(
        "the ILP witness does not meet the decision threshold for {0}; the decision is unresolved"
    )]
    UnresolvedDecision(String),
    /// The objective is unbounded.
    #[error("the ILP objective is unbounded")]
    Unbounded,
    /// The configured time limit was reached before optimality was proven.
    #[error("the ILP solver reached its time limit before proving optimality")]
    Timeout,
    /// The selected backend failed for another reason.
    #[error("the ILP backend failed: {0}")]
    BackendFailure(String),
    /// Type-erased dispatch received a value other than a supported ILP variant.
    #[error("the ILP backend requires bool/i64 variables and i64/f64 coefficients")]
    UnsupportedProblemType,
    /// No ILP pipeline is registered for the exact problem variant.
    #[error("no ILP pipeline is registered for {0}")]
    MissingPipeline(String),
    /// The solver capability registry is invalid.
    #[error("solver capability registry is invalid: {0}")]
    InvalidRegistry(String),
    /// A registered pipeline returned a solution for a different source type.
    #[error("registered ILP pipeline returned the wrong solution type for {0}")]
    PipelineTypeMismatch(String),
    /// HiGHS reported an optimal solution that is invalid after integer rounding.
    #[error("the ILP backend returned an invalid rounded solution: {0}")]
    InvalidSolution(String),
    /// An exact integer in the model cannot be transported through the f64 backend API.
    #[error("the ILP backend cannot represent an exact model integer: {0}")]
    InexactTransport(#[from] crate::types::ExactI64ToF64Error),
    /// A target witness could not be mapped back to the source problem.
    #[error(transparent)]
    Extraction(#[from] crate::rules::ExtractionError),
    /// A registered reduction could not construct its target instance.
    #[error(transparent)]
    Reduction(#[from] crate::rules::ReductionError),
}

// Keep adapter details out of the public error vocabulary.
impl From<IlpBackendError> for ILPSolveError {
    fn from(error: IlpBackendError) -> Self {
        match error {
            IlpBackendError::Infeasible => Self::Infeasible,
            IlpBackendError::Unbounded => Self::Unbounded,
            IlpBackendError::Timeout => Self::Timeout,
            IlpBackendError::BackendFailure(message) => Self::BackendFailure(message),
            IlpBackendError::InvalidSolution(message) => Self::InvalidSolution(message),
            IlpBackendError::InexactTransport(error) => Self::InexactTransport(error),
        }
    }
}

/// An ILP solver using the HiGHS backend.
///
/// Registered reductions map a source problem to its native `ILP<V, C>` terminal.
/// A shared adapter sends that ILP to HiGHS before source solution extraction.
/// Optimality and infeasibility are assessed within HiGHS numerical tolerances.
/// Zero MIP gaps do not make floating-point solving mathematically exact.
///
/// # Example
///
/// ```rust
/// use problemreductions::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
/// use problemreductions::solvers::ILPSolver;
///
/// // Create a simple binary ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
/// let ilp = ILP::<bool, f64>::new(
///     2,
///     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
///     vec![(0, 1.0), (1, 2.0)],
///     ObjectiveSense::Maximize,
/// )?;
///
/// let solver = ILPSolver::new();
/// let solution = solver.solve(&ilp)?;
/// println!("Solution: {:?}", solution);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct ILPSolver {
    /// Time limit in seconds (None = no limit).
    pub time_limit: Option<f64>,
}

impl ILPSolver {
    /// Create a new ILP solver with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an ILP solver with a time limit.
    pub fn with_time_limit(seconds: f64) -> Self {
        Self {
            time_limit: Some(seconds),
        }
    }

    /// Solve a problem through its registered ILP pipeline.
    ///
    /// Returns a classified error when the problem is infeasible, the time
    /// limit is reached, the pipeline is missing, or the backend fails.
    pub fn solve<P>(&self, problem: &P) -> Result<P::Solution, ILPSolveError>
    where
        P: Problem + 'static,
        P::Solution: 'static,
    {
        let key = ExactProblemKey::new(P::NAME, crate::export::variant_to_map(P::variant()));
        let registry = solver_capability_registry()
            .map_err(|error| ILPSolveError::InvalidRegistry(error.to_string()))?;
        let pipeline = registry
            .lookup(&key)
            .ilp
            .ok_or_else(|| ILPSolveError::MissingPipeline(key.label()))?;
        pipeline.solve_typed(problem, &HighsAdapter::new(self.time_limit))
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/solver.rs"]
mod tests;
