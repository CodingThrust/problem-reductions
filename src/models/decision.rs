//! Generic decision wrapper for optimization problems.

use crate::rules::{AggregateReductionResult, ReduceTo, ReduceToAggregate, ReductionResult};
use crate::traits::Problem;
use crate::types::{OptimizationValue, Or};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};

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
/// These are used for size expressions and `ProblemSize` extraction.
///
/// Callers must define inherent methods on `Decision<Inner>` (delegating to `self.inner()`)
/// before invoking this macro.
#[macro_export]
macro_rules! register_decision_variant {
    (
        $inner:ty,
        $name:literal,
        $complexity:literal,
        $aliases:expr,
        $description:literal,
        category: $category:expr,
        dims: [$($dim:expr),* $(,)?],
        fields: [$($field:expr),* $(,)?],
        size_getters: [$(($sg_name:literal, $sg_method:ident)),* $(,)?],
        decode: $decoder:expr
        $(, $random:ident)?
    ) => {
        impl $crate::registry::CreateSpec
            for $crate::models::decision::DecisionCreateSpec<$inner>
        {
            const FIELDS: &'static [$crate::registry::FieldInfo] = &[$($field),*];
            const INPUTS: &'static [$crate::registry::CreateInputInfo] = &[
                $($crate::registry::CreateInputInfo::from_field($field)),*
            ];
        }

        $crate::register_decision_variant!(@declare $inner, $complexity, $decoder $(, $random)?);

        $crate::inventory::submit! {
            $crate::registry::ProblemSchemaEntry {
                name: $name,
                display_name: $crate::register_decision_variant!(@display_name $name),
                aliases: $aliases,
                dimensions: &[$($dim),*],
                category: $category,
                module_path: module_path!(),
                description: $description,
                fields: &[$($field),*],
            }
        }

        // Decision<P> → P: both witness (identity config) and aggregate (solve + compare)
        $crate::inventory::submit! {
            $crate::rules::ReductionEntry {
                source_name: $name,
                target_name: <$inner as $crate::traits::Problem>::NAME,
                source_variant_fn: <$crate::models::decision::Decision<$inner> as $crate::traits::Problem>::variant,
                target_variant_fn: <$inner as $crate::traits::Problem>::variant,
                size_declarations_fn: || $crate::rules::registry::ReductionSizeDeclarations {
                    relation: Some($crate::size::SizeRelation::Exact),
                    fields: vec![$(($sg_name, $crate::expr::Expr::variable($sg_name))),*],
                    unavailable: vec![],
                },
                module_path: module_path!(),
                reduce_fn: Some(|any| {
                    let source = any
                        .downcast_ref::<$crate::models::decision::Decision<$inner>>()
                        .ok_or_else($crate::rules::ReductionError::source_type_mismatch::<
                            $crate::models::decision::Decision<$inner>,
                            $inner,
                        >)?;
                    let result =
                        <$crate::models::decision::Decision<$inner> as $crate::rules::ReduceTo<$inner>>::reduce_to(source)?;
                    Ok(Box::new(result))
                }),
                reduce_aggregate_fn: Some(|any| {
                    let source = any
                        .downcast_ref::<$crate::models::decision::Decision<$inner>>()
                        .ok_or_else($crate::rules::ReductionError::source_type_mismatch::<
                            $crate::models::decision::Decision<$inner>,
                            $inner,
                        >)?;
                    let result =
                        <$crate::models::decision::Decision<$inner> as $crate::rules::ReduceToAggregate<$inner>>::reduce_to_aggregate(source)?;
                    Ok(Box::new(result))
                }),
                turing: false,
            }
        }

        // Reverse edge: P → Decision<P> (Turing/multi-query reduction via binary search)
        $crate::inventory::submit! {
            $crate::rules::ReductionEntry {
                source_name: <$inner as $crate::traits::Problem>::NAME,
                target_name: $name,
                source_variant_fn: <$inner as $crate::traits::Problem>::variant,
                target_variant_fn: <$crate::models::decision::Decision<$inner> as $crate::traits::Problem>::variant,
                size_declarations_fn: || $crate::rules::registry::ReductionSizeDeclarations {
                    relation: Some($crate::size::SizeRelation::Exact),
                    fields: vec![$(($sg_name, $crate::expr::Expr::variable($sg_name))),*],
                    unavailable: vec![],
                },
                module_path: module_path!(),
                reduce_fn: None,
                reduce_aggregate_fn: None,
                turing: true,
            }
        }
    };

    (@declare $inner:ty, $complexity:literal, $decoder:expr, random) => {
        $crate::declare_variants! {
            default $crate::models::decision::Decision<$inner> => $complexity create $crate::models::decision::DecisionCreateSpec<$inner> random,
        }
        $crate::register_brute_force! {
            $crate::models::decision::Decision<$inner> decode $decoder,
        }
    };
    (@declare $inner:ty, $complexity:literal, $decoder:expr) => {
        $crate::declare_variants! {
            default $crate::models::decision::Decision<$inner> => $complexity create $crate::models::decision::DecisionCreateSpec<$inner>,
        }
        $crate::register_brute_force! {
            $crate::models::decision::Decision<$inner> decode $decoder,
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

/// Flat construction DTO used by [`register_decision_variant!`].
///
/// Persisted decision problems remain `{ "inner": ..., "bound": ... }`, while
/// construction inputs expose the inner problem's fields beside `bound`.
#[doc(hidden)]
pub struct DecisionCreateSpec<P>
where
    P: Problem,
    P::Value: OptimizationValue,
{
    inner: P,
    bound: <P::Value as OptimizationValue>::Inner,
}

impl<'de, P> Deserialize<'de> for DecisionCreateSpec<P>
where
    P: Problem + DeserializeOwned,
    P::Value: OptimizationValue,
    <P::Value as OptimizationValue>::Inner: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let mut inputs = value.as_object().cloned().ok_or_else(|| {
            serde::de::Error::custom("decision construction inputs must be an object")
        })?;
        let bound = inputs
            .remove("bound")
            .ok_or_else(|| serde::de::Error::missing_field("bound"))?;
        let inner = serde_json::from_value(serde_json::Value::Object(inputs))
            .map_err(serde::de::Error::custom)?;
        let bound = serde_json::from_value(bound).map_err(serde::de::Error::custom)?;
        Ok(Self { inner, bound })
    }
}

impl<P> From<DecisionCreateSpec<P>> for Decision<P>
where
    P: Problem,
    P::Value: OptimizationValue,
{
    fn from(spec: DecisionCreateSpec<P>) -> Self {
        Self::new(spec.inner, spec.bound)
    }
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
    type Solution = P::Solution;
    type Value = Or;

    fn size_parameter_names() -> &'static [&'static str] {
        P::size_parameter_names()
    }

    fn size(&self) -> crate::types::ProblemSize {
        self.inner.size()
    }

    fn evaluate(&self, config: &Self::Solution) -> Result<Or, crate::traits::EvaluationError> {
        Ok({
            Or(<P::Value as OptimizationValue>::meets_bound(
                &self.inner.evaluate(config)?,
                &self.bound,
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        P::variant()
    }
}

impl<P> crate::solvers::BruteForceProblem for Decision<P>
where
    P: DecisionProblemMeta + crate::solvers::BruteForceProblem,
    P::Value: OptimizationValue,
{
    fn dimensions(&self) -> Vec<usize> {
        self.inner.dimensions()
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

    fn reduce_to_aggregate(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(DecisionToOptimizationResult {
            target: self.inner.clone(),
            bound: self.bound.clone(),
        })
    }
}

/// Witness reduction result for `Decision<P> -> P`.
///
/// The configuration spaces are identical — a config that is optimal for
/// `P` and meets the bound is a valid `Decision<P>` witness. The
/// `extract_solution` is the identity function.
#[derive(Debug, Clone)]
pub struct DecisionToOptimizationWitnessResult<P>
where
    P: Problem,
    P::Value: OptimizationValue,
{
    target: P,
}

impl<P> ReductionResult for DecisionToOptimizationWitnessResult<P>
where
    P: DecisionProblemMeta + 'static,
    P::Solution: Clone,
    P::Value: OptimizationValue + Serialize + DeserializeOwned,
{
    type Source = Decision<P>;
    type Target = P;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.clone())
    }
}

impl<P> ReduceTo<P> for Decision<P>
where
    P: DecisionProblemMeta + Clone + 'static,
    P::Solution: Clone,
    P::Value: OptimizationValue + Serialize + DeserializeOwned,
{
    type Result = DecisionToOptimizationWitnessResult<P>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(DecisionToOptimizationWitnessResult {
            target: self.inner.clone(),
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/models/decision.rs"]
mod tests;
