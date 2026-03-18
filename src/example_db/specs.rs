//! Shared example specification types and helper functions.
//!
//! These types describe canonical model and rule examples with metadata
//! that can be validated against the catalog and reduction registry.

use crate::export::{ModelExample, ProblemSide, RuleExample, SampleEval, SolutionPair};
use crate::models::algebraic::{VariableDomain, ILP};
use crate::prelude::{OptimizationProblem, Problem, ReduceTo, ReductionResult};
use crate::rules::{MinimizeSteps, ReductionGraph};
use crate::solvers::{BruteForce, ILPSolver};
use crate::types::ProblemSize;
use serde::Serialize;
use std::any::Any;
#[cfg(feature = "ilp-solver")]
use std::sync::OnceLock;

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
    solutions: Vec<SolutionPair>,
) -> RuleExample
where
    S: Problem + Serialize,
    T: Problem + Serialize,
{
    RuleExample {
        source: ProblemSide::from_problem(source),
        target: ProblemSide::from_problem(target),
        solutions,
    }
}

pub fn direct_best_example<S, T, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: OptimizationProblem + Serialize + 'static,
    T::Metric: Serialize,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<T>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions = choose_best_target_solution(target, |target_config| {
        build_solution_pair(&source, &reduction, target_config, &keep)
    })
    .into_iter()
    .collect();
    assemble_rule_example(&source, target, solutions)
}

pub fn direct_satisfying_example<S, T, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: Problem<Metric = bool> + Serialize + 'static,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<T>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions = choose_satisfying_target_solution(target, |target_config| {
        build_solution_pair(&source, &reduction, target_config, &keep)
    })
    .into_iter()
    .collect();
    assemble_rule_example(&source, target, solutions)
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
    let solutions = choose_best_target_solution(target, |target_config| {
        build_solution_pair(&source, &reduction, target_config, &keep)
    })
    .into_iter()
    .collect();
    assemble_rule_example(&source, target, solutions)
}

pub fn keep_bool_source<S>(source: &S, config: &[usize]) -> bool
where
    S: Problem<Metric = bool>,
{
    source.evaluate(config)
}

fn choose_best_target_solution<T, Keep>(target: &T, keep: Keep) -> Option<SolutionPair>
where
    T: OptimizationProblem + 'static,
    Keep: Fn(&[usize]) -> Option<SolutionPair>,
{
    choose_available_ilp_solution(target)
        .as_deref()
        .and_then(&keep)
        .or_else(|| first_matching_solution(BruteForce::new().find_all_best(target), keep))
}

fn choose_satisfying_target_solution<T, Keep>(target: &T, keep: Keep) -> Option<SolutionPair>
where
    T: Problem<Metric = bool> + 'static,
    Keep: Fn(&[usize]) -> Option<SolutionPair>,
{
    choose_available_ilp_solution(target)
        .as_deref()
        .and_then(&keep)
        .or_else(|| first_matching_solution(BruteForce::new().find_all_satisfying(target), keep))
}

fn build_solution_pair<S, R, Keep>(
    source: &S,
    reduction: &R,
    target_config: &[usize],
    keep: &Keep,
) -> Option<SolutionPair>
where
    S: Problem,
    R: ReductionResult<Source = S>,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let source_config = reduction.extract_solution(target_config);
    keep(source, &source_config).then_some(SolutionPair {
        source_config,
        target_config: target_config.to_vec(),
    })
}

fn first_matching_solution<Keep>(
    mut candidates: Vec<Vec<usize>>,
    keep: Keep,
) -> Option<SolutionPair>
where
    Keep: Fn(&[usize]) -> Option<SolutionPair>,
{
    candidates.sort();
    candidates.iter().find_map(|candidate| keep(candidate))
}

#[cfg(feature = "ilp-solver")]
fn choose_available_ilp_solution<T>(problem: &T) -> Option<Vec<usize>>
where
    T: Problem + 'static,
{
    let problem_any = problem as &dyn Any;
    if let Some(ilp) = problem_any.downcast_ref::<ILP<bool>>() {
        return ILPSolver::new().solve(ilp);
    }
    if let Some(ilp) = problem_any.downcast_ref::<ILP<i32>>() {
        return ILPSolver::new().solve(ilp);
    }

    let graph = ilp_reduction_graph();
    let source_variant = ReductionGraph::variant_to_map(&T::variant());
    let input_size = ProblemSize::new(vec![]);

    let bool_variant = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    if let Some(path) = graph.find_cheapest_path(
        T::NAME,
        &source_variant,
        "ILP",
        &bool_variant,
        &input_size,
        &MinimizeSteps,
    ) {
        let chain = graph.reduce_along_path(&path, problem as &dyn Any)?;
        let ilp = chain.target_problem::<ILP<bool>>();
        let ilp_solution = ILPSolver::new().solve(ilp)?;
        return Some(chain.extract_solution(&ilp_solution));
    }

    let i32_variant = ReductionGraph::variant_to_map(&ILP::<i32>::variant());
    let path = graph.find_cheapest_path(
        T::NAME,
        &source_variant,
        "ILP",
        &i32_variant,
        &input_size,
        &MinimizeSteps,
    )?;
    let chain = graph.reduce_along_path(&path, problem as &dyn Any)?;
    let ilp = chain.target_problem::<ILP<i32>>();
    let ilp_solution = ILPSolver::new().solve(ilp)?;
    Some(chain.extract_solution(&ilp_solution))
}

#[cfg(feature = "ilp-solver")]
fn ilp_reduction_graph() -> &'static ReductionGraph {
    static GRAPH: OnceLock<ReductionGraph> = OnceLock::new();
    GRAPH.get_or_init(ReductionGraph::new)
}

#[cfg(not(feature = "ilp-solver"))]
fn choose_available_ilp_solution<T>(_problem: &T) -> Option<Vec<usize>>
where
    T: Problem + 'static,
{
    None
}
