//! Shared example specification types and helper functions.
//!
//! These types describe canonical model and rule examples with metadata
//! that can be validated against the catalog and reduction registry.

use crate::export::{
    lookup_overhead, overhead_to_json, variant_to_map, ModelExample, ProblemSide, RuleExample,
    SampleEval, SolutionPair,
};
use crate::models::algebraic::{VariableDomain, ILP};
use crate::prelude::{OptimizationProblem, Problem, ReduceTo, ReductionResult};
use crate::rules::{PathCostFn, ReductionGraph};
use crate::solvers::{BruteForce, ILPSolver};
use crate::types::ProblemSize;
use serde::Serialize;

/// Specification for a canonical model example.
#[allow(dead_code)]
pub struct ModelExampleSpec {
    /// Unique example identifier.
    pub id: &'static str,
    /// Builder function that produces the full exported example.
    pub build: fn() -> ModelExample,
}

/// Specification for a canonical rule example.
#[allow(dead_code)]
pub struct RuleExampleSpec {
    /// Unique example identifier.
    pub id: &'static str,
    /// Builder function that produces the full exported example.
    pub build: fn() -> RuleExample,
}

// ---- Model example helpers ----

pub fn sample_eval<P>(problem: &P, config: Vec<usize>) -> SampleEval
where
    P: Problem,
    P::Metric: Serialize,
{
    let metric =
        serde_json::to_value(problem.evaluate(&config)).expect("Failed to serialize metric");
    SampleEval { config, metric }
}

pub fn optimization_example<P>(problem: P, samples: Vec<Vec<usize>>) -> ModelExample
where
    P: OptimizationProblem + Serialize,
    P::Metric: Serialize,
{
    let sample_evals = samples
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    let optimal = BruteForce::new()
        .find_all_best(&problem)
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    ModelExample::from_problem(&problem, sample_evals, optimal)
}

pub fn satisfaction_example<P>(problem: P, samples: Vec<Vec<usize>>) -> ModelExample
where
    P: Problem<Metric = bool> + Serialize,
{
    let sample_evals = samples
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    let satisfying = BruteForce::new()
        .find_all_satisfying(&problem)
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    ModelExample::from_problem(&problem, sample_evals, satisfying)
}

pub fn explicit_example<P>(
    problem: P,
    samples: Vec<Vec<usize>>,
    optimal_configs: Vec<Vec<usize>>,
) -> ModelExample
where
    P: Problem + Serialize,
    P::Metric: Serialize,
{
    let sample_evals = samples
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    let optimal = optimal_configs
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    ModelExample::from_problem(&problem, sample_evals, optimal)
}

// ---- Rule example helpers ----

pub fn assemble_rule_example<S, T>(
    source: &S,
    target: &T,
    overhead: crate::rules::ReductionOverhead,
    solutions: Vec<SolutionPair>,
) -> RuleExample
where
    S: Problem + Serialize,
    T: Problem + Serialize,
{
    RuleExample {
        source: ProblemSide::from_problem(source),
        target: ProblemSide::from_problem(target),
        overhead: overhead_to_json(&overhead),
        solutions,
    }
}

pub fn direct_overhead<S, T>() -> crate::rules::ReductionOverhead
where
    S: Problem,
    T: Problem,
{
    let source_variant = variant_to_map(S::variant());
    let target_variant = variant_to_map(T::variant());
    if let Some(oh) = lookup_overhead(S::NAME, &source_variant, T::NAME, &target_variant) {
        return oh;
    }
    let graph = ReductionGraph::new();
    let src = graph
        .default_variant_for(S::NAME)
        .unwrap_or_else(|| source_variant.clone());
    let tgt = graph
        .default_variant_for(T::NAME)
        .unwrap_or_else(|| target_variant.clone());
    lookup_overhead(S::NAME, &src, T::NAME, &tgt).unwrap_or_else(|| {
        panic!(
            "missing direct overhead for {} -> {} (tried exact {:?}->{:?} and default {:?}->{:?})",
            S::NAME,
            T::NAME,
            source_variant,
            target_variant,
            src,
            tgt
        )
    })
}

pub fn direct_best_example<S, T, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: OptimizationProblem + Serialize,
    T::Metric: Serialize,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<T>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions = BruteForce::new()
        .find_all_best(target)
        .into_iter()
        .filter_map(|target_config| {
            let source_config = reduction.extract_solution(&target_config);
            keep(&source, &source_config).then_some(SolutionPair {
                source_config,
                target_config,
            })
        })
        .collect();
    assemble_rule_example(&source, target, direct_overhead::<S, T>(), solutions)
}

pub fn direct_satisfying_example<S, T, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: Problem<Metric = bool> + Serialize,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<T>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions = BruteForce::new()
        .find_all_satisfying(target)
        .into_iter()
        .filter_map(|target_config| {
            let source_config = reduction.extract_solution(&target_config);
            keep(&source, &source_config).then_some(SolutionPair {
                source_config,
                target_config,
            })
        })
        .collect();
    assemble_rule_example(&source, target, direct_overhead::<S, T>(), solutions)
}

pub fn direct_ilp_example<S, V, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<ILP<V>>,
    ILP<V>: Serialize,
    V: VariableDomain,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<ILP<V>>::reduce_to(&source);
    let target = reduction.target_problem();
    let target_config = ILPSolver::new()
        .solve(target)
        .expect("canonical ILP target example should solve");
    let source_config = reduction.extract_solution(&target_config);
    let solutions = if keep(&source, &source_config) {
        vec![SolutionPair {
            source_config,
            target_config,
        }]
    } else {
        Vec::new()
    };
    assemble_rule_example(&source, target, direct_overhead::<S, ILP<V>>(), solutions)
}

pub fn path_best_example<S, T, C, Keep>(
    source: S,
    input_size: ProblemSize,
    cost: C,
    keep: Keep,
) -> RuleExample
where
    S: Problem + Serialize + 'static,
    T: OptimizationProblem + Serialize + 'static,
    T::Metric: Serialize,
    C: PathCostFn,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let graph = ReductionGraph::new();
    let source_variant = variant_to_map(S::variant());
    let target_variant = variant_to_map(T::variant());
    let path = graph
        .find_cheapest_path(
            S::NAME,
            &source_variant,
            T::NAME,
            &target_variant,
            &input_size,
            &cost,
        )
        .expect("canonical path example should exist");
    let chain = graph
        .reduce_along_path(&path, &source as &dyn std::any::Any)
        .expect("canonical path example should execute");
    let target = chain.target_problem::<T>();
    let solutions = BruteForce::new()
        .find_all_best(target)
        .into_iter()
        .filter_map(|target_config| {
            let source_config = chain.extract_solution(&target_config);
            keep(&source, &source_config).then_some(SolutionPair {
                source_config,
                target_config,
            })
        })
        .collect();
    assemble_rule_example(
        &source,
        target,
        graph.compose_path_overhead(&path),
        solutions,
    )
}

pub fn path_ilp_example<S, V, C, Keep>(
    source: S,
    input_size: ProblemSize,
    cost: C,
    keep: Keep,
) -> RuleExample
where
    S: Problem + Serialize + 'static,
    ILP<V>: Serialize + 'static,
    V: VariableDomain,
    C: PathCostFn,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let graph = ReductionGraph::new();
    let source_variant = variant_to_map(S::variant());
    let target_variant = variant_to_map(ILP::<V>::variant());
    let path = graph
        .find_cheapest_path(
            S::NAME,
            &source_variant,
            ILP::<V>::NAME,
            &target_variant,
            &input_size,
            &cost,
        )
        .expect("canonical ILP path example should exist");
    let chain = graph
        .reduce_along_path(&path, &source as &dyn std::any::Any)
        .expect("canonical ILP path example should execute");
    let target = chain.target_problem::<ILP<V>>();
    let target_config = ILPSolver::new()
        .solve(target)
        .expect("canonical ILP path target should solve");
    let source_config = chain.extract_solution(&target_config);
    let solutions = if keep(&source, &source_config) {
        vec![SolutionPair {
            source_config,
            target_config,
        }]
    } else {
        Vec::new()
    };
    assemble_rule_example(
        &source,
        target,
        graph.compose_path_overhead(&path),
        solutions,
    )
}

pub fn keep_bool_source<S>(source: &S, config: &[usize]) -> bool
where
    S: Problem<Metric = bool>,
{
    source.evaluate(config)
}
