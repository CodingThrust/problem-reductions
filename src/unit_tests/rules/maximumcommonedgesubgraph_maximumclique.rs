use super::*;
use crate::models::graph::{LabelledArc, LabelledDigraph};
use crate::rules::AggregateReductionResult;
use crate::rules::test_helpers::{
    assert_optimization_round_trip_from_optimization_target, assert_parameter_predictions,
};
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::types::Max;

type Triples<'a> = &'a [(usize, usize, usize)];

fn digraph(num_vertices: usize, arcs: Triples) -> LabelledDigraph {
    LabelledDigraph::new(
        num_vertices,
        arcs.iter()
            .map(|&(u, label, v)| LabelledArc::new(u, label, v))
            .collect(),
    )
}

fn instance(n1: usize, a1: Triples, n2: usize, a2: Triples) -> MaximumCommonEdgeSubgraph {
    MaximumCommonEdgeSubgraph::new(digraph(n1, a1), digraph(n2, a2))
}

fn reduce(source: &MaximumCommonEdgeSubgraph) -> ReductionMCESToClique {
    ReduceTo::<MaximumClique<SimpleGraph, One>>::reduce_to(source).unwrap()
}

/// The labelled four-cycle against a five-vertex graph with a competing first arc.
fn cycle_instance() -> MaximumCommonEdgeSubgraph {
    instance(
        4,
        &[(0, 0, 1), (1, 1, 2), (2, 2, 3), (3, 3, 0)],
        5,
        &[(0, 0, 1), (1, 1, 3), (3, 2, 4), (4, 3, 0), (2, 0, 3)],
    )
}

fn optimum_value<P>(problem: &P) -> P::Value
where
    P: Problem + 'static,
    P::Solution: 'static,
    P::Value: crate::types::SolutionAggregate + 'static,
{
    BruteForce::new().solve_with_witnesses(problem).unwrap().0
}

fn num_candidates(source: &MaximumCommonEdgeSubgraph) -> usize {
    ReductionResult::target_problem(&reduce(source)).num_vertices()
}

fn source_value(source: &MaximumCommonEdgeSubgraph, config: &[usize]) -> i64 {
    source.evaluate(&config.to_vec()).unwrap().0.unwrap()
}

fn selection(size: usize, chosen: &[usize]) -> Vec<bool> {
    (0..size).map(|index| chosen.contains(&index)).collect()
}

/// Every clique of the target (including the empty one) as an increasing vertex list.
fn all_cliques(target: &MaximumClique<SimpleGraph, One>) -> Vec<Vec<usize>> {
    fn grow(
        graph: &SimpleGraph,
        current: &mut Vec<usize>,
        candidates: Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) {
        out.push(current.clone());
        for (position, &vertex) in candidates.iter().enumerate() {
            let rest = candidates[position + 1..]
                .iter()
                .copied()
                .filter(|&other| graph.has_edge(vertex, other))
                .collect();
            current.push(vertex);
            grow(graph, current, rest, out);
            current.pop();
        }
    }
    let mut out = Vec::new();
    grow(
        target.graph(),
        &mut Vec::new(),
        (0..target.num_vertices()).collect(),
        &mut out,
    );
    out
}

/// Optimum of the source by trying every map.
fn source_optimum(source: &MaximumCommonEdgeSubgraph) -> i64 {
    optimum_value(source)
        .0
        .expect("the empty map is always feasible")
}

/// Checks the validation plan's relations on one instance: optimum equality, extraction
/// feasibility and dominance for every clique, equality at maximum cliques.
fn check_every_clique(source: &MaximumCommonEdgeSubgraph) {
    let reduction = reduce(source);
    let target = ReductionResult::target_problem(&reduction);
    let cliques = all_cliques(target);
    let best = cliques.iter().map(Vec::len).max().unwrap() as i64;
    let optimum = source_optimum(source);
    assert_eq!(best, optimum);
    for chosen in &cliques {
        let config = reduction
            .extract_solution(&selection(target.num_vertices(), chosen))
            .unwrap();
        assert!(source.is_valid_solution(&config));
        let value = source_value(source, &config);
        assert!(value >= chosen.len() as i64);
        if chosen.len() as i64 == best {
            assert_eq!(value, optimum);
        }
    }
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_closed_loop() {
    let source = cycle_instance();
    let reduction = reduce(&source);
    let target = ReductionResult::target_problem(&reduction);
    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);
    assert_parameter_predictions(&source, &reduction);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumCommonEdgeSubgraph->MaximumClique closed loop",
    );
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_issue_example() {
    let source = cycle_instance();
    let reduction = reduce(&source);
    let target = ReductionResult::target_problem(&reduction);
    // Candidates in order: (0,0), (0,4), (1,1), (2,2), (3,3).
    let mut edges = target.graph().edges();
    edges.sort_unstable();
    assert_eq!(edges, vec![(0, 2), (0, 3), (0, 4), (2, 3), (2, 4), (3, 4)]);
    assert!(
        edges.iter().all(|&(left, right)| left != 1 && right != 1),
        "the competing candidate is isolated"
    );
    let optimal = reduction
        .extract_solution(&vec![true, false, true, true, true])
        .unwrap();
    assert_eq!(optimal, vec![0, 1, 3, 4]);
    assert_eq!(source_value(&source, &optimal), 4);
    // Suboptimal cliques, with sentinel 5 for unmatched vertices.
    let two = reduction
        .extract_solution(&vec![true, false, true, false, false])
        .unwrap();
    assert_eq!(two, vec![0, 1, 3, 5]);
    assert_eq!(source_value(&source, &two), 2);
    let one = reduction
        .extract_solution(&vec![false, true, false, false, false])
        .unwrap();
    assert_eq!(one, vec![2, 3, 5, 5]);
    assert_eq!(source_value(&source, &one), 1);
    // Threshold five is infeasible on both sides: the optimum is four.
    assert_eq!(source_optimum(&source), 4);
    assert_eq!(optimum_value(target), Max(Some(4)));
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_exhaustive_small_graph_pairs() {
    // Every unlabelled looped digraph pair with at most two vertices.
    let graphs = |n: usize| -> Vec<Vec<(usize, usize, usize)>> {
        let slots: Vec<_> = (0..n)
            .flat_map(|u| (0..n).map(move |v| (u, 0, v)))
            .collect();
        (0u32..1 << slots.len())
            .map(|mask| {
                slots
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| mask >> k & 1 == 1)
                    .map(|(_, arc)| *arc)
                    .collect()
            })
            .collect()
    };
    let mut count = 0;
    for n1 in 0..=2 {
        for n2 in 0..=2 {
            for first in graphs(n1) {
                for second in graphs(n2) {
                    check_every_clique(&instance(n1, &first, n2, &second));
                    count += 1;
                }
            }
        }
    }
    assert_eq!(count, 19 * 19);
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_labelled_samples() {
    // Deterministic pseudo-random labelled instances: 3 labels, up to 6 arcs, up to 5 vertices.
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let mut next = |bound: usize| {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state % bound as u64) as usize
    };
    for round in 0..75 {
        let (n1, n2) = (1 + round % 5, 1 + (round / 5) % 5);
        let mut draw = |n: usize| {
            let mut arcs: Vec<(usize, usize, usize)> = Vec::new();
            for _ in 0..next(7) {
                let arc = (next(n), next(3), next(n));
                if !arcs.contains(&arc) {
                    arcs.push(arc);
                }
            }
            arcs
        };
        let (first, second) = (draw(n1), draw(n2));
        let source = instance(n1, &first, n2, &second);
        check_every_clique(&source);
    }
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_nonoptimal_clique_scores_higher() {
    // Three consecutive arcs of a labelled four-cycle fix all four images and preserve the fourth.
    let cycle: Triples = &[(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 0, 0)];
    let source = instance(4, cycle, 4, cycle);
    let reduction = reduce(&source);
    let target = ReductionResult::target_problem(&reduction);
    // The identity candidates are the pairs (0,0), (1,1), (2,2): vertices 0, 5, 10.
    let chosen = [0, 5, 10];
    let selected = selection(target.num_vertices(), &chosen);
    assert_eq!(target.evaluate(&selected).unwrap(), Max(Some(3)));
    let config = reduction.extract_solution(&selected).unwrap();
    assert_eq!(config, vec![0, 1, 2, 3]);
    assert!(source_value(&source, &config) > 3);
    assert_eq!(source_optimum(&source), 4);
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_special_cases() {
    // Empty arc sets: empty target, all-unmatched extraction.
    let source = instance(3, &[], 2, &[(0, 0, 1)]);
    let reduction = reduce(&source);
    assert_eq!(
        ReductionResult::target_problem(&reduction).num_vertices(),
        0
    );
    assert_eq!(reduction.extract_solution(&vec![]).unwrap(), vec![2, 2, 2]);
    let source = instance(2, &[(0, 0, 1)], 3, &[]);
    assert_eq!(num_candidates(&source), 0);
    // Loop against non-loop is discarded, and so is the converse.
    let source = instance(2, &[(0, 0, 0)], 2, &[(0, 0, 1), (1, 0, 0)]);
    assert_eq!(num_candidates(&source), 0);
    let source = instance(2, &[(0, 0, 1)], 1, &[(0, 0, 0)]);
    assert_eq!(num_candidates(&source), 0);
    // Loops map to loops; two source loops cannot share one target loop.
    let source = instance(2, &[(0, 0, 0), (1, 0, 1)], 2, &[(0, 0, 0)]);
    let reduction = reduce(&source);
    assert_eq!(
        ReductionResult::target_problem(&reduction).num_vertices(),
        2
    );
    assert_eq!(ReductionResult::target_problem(&reduction).num_edges(), 0);
    check_every_clique(&source);
    // More source than target vertices: the map is partial.
    check_every_clique(&instance(
        4,
        &[(0, 0, 1), (1, 0, 2), (2, 0, 3)],
        2,
        &[(0, 0, 1), (1, 0, 0)],
    ));
    // Antiparallel arcs, and several labels on one ordered pair, stay distinct arcs.
    check_every_clique(&instance(
        2,
        &[(0, 0, 1), (1, 0, 0)],
        2,
        &[(0, 0, 1), (1, 0, 0)],
    ));
    check_every_clique(&instance(
        2,
        &[(0, 0, 1), (0, 1, 1)],
        2,
        &[(0, 0, 1), (1, 1, 0)],
    ));
    // Reversed arcs are not identified without a vertex swap; disconnected sources are fine.
    check_every_clique(&instance(2, &[(0, 0, 1)], 2, &[(1, 0, 0)]));
    check_every_clique(&instance(
        4,
        &[(0, 0, 1), (2, 0, 3)],
        3,
        &[(0, 0, 1), (1, 0, 2)],
    ));
    // Arbitrary labels.
    check_every_clique(&instance(
        3,
        &[(0, 40, 1), (1, 7, 2), (2, 40, 0)],
        3,
        &[(2, 40, 1), (1, 7, 0), (0, 40, 2)],
    ));
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_infeasible_threshold_instance() {
    // Disjoint label sets: no arc can be preserved, so the optimum is zero.
    let source = instance(
        3,
        &[(0, 0, 1), (1, 0, 2), (2, 0, 0)],
        3,
        &[(0, 1, 1), (1, 1, 2), (2, 1, 0)],
    );
    let reduction = reduce(&source);
    assert_eq!(
        ReductionResult::target_problem(&reduction).num_vertices(),
        0
    );
    let value = optimum_value(ReductionResult::target_problem(&reduction));
    assert_eq!(value, Max(Some(0)));
    assert_eq!(
        AggregateReductionResult::extract_value(&reduction, value),
        source.evaluate(&vec![3, 3, 3]).unwrap()
    );
    assert_eq!(source_optimum(&source), 0);
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_extract_rejects_invalid_witnesses() {
    let source = cycle_instance();
    let reduction = reduce(&source);
    // Vertices 0 and 1 are not adjacent.
    assert!(
        reduction
            .extract_solution(&vec![true, true, false, false, false])
            .is_err()
    );
    // Wrong lengths.
    assert!(reduction.extract_solution(&vec![]).is_err());
    assert!(
        reduction
            .extract_solution(&vec![true, false, true, true, true, false])
            .is_err()
    );
    assert!(reduction.extract_solution(&vec![true; 4]).is_err());
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_rejects_malformed_raw_graphs() {
    let good = digraph(2, &[(0, 0, 1)]);
    let duplicated = LabelledDigraph {
        num_vertices: 2,
        arcs: vec![LabelledArc::new(0, 0, 1), LabelledArc::new(0, 0, 1)],
    };
    let outside = LabelledDigraph {
        num_vertices: 2,
        arcs: vec![LabelledArc::new(0, 0, 2)],
    };
    for (first, second) in [
        (duplicated.clone(), good.clone()),
        (good.clone(), duplicated),
        (outside.clone(), good.clone()),
        (good, outside),
    ] {
        let source = MaximumCommonEdgeSubgraph::new(first, second);
        let error = ReduceTo::<MaximumClique<SimpleGraph, One>>::reduce_to(&source)
            .expect_err("malformed raw representation must be rejected");
        assert!(matches!(error, ReductionError::InvalidTarget { .. }));
    }
}

#[test]
fn test_maximumcommonedgesubgraph_to_maximumclique_aggregate_value_matches_brute_force() {
    for source in [
        cycle_instance(),
        instance(3, &[(0, 0, 1)], 3, &[(0, 0, 1), (1, 0, 2)]),
        instance(2, &[], 2, &[]),
    ] {
        let reduction = reduce(&source);
        let target_value = optimum_value(ReductionResult::target_problem(&reduction));
        assert_eq!(
            AggregateReductionResult::extract_value(&reduction, target_value),
            optimum_value(&source)
        );
    }
}
