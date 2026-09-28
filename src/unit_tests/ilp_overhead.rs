//! Numeric parameter contracts and composed ILP/QUBO workflows.
use crate::models::algebraic::{Bounded, ILP, QUBO};
use crate::models::graph::*;
use crate::models::misc::*;
use crate::parameters::ParameterRelation;
use crate::rules::{ReduceTo, ReductionGraph, ReductionPath, ReductionResult, ReductionStep};
use crate::solvers::{BruteForce, BruteForceProblem, ILPSolver};
use crate::topology::{DirectedGraph, SimpleGraph};
use crate::{
    types::{Min, One},
    Problem,
};

type BoundedILP = ILP<i64, i64, Bounded>;

#[test]
fn subset_sum_lattice_qubo_predictions_and_solution_recovery() {
    use crate::models::{algebraic::ClosestVectorProblem, Decision};
    for (sizes, target) in [(vec![1], 1), (vec![2], 1)] {
        check_path(
            SubsetSum::new(sizes, target),
            ReductionPath {
                steps: vec![
                    step::<SubsetSum>(),
                    step::<Decision<ClosestVectorProblem>>(),
                    step::<ClosestVectorProblem>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

#[test]
fn factoring_circuit_sat_qubo_predictions_and_solution_recovery() {
    use crate::models::formula::{CircuitSAT, NAESatisfiability, Satisfiability};
    for target in [1u32, 3] {
        check_path(
            Factoring::with_factor_bits(target, 1, 1),
            ReductionPath {
                steps: vec![
                    step::<Factoring>(),
                    step::<CircuitSAT>(),
                    step::<Satisfiability>(),
                    step::<NAESatisfiability>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

fn check_contract<S: Problem + ReduceTo<T>, T: Problem>(source: &S) {
    let target = source.reduce_to().unwrap();
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == S::NAME
                && entry.target_name == T::NAME
                && entry.source_variant() == S::variant()
                && entry.target_variant() == T::variant()
        })
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    assert!(
        contract.unavailable().is_empty(),
        "{} -> {}",
        S::NAME,
        T::NAME
    );
    let transform = contract.transform().unwrap();
    let predicted = transform.evaluate(&source.parameters()).unwrap();
    for (field, actual) in target.target_problem().parameters().iter() {
        let prediction = predicted.get(field).expect(field);
        match transform.relation(field).unwrap() {
            ParameterRelation::Exact => assert_eq!(prediction, actual, "{}: {field}", S::NAME),
            ParameterRelation::UpperBound => assert!(
                prediction >= actual,
                "{}: {field}: {prediction} < {actual}",
                S::NAME
            ),
        }
    }
}

fn step<P: Problem>() -> ReductionStep {
    ReductionStep {
        name: P::NAME.into(),
        variant: ReductionGraph::variant_to_map(&P::variant()),
    }
}

fn check_qubo<S, T>(source: S)
where
    S: BruteForceProblem + ReduceTo<T> + 'static,
    S::Solution: 'static,
    S::Value: PartialEq + std::fmt::Debug + crate::types::SolutionAggregate + 'static,
    T: Problem,
{
    check_contract::<S, T>(&source);
    let mut steps = vec![step::<S>(), step::<T>()];
    if T::variant() != ILP::<bool>::variant() {
        steps.push(step::<ILP<bool>>());
    }
    steps.push(step::<QUBO<i64>>());
    check_path(source, ReductionPath { steps });
}

fn check_path<S>(source: S, path: ReductionPath)
where
    S: BruteForceProblem + 'static,
    S::Solution: 'static,
    S::Value: PartialEq + std::fmt::Debug + crate::types::SolutionAggregate + 'static,
{
    let graph = ReductionGraph::new();
    let predicted = graph
        .compose_path_parameter_transform(&path)
        .unwrap()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
    let target = chain.target_problem::<QUBO<i64>>();
    for (field, actual) in target.parameters().iter() {
        assert!(
            predicted.get(field).expect(field) >= actual,
            "{}: {field}",
            S::NAME
        );
    }
    let expected = BruteForce::new().solve(&source).unwrap();
    let solution = ILPSolver::new().solve(target).unwrap();
    let recovered = chain.extract_solution::<S::Solution, _>(&solution);
    match expected {
        Some(witness) => assert_eq!(
            source.evaluate(&recovered.unwrap()).unwrap(),
            source.evaluate(&witness).unwrap()
        ),
        None => assert!(recovered.is_err()),
    }
}

#[test]
fn capacity_assignment_magnitude_and_qubo() {
    for (delay, budget, bits) in [
        (0, 0, 1),
        (7, 1, 3),
        (8, 1, 4),
        (1, 8, 4),
        (-8, 1, 4),
        (1, -8, 4),
        (i64::MIN, 0, 64),
        (0, i64::MIN, 64),
        (i64::MAX, 0, 63),
    ] {
        let source = CapacityAssignment::new(
            vec![i64::MAX],
            vec![vec![i64::MAX]],
            vec![vec![delay]],
            budget,
        );
        assert_eq!(source.parameters().get("max_delay_bits"), Some(bits));
        check_contract::<_, ILP<bool>>(&source);
    }
    check_contract::<_, ILP<bool>>(&CapacityAssignment::new(vec![1], vec![], vec![], 8));
    for budget in [-3, 0, 8] {
        check_qubo::<_, ILP<bool>>(CapacityAssignment::new(
            vec![1, 2],
            vec![vec![1, 3]],
            vec![vec![2, -2]],
            budget,
        ));
    }
}

#[test]
fn partially_ordered_knapsack_magnitude_and_qubo() {
    for (weight, capacity, bits) in [
        (0, 0, 1),
        (7, 1, 3),
        (8, 1, 4),
        (1, 8, 4),
        (i64::MAX, 0, 63),
        (0, i64::MAX, 63),
    ] {
        let source = PartiallyOrderedKnapsack::new(vec![weight], vec![i64::MAX], vec![], capacity);
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, ILP<bool>>(&source);
    }
    check_contract::<_, ILP<bool>>(&PartiallyOrderedKnapsack::new(vec![], vec![], vec![], 8));
    check_qubo::<_, ILP<bool>>(PartiallyOrderedKnapsack::new(
        vec![2, 3],
        vec![1, 9],
        vec![(0, 1)],
        3,
    ));
}

#[test]
fn constrained_path_magnitude_and_qubo() {
    for (weight, bound, bits) in [
        (1, 1, 1),
        (8, 1, 4),
        (1, 8, 4),
        (i64::MAX, 1, 63),
        (1, i64::MAX, 63),
    ] {
        let source = ShortestWeightConstrainedPath::new(
            SimpleGraph::path(2),
            vec![i64::MAX],
            vec![weight],
            0,
            1,
            bound,
        );
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    check_contract::<_, BoundedILP>(&ShortestWeightConstrainedPath::<_, i64>::new(
        SimpleGraph::empty(8),
        vec![],
        vec![],
        0,
        0,
        1,
    ));
    for bound in [1, 2] {
        check_qubo::<_, BoundedILP>(ShortestWeightConstrainedPath::new(
            SimpleGraph::path(2),
            vec![3],
            vec![2],
            0,
            1,
            bound,
        ));
    }
}

#[test]
fn bounded_forest_magnitude_and_incoming_qubo() {
    for (weights, bound, bits) in [
        (vec![], 8, 4),
        (vec![8], 1, 4),
        (vec![1], 8, 4),
        (vec![i64::MAX], 1, 63),
        (vec![1], i64::MAX, 63),
        (vec![0; 8], 1, 1),
    ] {
        let source = BoundedComponentSpanningForest::new(
            SimpleGraph::empty(weights.len()),
            weights,
            1,
            bound,
        );
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    check_qubo::<_, BoundedILP>(BoundedComponentSpanningForest::new(
        SimpleGraph::path(2),
        vec![1, 1],
        1,
        2,
    ));
    for graph in [SimpleGraph::empty(0), SimpleGraph::empty(3)] {
        let source = PartitionIntoPathsOfLength2::new(graph);
        check_contract::<_, BoundedComponentSpanningForest<SimpleGraph, i64>>(&source);
        check_path(
            source,
            ReductionPath {
                steps: vec![
                    step::<PartitionIntoPathsOfLength2<SimpleGraph>>(),
                    step::<BoundedComponentSpanningForest<SimpleGraph, i64>>(),
                    step::<BoundedILP>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

#[test]
fn acyclic_partition_magnitude_and_qubo() {
    for (weight, bound, bits) in [(0, 1, 1), (8, 1, 4), (1, -8, 4), (i64::MIN, 1, 64)] {
        let source = AcyclicPartition::new(DirectedGraph::empty(1), vec![weight], vec![], bound, 0);
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(bits)
        );
        check_contract::<_, ILP<bool>>(&source);
    }
    check_contract::<_, ILP<bool>>(&AcyclicPartition::new(
        DirectedGraph::new(1, vec![(0, 0)]),
        vec![1],
        vec![1],
        1,
        1,
    ));
    for bound in [1, 3] {
        check_qubo::<_, ILP<bool>>(AcyclicPartition::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1, 2],
            vec![1],
            bound,
            0,
        ));
    }
    use crate::models::formula::{CNFClause, KSatisfiability};
    use crate::variant::K3;
    for clauses in [
        vec![],
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
        vec![CNFClause::new(vec![1, 1, 1])],
    ] {
        check_contract::<_, AcyclicPartition<i64>>(&KSatisfiability::<K3>::new(1, clauses));
    }
}

#[test]
fn branching_magnitude_and_qubo() {
    for (weight, threshold, bits) in [(1, 8, 4), (8, 1, 4), (i64::MIN, 0, 64), (1, i64::MIN, 64)] {
        let source = MultipleChoiceBranching::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![weight],
            vec![vec![0]],
            threshold,
        );
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    for threshold in [1, 3] {
        check_qubo::<_, BoundedILP>(MultipleChoiceBranching::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![2],
            vec![vec![0]],
            threshold,
        ));
    }
}

#[test]
fn capacitated_tree_magnitude_and_qubo() {
    let source = MinimumCapacitatedSpanningTree::new(
        SimpleGraph::path(3),
        vec![i64::MAX; 2],
        0,
        vec![0, 7, 7],
        8,
    );
    assert_eq!(source.parameters().get("max_requirement_bits"), Some(4));
    check_contract::<_, BoundedILP>(&source);
    for capacity in [1, 2] {
        check_qubo::<_, BoundedILP>(MinimumCapacitatedSpanningTree::new(
            SimpleGraph::path(2),
            vec![3],
            0,
            vec![0, 2],
            capacity,
        ));
    }
}

#[test]
fn multicenter_magnitude_products_and_qubo() {
    let source = MinMaxMulticenter::new(SimpleGraph::path(3), vec![8; 3], vec![8; 2], 1);
    assert_eq!(
        source.parameters().get("max_numeric_magnitude_bits"),
        Some(4)
    );
    check_contract::<_, BoundedILP>(&source);
    for graph in [SimpleGraph::path(2), SimpleGraph::empty(2)] {
        let lengths = vec![2; crate::topology::Graph::num_edges(&graph)];
        check_qubo::<_, BoundedILP>(MinMaxMulticenter::new(graph, vec![2, 1], lengths, 1));
    }
}

#[test]
fn multicenter_distance_overflow_is_typed() {
    let source = MinMaxMulticenter::new(SimpleGraph::path(3), vec![1; 3], vec![i64::MAX; 2], 1);
    assert!(matches!(
        source.evaluate(&vec![true, false, false]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
    assert!(matches!(
        ReduceTo::<BoundedILP>::reduce_to(&source),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn flow_shop_magnitude_and_qubo() {
    for (time, deadline, bits) in [(1, 8, 4), (8, 1, 4), (0, 0, 1), (7, 7, 3)] {
        let source = FlowShopScheduling::new(1, vec![vec![time]; 2], deadline);
        assert_eq!(source.parameters().get("max_time_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    check_contract::<_, BoundedILP>(&FlowShopScheduling::new(0, vec![vec![]; 2], 8));
    for deadline in [1, 2] {
        check_qubo::<_, BoundedILP>(FlowShopScheduling::new(1, vec![vec![1]; 2], deadline));
    }
}

#[test]
fn minimum_tardiness_negative_deadlines_preserve_optimum() {
    // Every schedule has exactly one tardy task, even for the smallest deadline.
    for deadline in [-1, i64::MIN] {
        let source = MinimumTardinessSequencing::<One>::new(1, vec![deadline], vec![]);
        assert_eq!(source.evaluate(&vec![0]).unwrap(), Min(Some(1)));
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let witness = ILPSolver::new().solve(reduced.target_problem()).unwrap();
        assert_eq!(
            source
                .evaluate(&reduced.extract_solution(&witness).unwrap())
                .unwrap(),
            Min(Some(1))
        );
        let source =
            MinimumTardinessSequencing::<i64>::with_lengths(vec![2], vec![deadline], vec![]);
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let witness = ILPSolver::new().solve(reduced.target_problem()).unwrap();
        assert_eq!(
            source
                .evaluate(&reduced.extract_solution(&witness).unwrap())
                .unwrap(),
            Min(Some(1))
        );
    }
}

#[test]
fn minimum_tardiness_magnitude_and_qubo() {
    for deadline in [i64::MIN, 0, i64::MAX] {
        check_qubo::<_, ILP<bool>>(MinimumTardinessSequencing::<One>::new(
            2,
            vec![deadline, 1],
            vec![(0, 1)],
        ));
        let source = MinimumTardinessSequencing::<i64>::with_lengths(
            vec![2, 1],
            vec![deadline, 2],
            vec![(0, 1)],
        );
        assert_eq!(source.parameters().get("max_processing_time_bits"), Some(2));
        check_qubo::<_, ILP<bool>>(source);
    }
}

#[test]
fn completion_time_magnitude_and_qubo() {
    let source = SchedulingToMinimizeWeightedCompletionTime::new(vec![7, 7], vec![i64::MAX; 2], 2);
    assert_eq!(source.parameters().get("max_processing_time_bits"), Some(3));
    check_contract::<_, BoundedILP>(&source);
    check_qubo::<_, BoundedILP>(SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2],
        vec![2, 1],
        2,
    ));
    let source =
        SequencingToMinimizeWeightedCompletionTime::new(vec![7, 7], vec![i64::MAX; 2], vec![]);
    assert_eq!(source.parameters().get("max_processing_time_bits"), Some(3));
    check_contract::<_, BoundedILP>(&source);
    check_qubo::<_, BoundedILP>(SequencingToMinimizeWeightedCompletionTime::new(
        vec![1, 2],
        vec![1, 2],
        vec![(0, 1)],
    ));
    let source = OptimalLinearArrangement::new(SimpleGraph::path(2));
    check_contract::<_, SequencingToMinimizeWeightedCompletionTime>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<OptimalLinearArrangement<SimpleGraph>>(),
                step::<SequencingToMinimizeWeightedCompletionTime>(),
                step::<BoundedILP>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
}

#[test]
fn cumulative_cost_magnitude_and_qubo() {
    let source = SequencingToMinimizeMaximumCumulativeCost::new(vec![-8, 8], vec![]);
    assert_eq!(source.parameters().get("max_cost_bits"), Some(4));
    check_contract::<_, BoundedILP>(&source);
    for costs in [vec![-2, 3], vec![-2, -1], vec![]] {
        check_qubo::<_, BoundedILP>(SequencingToMinimizeMaximumCumulativeCost::new(
            costs,
            vec![],
        ));
    }
}

#[test]
fn tardy_task_weight_magnitude_and_incoming_qubo() {
    for deadlines in [vec![i64::MIN; 2], vec![i64::MAX; 2]] {
        let source = SequencingToMinimizeTardyTaskWeight::new(vec![-1, 2], vec![1, -1], deadlines);
        assert_eq!(source.parameters().get("max_processing_time_bits"), Some(2));
        check_qubo::<_, ILP<bool>>(source);
    }
    use crate::models::Decision;
    let source = Partition::new(vec![1, 1]).unwrap();
    check_contract::<_, Decision<SequencingToMinimizeTardyTaskWeight>>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<Partition>(),
                step::<Decision<SequencingToMinimizeTardyTaskWeight>>(),
                step::<SequencingToMinimizeTardyTaskWeight>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
}

#[test]
fn weighted_tardiness_magnitude_and_qubo() {
    for (length, weight, deadline, bound) in
        [(8, 1, 1, 1), (1, 8, 1, 1), (1, 1, 8, 1), (1, 1, 1, 8)]
    {
        let source = SequencingToMinimizeWeightedTardiness::new(
            vec![length],
            vec![weight],
            vec![deadline],
            bound,
        );
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(4)
        );
        check_contract::<_, BoundedILP>(&source);
    }
    for bound in [0, 2] {
        check_qubo::<_, BoundedILP>(SequencingToMinimizeWeightedTardiness::new(
            vec![2],
            vec![1],
            vec![1],
            bound,
        ));
    }
}

#[test]
fn setup_time_magnitude_and_qubo() {
    for (length, deadline, setup) in [(8, 1, 1), (1, 8, 1), (1, 1, 8)] {
        let source = SequencingWithDeadlinesAndSetUpTimes::new(
            vec![length; 2],
            vec![deadline; 2],
            vec![0, 1],
            vec![setup; 2],
        );
        assert_eq!(source.parameters().get("max_time_bits"), Some(4));
        check_contract::<_, ILP<bool>>(&source);
    }
    for deadline in [2, 3] {
        check_qubo::<_, ILP<bool>>(SequencingWithDeadlinesAndSetUpTimes::new(
            vec![1; 2],
            vec![deadline; 2],
            vec![0, 1],
            vec![1; 2],
        ));
    }
}

#[test]
fn resource_scheduling_packs_slots_and_ignores_excess_processors() {
    let source = ResourceConstrainedScheduling::new(2, vec![2], vec![vec![1]; 2], 1000).unwrap();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(reduction.target_problem().num_vars() <= 4);
    // These instances must not allocate or iterate through the numeric deadline.
    let source =
        ResourceConstrainedScheduling::new(usize::MAX, vec![2], vec![vec![1]; 2], i64::MAX)
            .unwrap();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(reduction.target_problem().num_vars() <= 4);
    assert!(
        source
            .evaluate(&vec![0, (i64::MAX - 1) as usize])
            .unwrap()
            .0
    );
    let witness = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    assert!(
        source
            .evaluate(&reduction.extract_solution(&witness).unwrap())
            .unwrap()
            .0
    );
    let empty = ResourceConstrainedScheduling::new(0, vec![0], vec![], i64::MAX).unwrap();
    assert!(empty.evaluate(&vec![]).unwrap().0);
    assert_eq!(
        ReduceTo::<ILP<bool>>::reduce_to(&empty)
            .unwrap()
            .target_problem()
            .num_vars(),
        0
    );
}

#[test]
fn resource_magnitude_and_incoming_predictions() {
    for (requirement, bound, bits) in [
        (0, 0, 1),
        (8, 1, 4),
        (1, 8, 4),
        (i64::MAX, 0, 63),
        (0, i64::MAX, 63),
    ] {
        let source =
            ResourceConstrainedScheduling::new(2, vec![bound], vec![vec![requirement]], 2).unwrap();
        assert_eq!(source.parameters().get("max_resource_bits"), Some(bits));
        check_contract::<_, ILP<bool>>(&source);
    }
    for processors in [0, 1, 2] {
        check_qubo::<_, ILP<bool>>(
            ResourceConstrainedScheduling::new(processors, vec![1], vec![vec![1]; 2], 2).unwrap(),
        );
    }
    let source = ThreePartition::new(vec![1; 3], 3);
    check_contract::<_, ResourceConstrainedScheduling>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<ThreePartition>(),
                step::<ResourceConstrainedScheduling>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
    use crate::models::set::ThreeDimensionalMatching;
    for (size, triples) in [
        (0, vec![]),
        (1, vec![]),
        (1, vec![(0, 0, 0)]),
        (2, vec![(0, 0, 0), (1, 1, 1)]),
    ] {
        let source = ThreeDimensionalMatching::new(size, triples);
        check_contract::<_, ThreePartition>(&source);
        let path = ReductionPath {
            steps: vec![
                step::<ThreeDimensionalMatching>(),
                step::<ThreePartition>(),
                step::<ResourceConstrainedScheduling>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        };
        let predicted = ReductionGraph::new()
            .compose_path_parameter_transform(&path)
            .unwrap()
            .unwrap()
            .evaluate(&source.parameters())
            .unwrap();
        assert!(predicted.get("num_vars").is_some());
        assert!(predicted.get("num_quadratic_terms").is_some());
    }
}

#[test]
fn circuit_threshold_normalization_and_incoming_qubo() {
    use crate::models::Decision;
    for threshold in [i64::MIN, 0, 3, 4, i64::MAX] {
        let source = Decision::new(
            LongestCircuit::new(SimpleGraph::cycle(3), vec![1i64; 3]),
            threshold,
        );
        assert_eq!(source.parameters().get("max_length_bits"), Some(1));
        check_contract::<_, ILP<bool>>(&source);
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert!(reduced.target_problem().max_constraint_magnitude_bits() <= 5);
        assert_eq!(
            ILPSolver::new().solve(reduced.target_problem()).is_ok(),
            threshold <= 3
        );
    }
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    check_contract::<_, Decision<LongestCircuit<SimpleGraph, i64>>>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<HamiltonianCircuit<SimpleGraph>>(),
                step::<Decision<LongestCircuit<SimpleGraph, i64>>>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
}

#[test]
fn multiprocessor_predictions_compose_and_recover_through_qubo() {
    for sizes in [vec![1, 1], vec![3; 4], vec![1, 2], vec![2]] {
        check_path(
            Partition::new(sizes).unwrap(),
            ReductionPath {
                steps: vec![
                    step::<Partition>(),
                    step::<MultiprocessorScheduling>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}
