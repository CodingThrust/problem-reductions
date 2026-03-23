//! Core traits for problem reductions.

use crate::traits::Problem;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::Any;
use std::marker::PhantomData;

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
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize>;
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
/// let reduction = sat_problem.reduce_to();
/// let is_problem = reduction.target_problem();
///
/// // Solve and extract solutions
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(is_problem);
/// let sat_solutions: Vec<_> = solutions.iter()
///     .map(|s| reduction.extract_solution(s))
///     .collect();
/// ```
pub trait ReduceTo<T: Problem>: Problem {
    /// The reduction result type.
    type Result: ReductionResult<Source = Self, Target = T>;

    /// Reduce this problem to the target problem type.
    fn reduce_to(&self) -> Self::Result;
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
        target_value: <Self::Target as Problem>::Value,
    ) -> <Self::Source as Problem>::Value;
}

/// Trait for problems that can be reduced to target type T for aggregate-value
/// workflows.
pub trait ReduceToAggregate<T: Problem>: Problem {
    /// The reduction result type.
    type Result: AggregateReductionResult<Source = Self, Target = T>;

    /// Reduce this problem to the target problem type.
    fn reduce_to_aggregate(&self) -> Self::Result;
}

/// Generic reduction result for natural-edge (subtype) reductions.
///
/// Used when a problem on a specific graph type is trivially reducible to
/// the same problem on a more general graph type (e.g., `MIS<Triangular>` →
/// `MIS<SimpleGraph>`). The solution mapping is identity — vertex indices
/// are preserved.
#[derive(Debug, Clone)]
pub struct ReductionAutoCast<S: Problem, T: Problem> {
    target: T,
    _phantom: PhantomData<S>,
}

impl<S: Problem, T: Problem> ReductionAutoCast<S, T> {
    /// Create a new auto-cast reduction result.
    pub fn new(target: T) -> Self {
        Self {
            target,
            _phantom: PhantomData,
        }
    }
}

impl<S: Problem, T: Problem> ReductionResult for ReductionAutoCast<S, T> {
    type Source = S;
    type Target = T;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

impl<S: Problem, T: Problem<Value = S::Value>> AggregateReductionResult for ReductionAutoCast<S, T> {
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
    fn extract_solution_dyn(&self, target_solution: &[usize]) -> Vec<usize>;
}

impl<R: ReductionResult + 'static> DynReductionResult for R
where
    R::Target: 'static,
{
    fn target_problem_any(&self) -> &dyn Any {
        self.target_problem() as &dyn Any
    }
    fn extract_solution_dyn(&self, target_solution: &[usize]) -> Vec<usize> {
        self.extract_solution(target_solution)
    }
}

/// Type-erased aggregate reduction result for runtime-discovered paths.
pub trait DynAggregateReductionResult {
    /// Get the target problem as a type-erased reference.
    fn target_problem_any(&self) -> &dyn Any;
    /// Extract an aggregate value from target space to source space.
    fn extract_value_dyn(&self, target_value: serde_json::Value) -> serde_json::Value;
}

impl<R: AggregateReductionResult + 'static> DynAggregateReductionResult for R
where
    R::Target: 'static,
    <R::Target as Problem>::Value: Serialize + DeserializeOwned,
    <R::Source as Problem>::Value: Serialize,
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
}

#[cfg(test)]
#[path = "../unit_tests/rules/traits.rs"]
mod tests;
