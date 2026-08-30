use crate::rules::{ReductionChain, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::SolutionAggregate;
use std::collections::HashSet;

fn verify_optimization_round_trip<Source, TargetSolution, Extract>(
    source: &Source,
    target_solutions: Vec<TargetSolution>,
    extract_solution: Extract,
    target_solution_kind: &str,
    context: &str,
) where
    Source: Problem + 'static,
    Source::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    <Source as Problem>::Value: SolutionAggregate + std::fmt::Debug + PartialEq,
    Extract: Fn(&TargetSolution) -> Source::Solution,
{
    assert!(
        !target_solutions.is_empty(),
        "{context}: target solver found no {target_solution_kind} solutions"
    );

    let solver = BruteForce::new();
    let reference_solutions: HashSet<Source::Solution> = solver
        .find_all_witnesses(source)
        .unwrap()
        .into_iter()
        .collect();
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
    let extracted: HashSet<Source::Solution> =
        target_solutions.iter().map(extract_solution).collect();
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

fn verify_satisfaction_round_trip<Source, TargetSolution, Extract>(
    source: &Source,
    target_solutions: Vec<TargetSolution>,
    extract_solution: Extract,
    target_solution_kind: &str,
    context: &str,
) where
    Source: Problem + 'static,
    Source::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    <Source as Problem>::Value: SolutionAggregate + std::fmt::Debug,
    Extract: Fn(&TargetSolution) -> Source::Solution,
{
    assert!(
        !target_solutions.is_empty(),
        "{context}: target solver found no {target_solution_kind} solutions"
    );
    let extracted: HashSet<Source::Solution> =
        target_solutions.iter().map(extract_solution).collect();
    assert!(
        !extracted.is_empty(),
        "{context}: no extracted source solutions"
    );
    let optimal_solution = BruteForce::new()
        .solve(source)
        .unwrap()
        .expect("source problem must be feasible");
    let total = source.evaluate(&optimal_solution).unwrap();
    for source_solution in &extracted {
        let value = source.evaluate(source_solution).unwrap();
        assert!(
            <Source::Value as SolutionAggregate>::contributes_to_solution(&value, &total),
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
    R::Source: Problem + 'static,
    R::Target: Problem + 'static,
    <R::Source as Problem>::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    <R::Target as Problem>::Solution: 'static,
    <R::Source as Problem>::Value: SolutionAggregate + std::fmt::Debug + PartialEq,
    <R::Target as Problem>::Value: SolutionAggregate,
{
    let target_solutions = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    verify_optimization_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution).unwrap(),
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
    R::Source: Problem + 'static,
    R::Target: Problem + 'static,
    <R::Source as Problem>::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    <R::Target as Problem>::Solution: 'static,
    <R::Source as Problem>::Value: SolutionAggregate + std::fmt::Debug + PartialEq,
    <R::Target as Problem>::Value: SolutionAggregate,
{
    let target_solutions = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    verify_optimization_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution).unwrap(),
        "satisfying",
        context,
    );
}

pub(crate) fn assert_optimization_round_trip_chain<Source, Target>(
    source: &Source,
    chain: &ReductionChain,
    context: &str,
) where
    Source: Problem + 'static,
    Target: Problem + 'static,
    Source::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    Target::Solution: 'static,
    <Source as Problem>::Value: SolutionAggregate + std::fmt::Debug + PartialEq,
    <Target as Problem>::Value: SolutionAggregate,
{
    let target_solutions = BruteForce::new()
        .find_all_witnesses(chain.target_problem::<Target>())
        .unwrap();
    verify_optimization_round_trip(
        source,
        target_solutions,
        |target_solution| {
            chain
                .extract_solution::<Source::Solution, Target::Solution>(target_solution)
                .unwrap()
        },
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
    R::Source: Problem + 'static,
    R::Target: Problem + 'static,
    <R::Source as Problem>::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    <R::Target as Problem>::Solution: 'static,
    <R::Source as Problem>::Value: SolutionAggregate + std::fmt::Debug,
    <R::Target as Problem>::Value: SolutionAggregate,
{
    let target_solutions = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    verify_satisfaction_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution).unwrap(),
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
    R::Source: Problem + 'static,
    R::Target: Problem + 'static,
    <R::Source as Problem>::Solution: Eq + std::hash::Hash + std::fmt::Debug + 'static,
    <R::Target as Problem>::Solution: 'static,
    <R::Source as Problem>::Value: SolutionAggregate + std::fmt::Debug,
    <R::Target as Problem>::Value: SolutionAggregate,
{
    let target_solutions = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    verify_satisfaction_round_trip(
        source,
        target_solutions,
        |target_solution| reduction.extract_solution(target_solution).unwrap(),
        "satisfying",
        context,
    );
}

pub(crate) fn assert_bf_vs_ilp<R>(source: &R::Source, reduction: &R)
where
    R: ReductionResult,
    R::Source: Problem + 'static,
    R::Target: Problem<Solution = Vec<i64>> + 'static,
    <R::Source as Problem>::Value: SolutionAggregate + std::fmt::Debug + PartialEq,
{
    use crate::solvers::ILPSolver;
    let bf_solution = BruteForce::new()
        .solve(source)
        .unwrap()
        .expect("source problem must be feasible");
    let bf_value = source.evaluate(&bf_solution).unwrap();
    let ilp_solution = ILPSolver::new()
        .solve_dyn(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), bf_value);
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
    use crate::traits::Problem;
    use crate::types::{Max, Or};

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    struct ToyExtremumProblem;

    impl ToyExtremumProblem {
        fn num_variables(&self) -> usize {
            2
        }
    }

    impl Problem for ToyExtremumProblem {
        const NAME: &'static str = "ToyExtremumProblem";
        type Solution = Vec<usize>;
        type Value = Max<i64>;

        crate::problem_parameters![("num_variables", num_variables)];

        fn evaluate(
            &self,
            config: &Self::Solution,
        ) -> Result<Self::Value, crate::traits::EvaluationError> {
            Ok({
                match config.as_slice() {
                    [1, 0] | [0, 1] => Max(Some(1)),
                    _ => Max(None),
                }
            })
        }

        fn variant() -> Vec<(&'static str, &'static str)> {
            vec![]
        }
    }

    impl crate::solvers::BruteForceProblem for ToyExtremumProblem {
        fn dimensions(&self) -> Vec<usize> {
            vec![2, 2]
        }
    }

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    struct ToyOrProblem;

    impl ToyOrProblem {
        fn num_variables(&self) -> usize {
            2
        }
    }

    impl Problem for ToyOrProblem {
        const NAME: &'static str = "ToyOrProblem";
        type Solution = Vec<usize>;
        type Value = Or;

        crate::problem_parameters![("num_variables", num_variables)];

        fn evaluate(
            &self,
            config: &Self::Solution,
        ) -> Result<Self::Value, crate::traits::EvaluationError> {
            Ok(Or(matches!(config.as_slice(), [1, 0] | [0, 1])))
        }

        fn variant() -> Vec<(&'static str, &'static str)> {
            vec![]
        }
    }

    impl crate::solvers::BruteForceProblem for ToyOrProblem {
        fn dimensions(&self) -> Vec<usize> {
            vec![2, 2]
        }
    }

    crate::declare_variants! {
        default ToyExtremumProblem => "2^num_variables",
        default ToyOrProblem => "2^num_variables",
    }

    crate::register_brute_force! {
        ToyExtremumProblem,
        ToyOrProblem,
    }

    inventory::submit! {
        crate::registry::ProblemSchemaEntry {
            name: "ToyExtremumProblem",
            display_name: "Toy Extremum Test Problem",
            aliases: &[],
            dimensions: &[],
            category: crate::registry::ProblemCategory::Misc,
            module_path: module_path!(),
            description: "Test problem for optimization reduction helpers",
            fields: &[],
        }
    }

    inventory::submit! {
        crate::registry::ProblemSchemaEntry {
            name: "ToyOrProblem",
            display_name: "Toy Satisfaction Test Problem",
            aliases: &[],
            dimensions: &[],
            category: crate::registry::ProblemCategory::Misc,
            module_path: module_path!(),
            description: "Test problem for satisfaction reduction helpers",
            fields: &[],
        }
    }

    struct OptToOptReduction {
        target: ToyExtremumProblem,
    }

    impl ReductionResult for OptToOptReduction {
        type Source = ToyExtremumProblem;
        type Target = ToyExtremumProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(
            &self,
            target_solution: &<Self::Target as crate::traits::Problem>::Solution,
        ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution>
        {
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

            Ok(target_solution.to_vec())
        }
    }

    struct OptToSatReduction {
        target: ToyOrProblem,
    }

    impl ReductionResult for OptToSatReduction {
        type Source = ToyExtremumProblem;
        type Target = ToyOrProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(
            &self,
            target_solution: &<Self::Target as crate::traits::Problem>::Solution,
        ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution>
        {
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

            Ok(target_solution.to_vec())
        }
    }

    struct SatToOptReduction {
        target: ToyExtremumProblem,
    }

    impl ReductionResult for SatToOptReduction {
        type Source = ToyOrProblem;
        type Target = ToyExtremumProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(
            &self,
            target_solution: &<Self::Target as crate::traits::Problem>::Solution,
        ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution>
        {
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

            Ok(target_solution.to_vec())
        }
    }

    struct SatToSatReduction {
        target: ToyOrProblem,
    }

    impl ReductionResult for SatToSatReduction {
        type Source = ToyOrProblem;
        type Target = ToyOrProblem;

        fn target_problem(&self) -> &Self::Target {
            &self.target
        }

        fn extract_solution(
            &self,
            target_solution: &<Self::Target as crate::traits::Problem>::Solution,
        ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution>
        {
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

            Ok(target_solution.to_vec())
        }
    }

    #[test]
    fn test_optimization_round_trip_wrappers_accept_identity_reductions() {
        let source = ToyExtremumProblem;

        assert_optimization_round_trip_from_optimization_target(
            &source,
            &OptToOptReduction {
                target: ToyExtremumProblem,
            },
            "extremum->extremum",
        );
        assert_optimization_round_trip_from_satisfaction_target(
            &source,
            &OptToSatReduction {
                target: ToyOrProblem,
            },
            "extremum->witness",
        );
    }

    #[test]
    fn test_satisfaction_round_trip_wrappers_accept_identity_reductions() {
        let source = ToyOrProblem;

        assert_satisfaction_round_trip_from_optimization_target(
            &source,
            &SatToOptReduction {
                target: ToyExtremumProblem,
            },
            "witness->extremum",
        );
        assert_satisfaction_round_trip_from_satisfaction_target(
            &source,
            &SatToSatReduction {
                target: ToyOrProblem,
            },
            "witness->witness",
        );
    }
}
