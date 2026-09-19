use crate::models::algebraic::MinimumWeightDecoding;
use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::graph::{
    BottleneckTravelingSalesman, HamiltonianCircuit, MinimumVertexCover, TravelingSalesman,
};
use crate::models::misc::{
    BinPacking, Knapsack, MinimumAxiomSet, MinimumFaultDetectionTestSet, Partition,
    SumOfSquaresPartition,
};
use crate::models::set::{ExactCoverBy3Sets, MaximumSetPacking, ThreeDimensionalMatching};
use crate::rules::{AggregateReductionResult, ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Aggregate, Extremum, One, Or, SolutionAggregate};
use crate::variant::K3;

#[test]
fn decision_graph_encodings_preserve_small_and_native_graph_cases() {
    use crate::models::graph::{
        BalancedCompleteBipartiteSubgraph, KClique, KColoring, PartitionIntoCliques, RuralPostman,
        StrongConnectivityAugmentation, SubgraphIsomorphism,
    };
    use crate::models::misc::{Clustering, ConjunctiveBooleanQuery};
    use crate::variant::KN;
    for graph in [
        SimpleGraph::empty(0),
        SimpleGraph::empty(1),
        SimpleGraph::path(2),
        SimpleGraph::path(3),
        SimpleGraph::cycle(3),
        SimpleGraph::new(3, vec![(0, 0), (0, 1), (0, 1)]),
    ] {
        check_decision::<_, StrongConnectivityAugmentation<i64>>(&HamiltonianCircuit::new(
            graph.clone(),
        ));
        check_decision::<_, RuralPostman<SimpleGraph, i64>>(&HamiltonianCircuit::new(
            graph.clone(),
        ));
        check_decision::<_, Clustering>(&KColoring::<K3, _>::new(graph.clone()));
        check_decision::<_, PartitionIntoCliques<SimpleGraph>>(&KColoring::<KN, _>::with_k(
            graph.clone(),
            4,
        ));
        for k in 1..=graph.num_vertices() {
            let source = KClique::new(graph.clone(), k);
            check_decision::<_, ConjunctiveBooleanQuery>(&source);
            check_decision::<_, BalancedCompleteBipartiteSubgraph>(&source);
            check_decision::<_, SubgraphIsomorphism>(&source);
        }
    }
}

fn check_decision<S, T>(source: &S)
where
    S: Problem<Value = Or> + ReduceTo<T> + 'static,
    T: Problem + 'static,
    S::Solution: 'static,
    T::Solution: 'static,
    T::Value: SolutionAggregate,
    <S as ReduceTo<T>>::Result: AggregateReductionResult<Source = S, Target = T>,
{
    let reduction = source.reduce_to().unwrap();
    let target = ReductionResult::target_problem(&reduction);
    let (total, witnesses) = BruteForce::new().solve_with_witnesses(target).unwrap();
    let expected = Or(BruteForce::new().solve(source).unwrap().is_some());
    assert_eq!(reduction.extract_value(total), expected);
    for witness in witnesses {
        let decoded = reduction.extract_solution(&witness);
        if expected.0 {
            assert_eq!(source.evaluate(&decoded.unwrap()).unwrap(), expected);
        } else {
            assert!(
                decoded.is_err(),
                "a NO result must not produce a source witness"
            );
        }
    }
}

fn check_binary_ilp<S>(source: &S)
where
    S: Problem<Value = Or> + ReduceTo<crate::models::algebraic::ILP<bool>> + 'static,
    S::Solution: 'static,
    <S as ReduceTo<crate::models::algebraic::ILP<bool>>>::Result:
        AggregateReductionResult<Source = S, Target = crate::models::algebraic::ILP<bool>>,
{
    let reduction = source.reduce_to().unwrap();
    let target = ReductionResult::target_problem(&reduction);
    assert!(target.num_vars() <= 16, "keep exhaustive ILP checks small");
    let mut total = Extremum::minimize(None);
    for mask in 0..1usize << target.num_vars() {
        let assignment = (0..target.num_vars())
            .map(|bit| ((mask >> bit) & 1) as i64)
            .collect();
        let value = target.evaluate(&assignment).unwrap();
        total = total.combine(value).unwrap();
        if value.value.is_some() {
            let decoded = reduction.extract_solution(&assignment).unwrap();
            assert_eq!(source.evaluate(&decoded).unwrap(), Or(true));
        } else {
            assert!(reduction.extract_solution(&assignment).is_err());
        }
    }
    assert_eq!(
        reduction.extract_value(total),
        Or(BruteForce::new().solve(source).unwrap().is_some())
    );
}

#[test]
fn binary_ilp_encodings_preserve_degenerate_graphs() {
    use crate::models::graph::{DisjointConnectingPaths, HamiltonianPath, SubgraphIsomorphism};
    for graph in [
        SimpleGraph::empty(2),
        SimpleGraph::new(2, vec![(0, 0), (1, 1)]),
        SimpleGraph::new(2, vec![(0, 1), (0, 1)]),
    ] {
        check_binary_ilp(&DisjointConnectingPaths::new(graph.clone(), vec![(0, 1)]));
        check_binary_ilp(&HamiltonianPath::new(graph));
    }
    for host in [SimpleGraph::empty(1), SimpleGraph::new(1, vec![(0, 0)])] {
        check_binary_ilp(&SubgraphIsomorphism::new(
            host,
            SimpleGraph::new(1, vec![(0, 0)]),
        ));
    }
}

#[test]
fn zero_column_matrices_have_no_blocks() {
    use crate::models::algebraic::ConsecutiveBlockMinimization;
    check_binary_ilp(&ConsecutiveBlockMinimization::new(vec![vec![], vec![]], 0));
}

#[test]
fn empty_tree_storage_still_obeys_the_budget() {
    use crate::models::{algebraic::ILP, set::RootedTreeStorageAssignment};
    use crate::solvers::{ILPSolveError, ILPSolver};
    for n in 0..=2 {
        for bound in [-1, 0] {
            let source = RootedTreeStorageAssignment::new(n, vec![], bound);
            let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
            let expected = bound >= 0;
            assert_eq!(
                BruteForce::new().solve(&source).unwrap().is_some(),
                expected
            );
            let target = ReductionResult::target_problem(&reduction);
            let value = match ILPSolver::new().solve(target) {
                Ok(solution) => {
                    let decoded = reduction.extract_solution(&solution).unwrap();
                    assert_eq!(source.evaluate(&decoded).unwrap(), Or(true));
                    target.evaluate(&solution).unwrap()
                }
                Err(ILPSolveError::Infeasible) => Extremum::minimize(None),
                Err(error) => panic!("{error}"),
            };
            assert_eq!(reduction.extract_value(value), Or(expected));
        }
    }
}

#[test]
fn hamiltonian_tour_thresholds_match_all_small_graphs() {
    for n in 0..=4 {
        let edges: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..1usize << edges.len() {
            let graph = SimpleGraph::new(
                n,
                edges
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &e)| (mask & (1 << i) != 0).then_some(e))
                    .collect(),
            );
            let source = HamiltonianCircuit::new(graph);
            check_decision::<_, TravelingSalesman<SimpleGraph, i64>>(&source);
            check_decision::<_, BottleneckTravelingSalesman>(&source);
        }
    }
}

#[test]
fn exact_cover_thresholds_include_uncovered_elements() {
    for source in [
        ExactCoverBy3Sets::new(0, vec![]),
        ExactCoverBy3Sets::new(3, vec![]),
        ExactCoverBy3Sets::new(3, vec![[0, 1, 2]]),
        ExactCoverBy3Sets::new(6, vec![[0, 1, 2]]),
        ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]),
        ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4], [0, 4, 5]]),
    ] {
        check_decision::<_, MaximumSetPacking<One>>(&source);
        check_decision::<_, MinimumAxiomSet>(&source);
        check_decision::<_, MinimumFaultDetectionTestSet>(&source);
    }
}

#[test]
fn partition_thresholds_distinguish_odd_totals_and_singletons() {
    for sizes in [
        vec![1],
        vec![2],
        vec![1, 1],
        vec![1, 2],
        vec![1, 3],
        vec![1, 1, 2],
        vec![2, 2, 2],
    ] {
        let source = Partition::new(sizes).unwrap();
        check_decision::<_, BinPacking<i64>>(&source);
        check_decision::<_, Knapsack>(&source);
        check_decision::<_, SumOfSquaresPartition>(&source);
    }
}

#[test]
fn matching_decoding_threshold_handles_empty_and_unsatisfiable_instances() {
    for source in [
        ThreeDimensionalMatching::new(0, vec![]),
        ThreeDimensionalMatching::new(1, vec![]),
        ThreeDimensionalMatching::new(1, vec![(0, 0, 0)]),
        ThreeDimensionalMatching::new(2, vec![(0, 0, 0)]),
        ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1)]),
    ] {
        check_decision::<_, MinimumWeightDecoding>(&source);
    }
}

#[test]
fn sat_cover_threshold_supports_short_and_empty_clauses() {
    for clauses in [
        vec![],
        vec![vec![]],
        vec![vec![1]],
        vec![vec![1], vec![-1]],
        vec![vec![1, -1, 1]],
    ] {
        let source = KSatisfiability::<K3>::new_allow_less(
            1,
            clauses.into_iter().map(CNFClause::new).collect(),
        );
        check_decision::<_, MinimumVertexCover<SimpleGraph, i64>>(&source);
        check_decision::<_, crate::models::graph::KClique<SimpleGraph>>(&source);
        check_decision::<_, crate::models::graph::Kernel>(&source);
        check_decision::<_, crate::models::misc::SubsetSum>(&source);
    }
}

#[test]
fn partition_decision_encodings_preserve_yes_and_no() {
    use crate::models::graph::IntegralFlowWithMultipliers;
    use crate::models::misc::{
        CosineProductIntegration, MultiprocessorScheduling, ProductionPlanning, SubsetSum,
    };
    for sizes in [vec![1], vec![2], vec![1, 1], vec![1, 2], vec![1, 3]] {
        let source = Partition::new(sizes).unwrap();
        check_decision::<_, CosineProductIntegration>(&source);
        check_decision::<_, MultiprocessorScheduling>(&source);
        check_decision::<_, ProductionPlanning>(&source);
        check_decision::<_, SubsetSum>(&source);
        check_decision::<_, IntegralFlowWithMultipliers>(&source);
    }
}

#[test]
fn rooted_tree_mapping_preserves_empty_graphs_loops_and_negative_bounds() {
    use crate::models::graph::RootedTreeArrangement;
    use crate::models::set::RootedTreeStorageAssignment;
    for graph in [
        SimpleGraph::empty(0),
        SimpleGraph::empty(1),
        SimpleGraph::new(2, vec![(0, 0), (0, 1), (0, 1)]),
    ] {
        for bound in [i64::MIN, -1, 0, 1, 2] {
            check_decision::<_, RootedTreeStorageAssignment>(&RootedTreeArrangement::new(
                graph.clone(),
                bound,
            ));
        }
    }
}

#[test]
fn path_partition_requires_distinct_edges_between_distinct_vertices() {
    use crate::models::graph::{BoundedComponentSpanningForest, PartitionIntoPathsOfLength2};
    for (edges, expected) in [
        (vec![(0, 0), (1, 1)], false),
        (vec![(0, 1), (0, 1)], false),
        (vec![(0, 1), (1, 2), (0, 1)], true),
    ] {
        let source = PartitionIntoPathsOfLength2::new(SimpleGraph::new(3, edges));
        assert_eq!(source.evaluate(&vec![0, 0, 0]).unwrap(), Or(expected));
        check_decision::<_, BoundedComponentSpanningForest<SimpleGraph, i64>>(&source);
        check_binary_ilp(&source);
    }
}

#[test]
fn sat_empty_conjunction_and_empty_clause_preserve_opposite_answers() {
    use crate::models::formula::Satisfiability;
    use crate::models::graph::KColoring;
    use crate::models::misc::TimetableDesign;
    for clauses in [vec![], vec![CNFClause::new(vec![])]] {
        let source = Satisfiability::new(0, clauses.clone());
        check_decision::<_, KSatisfiability<K3>>(&source);
        check_decision::<_, KColoring<K3, SimpleGraph>>(&source);
        check_decision::<_, TimetableDesign>(&KSatisfiability::<K3>::new_allow_less(0, clauses));
    }
}

#[test]
fn numerical_matching_checks_pair_sums_without_wrapping() {
    use crate::models::misc::NumericalMatchingWithTargetSums;
    for (x, y, target, answer) in [
        (i64::MAX, 1, i64::MIN, false),
        (i64::MIN, -1, i64::MAX, false),
        (i64::MAX, -1, i64::MAX - 1, true),
    ] {
        let source = NumericalMatchingWithTargetSums::new(vec![x], vec![y], vec![target]);
        assert_eq!(source.evaluate(&vec![0]).unwrap(), Or(answer));
        check_binary_ilp(&source);
    }
}
