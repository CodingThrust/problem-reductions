//! Core traits for problem reductions.

use crate::traits::Problem;
use serde::de::DeserializeOwned;
use serde::Serialize;
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
    #[error("{0}")]
    InvalidTargetSolution(String),
    #[error("{source_problem} -> {target_problem}: {message}")]
    Reduction {
        source_problem: &'static str,
        target_problem: &'static str,
        message: String,
    },
    #[error("target evaluation failed during extraction: {0}")]
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

/// Ask the target model to validate the structure of a typed solution.
pub(crate) fn validate_target_solution<P: Problem>(
    target: &P,
    solution: &P::Solution,
) -> ExtractionResult<()> {
    target.evaluate(solution)?;
    Ok(())
}

/// Result of reducing a source problem to a target problem.
///
/// This trait encapsulates the target problem and provides methods
/// to extract solutions back to the source problem space.
pub trait ReductionResult {
    /// The source problem type.
    type Source: Problem;
    /// The target problem type.
    type Target: Problem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Extract a solution from target problem space to source problem space.
    ///
    /// # Arguments
    /// * `target_solution` - A solution to the target problem
    ///
    /// # Returns
    /// The corresponding solution in the source problem space
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> ExtractionResult<<Self::Source as crate::traits::Problem>::Solution>;
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
/// // Solve and extract solutions
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(is_problem).unwrap();
/// let sat_solutions: Vec<_> = solutions.iter()
///     .map(|s| reduction.extract_solution(s))
///     .collect();
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

/// Result of reducing a source problem to a target problem for aggregate values.
///
/// Unlike [`ReductionResult`], this trait maps aggregate values back from target
/// space to source space instead of mapping witness configurations.
pub trait AggregateReductionResult {
    /// The source problem type.
    type Source: Problem;
    /// The target problem type.
    type Target: Problem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Extract an aggregate value from target problem space back to source space.
    fn extract_value(
        &self,
        target_value: <Self::Target as crate::traits::Problem>::Value,
    ) -> <Self::Source as crate::traits::Problem>::Value;
}

/// Trait for problems that can be reduced to target type T for aggregate-value
/// workflows.
pub trait ReduceToAggregate<T: Problem>: Problem {
    /// The reduction result type.
    type Result: AggregateReductionResult<Source = Self, Target = T>;

    /// Reduce this problem to the target problem type.
    fn reduce_to_aggregate(&self) -> Result<Self::Result, ReductionError>;
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
    S::Solution: Clone,
{
    type Source = S;
    type Target = T;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &T::Solution) -> ExtractionResult<S::Solution> {
        validate_target_solution(self.target_problem(), target_solution)?;
        Ok(target_solution.clone())
    }
}

impl<S: Problem, T: Problem<Value = S::Value>> AggregateReductionResult
    for VariantReductionResult<S, T>
{
    type Source = S;
    type Target = T;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: T::Value) -> S::Value {
        target_value
    }
}

/// Type-erased reduction result for runtime-discovered paths.
///
/// Implemented automatically for all `ReductionResult` types via blanket impl.
/// Used internally by `ReductionChain`.
pub trait DynReductionResult {
    /// Get the target problem as a type-erased reference.
    fn target_problem_any(&self) -> &dyn Any;
    /// Extract a solution from target space to source space.
    fn extract_solution_dyn(&self, target_solution: &dyn Any) -> ExtractionResult<Box<dyn Any>>;
    /// Serialize a source-space solution after the complete extraction chain.
    fn source_solution_json(
        &self,
        source_solution: &dyn Any,
    ) -> ExtractionResult<serde_json::Value>;
    /// Deserialize the concrete target witness at the dynamic boundary.
    fn target_solution_from_json(
        &self,
        target_solution: serde_json::Value,
    ) -> ExtractionResult<Box<dyn Any>>;
}

impl<R: ReductionResult + 'static> DynReductionResult for R
where
    R::Target: 'static,
    <R::Target as Problem>::Solution: 'static,
    <R::Target as Problem>::Solution: serde::de::DeserializeOwned,
    <R::Source as Problem>::Solution: 'static,
    <R::Source as Problem>::Solution: serde::Serialize,
{
    fn target_problem_any(&self) -> &dyn Any {
        self.target_problem() as &dyn Any
    }
    fn extract_solution_dyn(&self, target_solution: &dyn Any) -> ExtractionResult<Box<dyn Any>> {
        let target_solution = target_solution
            .downcast_ref::<<R::Target as Problem>::Solution>()
            .ok_or_else(|| {
                ExtractionError::invalid(format!(
                    "target solution type mismatch: expected {}",
                    std::any::type_name::<<R::Target as Problem>::Solution>()
                ))
            })?;
        self.extract_solution(target_solution)
            .map(|solution| Box::new(solution) as Box<dyn Any>)
            .map_err(|error| error.for_reduction::<R::Source, R::Target>())
    }

    fn source_solution_json(
        &self,
        source_solution: &dyn Any,
    ) -> ExtractionResult<serde_json::Value> {
        let source_solution = source_solution
            .downcast_ref::<<R::Source as Problem>::Solution>()
            .ok_or_else(|| ExtractionError::invalid("source solution type mismatch"))?;
        serde_json::to_value(source_solution).map_err(|error| {
            ExtractionError::invalid(format!("source solution serialization failed: {error}"))
        })
    }

    fn target_solution_from_json(
        &self,
        target_solution: serde_json::Value,
    ) -> ExtractionResult<Box<dyn Any>> {
        serde_json::from_value::<<R::Target as Problem>::Solution>(target_solution)
            .map(|solution| Box::new(solution) as Box<dyn Any>)
            .map_err(|error| {
                ExtractionError::invalid(format!("target solution deserialization failed: {error}"))
            })
    }
}

/// Type-erased aggregate reduction result for runtime-discovered paths.
pub trait DynAggregateReductionResult {
    /// Get the target problem as a type-erased reference.
    fn target_problem_any(&self) -> &dyn Any;
    /// Extract an aggregate value from target space to source space.
    fn extract_value_dyn(&self, target_value: serde_json::Value) -> serde_json::Value;
    /// Map the value of a target solution without erasing the source value's type.
    /// The caller must establish that the solution realizes the target aggregate
    /// before interpreting the result as the source aggregate.
    fn extract_value_from_solution_dyn(
        &self,
        target_solution: &dyn Any,
    ) -> ExtractionResult<Box<dyn Any>>;
}

impl<R: AggregateReductionResult + 'static> DynAggregateReductionResult for R
where
    R::Target: 'static,
    <R::Target as Problem>::Solution: 'static,
    <R::Target as Problem>::Value: Serialize + DeserializeOwned,
    <R::Source as Problem>::Value: Serialize + 'static,
{
    fn target_problem_any(&self) -> &dyn Any {
        self.target_problem() as &dyn Any
    }

    fn extract_value_dyn(&self, target_value: serde_json::Value) -> serde_json::Value {
        let target_value = serde_json::from_value(target_value)
            .expect("DynAggregateReductionResult target value deserialize failed");
        let source_value = self.extract_value(target_value);
        serde_json::to_value(source_value)
            .expect("DynAggregateReductionResult source value serialize failed")
    }

    fn extract_value_from_solution_dyn(
        &self,
        target_solution: &dyn Any,
    ) -> ExtractionResult<Box<dyn Any>> {
        let target_solution = target_solution
            .downcast_ref::<<R::Target as Problem>::Solution>()
            .ok_or_else(|| {
                ExtractionError::invalid(format!(
                    "target solution type mismatch: expected {}",
                    std::any::type_name::<<R::Target as Problem>::Solution>()
                ))
            })?;
        let target_value = self.target_problem().evaluate(target_solution)?;
        Ok(Box::new(self.extract_value(target_value)))
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/traits.rs"]
mod tests;
