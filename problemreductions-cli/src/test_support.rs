use crate::dispatch::{PathStep, ProblemJsonOutput, ReductionBundle};
use problemreductions::models::algebraic::{ObjectiveSense, ILP};
use problemreductions::registry::{
    CreateInputCodec, CreateInputInfo, FieldInfo, ProblemSchemaEntry, VariantEntry,
};
use problemreductions::rules::registry::{ReductionEntry, ReductionSizeDeclarations};
use problemreductions::rules::{AggregateReductionResult, ReductionAutoCast};
use problemreductions::solvers::{BruteForce, Solver};
use problemreductions::traits::Problem;
use problemreductions::types::{Extremum, ProblemSize, Sum};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

pub(crate) const AGGREGATE_SOURCE_NAME: &str = "CliTestAggregateValueSource";
pub(crate) const AGGREGATE_TARGET_NAME: &str = "CliTestAggregateValueTarget";

const AGGREGATE_SOURCE_INPUTS: &[CreateInputInfo] = &[CreateInputInfo {
    name: "values",
    type_name: "Vec<i64>",
    description: "Values included by selected configuration bits",
    required: true,
    codec: CreateInputCodec::CommaSeparated,
}];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AggregateValueSource {
    values: Vec<i64>,
}

impl AggregateValueSource {
    pub(crate) fn sample() -> Self {
        Self {
            values: vec![2, 5, 7],
        }
    }
}

impl Problem for AggregateValueSource {
    const NAME: &'static str = AGGREGATE_SOURCE_NAME;
    type Value = Sum<i64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.values.len()]
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<Self::Value, problemreductions::traits::EvaluationError> {
        Ok({
            let total = self
                .values
                .iter()
                .zip(config.iter().copied())
                .filter_map(|(value, bit)| (bit == 1).then_some(*value))
                .sum();
            Sum(total)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AggregateValueTarget {
    base: i64,
}

impl AggregateValueTarget {
    pub(crate) fn sample() -> Self {
        Self { base: 11 }
    }
}

impl Problem for AggregateValueTarget {
    const NAME: &'static str = AGGREGATE_TARGET_NAME;
    type Value = Sum<i64>;

    fn dims(&self) -> Vec<usize> {
        vec![2]
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<Self::Value, problemreductions::traits::EvaluationError> {
        Ok(Sum(self.base + config.iter().sum::<usize>() as i64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

#[derive(Debug, Clone)]
struct AggregateValueToIlpReduction {
    target: ILP<bool>,
}

impl AggregateReductionResult for AggregateValueToIlpReduction {
    type Source = AggregateValueSource;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, _target_value: Extremum<f64>) -> Sum<i64> {
        Sum(0)
    }
}

fn solve_value<P>(any: &dyn Any) -> Result<String, problemreductions::solvers::SolveError>
where
    P: Problem + Serialize + 'static,
    P::Value: problemreductions::types::Aggregate + std::fmt::Display,
{
    let problem = any
        .downcast_ref::<P>()
        .expect("test solve_value downcast failed");
    let solver = BruteForce::new();
    Ok(problemreductions::registry::format_metric(
        &solver.solve(problem)?,
    ))
}

fn solve_witness<P>(
    any: &dyn Any,
) -> Result<Option<(Vec<usize>, String)>, problemreductions::solvers::SolveError>
where
    P: Problem + Serialize + 'static,
    P::Value: problemreductions::types::Aggregate + std::fmt::Display,
{
    let problem = any
        .downcast_ref::<P>()
        .expect("test solve_witness downcast failed");
    let solver = BruteForce::new();
    let Some(config) = solver.find_witness(problem)? else {
        return Ok(None);
    };
    let evaluation = problemreductions::registry::format_metric(&problem.evaluate(&config)?);
    Ok(Some((config, evaluation)))
}

problemreductions::inventory::submit! {
    ProblemSchemaEntry {
        name: AggregateValueSource::NAME,
        display_name: "CLI test aggregate value source",
        aliases: &[],
        dimensions: &[],
        category: problemreductions::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test-only dynamically discovered construction model",
        fields: &[FieldInfo {
            name: "values",
            type_name: "Vec<i64>",
            description: "Values included by selected configuration bits",
        }],
    }
}

problemreductions::inventory::submit! {
    ProblemSchemaEntry {
        name: AggregateValueTarget::NAME,
        display_name: "CLI test aggregate value target",
        aliases: &[],
        dimensions: &[],
        category: problemreductions::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test-only aggregate reduction target",
        fields: &[FieldInfo {
            name: "base",
            type_name: "i64",
            description: "Base aggregate value",
        }],
    }
}

problemreductions::inventory::submit! {
    VariantEntry {
        name: AggregateValueSource::NAME,
        variant_fn: AggregateValueSource::variant,
        complexity: "2^num_values",
        complexity_eval_fn: |_| 1.0,
        is_default: true,
        aliases: &[],
        create_inputs: Some(AGGREGATE_SOURCE_INPUTS),
        construct_fn: |data| {
            problemreductions::registry::validate_create_inputs(AGGREGATE_SOURCE_INPUTS, &data)?;
            let problem: AggregateValueSource = serde_json::from_value(data)
                .map_err(|error| problemreductions::registry::ConstructionError::InvalidInput(error.to_string()))?;
            Ok(Box::new(problem))
        },
        random: None,
        factory: |data| {
            let problem: AggregateValueSource = serde_json::from_value(data)?;
            Ok(Box::new(problem))
        },
        serialize_fn: |any| {
            let problem = any.downcast_ref::<AggregateValueSource>()?;
            Some(serde_json::to_value(problem).expect("serialize AggregateValueSource failed"))
        },
        solve_value_fn: solve_value::<AggregateValueSource>,
        solve_witness_fn: solve_witness::<AggregateValueSource>,
    }
}

problemreductions::inventory::submit! {
    VariantEntry {
        name: AggregateValueTarget::NAME,
        variant_fn: AggregateValueTarget::variant,
        complexity: "2",
        complexity_eval_fn: |_| 1.0,
        is_default: true,
        aliases: &[],
        create_inputs: None,
        construct_fn: |data| {
            let problem: AggregateValueTarget = serde_json::from_value(data)
                .map_err(|error| problemreductions::registry::ConstructionError::InvalidInput(error.to_string()))?;
            Ok(Box::new(problem))
        },
        random: None,
        factory: |data| {
            let problem: AggregateValueTarget = serde_json::from_value(data)?;
            Ok(Box::new(problem))
        },
        serialize_fn: |any| {
            let problem = any.downcast_ref::<AggregateValueTarget>()?;
            Some(serde_json::to_value(problem).expect("serialize AggregateValueTarget failed"))
        },
        solve_value_fn: solve_value::<AggregateValueTarget>,
        solve_witness_fn: solve_witness::<AggregateValueTarget>,
    }
}

problemreductions::inventory::submit! {
    ReductionEntry {
        source_name: AggregateValueSource::NAME,
        target_name: AggregateValueTarget::NAME,
        source_variant_fn: AggregateValueSource::variant,
        target_variant_fn: AggregateValueTarget::variant,
        size_declarations_fn: || ReductionSizeDeclarations {
            relation: None,
            fields: vec![],
            unavailable: vec![problemreductions::rules::registry::UnavailableSizeField {
                field: "size",
                reason: "the synthetic aggregate target has no size model",
            }],
        },
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: Some(|any: &dyn Any| {
            let source = any
                .downcast_ref::<AggregateValueSource>()
                .expect("aggregate reduction downcast failed");
            Ok(Box::new(ReductionAutoCast::<AggregateValueSource, AggregateValueTarget>::new(
                AggregateValueTarget {
                    base: source.values.iter().sum(),
                },
            )))
        }),
        turing: false,
        source_size_measure_fn: |_| ProblemSize::new(vec![]),
        target_size_measure_fn: |_| ProblemSize::new(vec![]),
    }
}

problemreductions::inventory::submit! {
    ReductionEntry {
        source_name: AggregateValueSource::NAME,
        target_name: ILP::<bool>::NAME,
        source_variant_fn: AggregateValueSource::variant,
        target_variant_fn: ILP::<bool>::variant,
        size_declarations_fn: || ReductionSizeDeclarations {
            relation: None,
            fields: vec![],
            unavailable: vec![problemreductions::rules::registry::UnavailableSizeField {
                field: "size",
                reason: "the synthetic aggregate target has no size model",
            }],
        },
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: Some(|any: &dyn Any| {
            let _source = any
                .downcast_ref::<AggregateValueSource>()
                .expect("aggregate ILP reduction downcast failed");
            Ok(Box::new(AggregateValueToIlpReduction {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
            }))
        }),
        turing: false,
        source_size_measure_fn: |_| ProblemSize::new(vec![]),
        target_size_measure_fn: |_| ProblemSize::new(vec![]),
    }
}

#[cfg_attr(not(feature = "mcp"), allow(dead_code))]
pub(crate) fn aggregate_problem_json() -> String {
    serde_json::to_string(&ProblemJsonOutput {
        problem_type: AggregateValueSource::NAME.to_string(),
        variant: BTreeMap::new(),
        data: serde_json::to_value(AggregateValueSource::sample()).unwrap(),
    })
    .unwrap()
}

pub(crate) fn aggregate_bundle() -> ReductionBundle {
    ReductionBundle {
        source: ProblemJsonOutput {
            problem_type: AggregateValueSource::NAME.to_string(),
            variant: BTreeMap::new(),
            data: serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        },
        target: ProblemJsonOutput {
            problem_type: AggregateValueTarget::NAME.to_string(),
            variant: BTreeMap::new(),
            data: serde_json::to_value(AggregateValueTarget::sample()).unwrap(),
        },
        path: vec![
            PathStep {
                name: AggregateValueSource::NAME.to_string(),
                variant: BTreeMap::new(),
            },
            PathStep {
                name: AggregateValueTarget::NAME.to_string(),
                variant: BTreeMap::new(),
            },
        ],
    }
}
