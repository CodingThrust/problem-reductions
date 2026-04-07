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

/// Register the boilerplate inventory entries for a concrete `Decision<P>` variant.
///
/// The `size_getters` parameter defines problem-specific size fields as
/// `(name, getter_on_inner)` pairs, e.g., `[("num_vertices", num_vertices), ("num_edges", num_edges)]`.
/// These are used for overhead expressions and `ProblemSize` extraction.
/// The macro automatically adds a `("k", k)` entry for `source_size_fn` on the Decision side.
///
/// Callers must define inherent methods on `Decision<Inner>` (delegating to `self.inner()`)
/// and a `k()` method (from `self.bound()`) **before** invoking this macro.
#[macro_export]
macro_rules! register_decision_variant {
    (
        $inner:ty,
        $name:literal,
        $complexity:literal,
        $aliases:expr,
        $description:literal,
        dims: [$($dim:expr),* $(,)?],
        fields: [$($field:expr),* $(,)?],
        size_getters: [$(($sg_name:literal, $sg_method:ident)),* $(,)?]
    ) => {
        $crate::declare_variants! {
            default $crate::models::decision::Decision<$inner> => $complexity,
        }

        $crate::inventory::submit! {
            $crate::registry::ProblemSchemaEntry {
                name: $name,
                display_name: $crate::register_decision_variant!(@display_name $name),
                aliases: $aliases,
                dimensions: &[$($dim),*],
                module_path: module_path!(),
                description: $description,
                fields: &[$($field),*],
            }
        }

        $crate::inventory::submit! {
            $crate::rules::ReductionEntry {
                source_name: $name,
                target_name: <$inner as $crate::traits::Problem>::NAME,
                source_variant_fn: <$crate::models::decision::Decision<$inner> as $crate::traits::Problem>::variant,
                target_variant_fn: <$inner as $crate::traits::Problem>::variant,
                overhead_fn: || $crate::rules::ReductionOverhead::identity(&[$($sg_name),*]),
                module_path: module_path!(),
                reduce_fn: None,
                reduce_aggregate_fn: Some(|any| {
                    let source = any
                        .downcast_ref::<$crate::models::decision::Decision<$inner>>()
                        .expect(concat!($name, " aggregate reduction source type mismatch"));
                    Box::new(
                        <$crate::models::decision::Decision<$inner> as $crate::rules::ReduceToAggregate<$inner>>::reduce_to_aggregate(source),
                    )
                }),
                capabilities: $crate::rules::EdgeCapabilities::aggregate_only(),
                overhead_eval_fn: |any| {
                    let source = any
                        .downcast_ref::<$crate::models::decision::Decision<$inner>>()
                        .expect(concat!($name, " overhead source type mismatch"));
                    $crate::types::ProblemSize::new(vec![
                        $(($sg_name, source.$sg_method())),*
                    ])
                },
                source_size_fn: |any| {
                    let source = any
                        .downcast_ref::<$crate::models::decision::Decision<$inner>>()
                        .expect(concat!($name, " size source type mismatch"));
                    $crate::types::ProblemSize::new(vec![
                        $(($sg_name, source.$sg_method()),)*
                        ("k", source.k()),
                    ])
                },
            }
        }

        // Reverse edge: P → Decision<P> (Turing/multi-query reduction via binary search)
        $crate::inventory::submit! {
            $crate::rules::ReductionEntry {
                source_name: <$inner as $crate::traits::Problem>::NAME,
                target_name: $name,
                source_variant_fn: <$inner as $crate::traits::Problem>::variant,
                target_variant_fn: <$crate::models::decision::Decision<$inner> as $crate::traits::Problem>::variant,
                overhead_fn: || $crate::rules::ReductionOverhead::identity(&[$($sg_name),*]),
                module_path: module_path!(),
                reduce_fn: None,
                reduce_aggregate_fn: None,
                capabilities: $crate::rules::EdgeCapabilities::turing(),
                overhead_eval_fn: |any| {
                    let source = any
                        .downcast_ref::<$inner>()
                        .expect(concat!($name, " turing overhead source type mismatch"));
                    $crate::types::ProblemSize::new(vec![
                        $(($sg_name, source.$sg_method())),*
                    ])
                },
                source_size_fn: |any| {
                    let source = any
                        .downcast_ref::<$inner>()
                        .expect(concat!($name, " turing size source type mismatch"));
                    $crate::types::ProblemSize::new(vec![
                        $(($sg_name, source.$sg_method())),*
                    ])
                },
            }
        }
    };
    (@display_name "DecisionMinimumVertexCover") => {
        "Decision Minimum Vertex Cover"
    };
    (@display_name "DecisionMinimumDominatingSet") => {
        "Decision Minimum Dominating Set"
    };
    (@display_name "DecisionMaximumIndependentSet") => {
        "Decision Maximum Independent Set"
    };
    (@display_name $name:literal) => {
        $name
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
