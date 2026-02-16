#[test]
fn test_traits_compile() {
    // Traits should compile - actual tests in reduction implementations
}

use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;

#[derive(Clone)]
struct SourceProblem;
#[derive(Clone)]
struct TargetProblem;

impl Problem for SourceProblem {
    const NAME: &'static str = "Source";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2, 2]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        (config[0] + config[1]) as i32
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
    fn problem_size_names() -> &'static [&'static str] {
        &["num_vars"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![2]
    }
}

impl Problem for TargetProblem {
    const NAME: &'static str = "Target";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2, 2]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        (config[0] + config[1]) as i32
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
    fn problem_size_names() -> &'static [&'static str] {
        &["num_vars"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![2]
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
    fn extract_solution(&self, target_config: &[usize]) -> Vec<usize> {
        target_config.to_vec()
    }
}

impl ReduceTo<TargetProblem> for SourceProblem {
    type Result = TestReduction;
    fn reduce_to(&self) -> TestReduction {
        TestReduction {
            target: TargetProblem,
        }
    }
}

#[test]
fn test_reduction() {
    let source = SourceProblem;
    let result = <SourceProblem as ReduceTo<TargetProblem>>::reduce_to(&source);
    let target = result.target_problem();
    assert_eq!(target.evaluate(&[1, 1]), 2);
    assert_eq!(result.extract_solution(&[1, 0]), vec![1, 0]);
}
