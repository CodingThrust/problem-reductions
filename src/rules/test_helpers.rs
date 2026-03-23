use crate::rules::{ReductionChain, ReductionResult};
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};
use std::collections::HashSet;

fn verify_optimization_round_trip<Source, Extract>(
    source: &Source,
    target_solutions: Vec<Vec<usize>>,
    extract_solution: Extract,
    target_solution_kind: &str,
    context: &str,
) where
    Source: OptimizationProblem + 'static,
    <Source as OptimizationProblem>::Objective: std::fmt::Debug + PartialEq,
    <Source as Problem>::Value: std::fmt::Debug + PartialEq,
    Extract: Fn(&[usize]) -> Vec<usize>,
{
    assert!(
        !target_solutions.is_empty(),
        "{context}: target solver found no {target_solution_kind} solutions"
    );

    let solver = BruteForce::new();
    let reference_solutions: HashSet<Vec<usize>> =
        solver.find_all_best(source).into_iter().collect();
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
        assert!(
            extracted_metric.is_valid(),
            "{context}: extracted source solution is infeasible: {:?}",
            source_solution
        );
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
    Source: SatisfactionProblem + 'static,
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
    for source_solution in &extracted {
        assert!(
            source.evaluate(source_solution),
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
    R::Source: OptimizationProblem + 'static,
    R::Target: OptimizationProblem + 'static,
    <R::Source as OptimizationProblem>::Objective: std::fmt::Debug + PartialEq,
    <R::Source as Problem>::Value: std::fmt::Debug + PartialEq,
{
    let target_solutions = BruteForce::new().find_all_best(reduction.target_problem());
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
    R::Source: OptimizationProblem + 'static,
    R::Target: SatisfactionProblem + 'static,
    <R::Source as OptimizationProblem>::Objective: std::fmt::Debug + PartialEq,
    <R::Source as Problem>::Value: std::fmt::Debug + PartialEq,
{
    let target_solutions = BruteForce::new().find_all_satisfying(reduction.target_problem());
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
    Source: OptimizationProblem + 'static,
    Target: OptimizationProblem + 'static,
    <Source as OptimizationProblem>::Objective: std::fmt::Debug + PartialEq,
    <Source as Problem>::Value: std::fmt::Debug + PartialEq,
{
    let target_solutions = BruteForce::new().find_all_best(chain.target_problem::<Target>());
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
    R::Source: SatisfactionProblem + 'static,
    R::Target: OptimizationProblem + 'static,
{
    let target_solutions = BruteForce::new().find_all_best(reduction.target_problem());
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
    R::Source: SatisfactionProblem + 'static,
    R::Target: SatisfactionProblem + 'static,
{
    let target_solutions = BruteForce::new().find_all_satisfying(reduction.target_problem());
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
    P: OptimizationProblem + 'static,
{
    BruteForce::new().find_best(problem)
}

pub(crate) fn solve_satisfaction_problem<P>(problem: &P) -> Option<Vec<usize>>
where
    P: SatisfactionProblem + 'static,
{
    BruteForce::new().find_satisfying(problem)
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
    use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};
    use crate::types::{Direction, SolutionSize};

    #[derive(Clone)]
    struct ToyOptimizationProblem;

    impl Problem for ToyOptimizationProblem {
        const NAME: &'static str = "ToyOptimizationProblem";
        type Value = SolutionSize<i32>;

        fn dims(&self) -> Vec<usize> {
            vec![2, 2]
        }

        fn evaluate(&self, config: &[usize]) -> Self::Value {
            match config {
                [1, 0] | [0, 1] => SolutionSize::Valid(1),
                _ => SolutionSize::Invalid,
            }
        }

        fn variant() -> Vec<(&'static str, &'static str)> {
            vec![]
        }
    }

    impl OptimizationProblem for ToyOptimizationProblem {
        type Objective = i32;

        fn direction(&self) -> Direction {
            Direction::Maximize
        }
    }

    #[derive(Clone)]
    struct ToySatisfactionProblem;

    impl Problem for ToySatisfactionProblem {
        const NAME: &'static str = "ToySatisfactionProblem";
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

    impl SatisfactionProblem for ToySatisfactionProblem {}

    struct OptToOptReduction {
        target: ToyOptimizationProblem,
    }

    impl ReductionResult for OptToOptReduction {
        type Source = ToyOptimizationProblem;
        type Target = ToyOptimizationProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    struct OptToSatReduction {
        target: ToySatisfactionProblem,
    }

    impl ReductionResult for OptToSatReduction {
        type Source = ToyOptimizationProblem;
        type Target = ToySatisfactionProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    struct SatToOptReduction {
        target: ToyOptimizationProblem,
    }

    impl ReductionResult for SatToOptReduction {
        type Source = ToySatisfactionProblem;
        type Target = ToyOptimizationProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    struct SatToSatReduction {
        target: ToySatisfactionProblem,
    }

    impl ReductionResult for SatToSatReduction {
        type Source = ToySatisfactionProblem;
        type Target = ToySatisfactionProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
            target_solution.to_vec()
        }
    }

    #[test]
    fn test_optimization_round_trip_wrappers_accept_identity_reductions() {
        let source = ToyOptimizationProblem;

        assert_optimization_round_trip_from_optimization_target(
            &source,
            &OptToOptReduction {
                target: ToyOptimizationProblem,
            },
            "opt->opt",
        );
        assert_optimization_round_trip_from_satisfaction_target(
            &source,
            &OptToSatReduction {
                target: ToySatisfactionProblem,
            },
            "opt->sat",
        );
    }

    #[test]
    fn test_satisfaction_round_trip_wrappers_accept_identity_reductions() {
        let source = ToySatisfactionProblem;

        assert_satisfaction_round_trip_from_optimization_target(
            &source,
            &SatToOptReduction {
                target: ToyOptimizationProblem,
            },
            "sat->opt",
        );
        assert_satisfaction_round_trip_from_satisfaction_target(
            &source,
            &SatToSatReduction {
                target: ToySatisfactionProblem,
            },
            "sat->sat",
        );
    }
}
