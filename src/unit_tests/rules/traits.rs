#[test]
fn test_traits_compile() {
    // Traits should compile - actual tests in reduction implementations
}

use crate::rules::traits::{
    validate_target_solution, AggregateReductionResult, DynAggregateReductionResult, ReduceTo,
    ReduceToAggregate, ReductionResult,
};
use crate::traits::Problem;
use crate::types::Sum;
use serde_json::json;

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
    type Value = i64;

    crate::problem_size![("num_variables", num_variables)];
    fn evaluate(&self, config: &Self::Solution) -> Result<i64, crate::traits::EvaluationError> {
        if config.len() != 2 || config.iter().any(|&value| value >= 2) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "expected two binary target values".to_string(),
            ));
        }
        Ok((config[0] + config[1]) as i64)
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for SourceProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2, 2]
    }
}

impl Problem for TargetProblem {
    const NAME: &'static str = "Target";
    type Solution = Vec<usize>;
    type Value = i64;

    crate::problem_size![("num_variables", num_variables)];
    fn evaluate(&self, config: &Self::Solution) -> Result<i64, crate::traits::EvaluationError> {
        if config.len() != 2 || config.iter().any(|&value| value >= 2) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "expected two binary target values".to_string(),
            ));
        }
        Ok((config[0] + config[1]) as i64)
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for TargetProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2, 2]
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
    fn extract_solution(
        &self,
        target_config: &<Self::Target as Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as Problem>::Solution> {
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
    assert_eq!(target.evaluate(&vec![1, 1]).unwrap(), 2);
    assert_eq!(result.extract_solution(&vec![1, 0]).unwrap(), vec![1, 0]);
}

#[test]
fn target_solution_validation_rejects_shape_and_domain_errors() {
    let target = TargetProblem;

    assert!(validate_target_solution(&target, &vec![1, 0]).is_ok());
    assert!(validate_target_solution(&target, &vec![1]).is_err());
    assert!(validate_target_solution(&target, &vec![1, 0, 0]).is_err());
    assert!(validate_target_solution(&target, &vec![1, 2]).is_err());
}

#[derive(Clone)]
struct AggregateSourceProblem;

#[derive(Clone)]
struct AggregateTargetProblem;

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
    type Value = Sum<u64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().sum::<usize>() as u64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregateSourceProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2]
    }
}

impl Problem for AggregateTargetProblem {
    const NAME: &'static str = "AggregateTarget";
    type Solution = Vec<usize>;
    type Value = Sum<u64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().sum::<usize>() as u64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregateTargetProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2]
    }
}

struct TestAggregateReduction {
    target: AggregateTargetProblem,
    offset: u64,
}

impl AggregateReductionResult for TestAggregateReduction {
    type Source = AggregateSourceProblem;
    type Target = AggregateTargetProblem;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Sum<u64>) -> Sum<u64> {
        Sum(target_value.0 + self.offset)
    }
}

impl ReduceToAggregate<AggregateTargetProblem> for AggregateSourceProblem {
    type Result = TestAggregateReduction;

    fn reduce_to_aggregate(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(TestAggregateReduction {
            target: AggregateTargetProblem,
            offset: 3,
        })
    }
}

#[test]
fn test_aggregate_reduction_extracts_value() {
    let source = AggregateSourceProblem;
    let result =
        <AggregateSourceProblem as ReduceToAggregate<AggregateTargetProblem>>::reduce_to_aggregate(
            &source,
        )
        .expect("reduction should succeed");

    assert_eq!(result.extract_value(Sum(7)), Sum(10));
}

#[test]
fn test_dyn_aggregate_reduction_result_extracts_value() {
    let result = TestAggregateReduction {
        target: AggregateTargetProblem,
        offset: 2,
    };
    let dyn_result: &dyn DynAggregateReductionResult = &result;

    assert!(dyn_result
        .target_problem_any()
        .downcast_ref::<AggregateTargetProblem>()
        .is_some());
    assert_eq!(dyn_result.extract_value_dyn(json!(7)), json!(9));
}
