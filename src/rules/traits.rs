//! Core traits for problem reductions.

use crate::solvers::{ProblemOutcome, SolveOutcome};
use crate::traits::Problem;
use std::any::Any;
use std::marker::PhantomData;

/// Failure to construct a target instance for a registered reduction edge.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReductionError {
    #[error("{source_problem} -> {target_problem}: target construction failed: {cause}")]
    Construction {
        source_problem: &'static str,
        target_problem: &'static str,
        #[source]
        cause: crate::registry::ConstructionError,
    },
    #[error("{source_problem} -> {target_problem}: integer overflow while {operation}")]
    IntegerOverflow {
        source_problem: &'static str,
        target_problem: &'static str,
        operation: String,
    },
    #[error("{source_problem} -> {target_problem}: non-finite value while {operation}")]
    NonFiniteResult {
        source_problem: &'static str,
        target_problem: &'static str,
        operation: String,
    },
    #[error("{source_problem} -> {target_problem}: {cause}")]
    InexactFloatConversion {
        source_problem: &'static str,
        target_problem: &'static str,
        #[source]
        cause: crate::types::ExactI64ToF64Error,
    },
    #[error("{source_problem} -> {target_problem}: {message}")]
    InvalidTarget {
        source_problem: &'static str,
        target_problem: &'static str,
        message: String,
    },
    #[error(
        "{source_problem} -> {target_problem}: reduction executor expected source type `{expected}`"
    )]
    SourceTypeMismatch {
        source_problem: &'static str,
        target_problem: &'static str,
        expected: &'static str,
    },
}

impl ReductionError {
    pub(crate) fn for_reduction<S: Problem, T: Problem>(self) -> Self {
        match self {
            Self::Construction { cause, .. } => Self::construction::<S, T>(cause),
            Self::IntegerOverflow { operation, .. } => Self::integer_overflow::<S, T>(operation),
            Self::NonFiniteResult { operation, .. } => Self::non_finite_result::<S, T>(operation),
            Self::InexactFloatConversion { cause, .. } => {
                Self::inexact_float_conversion::<S, T>(cause)
            }
            Self::InvalidTarget { message, .. } => Self::invalid_target::<S, T>(message),
            Self::SourceTypeMismatch { expected, .. } => Self::SourceTypeMismatch {
                source_problem: S::NAME,
                target_problem: T::NAME,
                expected,
            },
        }
    }

    /// Report that a type-erased executor received the wrong source problem type.
    pub fn source_type_mismatch<S: Problem, T: Problem>() -> Self {
        Self::SourceTypeMismatch {
            source_problem: S::NAME,
            target_problem: T::NAME,
            expected: std::any::type_name::<S>(),
        }
    }

    /// Report integer overflow while constructing a reduction target.
    pub fn integer_overflow<S: Problem, T: Problem>(operation: impl Into<String>) -> Self {
        Self::IntegerOverflow {
            source_problem: S::NAME,
            target_problem: T::NAME,
            operation: operation.into(),
        }
    }

    /// Report that an exact integer cannot be represented in a floating-point target field.
    pub fn inexact_float_conversion<S: Problem, T: Problem>(
        cause: crate::types::ExactI64ToF64Error,
    ) -> Self {
        Self::InexactFloatConversion {
            source_problem: S::NAME,
            target_problem: T::NAME,
            cause,
        }
    }

    /// Report non-finite arithmetic while constructing a reduction target.
    pub fn non_finite_result<S: Problem, T: Problem>(operation: impl Into<String>) -> Self {
        Self::NonFiniteResult {
            source_problem: S::NAME,
            target_problem: T::NAME,
            operation: operation.into(),
        }
    }

    /// Report that derived data cannot form a valid reduction target.
    pub fn invalid_target<S: Problem, T: Problem>(message: impl Into<String>) -> Self {
        Self::InvalidTarget {
            source_problem: S::NAME,
            target_problem: T::NAME,
            message: message.into(),
        }
    }

    /// Preserve a target constructor's validation error with edge context.
    pub fn construction<S: Problem, T: Problem>(cause: crate::registry::ConstructionError) -> Self {
        Self::Construction {
            source_problem: S::NAME,
            target_problem: T::NAME,
            cause,
        }
    }
}

/// Failure to map a target witness back into the source configuration space.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ExtractionError {
    #[error("the target result does not establish the conditions required for source recovery")]
    InsufficientSolutionQuality,
    #[error("{0}")]
    InvalidTargetSolution(String),
    #[error("{source_problem} -> {target_problem}: {message}")]
    Reduction {
        source_problem: &'static str,
        target_problem: &'static str,
        message: String,
    },
    #[error("problem evaluation failed during recovery: {0}")]
    Evaluation(#[from] crate::traits::EvaluationError),
}

impl ExtractionError {
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::InvalidTargetSolution(message.into())
    }

    fn for_reduction<S: Problem, T: Problem>(self) -> Self {
        match self {
            Self::InvalidTargetSolution(message) => Self::Reduction {
                source_problem: S::NAME,
                target_problem: T::NAME,
                message,
            },
            error => error,
        }
    }
}

pub type ExtractionResult<T> = std::result::Result<T, ExtractionError>;

/// Result of reducing a source problem to a target problem.
///
/// Stores the target and recovers complete source results using the executed mapping.
pub trait ReductionResult {
    /// The source problem type.
    type Source: Problem;
    /// The target problem type.
    type Target: Problem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Recover the complete source result using this execution's mathematical relation.
    /// `source` must be the instance used to construct this reduction result.
    /// Optimal results must preserve optimality or prove source infeasibility.
    /// Feasible incumbents may establish only what the rule proves; insufficient
    /// witness quality is an error, never evidence of source infeasibility.
    fn recover_result(
        &self,
        source: &Self::Source,
        target: crate::solvers::ProblemOutcome<Self::Target>,
    ) -> ExtractionResult<crate::solvers::ProblemOutcome<Self::Source>>;
}

/// Recover using a rule that preserves optimality, feasibility, and infeasibility.
///
/// The caller must establish all three implications for its mathematical mapping.
/// Rules requiring optimum thresholds or rejecting feasible incumbents must instead
/// interpret those outcomes in their own `recover_result` implementation.
/// Source evaluation errors propagate; they never establish source infeasibility.
pub(super) fn recover_preserving_status<P: Problem, S, V>(
    source: &P,
    target: SolveOutcome<S, V>,
    map_solution: impl FnOnce(&S) -> ExtractionResult<P::Solution>,
) -> ExtractionResult<ProblemOutcome<P>> {
    match target {
        SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
        SolveOutcome::Optimal { solution, .. } => {
            Ok(SolveOutcome::optimal(source, map_solution(&solution)?)?)
        }
        SolveOutcome::Feasible { solution, .. } => {
            Ok(SolveOutcome::feasible(source, map_solution(&solution)?)?)
        }
    }
}

/// Trait for problems that can be reduced to target type T.
///
/// # Example
/// ```text
/// // Example showing reduction workflow
/// use problemreductions::prelude::*;
/// use problemreductions::rules::ReduceTo;
///
/// let sat_problem: Satisfiability = Satisfiability::new(
///     3,  // 3 variables
///     vec![
///         CNFClause::new(vec![0, 1]),     // (x0 OR x1)
///         CNFClause::new(vec![1, 2]),     // (x1 OR x2)
///     ]
/// );
///
/// // Reduce to Independent Set
/// let reduction = sat_problem.reduce_to().expect("reduction should succeed");
/// let is_problem = reduction.target_problem();
///
/// // Solve the target and recover its complete source result.
/// let solution = BruteForce::new().solve(is_problem)?.unwrap();
/// let target_result = SolveOutcome::optimal(is_problem, solution)?;
/// let source_result = reduction.recover_result(&sat_problem, target_result)?;
/// ```
pub trait ReduceTo<T: Problem>: Problem {
    /// The reduction result type.
    type Result: ReductionResult<Source = Self, Target = T>;

    /// Attach this reduction edge to a target-construction failure.
    fn target_construction(error: crate::registry::ConstructionError) -> ReductionError
    where
        Self: Sized,
    {
        ReductionError::construction::<Self, T>(error)
    }

    /// Convert a structural count used by the target's exact integer algebra.
    fn exact_i64(value: usize, operation: impl Into<String>) -> Result<i64, ReductionError>
    where
        Self: Sized,
    {
        i64::try_from(value).map_err(|_| ReductionError::integer_overflow::<Self, T>(operation))
    }

    /// Reduce this problem to the target problem type.
    fn reduce_to(&self) -> Result<Self::Result, ReductionError>;
}

/// Reduction result for an explicit conversion between variants of one model.
///
/// The target witness is also the source witness.
#[derive(Debug, Clone)]
pub struct VariantReductionResult<S: Problem, T: Problem> {
    target: T,
    _phantom: PhantomData<S>,
}

impl<S: Problem, T: Problem> VariantReductionResult<S, T> {
    /// Store the constructed target variant.
    pub fn new(target: T) -> Self {
        Self {
            target,
            _phantom: PhantomData,
        }
    }
}

impl<S, T> ReductionResult for VariantReductionResult<S, T>
where
    S: Problem,
    T: Problem<Solution = S::Solution>,
{
    type Source = S;
    type Target = T;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &S,
        target: crate::solvers::ProblemOutcome<T>,
    ) -> ExtractionResult<crate::solvers::ProblemOutcome<S>> {
        use crate::solvers::SolveOutcome;
        Ok(match target {
            SolveOutcome::Optimal { solution, .. } => SolveOutcome::optimal(source, solution)?,
            SolveOutcome::Feasible { solution, .. } => SolveOutcome::feasible(source, solution)?,
            SolveOutcome::Infeasible => SolveOutcome::Infeasible,
        })
    }
}

/// Type erasure for executed reduction results. Mathematical recovery remains typed.
pub trait DynReductionResult {
    fn target_problem_any(&self) -> &dyn Any;
    fn recover_result_dyn(
        &self,
        source: &dyn Any,
        target: crate::solvers::ErasedOutcome,
    ) -> ExtractionResult<crate::solvers::ErasedOutcome>;
    /// Decode and evaluate once, returning the typed result and its canonical JSON.
    fn target_result_from_json(
        &self,
        target: crate::solvers::SolveOutcome,
    ) -> ExtractionResult<(crate::solvers::ErasedOutcome, crate::solvers::SolveOutcome)>;
    fn source_result_json(
        &self,
        source: crate::solvers::ErasedOutcome,
    ) -> ExtractionResult<crate::solvers::SolveOutcome>;
}

impl<R: ReductionResult + 'static> DynReductionResult for R
where
    R::Source: 'static,
    R::Target: 'static,
    <R::Target as Problem>::Solution: serde::de::DeserializeOwned + serde::Serialize + 'static,
    <R::Target as Problem>::Value: std::fmt::Display + 'static,
    <R::Source as Problem>::Solution: serde::Serialize + 'static,
    <R::Source as Problem>::Value: std::fmt::Display + 'static,
{
    fn target_problem_any(&self) -> &dyn Any {
        self.target_problem()
    }

    fn recover_result_dyn(
        &self,
        source: &dyn Any,
        target: crate::solvers::ErasedOutcome,
    ) -> ExtractionResult<crate::solvers::ErasedOutcome> {
        let source = source
            .downcast_ref::<R::Source>()
            .ok_or_else(|| ExtractionError::invalid("source problem type mismatch"))?;
        let target = crate::solvers::downcast_outcome(target)?;
        self.recover_result(source, target)
            .map(crate::solvers::erase_outcome)
            .map_err(ExtractionError::for_reduction::<R::Source, R::Target>)
    }

    fn target_result_from_json(
        &self,
        target: crate::solvers::SolveOutcome,
    ) -> ExtractionResult<(crate::solvers::ErasedOutcome, crate::solvers::SolveOutcome)> {
        use crate::solvers::SolveOutcome;
        // Numeric evaluation is model-owned, not parsed from a display string.
        let decode = |solution| {
            serde_json::from_value(solution).map_err(|error| {
                ExtractionError::invalid(format!("invalid solution JSON: {error}"))
            })
        };
        let target = match target {
            SolveOutcome::Optimal { solution, .. } => {
                SolveOutcome::optimal(self.target_problem(), decode(solution)?)?
            }
            SolveOutcome::Feasible { solution, .. } => {
                SolveOutcome::feasible(self.target_problem(), decode(solution)?)?
            }
            SolveOutcome::Infeasible => SolveOutcome::Infeasible,
        };
        let target_json = crate::solvers::outcome_to_json(&target)?;
        Ok((crate::solvers::erase_outcome(target), target_json))
    }

    fn source_result_json(
        &self,
        source: crate::solvers::ErasedOutcome,
    ) -> ExtractionResult<crate::solvers::SolveOutcome> {
        let source: crate::solvers::ProblemOutcome<R::Source> =
            crate::solvers::downcast_outcome(source)?;
        crate::solvers::outcome_to_json(&source)
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/traits.rs"]
mod tests;
