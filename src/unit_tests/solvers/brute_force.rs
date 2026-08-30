use super::*;
use crate::traits::Problem;
use crate::types::{AggregationError, Max, Min, Or, Sum};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct MaxSumProblem {
    weights: Vec<i64>,
}

impl Problem for MaxSumProblem {
    const NAME: &'static str = "MaxSumProblem";
    type Solution = Vec<usize>;
    type Value = Max<i64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Max(Some(
                config
                    .iter()
                    .zip(&self.weights)
                    .map(|(&c, &w)| if c == 1 { w } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for MaxSumProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct MinSumProblem {
    weights: Vec<i64>,
}

impl Problem for MinSumProblem {
    const NAME: &'static str = "MinSumProblem";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Min(Some(
                config
                    .iter()
                    .zip(&self.weights)
                    .map(|(&c, &w)| if c == 1 { w } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for MinSumProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct SatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl Problem for SatProblem {
    const NAME: &'static str = "SatProblem";
    type Solution = Vec<usize>;
    type Value = Or;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Or(self.satisfying.iter().any(|s| s == config)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "bool")]
    }
}

impl crate::solvers::BruteForceProblem for SatProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct EvaluationFailureProblem;

impl Problem for EvaluationFailureProblem {
    const NAME: &'static str = "EvaluationFailureProblem";
    type Solution = Vec<usize>;
    type Value = Or;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(&self, config: &Self::Solution) -> Result<Or, crate::traits::EvaluationError> {
        if config.as_slice() == [1] {
            Err(crate::traits::EvaluationError::IntegerOverflow(
                "evaluating test configuration".to_string(),
            ))
        } else {
            Ok(Or(false))
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for EvaluationFailureProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2]
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AggregationFailureProblem;

impl Problem for AggregationFailureProblem {
    const NAME: &'static str = "AggregationFailureProblem";
    type Solution = Vec<usize>;
    type Value = Max<f64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(&self, _: &Self::Solution) -> Result<Max<f64>, crate::traits::EvaluationError> {
        Ok(Max(Some(f64::NAN)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregationFailureProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2]
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct CountingSatProblem {
    #[serde(skip)]
    evaluations: Rc<Cell<usize>>,
}

impl Problem for CountingSatProblem {
    const NAME: &'static str = "CountingSatProblem";
    type Solution = Vec<usize>;
    type Value = Or;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            self.evaluations.set(self.evaluations.get() + 1);
            Or(config.as_slice() == [0, 0])
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for CountingSatProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2, 2]
    }
}

crate::declare_variants! {
    default MaxSumProblem => "2^num_variables",
    default MinSumProblem => "2^num_variables",
    default SatProblem => "2^num_variables",
    default EvaluationFailureProblem => "2^num_variables",
    default AggregationFailureProblem => "2^num_variables",
    default CountingSatProblem => "2^num_variables",
}

crate::register_brute_force! {
    MaxSumProblem,
    MinSumProblem,
    SatProblem,
    EvaluationFailureProblem,
    AggregationFailureProblem,
    CountingSatProblem,
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "MaxSumProblem",
        display_name: "Maximum Sum Test Problem",
        aliases: &[],
        dimensions: &[
            crate::registry::VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            crate::registry::VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem for maximum aggregation",
        fields: &[],
    }
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "MinSumProblem",
        display_name: "Minimum Sum Test Problem",
        aliases: &[],
        dimensions: &[
            crate::registry::VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            crate::registry::VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem for minimum aggregation",
        fields: &[],
    }
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "SatProblem",
        display_name: "Satisfaction Test Problem",
        aliases: &[],
        dimensions: &[
            crate::registry::VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            crate::registry::VariantDimension::new("weight", "bool", &["bool"]),
        ],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem for satisfaction aggregation",
        fields: &[],
    }
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "EvaluationFailureProblem",
        display_name: "Evaluation Failure Test Problem",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem that exposes evaluation failures",
        fields: &[],
    }
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "AggregationFailureProblem",
        display_name: "Aggregation Failure Test Problem",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem that exposes aggregation failures",
        fields: &[],
    }
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "CountingSatProblem",
        display_name: "Counting Satisfaction Test Problem",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem that counts reference evaluations",
        fields: &[],
    }
}

#[test]
fn test_solver_solves_max_value() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Max(Some(6))
    );
}

#[test]
fn test_solver_solves_min_value() {
    let problem = MinSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Min(Some(0))
    );
}

#[test]
fn test_solver_solves_satisfaction_value() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Or(true)
    );
}

#[test]
fn test_solver_solve() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&problem).unwrap(), Some(vec![1, 1, 1]));
}

#[test]
fn test_solver_solve_for_satisfaction_problem() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let witness = solver.solve(&problem).unwrap();
    assert!(witness.is_some());
    assert_eq!(problem.evaluate(&witness.unwrap()).unwrap(), Or(true));
}

#[test]
fn test_solver_solve_stops_after_first_optimal_configuration() {
    let evaluations = Rc::new(Cell::new(0));
    let problem = CountingSatProblem {
        evaluations: Rc::clone(&evaluations),
    };

    assert_eq!(BruteForce::new().solve(&problem).unwrap(), Some(vec![0, 0]));
    // The absorbing aggregate and the witness pass both stop at the first
    // satisfying configuration.
    assert_eq!(evaluations.get(), 2);
}

#[test]
fn test_sum_fold_combines_values_without_problem_solving() {
    let total = [Sum(1_u64), Sum(2), Sum(3)]
        .into_iter()
        .try_fold(Sum::identity(), Aggregate::combine)
        .unwrap();
    assert_eq!(total, Sum(6));
}

#[test]
fn test_solver_find_all_witnesses() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let witnesses = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(witnesses.len(), 2);
    assert!(witnesses.contains(&vec![1, 0]));
    assert!(witnesses.contains(&vec![0, 1]));
}

#[test]
fn test_sum_fold_uses_every_input_value() {
    let total = [Sum(0_u64), Sum(2), Sum(1), Sum(3)]
        .into_iter()
        .try_fold(Sum::identity(), Aggregate::combine)
        .unwrap();
    assert_eq!(total, Sum(6));
}

#[test]
fn test_solver_with_real_mis() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i64; 3],
    );
    let solver = BruteForce::new();

    let best = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(best.len(), 3);
    for sol in &best {
        assert_eq!(sol.iter().filter(|&&selected| selected).count(), 1);
        assert!(problem.evaluate(sol).unwrap().is_valid());
    }
}

#[test]
fn test_solver_with_real_sat() {
    use crate::models::formula::{CNFClause, Satisfiability};
    use crate::traits::Problem;

    let problem = Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_solve_with_witnesses_max() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let (value, witnesses) = solver.solve_with_witnesses(&problem).unwrap();
    assert_eq!(value, Max(Some(6)));
    assert_eq!(witnesses, vec![vec![1, 1, 1]]);
}

#[test]
fn test_sum_fold_preserves_zero_identity() {
    assert_eq!(Sum::<u64>::identity().combine(Sum(6)).unwrap(), Sum(6));
}

#[test]
fn solve_with_witnesses_enumerates_only_aggregate_and_witness_passes() {
    let evaluations = Rc::new(Cell::new(0));
    let problem = CountingSatProblem {
        evaluations: Rc::clone(&evaluations),
    };

    let (value, witnesses) = BruteForce::new().solve_with_witnesses(&problem).unwrap();

    assert_eq!(value, Or(true));
    assert_eq!(witnesses, vec![vec![0, 0]]);
    // One evaluation reaches the absorbing aggregate; the witness pass then
    // enumerates all four configurations.
    assert_eq!(evaluations.get(), 5);
}

#[test]
fn test_solver_trait_solve() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Max(Some(6))
    );
}

#[test]
fn test_solver_preserves_evaluation_errors() {
    let error = BruteForce::new()
        .solve(&EvaluationFailureProblem)
        .unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::SolveError::Evaluation(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn test_solver_preserves_aggregation_errors() {
    let error = BruteForce::new()
        .solve(&AggregationFailureProblem)
        .unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::SolveError::Aggregation(AggregationError::UnorderedComparison)
    ));
}

#[test]
fn cartesian_indices_enumerates_mixed_dimensions() {
    let indices = CartesianIndices::new(vec![2, 3])
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        indices,
        vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 1],
            vec![1, 2],
        ]
    );
}

#[test]
fn cartesian_indices_empty_dimensions_have_one_candidate() {
    assert_eq!(
        CartesianIndices::new(vec![]).unwrap().collect::<Vec<_>>(),
        vec![Vec::<usize>::new()]
    );
}

#[test]
fn cartesian_indices_zero_dimension_has_no_candidates() {
    assert!(CartesianIndices::new(vec![2, 0, 3])
        .unwrap()
        .next()
        .is_none());
}

#[test]
fn cartesian_indices_is_exact_size() {
    let mut indices = CartesianIndices::new(vec![2, 3]).unwrap();
    assert_eq!(indices.len(), 6);
    indices.next();
    assert_eq!(indices.len(), 5);
}

#[test]
fn cartesian_indices_reports_cardinality_overflow() {
    assert!(matches!(
        CartesianIndices::new(vec![usize::MAX, 2]),
        Err(crate::solvers::SolveError::SearchSpaceOverflow(dimensions))
            if dimensions == vec![usize::MAX, 2]
    ));
}
