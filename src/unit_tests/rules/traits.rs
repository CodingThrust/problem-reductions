#[test]
fn test_traits_compile() {
    // Traits should compile - actual tests in reduction implementations
}

use crate::rules::traits::{
    validate_target_solution, AggregateReductionResult, DynAggregateReductionResult, ReduceTo,
    ReduceToAggregate, ReductionResult,
};
use crate::solvers::BruteForceProblem as _;
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

    crate::problem_parameters![("num_variables", num_variables)];
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

    crate::problem_parameters![("num_variables", num_variables)];
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

    assert_eq!(validate_target_solution(&target, &vec![1, 0]).unwrap(), 1);
    assert!(validate_target_solution(&target, &vec![1]).is_err());
    assert!(validate_target_solution(&target, &vec![1, 0, 0]).is_err());
    assert!(validate_target_solution(&target, &vec![1, 2]).is_err());
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
    let reduction = source.reduce_to_aggregate().unwrap();
    let value = reduction
        .extract_value_from_solution_dyn(&vec![true, false])
        .unwrap();
    assert_eq!(value, json!(false));
    assert!(matches!(
        reduction.extract_value_from_solution_dyn(&vec![true]),
        Err(ExtractionError::Evaluation(_))
    ));
    assert!(matches!(
        reduction.extract_value_from_solution_dyn(&vec![1i64, 0]),
        Err(ExtractionError::InvalidTargetSolution(_))
    ));
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

    crate::problem_parameters![("num_variables", num_variables)];

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

    crate::problem_parameters![("num_variables", num_variables)];

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
    assert_eq!(dyn_result.extract_value_dyn(json!(7)).unwrap(), json!(9));
    assert!(matches!(
        dyn_result.extract_value_dyn(json!("not a count")),
        Err(crate::rules::ExtractionError::InvalidTargetSolution(_))
    ));
}

#[derive(Clone)]
struct UnserializableValue;

impl serde::Serialize for UnserializableValue {
    fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("aggregate cannot be serialized"))
    }
}

impl Problem for UnserializableValue {
    const NAME: &'static str = "UnserializableValue";
    type Solution = ();
    type Value = Self;
    fn parameter_names() -> &'static [&'static str] {
        &[]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        Default::default()
    }
    fn evaluate(&self, _: &()) -> Result<Self, crate::traits::EvaluationError> {
        Ok(self.clone())
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

struct UnserializableReduction(AggregateTargetProblem);

impl AggregateReductionResult for UnserializableReduction {
    type Source = UnserializableValue;
    type Target = AggregateTargetProblem;
    fn target_problem(&self) -> &Self::Target {
        &self.0
    }
    fn extract_value(&self, _: Sum<u64>) -> UnserializableValue {
        UnserializableValue
    }
}

#[test]
fn dynamic_aggregate_serialization_failure_returns_an_error() {
    let reduction = UnserializableReduction(AggregateTargetProblem);
    assert!(reduction
        .extract_value_from_solution_dyn(&vec![0usize])
        .is_err());
    let error = reduction.extract_value_dyn(json!(0)).unwrap_err();
    assert!(matches!(
        error,
        crate::rules::ExtractionError::InvalidTargetSolution(_)
    ));
    assert!(error
        .to_string()
        .contains("source aggregate serialization failed"));
}

#[derive(Clone)]
pub(crate) struct CountingOrCircuit;

#[derive(Clone)]
pub(crate) struct CountingTseitinFormula;

impl Problem for CountingOrCircuit {
    const NAME: &'static str = "CountingOrCircuit";
    type Solution = Vec<usize>;
    type Value = Sum<u64>;
    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        bits: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(u64::from(bits[0] != 0 || bits[1] != 0)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for CountingOrCircuit {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; 2]
    }
}

impl Problem for CountingTseitinFormula {
    const NAME: &'static str = "CountingTseitinFormula";
    type Solution = Vec<usize>;
    type Value = Sum<u64>;
    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        bits: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        let (x, y, z) = (bits[0] != 0, bits[1] != 0, bits[2] != 0);
        // z <=> (x OR y), with output z asserted.
        let clauses = [!x || z, !y || z, x || y || !z, z];
        Ok(Sum(u64::from(clauses.into_iter().all(|clause| clause))))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for CountingTseitinFormula {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; 3]
    }
}

pub(crate) struct CountingTseitinReduction(CountingTseitinFormula);

impl AggregateReductionResult for CountingTseitinReduction {
    type Source = CountingOrCircuit;
    type Target = CountingTseitinFormula;
    fn target_problem(&self) -> &Self::Target {
        &self.0
    }
    fn extract_value(&self, count: Sum<u64>) -> Sum<u64> {
        count
    }
}

impl ReduceToAggregate<CountingTseitinFormula> for CountingOrCircuit {
    type Result = CountingTseitinReduction;
    fn reduce_to_aggregate(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(CountingTseitinReduction(CountingTseitinFormula))
    }
}

#[test]
fn counting_reduction_preserves_the_number_of_satisfying_assignments() {
    use crate::solvers::BruteForce;
    let source = CountingOrCircuit;
    let reduction = source.reduce_to_aggregate().unwrap();
    let solver = BruteForce::new();
    let source_count = solver.solve_cartesian(&source, |bits| bits).unwrap();
    let target_count = solver
        .solve_cartesian(reduction.target_problem(), |bits| bits)
        .unwrap();
    assert_eq!(source_count, Sum(3));
    assert_eq!(target_count, Sum(3));
    assert_eq!(reduction.extract_value(target_count), source_count);
    assert_eq!(reduction.extract_value_dyn(json!(3)).unwrap(), json!(3));
}

#[derive(Clone)]
pub(crate) struct UniversalFormula {
    pub(crate) variable: usize,
    pub(crate) tautology: bool,
}

impl Problem for UniversalFormula {
    const NAME: &'static str = "UniversalFormula";
    type Solution = Vec<usize>;
    type Value = crate::types::And;
    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        bits: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        // x OR NOT x when tautology is true; otherwise just x.
        let x = bits[self.variable] != 0;
        Ok(crate::types::And(self.tautology || x))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for UniversalFormula {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; 2]
    }
}

pub(crate) struct RenameUniversalVariable(UniversalFormula);

impl AggregateReductionResult for RenameUniversalVariable {
    type Source = UniversalFormula;
    type Target = UniversalFormula;
    fn target_problem(&self) -> &Self::Target {
        &self.0
    }
    fn extract_value(&self, value: crate::types::And) -> crate::types::And {
        value
    }
}

impl ReduceToAggregate<UniversalFormula> for UniversalFormula {
    type Result = RenameUniversalVariable;
    fn reduce_to_aggregate(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(RenameUniversalVariable(Self {
            variable: 1 - self.variable,
            tautology: self.tautology,
        }))
    }
}

#[test]
fn universal_reduction_preserves_true_and_false_aggregates_without_witnesses() {
    use crate::solvers::BruteForce;
    for tautology in [false, true] {
        let source = UniversalFormula {
            variable: 0,
            tautology,
        };
        let reduction = source.reduce_to_aggregate().unwrap();
        let solver = BruteForce::new();
        let expected = solver.solve_cartesian(&source, |bits| bits).unwrap();
        let target_value = solver
            .solve_cartesian(reduction.target_problem(), |bits| bits)
            .unwrap();
        assert_eq!(expected, crate::types::And(tautology));
        assert_eq!(reduction.extract_value(target_value), expected);
        assert_eq!(
            reduction.extract_value_dyn(json!(tautology)).unwrap(),
            json!(tautology)
        );
        assert!(reduction.extract_value_dyn(json!("not a Boolean")).is_err());
    }
}
