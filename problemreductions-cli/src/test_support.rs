use crate::dispatch::{PathStep, ProblemJsonOutput, ReductionBundle};
use problemreductions::models::algebraic::{ObjectiveSense, ILP};
use problemreductions::registry::{
    CreateInputCodec, CreateInputInfo, FieldInfo, ProblemSchemaEntry, VariantEntry,
};
use problemreductions::rules::registry::{ReductionEntry, ReductionParameterDeclarations};
use problemreductions::rules::{AggregateReductionResult, VariantReductionResult};
use problemreductions::traits::Problem;
use problemreductions::types::{Aggregate, Extremum, Max, SolutionAggregate};
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

    fn num_values(&self) -> usize {
        self.values.len()
    }
}

impl Problem for AggregateValueSource {
    const NAME: &'static str = AGGREGATE_SOURCE_NAME;
    type Solution = Vec<bool>;
    type Value = Max<i64>;

    problemreductions::problem_parameters![("num_values", num_values)];

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Self::Value, problemreductions::traits::EvaluationError> {
        Ok({
            let total = self
                .values
                .iter()
                .zip(solution.iter().copied())
                .filter_map(|(value, selected)| selected.then_some(*value))
                .sum();
            Max(Some(total))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl problemreductions::solvers::BruteForceProblem for AggregateValueSource {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.values.len()]
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

    fn num_values(&self) -> usize {
        1
    }
}

impl Problem for AggregateValueTarget {
    const NAME: &'static str = AGGREGATE_TARGET_NAME;
    type Solution = Vec<bool>;
    type Value = Max<i64>;

    problemreductions::problem_parameters![("num_values", num_values)];

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Self::Value, problemreductions::traits::EvaluationError> {
        Ok(Max(Some(
            self.base + solution.iter().filter(|&&selected| selected).count() as i64,
        )))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl problemreductions::solvers::BruteForceProblem for AggregateValueTarget {
    fn dimensions(&self) -> Vec<usize> {
        vec![2]
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

    fn extract_value(&self, _target_value: Extremum<f64>) -> Max<i64> {
        Max(Some(0))
    }
}

fn decode_bits(indices: Vec<usize>) -> Vec<bool> {
    indices.into_iter().map(|index| index != 0).collect()
}

fn cartesian_indices(
    dimensions: Vec<usize>,
) -> Result<impl Iterator<Item = Vec<usize>>, problemreductions::solvers::SolveError> {
    let total = if dimensions.is_empty() {
        1
    } else if dimensions.contains(&0) {
        0
    } else {
        dimensions.iter().try_fold(1usize, |total, &dimension| {
            total.checked_mul(dimension).ok_or_else(|| {
                problemreductions::solvers::SolveError::SearchSpaceOverflow(dimensions.clone())
            })
        })?
    };
    Ok((0..total).map(move |mut index| {
        let mut coordinates = vec![0; dimensions.len()];
        for position in (0..dimensions.len()).rev() {
            coordinates[position] = index % dimensions[position];
            index /= dimensions[position];
        }
        coordinates
    }))
}

fn solve_cartesian<P>(problem: &P) -> Result<P::Value, problemreductions::solvers::SolveError>
where
    P: Problem<Solution = Vec<bool>> + problemreductions::solvers::BruteForceProblem,
    P::Value: Aggregate,
{
    let mut total = P::Value::identity();
    for indices in cartesian_indices(problem.dimensions())? {
        total = total.combine(problem.evaluate(&decode_bits(indices))?)?;
    }
    Ok(total)
}

fn solve_cartesian_solution<P>(
    problem: &P,
) -> Result<Option<P::Solution>, problemreductions::solvers::SolveError>
where
    P: Problem<Solution = Vec<bool>> + problemreductions::solvers::BruteForceProblem,
    P::Value: SolutionAggregate,
{
    let total = solve_cartesian(problem)?;
    for indices in cartesian_indices(problem.dimensions())? {
        let solution = decode_bits(indices);
        let value = problem.evaluate(&solution)?;
        if P::Value::contributes_to_solution(&value, &total) {
            return Ok(Some(solution));
        }
    }
    Ok(None)
}

fn solve_with_witnesses_cartesian<P>(
    problem: &P,
) -> Result<(P::Value, Vec<P::Solution>), problemreductions::solvers::SolveError>
where
    P: Problem<Solution = Vec<bool>> + problemreductions::solvers::BruteForceProblem,
    P::Value: SolutionAggregate,
{
    let total = solve_cartesian(problem)?;
    let mut witnesses = Vec::new();
    for indices in cartesian_indices(problem.dimensions())? {
        let solution = decode_bits(indices);
        let value = problem.evaluate(&solution)?;
        if P::Value::contributes_to_solution(&value, &total) {
            witnesses.push(solution);
        }
    }
    Ok((total, witnesses))
}

fn solve_dynamic<P>(
    any: &dyn Any,
) -> Result<Option<(serde_json::Value, String)>, problemreductions::solvers::SolveError>
where
    P: Problem<Solution = Vec<bool>>
        + problemreductions::solvers::BruteForceProblem
        + Serialize
        + 'static,
    P::Value: SolutionAggregate + std::fmt::Display,
{
    let problem = any.downcast_ref::<P>().expect("test solve downcast failed");
    let Some(solution) = solve_cartesian_solution(problem)? else {
        return Ok(None);
    };
    let evaluation = problemreductions::registry::format_metric(&problem.evaluate(&solution)?);
    Ok(Some((
        serde_json::to_value(solution).expect("test witness serialization failed"),
        evaluation,
    )))
}

fn solve_typed<P>(
    any: &dyn Any,
) -> Result<Option<Box<dyn Any>>, problemreductions::solvers::SolveError>
where
    P: Problem<Solution = Vec<bool>> + problemreductions::solvers::BruteForceProblem + 'static,
    P::Value: SolutionAggregate + 'static,
{
    let problem = any
        .downcast_ref::<P>()
        .expect("test typed solve downcast failed");
    Ok(solve_cartesian_solution(problem)?.map(|solution| Box::new(solution) as Box<dyn Any>))
}

fn solve_typed_with_witnesses<P>(
    any: &dyn Any,
) -> Result<Box<dyn Any>, problemreductions::solvers::SolveError>
where
    P: Problem<Solution = Vec<bool>> + problemreductions::solvers::BruteForceProblem + 'static,
    P::Value: SolutionAggregate + 'static,
{
    let problem = any
        .downcast_ref::<P>()
        .expect("test typed solve with witnesses downcast failed");
    Ok(Box::new(solve_with_witnesses_cartesian(problem)?))
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
        parameter_names_fn: AggregateValueSource::parameter_names,
        parameter_measure_fn: |any| {
            any.downcast_ref::<AggregateValueSource>()
                .expect("AggregateValueSource size type mismatch")
                .parameters()
        },
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
    }
}

problemreductions::inventory::submit! {
    problemreductions::solvers::BruteForceRegistration {
        source_name: AggregateValueSource::NAME,
        source_variant_fn: AggregateValueSource::variant,
        dimensions_fn: |any| {
            let problem = any
                .downcast_ref::<AggregateValueSource>()
                .expect("AggregateValueSource brute-force dimensions type mismatch");
            problemreductions::solvers::BruteForceProblem::dimensions(problem)
        },
        solve_fn: solve_dynamic::<AggregateValueSource>,
        solve_typed_fn: solve_typed::<AggregateValueSource>,
        solve_typed_with_witnesses_fn: solve_typed_with_witnesses::<AggregateValueSource>,
    }
}

problemreductions::inventory::submit! {
    VariantEntry {
        name: AggregateValueTarget::NAME,
        variant_fn: AggregateValueTarget::variant,
        complexity: "2",
        complexity_eval_fn: |_| 1.0,
        parameter_names_fn: AggregateValueTarget::parameter_names,
        parameter_measure_fn: |any| {
            any.downcast_ref::<AggregateValueTarget>()
                .expect("AggregateValueTarget size type mismatch")
                .parameters()
        },
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
    }
}

problemreductions::inventory::submit! {
    problemreductions::solvers::BruteForceRegistration {
        source_name: AggregateValueTarget::NAME,
        source_variant_fn: AggregateValueTarget::variant,
        dimensions_fn: |any| {
            let problem = any
                .downcast_ref::<AggregateValueTarget>()
                .expect("AggregateValueTarget brute-force dimensions type mismatch");
            problemreductions::solvers::BruteForceProblem::dimensions(problem)
        },
        solve_fn: solve_dynamic::<AggregateValueTarget>,
        solve_typed_fn: solve_typed::<AggregateValueTarget>,
        solve_typed_with_witnesses_fn: solve_typed_with_witnesses::<AggregateValueTarget>,
    }
}

problemreductions::inventory::submit! {
    ReductionEntry {
        source_name: AggregateValueSource::NAME,
        target_name: AggregateValueTarget::NAME,
        source_variant_fn: AggregateValueSource::variant,
        target_variant_fn: AggregateValueTarget::variant,
        parameter_declarations_fn: || ReductionParameterDeclarations {
            relation: None,
            fields: vec![],
            unavailable: vec![problemreductions::rules::registry::UnavailableParameterField {
                field: "num_values",
                reason: "the synthetic aggregate target has no parameter model",
            }],
        },
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: Some(|any: &dyn Any| {
            let source = any
                .downcast_ref::<AggregateValueSource>()
                .expect("aggregate reduction downcast failed");
            Ok(Box::new(VariantReductionResult::<AggregateValueSource, AggregateValueTarget>::new(
                AggregateValueTarget {
                    base: source.values.iter().sum(),
                },
            )))
        }),
        turing: false,
    }
}

problemreductions::inventory::submit! {
    ReductionEntry {
        source_name: AggregateValueSource::NAME,
        target_name: ILP::<bool>::NAME,
        source_variant_fn: AggregateValueSource::variant,
        target_variant_fn: ILP::<bool>::variant,
        parameter_declarations_fn: || ReductionParameterDeclarations {
            relation: None,
            fields: vec![],
            unavailable: vec![
                problemreductions::rules::registry::UnavailableParameterField {
                    field: "num_vars",
                    reason: "the synthetic aggregate-to-ILP reduction has no parameter model",
                },
                problemreductions::rules::registry::UnavailableParameterField {
                    field: "num_constraints",
                    reason: "the synthetic aggregate-to-ILP reduction has no parameter model",
                },
                problemreductions::rules::registry::UnavailableParameterField {
                    field: "num_nonzeros",
                    reason: "the synthetic aggregate-to-ILP reduction has no parameter model",
                },
            ],
        },
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: Some(|any: &dyn Any| {
            let _source = any
                .downcast_ref::<AggregateValueSource>()
                .expect("aggregate ILP reduction downcast failed");
            Ok(Box::new(AggregateValueToIlpReduction {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize)
                    .expect("empty ILP is valid"),
            }))
        }),
        turing: false,
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
