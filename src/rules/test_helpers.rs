use crate::rules::{ReductionChain, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::{ObjectiveProblem, Problem, WitnessProblem};
use crate::types::Aggregate;
use std::collections::HashSet;

fn verify_optimization_round_trip<Source, Extract>(
    source: &Source,
    target_solutions: Vec<Vec<usize>>,
    extract_solution: Extract,
    target_solution_kind: &str,
    context: &str,
) where
    Source: ObjectiveProblem + 'static,
    <Source as Problem>::Value: Aggregate + std::fmt::Debug + PartialEq,
    Extract: Fn(&[usize]) -> Vec<usize>,
{
    assert!(
        !target_solutions.is_empty(),
        "{context}: target solver found no {target_solution_kind} solutions"
    );

    let solver = BruteForce::new();
    let reference_solutions: HashSet<Vec<usize>> =
        solver.find_all_witnesses(source).into_iter().collect();
    assert!(
        !reference_solutions.is_empty(),
        "{context}: direct source solver found no optimal solutions"
    );

    let reference_metric = source.evaluate(
        reference_solutions
            .iter()
            .next()
            .expect("reference set is non-empty"),
    );
    let extracted: HashSet<Vec<usize>> = target_solutions
        .iter()
        .map(|target_solution| extract_solution(target_solution))
        .collect();
    assert!(
        !extracted.is_empty(),
        "{context}: no extracted source solutions"
    );
    assert!(
        extracted.is_subset(&reference_solutions),
        "{context}: extracted source solutions are not all directly optimal"
    );
    for source_solution in &extracted {
        let extracted_metric = source.evaluate(source_solution);
        assert_eq!(
            extracted_metric, reference_metric,
            "{context}: extracted source objective does not match direct solve"
        );
    }
}

fn verify_satisfaction_round_trip<Source, Extract>(
    source: &Source,
    target_solutions: Vec<Vec<usize>>,
    extract_solution: Extract,
    target_solution_kind: &str,
    context: &str,
) where
    Source: WitnessProblem + 'static,
    <Source as Problem>::Value: Aggregate + std::fmt::Debug,
    Extract: Fn(&[usize]) -> Vec<usize>,
{
    assert!(
        !target_solutions.is_empty(),
        "{context}: target solver found no {target_solution_kind} solutions"
    );
    let extracted: HashSet<Vec<usize>> = target_solutions
        .iter()
        .map(|target_solution| extract_solution(target_solution))
        .collect();
    assert!(
        !extracted.is_empty(),
        "{context}: no extracted source solutions"
    );
    let total = <BruteForce as crate::solvers::Solver>::solve(&BruteForce::new(), source);
    for source_solution in &extracted {
        let value = source.evaluate(source_solution);
        assert!(
            <Source::Value as Aggregate>::contributes_to_witnesses(&value, &total),
            "{context}: extracted source solution is not satisfying: {:?}",
            source_solution
        );
    }
}

pub(crate) fn assert_optimization_round_trip_from_optimization_target<R>(
    source: &R::Source,
    reduction: &R,
    context: &str,
) where
    R: ReductionResult,
    R::Source: ObjectiveProblem + 'static,
    R::Target: ObjectiveProblem + 'static,
    <R::Source as Problem>::Value: Aggregate + std::fmt::Debug + PartialEq,
    <R::Target as Problem>::Value: Aggregate,
{
    let target_solutions = BruteForce::new().find_all_witnesses(reduction.target_problem());
    verify_optimization_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution),
        "optimal",
        context,
    );
}

pub(crate) fn assert_optimization_round_trip_from_satisfaction_target<R>(
    source: &R::Source,
    reduction: &R,
    context: &str,
) where
    R: ReductionResult,
    R::Source: ObjectiveProblem + 'static,
    R::Target: WitnessProblem + 'static,
    <R::Source as Problem>::Value: Aggregate + std::fmt::Debug + PartialEq,
    <R::Target as Problem>::Value: Aggregate,
{
    let target_solutions = BruteForce::new().find_all_witnesses(reduction.target_problem());
    verify_optimization_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution),
        "satisfying",
        context,
    );
}

pub(crate) fn assert_optimization_round_trip_chain<Source, Target>(
    source: &Source,
    chain: &ReductionChain,
    context: &str,
) where
    Source: ObjectiveProblem + 'static,
    Target: ObjectiveProblem + 'static,
    <Source as Problem>::Value: Aggregate + std::fmt::Debug + PartialEq,
    <Target as Problem>::Value: Aggregate,
{
    let target_solutions = BruteForce::new().find_all_witnesses(chain.target_problem::<Target>());
    verify_optimization_round_trip(
        source,
        target_solutions,
        |target_solution| chain.extract_solution(target_solution),
        "optimal",
        context,
    );
}

pub(crate) fn assert_satisfaction_round_trip_from_optimization_target<R>(
    source: &R::Source,
    reduction: &R,
    context: &str,
) where
    R: ReductionResult,
    R::Source: WitnessProblem + 'static,
    R::Target: ObjectiveProblem + 'static,
    <R::Source as Problem>::Value: Aggregate + std::fmt::Debug,
    <R::Target as Problem>::Value: Aggregate,
{
    let target_solutions = BruteForce::new().find_all_witnesses(reduction.target_problem());
    verify_satisfaction_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution),
        "optimal",
        context,
    );
}

pub(crate) fn assert_satisfaction_round_trip_from_satisfaction_target<R>(
    source: &R::Source,
    reduction: &R,
    context: &str,
) where
    R: ReductionResult,
    R::Source: WitnessProblem + 'static,
    R::Target: WitnessProblem + 'static,
    <R::Source as Problem>::Value: Aggregate + std::fmt::Debug,
    <R::Target as Problem>::Value: Aggregate,
{
    let target_solutions = BruteForce::new().find_all_witnesses(reduction.target_problem());
    verify_satisfaction_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution),
        "satisfying",
        context,
    );
}

pub(crate) fn solve_optimization_problem<P>(problem: &P) -> Option<Vec<usize>>
where
    P: ObjectiveProblem + 'static,
    P::Value: Aggregate,
{
    BruteForce::new().find_witness(problem)
}

pub(crate) fn solve_satisfaction_problem<P>(problem: &P) -> Option<Vec<usize>>
where
    P: WitnessProblem + 'static,
    P::Value: Aggregate,
{
    BruteForce::new().find_witness(problem)
}

#[cfg(test)]
mod tests {
    use super::{
        assert_optimization_round_trip_from_optimization_target,
        assert_optimization_round_trip_from_satisfaction_target,
        assert_satisfaction_round_trip_from_optimization_target,
        assert_satisfaction_round_trip_from_satisfaction_target,
    };
    use crate::rules::ReductionResult;
    use crate::traits::{ObjectiveProblem, Problem, WitnessProblem};
    use crate::types::{ExtremumSense, Max};

    #[derive(Clone)]
    struct ToyObjectiveProblem;

    impl Problem for ToyObjectiveProblem {
        const NAME: &'static str = "ToyObjectiveProblem";
        type Value = Max<i32>;

        fn dims(&self) -> Vec<usize> {
            vec![2, 2]
        }

        fn evaluate(&self, config: &[usize]) -> Self::Value {
            match config {
                [1, 0] | [0, 1] => Max(Some(1)),
                _ => Max(None),
            }
        }

        fn variant() -> Vec<(&'static str, &'static str)> {
            vec![]
        }
    }

    impl ObjectiveProblem for ToyObjectiveProblem {
        type Objective = i32;

        fn direction(&self) -> ExtremumSense {
            ExtremumSense::Maximize
        }
    }

    #[derive(Clone)]
    struct ToyWitnessProblem;

    impl Problem for ToyWitnessProblem {
        const NAME: &'static str = "ToyWitnessProblem";
        type Value = bool;

        fn dims(&self) -> Vec<usize> {
            vec![2, 2]
        }

        fn evaluate(&self, config: &[usize]) -> Self::Value {
            matches!(config, [1, 0] | [0, 1])
        }

        fn variant() -> Vec<(&'static str, &'static str)> {
            vec![]
        }
    }

    impl WitnessProblem for ToyWitnessProblem {}

    struct OptToOptReduction {
        target: ToyObjectiveProblem,
    }

    impl ReductionResult for OptToOptReduction {
        type Source = ToyObjectiveProblem;
        type Target = ToyObjectiveProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    struct OptToSatReduction {
        target: ToyWitnessProblem,
    }

    impl ReductionResult for OptToSatReduction {
        type Source = ToyObjectiveProblem;
        type Target = ToyWitnessProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    struct SatToOptReduction {
        target: ToyObjectiveProblem,
    }

    impl ReductionResult for SatToOptReduction {
        type Source = ToyWitnessProblem;
        type Target = ToyObjectiveProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    struct SatToSatReduction {
        target: ToyWitnessProblem,
    }

    impl ReductionResult for SatToSatReduction {
        type Source = ToyWitnessProblem;
        type Target = ToyWitnessProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    #[test]
    fn test_optimization_round_trip_wrappers_accept_identity_reductions() {
        let source = ToyObjectiveProblem;

        assert_optimization_round_trip_from_optimization_target(
            &source,
            &OptToOptReduction {
                target: ToyObjectiveProblem,
            },
            "opt->opt",
        );
        assert_optimization_round_trip_from_satisfaction_target(
            &source,
            &OptToSatReduction {
                target: ToyWitnessProblem,
            },
            "opt->sat",
        );
    }

    #[test]
    fn test_satisfaction_round_trip_wrappers_accept_identity_reductions() {
        let source = ToyWitnessProblem;

        assert_satisfaction_round_trip_from_optimization_target(
            &source,
            &SatToOptReduction {
                target: ToyObjectiveProblem,
            },
            "sat->opt",
        );
        assert_satisfaction_round_trip_from_satisfaction_target(
            &source,
            &SatToSatReduction {
                target: ToyWitnessProblem,
            },
            "sat->sat",
        );
    }
}
