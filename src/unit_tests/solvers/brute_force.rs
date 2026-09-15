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

    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", self.weights.len() as u64)])
    }

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
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.weights.len())
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
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

    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", self.weights.len() as u64)])
    }

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
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.weights.len())
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
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

    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", self.num_vars as u64)])
    }

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
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.num_vars)
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct EvaluationFailureProblem;

impl Problem for EvaluationFailureProblem {
    const NAME: &'static str = "EvaluationFailureProblem";
    type Solution = Vec<usize>;
    type Value = Or;

    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", 1usize as u64)])
    }

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
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(1usize)
    }

    fn dimension(&self, variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok([2][variable])
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AggregationFailureProblem;

impl Problem for AggregationFailureProblem {
    const NAME: &'static str = "AggregationFailureProblem";
    type Solution = Vec<usize>;
    type Value = Max<f64>;

    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", 1usize as u64)])
    }

    fn evaluate(&self, _: &Self::Solution) -> Result<Max<f64>, crate::traits::EvaluationError> {
        Ok(Max(Some(f64::NAN)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregationFailureProblem {
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(1usize)
    }

    fn dimension(&self, variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok([2][variable])
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

    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", 2usize as u64)])
    }

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
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
    }

    fn dimension(&self, variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok([2, 2][variable])
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
fn cartesian_indices_reports_exhaustion_without_a_total_count() {
    let mut indices = CartesianIndices::new(vec![2, 3]).unwrap();
    assert_eq!(indices.size_hint(), (1, None));
    assert_eq!(indices.by_ref().count(), 6);
    assert_eq!(indices.size_hint(), (0, Some(0)));
    assert_eq!(indices.next(), None);
}

#[test]
fn cartesian_indices_visits_a_prefix_when_the_total_exceeds_usize() {
    let prefix = CartesianIndices::new(vec![usize::MAX, 2])
        .unwrap()
        .take(4)
        .collect::<Vec<_>>();
    assert_eq!(prefix, vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]);
}

#[test]
fn enumeration_reports_coordinate_count_and_storage_errors() {
    use crate::models::set::SetBasis;
    let count_overflow = SetBasis::new(2, vec![], usize::MAX);
    assert!(matches!(
        BruteForceProblem::num_variables(&count_overflow),
        Err(SolveError::IntegerOverflow(_))
    ));
    assert!(matches!(
        BruteForce::new().solve(&count_overflow),
        Err(SolveError::IntegerOverflow(_))
    ));
    let allocation_overflow = SetBasis::new(1, vec![], usize::MAX);
    assert!(matches!(
        cartesian_dimensions(&allocation_overflow),
        Err(SolveError::Allocation(_))
    ));
}

#[test]
fn window_product_does_not_restrict_construction_or_evaluation() {
    use crate::models::misc::ClosestSubstring;
    let problem = ClosestSubstring::new(1, vec![vec![0, 0]; 64], 1).unwrap();
    let restored: ClosestSubstring =
        serde_json::from_value(serde_json::to_value(&problem).unwrap()).unwrap();
    assert_eq!(restored.evaluate(&vec![0; 65]).unwrap(), Min(Some(0)));
    assert_eq!(restored.parameters(), problem.parameters());
    let dimensions = crate::solvers::cartesian_dimensions(&restored).unwrap();
    assert_eq!(dimensions[0], 1);
    assert_eq!(&dimensions[1..], &[2; 64]);
    let prefix: Vec<_> = CartesianIndices::new(dimensions).unwrap().take(2).collect();
    assert_eq!(prefix.len(), 2);
    for witness in prefix {
        assert_eq!(restored.evaluate(&witness).unwrap(), Min(Some(0)));
    }
}

#[test]
fn scalar_counts_report_unrepresentable_search_coordinates() {
    use crate::models::algebraic::BMF;
    use crate::models::misc::{ConsistencyOfDatabaseFrequencyTables, EnsembleComputation};
    let cases = [
        (
            "biclique slots",
            crate::models::graph::BicliqueCover::new(
                crate::topology::BipartiteGraph::new(1, 1, vec![(0, 0)]),
                usize::MAX,
            )
            .num_variables(),
        ),
        (
            "tree slots",
            crate::models::graph::KthBestSpanningTree::new(
                crate::topology::SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1i64, 1],
                usize::MAX,
                2,
            )
            .num_variables(),
        ),
        (
            "tile slots",
            crate::models::misc::SquareTiling::new(1, vec![(0, 0, 0, 0)], usize::MAX)
                .num_variables(),
        ),
        (
            "factor rows",
            BMF::new(vec![vec![true]; 2], usize::MAX).num_variables(),
        ),
        (
            "factor columns",
            BMF::new(vec![vec![true; 2]], usize::MAX).num_variables(),
        ),
        (
            "factor sum",
            BMF::new(vec![vec![true]], usize::MAX).num_variables(),
        ),
        (
            "operation operands",
            EnsembleComputation::new(1, vec![], usize::MAX).num_variables(),
        ),
        (
            "database entries",
            ConsistencyOfDatabaseFrequencyTables::new(usize::MAX, vec![1, 1], vec![], vec![])
                .num_variables(),
        ),
    ];
    for (context, result) in cases {
        assert!(
            matches!(result, Err(SolveError::IntegerOverflow(_))),
            "{context}: {result:?}"
        );
    }
}

#[test]
fn scalar_domains_report_unrepresentable_coordinate_cardinalities() {
    use crate::models::misc::{
        ConjunctiveQueryFoldability, EnsembleComputation, MinimumExternalMacroDataCompression,
        MinimumInternalMacroDataCompression,
    };
    let external = MinimumExternalMacroDataCompression::new(usize::MAX, vec![0], 1);
    let cases = [
        ("external symbol", external.dimension(0)),
        ("external pointer", external.dimension(1)),
        (
            "internal alphabet",
            MinimumInternalMacroDataCompression::new(usize::MAX, vec![0], 1).dimension(0),
        ),
        (
            "internal sentinel",
            MinimumInternalMacroDataCompression::new(usize::MAX - 1, vec![0], 1).dimension(0),
        ),
        (
            "operand labels",
            EnsembleComputation::new(usize::MAX, vec![], 1).dimension(0),
        ),
        (
            "distinguished labels",
            ConjunctiveQueryFoldability::new(usize::MAX, 1, 1, vec![], vec![], vec![]).dimension(0),
        ),
        (
            "undistinguished labels",
            ConjunctiveQueryFoldability::new(usize::MAX, 0, 1, vec![], vec![], vec![]).dimension(0),
        ),
    ];
    for (context, result) in cases {
        assert!(
            matches!(result, Err(SolveError::IntegerOverflow(_))),
            "{context}: {result:?}"
        );
    }
}

#[test]
fn string_domains_reserve_a_representable_sentinel() {
    use crate::models::misc::{
        LongestCommonSubsequence, ShortestCommonSupersequence, ShortestCommonSuperstring,
    };
    use crate::models::set::ConsecutiveSets;
    let cases = [
        (
            "subsequence",
            LongestCommonSubsequence::new(usize::MAX, vec![vec![0]]).dimension(0),
        ),
        (
            "supersequence",
            ShortestCommonSupersequence::new(usize::MAX, vec![vec![0]]).dimension(0),
        ),
        (
            "superstring",
            ShortestCommonSuperstring::new(usize::MAX, vec![vec![0]]).dimension(0),
        ),
        (
            "consecutive sets",
            ConsecutiveSets::new(usize::MAX, vec![vec![0]], 1).dimension(0),
        ),
    ];
    for (context, result) in cases {
        assert!(
            matches!(result, Err(SolveError::IntegerOverflow(_))),
            "{context}: {result:?}"
        );
    }
}

#[test]
fn decision_tree_slots_fail_before_enumeration_storage_is_allocated() {
    use crate::models::misc::MinimumDecisionTree;
    let objects = usize::BITS as usize + 1;
    let tests = objects.ilog2() as usize + 1;
    let matrix = (0..tests)
        .map(|bit| {
            (0..objects)
                .map(|object| object & (1 << bit) != 0)
                .collect()
        })
        .collect();
    let problem = MinimumDecisionTree::new(matrix, objects, tests);
    assert!(matches!(
        cartesian_dimensions(&problem),
        Err(SolveError::Evaluation(
            crate::traits::EvaluationError::IntegerOverflow(_)
        ))
    ));
}

#[test]
fn large_products_remain_symbolic_in_model_parameters() {
    use crate::models::misc::{
        ConsistencyOfDatabaseFrequencyTables, MinimumDiscretePlanarInverseKinematics,
    };
    let arm = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0; 64],
        (64.0, 0.0),
        vec![vec![0.0, 1.0]; 64],
        vec![vec![(0, 0), (0, 1), (1, 0), (1, 1)]; 63],
    )
    .unwrap();
    let restored: MinimumDiscretePlanarInverseKinematics =
        serde_json::from_value(serde_json::to_value(&arm).unwrap()).unwrap();
    assert_eq!(arm.parameters(), restored.parameters());
    assert_eq!(arm.evaluate(&vec![0; 64]).unwrap(), Min(Some(0.0)));
    assert_eq!(
        CartesianIndices::new(cartesian_dimensions(&arm).unwrap())
            .unwrap()
            .take(2)
            .count(),
        2
    );
    let database = ConsistencyOfDatabaseFrequencyTables::new(1, vec![2; 64], vec![], vec![]);
    let restored: ConsistencyOfDatabaseFrequencyTables =
        serde_json::from_value(serde_json::to_value(&database).unwrap()).unwrap();
    assert_eq!(database.parameters(), restored.parameters());
    assert_eq!(database.evaluate(&vec![0; 64]).unwrap(), Or(true));
}

#[test]
fn test_max_solution_selection() {
    assert!(Max::contributes_to_solution(&Max(Some(7)), &Max(Some(7))));
    assert!(!Max::contributes_to_solution(&Max(Some(3)), &Max(Some(7))));
    assert!(!Max::contributes_to_solution(&Max(None), &Max(Some(7))));
}

#[test]
fn test_min_solution_selection() {
    assert!(Min::contributes_to_solution(&Min(Some(3)), &Min(Some(3))));
    assert!(!Min::contributes_to_solution(&Min(Some(7)), &Min(Some(3))));
    assert!(!Min::contributes_to_solution(&Min(None), &Min(Some(3))));
}

#[test]
fn test_or_solution_selection() {
    assert!(Or::contributes_to_solution(&Or(true), &Or(true)));
    assert!(!Or::contributes_to_solution(&Or(false), &Or(true)));
    assert!(!Or::contributes_to_solution(&Or(true), &Or(false)));
}

#[test]
fn test_extremum_solution_selection() {
    // Matching value and sense -> contributes
    assert!(Extremum::contributes_to_solution(
        &Extremum::maximize(Some(10)),
        &Extremum::maximize(Some(10)),
    ));

    // Different value -> does not contribute
    assert!(!Extremum::contributes_to_solution(
        &Extremum::maximize(Some(5)),
        &Extremum::maximize(Some(10)),
    ));

    // None config -> does not contribute
    assert!(!Extremum::contributes_to_solution(
        &Extremum::<i64>::maximize(None),
        &Extremum::maximize(Some(10)),
    ));
}

#[test]
fn test_minimumcutintoboundedsets_selects_optimal_solutions() {
    type Value = <crate::models::graph::MinimumCutIntoBoundedSets<crate::topology::SimpleGraph, i64> as Problem>::Value;
    assert!(Value::contributes_to_solution(&Min(Some(3)), &Min(Some(3))));
}
