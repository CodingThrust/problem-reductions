#[test]
fn test_traits_compile() {
    // Traits should compile - actual tests in reduction implementations
}

use crate::rules::traits::{ReduceToV2, ReductionResultV2};
use crate::traits::ProblemV2;

#[derive(Clone)]
struct SourceProblem;
#[derive(Clone)]
struct TargetProblem;

impl ProblemV2 for SourceProblem {
    const NAME: &'static str = "Source";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2, 2]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        (config[0] + config[1]) as i32
    }
}

impl ProblemV2 for TargetProblem {
    const NAME: &'static str = "Target";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2, 2]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        (config[0] + config[1]) as i32
    }
}

#[derive(Clone)]
struct TestReduction {
    target: TargetProblem,
}

impl ReductionResultV2 for TestReduction {
    type Source = SourceProblem;
    type Target = TargetProblem;
    fn target_problem(&self) -> &TargetProblem {
        &self.target
    }
    fn extract_solution(&self, target_config: &[usize]) -> Vec<usize> {
        target_config.to_vec()
    }
}

impl ReduceToV2<TargetProblem> for SourceProblem {
    type Result = TestReduction;
    fn reduce_to(&self) -> TestReduction {
        TestReduction {
            target: TargetProblem,
        }
    }
}

#[test]
fn test_reduction_v2() {
    let source = SourceProblem;
    let result = <SourceProblem as ReduceToV2<TargetProblem>>::reduce_to(&source);
    let target = result.target_problem();
    assert_eq!(target.evaluate(&[1, 1]), 2);
    assert_eq!(result.extract_solution(&[1, 0]), vec![1, 0]);
}
