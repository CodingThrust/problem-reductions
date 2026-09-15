use crate::rules::traits::{DynReductionResult, ReduceTo, ReductionResult};
use crate::solvers::{downcast_outcome, erase_outcome, SolveOutcome};
use crate::traits::Problem;
use crate::types::Min;
use serde_json::json;

#[test]
fn recovery_preserves_status_and_evaluates_the_mapped_solution() {
    use crate::rules::traits::recover_preserving_status;

    let source = SourceProblem;
    let target = TargetProblem;
    let complement =
        |solution: &Vec<usize>| Ok(solution.iter().map(|value| 1 - value).collect::<Vec<_>>());
    assert_eq!(
        recover_preserving_status(
            &source,
            SolveOutcome::optimal(&target, vec![1, 1]).unwrap(),
            complement,
        )
        .unwrap(),
        SolveOutcome::Optimal {
            solution: vec![0, 0],
            evaluation: Min(Some(0)),
        }
    );
    assert_eq!(
        recover_preserving_status(
            &source,
            SolveOutcome::feasible(&target, vec![0, 0]).unwrap(),
            complement,
        )
        .unwrap(),
        SolveOutcome::Feasible {
            solution: vec![1, 1],
            evaluation: Min(Some(2)),
        }
    );
    assert_eq!(
        recover_preserving_status(
            &source,
            crate::solvers::ProblemOutcome::<TargetProblem>::Infeasible,
            |_| panic!("infeasibility has no witness to map"),
        )
        .unwrap(),
        SolveOutcome::Infeasible
    );
}

#[test]
fn recovery_propagates_mapping_and_evaluation_failures() {
    use crate::rules::traits::recover_preserving_status;
    use crate::rules::ExtractionError;
    use crate::traits::EvaluationError;

    for outcome in [
        SolveOutcome::optimal(&TargetProblem, vec![1, 1]).unwrap(),
        SolveOutcome::feasible(&TargetProblem, vec![1, 1]).unwrap(),
    ] {
        assert!(matches!(
            recover_preserving_status(&SourceProblem, outcome.clone(), |_| {
                Err(ExtractionError::InsufficientSolutionQuality)
            }),
            Err(ExtractionError::InsufficientSolutionQuality)
        ));
        assert!(matches!(
            recover_preserving_status(&SourceProblem, outcome, |_| Ok(vec![2, 0])),
            Err(ExtractionError::Evaluation(
                EvaluationError::InvalidConfiguration(_)
            ))
        ));
    }
}

#[derive(Clone)]
struct SourceProblem;
#[derive(Clone)]
struct TargetProblem;

impl SourceProblem {
    fn num_variables(&self) -> usize {
        2
    }
}

impl TargetProblem {
    fn num_variables(&self) -> usize {
        2
    }
}

impl Problem for SourceProblem {
    const NAME: &'static str = "Source";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_variables", num_variables)];
    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        if config.len() != 2 || config.iter().any(|&value| value >= 2) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "expected two binary target values".to_string(),
            ));
        }
        Ok(Min(Some((config[0] + config[1]) as i64)))
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl Problem for TargetProblem {
    const NAME: &'static str = "Target";
    type Solution = Vec<usize>;
    type Value = crate::types::Max<i64>;

    crate::problem_parameters![("num_variables", num_variables)];
    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        if config.len() != 2 || config.iter().any(|&value| value >= 2) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "expected two binary target values".to_string(),
            ));
        }
        Ok(crate::types::Max(Some((config[0] + config[1]) as i64)))
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

#[derive(Clone)]
struct TestReduction {
    target: TargetProblem,
}

impl ReductionResult for TestReduction {
    type Source = SourceProblem;
    type Target = TargetProblem;
    fn target_problem(&self) -> &TargetProblem {
        &self.target
    }
    fn recover_result(
        &self,
        source: &Self::Source,
        target: crate::solvers::ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<crate::solvers::ProblemOutcome<Self::Source>> {
        match target {
            crate::solvers::SolveOutcome::Infeasible => {
                Ok(crate::solvers::SolveOutcome::Infeasible)
            }
            crate::solvers::SolveOutcome::Optimal { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(crate::solvers::SolveOutcome::optimal(source, solution)?)
            }
            crate::solvers::SolveOutcome::Feasible { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(crate::solvers::SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl TestReduction {
    fn map_solution(
        &self,
        target_config: &<<Self as ReductionResult>::Target as Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<<Self as ReductionResult>::Source as Problem>::Solution>
    {
        Ok(target_config.to_vec())
    }
}

impl ReduceTo<TargetProblem> for SourceProblem {
    type Result = TestReduction;
    fn reduce_to(&self) -> Result<TestReduction, crate::rules::ReductionError> {
        Ok(TestReduction {
            target: TargetProblem,
        })
    }
}

#[test]
fn test_reduction() {
    let source = SourceProblem;
    let result = <SourceProblem as ReduceTo<TargetProblem>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = result.target_problem();
    assert_eq!(
        target.evaluate(&vec![1, 1]).unwrap(),
        crate::types::Max(Some(2))
    );
    assert_eq!(
        result
            .recover_result(
                &source,
                crate::solvers::SolveOutcome::optimal(result.target_problem(), vec![1, 0].clone())
                    .unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        vec![1, 0]
    );
}

#[test]
fn aggregate_value_from_solution_keeps_evaluation_errors_distinct_from_false() {
    use crate::models::decision::Decision;
    use crate::models::graph::MinimumVertexCover;
    use crate::rules::ExtractionError;
    use crate::topology::SimpleGraph;

    let source = Decision::new(
        MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i64; 2]),
        0,
    );
    let edge = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|edge| {
            edge.source_name == "DecisionMinimumVertexCover"
                && edge.target_name == "MinimumVertexCover"
                && (edge.source_variant_fn)()
                    == <Decision<MinimumVertexCover<SimpleGraph, i64>> as Problem>::variant()
        })
        .unwrap();
    let step = (edge.reduce_fn.unwrap())(&source).unwrap();
    let target = step
        .witness
        .target_result_from_json(SolveOutcome::Optimal {
            solution: json!([true, false]),
            evaluation: String::new(),
        })
        .unwrap()
        .0;
    assert!(matches!(
        step.witness.recover_result_dyn(&source, target).unwrap(),
        SolveOutcome::Infeasible
    ));
    assert!(matches!(
        step.witness.target_result_from_json(SolveOutcome::Optimal {
            solution: json!([true]),
            evaluation: String::new(),
        }),
        Err(ExtractionError::Evaluation(_))
    ));
    assert!(matches!(
        step.witness.recover_result_dyn(
            &source,
            erase_outcome(SolveOutcome::Optimal {
                solution: vec![1i64, 0],
                evaluation: crate::types::Min(Some(1i64)),
            })
        ),
        Err(ExtractionError::InvalidTargetSolution(_))
    ));
}

#[derive(Clone)]
struct AggregateSourceProblem;

#[derive(Clone)]
struct AggregateTargetProblem;

thread_local! {
    static TARGET_EVALUATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl AggregateSourceProblem {
    fn num_variables(&self) -> usize {
        1
    }
}

impl AggregateTargetProblem {
    fn num_variables(&self) -> usize {
        1
    }
}

impl Problem for AggregateSourceProblem {
    const NAME: &'static str = "AggregateSource";
    type Solution = Vec<usize>;
    type Value = Min<u64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Min(Some(config.iter().sum::<usize>() as u64)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl Problem for AggregateTargetProblem {
    const NAME: &'static str = "AggregateTarget";
    type Solution = Vec<usize>;
    type Value = Min<u64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        TARGET_EVALUATIONS.with(|count| count.set(count.get() + 1));
        Ok(Min(Some(config.iter().sum::<usize>() as u64)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

struct TestAggregateReduction {
    target: AggregateTargetProblem,
    offset: u64,
}

impl ReductionResult for TestAggregateReduction {
    type Source = AggregateSourceProblem;
    type Target = AggregateTargetProblem;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: crate::solvers::ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<crate::solvers::ProblemOutcome<Self::Source>> {
        Ok(match target {
            SolveOutcome::Optimal { mut solution, .. } => {
                solution[0] += self.offset as usize;
                SolveOutcome::optimal(source, solution)?
            }
            SolveOutcome::Feasible { mut solution, .. } => {
                solution[0] += self.offset as usize;
                SolveOutcome::feasible(source, solution)?
            }
            SolveOutcome::Infeasible => SolveOutcome::Infeasible,
        })
    }
}

impl ReduceTo<AggregateTargetProblem> for AggregateSourceProblem {
    type Result = TestAggregateReduction;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(TestAggregateReduction {
            target: AggregateTargetProblem,
            offset: 3,
        })
    }
}

#[test]
fn test_aggregate_reduction_extracts_value() {
    let source = AggregateSourceProblem;
    let result = <AggregateSourceProblem as ReduceTo<AggregateTargetProblem>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_eq!(
        result
            .recover_result(
                &source,
                SolveOutcome::optimal(result.target_problem(), vec![7]).unwrap()
            )
            .unwrap(),
        SolveOutcome::Optimal {
            solution: vec![10],
            evaluation: Min(Some(10))
        }
    );
}

#[test]
fn test_dyn_aggregate_reduction_result_extracts_value() {
    let result = TestAggregateReduction {
        target: AggregateTargetProblem,
        offset: 2,
    };
    let dyn_result: &dyn DynReductionResult = &result;

    assert!(dyn_result
        .target_problem_any()
        .downcast_ref::<AggregateTargetProblem>()
        .is_some());
    TARGET_EVALUATIONS.with(|count| count.set(0));
    let (target, target_json) = dyn_result
        .target_result_from_json(SolveOutcome::Optimal {
            solution: json!([7]),
            evaluation: "untrusted external evaluation".into(),
        })
        .unwrap();
    assert_eq!(
        target_json,
        SolveOutcome::Optimal {
            solution: json!([7]),
            evaluation: "Min(7)".into(),
        }
    );
    assert_eq!(TARGET_EVALUATIONS.with(|count| count.get()), 1);
    let recovered = dyn_result
        .recover_result_dyn(&AggregateSourceProblem, target)
        .unwrap();
    assert_eq!(TARGET_EVALUATIONS.with(|count| count.get()), 1);
    assert_eq!(
        downcast_outcome::<Vec<usize>, Min<u64>>(recovered).unwrap(),
        SolveOutcome::Optimal {
            solution: vec![9],
            evaluation: Min(Some(9))
        }
    );
}
