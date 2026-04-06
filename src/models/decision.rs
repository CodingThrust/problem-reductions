//! Generic decision wrapper for optimization problems.

use crate::rules::{AggregateReductionResult, ReduceToAggregate};
use crate::traits::Problem;
use crate::types::{OptimizationValue, Or};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Metadata for concrete optimization problems that expose a decision wrapper.
pub trait DecisionProblemMeta: Problem
where
    Self::Value: OptimizationValue,
{
    /// Problem name used by the corresponding `Decision<Self>` variant.
    const DECISION_NAME: &'static str;
}

/// Register the decision problem name for a concrete optimization problem.
#[macro_export]
macro_rules! decision_problem_meta {
    ($inner:ty, $name:literal) => {
        impl $crate::models::decision::DecisionProblemMeta for $inner {
            const DECISION_NAME: &'static str = $name;
        }
    };
}

/// Decision version of an optimization problem with a fixed objective bound.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision<P: Problem>
where
    P::Value: OptimizationValue,
{
    inner: P,
    bound: <P::Value as OptimizationValue>::Inner,
}

impl<P: Problem> Decision<P>
where
    P::Value: OptimizationValue,
{
    /// Create a decision wrapper around `inner` with the provided bound.
    pub fn new(inner: P, bound: <P::Value as OptimizationValue>::Inner) -> Self {
        Self { inner, bound }
    }

    /// Borrow the wrapped optimization problem.
    pub fn inner(&self) -> &P {
        &self.inner
    }

    /// Borrow the decision bound.
    pub fn bound(&self) -> &<P::Value as OptimizationValue>::Inner {
        &self.bound
    }
}

impl<P> Problem for Decision<P>
where
    P: DecisionProblemMeta,
    P::Value: OptimizationValue,
{
    const NAME: &'static str = P::DECISION_NAME;
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        self.inner.dims()
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(<P::Value as OptimizationValue>::meets_bound(
            &self.inner.evaluate(config),
            &self.bound,
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        P::variant()
    }
}

/// Aggregate reduction result for `Decision<P> -> P`.
#[derive(Debug, Clone)]
pub struct DecisionToOptimizationResult<P>
where
    P: Problem,
    P::Value: OptimizationValue,
{
    target: P,
    bound: <P::Value as OptimizationValue>::Inner,
}

impl<P> AggregateReductionResult for DecisionToOptimizationResult<P>
where
    P: DecisionProblemMeta + 'static,
    P::Value: OptimizationValue + Serialize + DeserializeOwned,
{
    type Source = Decision<P>;
    type Target = P;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: P::Value) -> Or {
        Or(<P::Value as OptimizationValue>::meets_bound(
            &target_value,
            &self.bound,
        ))
    }
}

impl<P> ReduceToAggregate<P> for Decision<P>
where
    P: DecisionProblemMeta + Clone + 'static,
    P::Value: OptimizationValue + Serialize + DeserializeOwned,
{
    type Result = DecisionToOptimizationResult<P>;

    fn reduce_to_aggregate(&self) -> Self::Result {
        DecisionToOptimizationResult {
            target: self.inner.clone(),
            bound: self.bound.clone(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/models/decision.rs"]
mod tests;
