use super::{leaf_bound, target_sizes};
use crate::models::{Decision, MaximumLeafSpanningTree, MinimumVertexCover};
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::One;
use crate::types::Or;

type Source = Decision<MinimumVertexCover<SimpleGraph, One>>;
type Target = Decision<MaximumLeafSpanningTree<SimpleGraph>>;

fn source(n: usize, edges: Vec<(usize, usize)>, bound: i64) -> Source {
    Decision::new(
        MinimumVertexCover::new(SimpleGraph::new(n, edges), vec![One; n]),
        bound,
    )
}

fn cycle(bound: i64) -> Source {
    source(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)], bound)
}

fn reduce(source: &Source) -> <Source as ReduceTo<Target>>::Result {
    ReduceTo::<Target>::reduce_to(source).unwrap()
}

fn degrees(target: &Target, bits: &[bool]) -> Vec<usize> {
    let mut degree = vec![0; target.num_vertices()];
    for ((u, v), selected) in target.inner().graph().edges().iter().zip(bits) {
        if *selected {
            degree[*u] += 1;
            degree[*v] += 1;
        }
    }
    degree
}

#[test]
fn test_decisionminimumvertexcover_to_decisionmaximumleafspanningtree_closed_loop() {
    let source = cycle(2);
    let reduction = reduce(&source);
    crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "four-cycle cover to leaf tree",
    );
    crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
}

#[test]
fn test_paper_example_and_infeasible_bound() {
    let witness = vec![
        true, true, true, true, true, false, false, true, true, false, false, true, true, true,
    ];
    for (bound, expected) in [(2, true), (1, false)] {
        let source = cycle(bound);
        let reduction = reduce(&source);
        let target = reduction.target_problem();
        assert_eq!(target.num_vertices(), 11);
        assert_eq!(target.num_edges(), 14);
        assert_eq!(*target.bound(), 10 - bound);
        assert_eq!(
            degrees(target, &witness),
            vec![3, 1, 3, 1, 1, 1, 1, 1, 6, 1, 1]
        );
        let (answer, solutions) = BruteForce::new().solve_with_witnesses(target).unwrap();
        let direct = BruteForce::new().solve_with_witnesses(&source).unwrap().0;
        assert_eq!(answer, Or(expected));
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&reduction, answer),
            direct
        );
        assert_eq!(solutions.is_empty(), !expected);
        if expected {
            assert_eq!(
                reduction.extract_solution(&witness).unwrap(),
                vec![true, false, true, false]
            );
        } else {
            assert!(reduction.extract_solution(&witness).is_err());
        }
    }
    #[cfg(feature = "example-db")]
    {
        let example = (super::canonical_rule_example_specs().pop().unwrap().build)();
        assert_eq!(
            example.solutions[0].target_config,
            serde_json::json!(witness)
        );
        assert_eq!(
            example.solutions[0].source_config,
            serde_json::json!([true, false, true, false])
        );
    }
}

#[test]
fn test_exact_extraction_with_internal_subdivision_vertex() {
    let source = source(2, vec![(0, 1)], 2);
    let reduction = reduce(&source);
    let witness = vec![true, false, true, true, true, true];
    assert_eq!(
        reduction.target_problem().evaluate(&witness).unwrap(),
        Or(true)
    );
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![true, false]
    );
}

#[test]
fn test_extraction_rejects_non_tree_meeting_leaf_threshold() {
    let source = source(6, vec![(0, 1), (1, 2), (0, 2)], 6);
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    // A cycle on {r,1,a_1,2}; the second private vertex is disconnected.
    let mut bits = vec![true; 6];
    bits.extend([true, false, true, true, true, false, true, false]);
    assert_eq!(
        bits.iter().filter(|&&x| x).count(),
        target.num_vertices() - 1
    );
    assert!(
        i64::try_from(degrees(target, &bits).iter().filter(|&&d| d == 1).count()).unwrap()
            >= *target.bound()
    );
    assert_eq!(target.evaluate(&bits).unwrap(), Or(false));
    assert!(reduction.extract_solution(&bits).is_err());
}

#[test]
fn test_extraction_rejects_wrong_lengths() {
    let reduction = reduce(&source(2, vec![(0, 1)], 2));
    let valid = vec![true, false, true, true, true, true];
    let mut long = valid.clone();
    long.push(false);
    for bits in [vec![], valid[..5].to_vec(), long] {
        assert!(reduction.extract_solution(&bits).is_err());
    }
    assert!(reduction.extract_solution(&vec![false; 6]).is_err());
    assert!(reduction.extract_solution(&vec![true; 6]).is_err());
}

#[test]
fn test_suboptimal_trees_obey_decision_bound() {
    for (parents, leaves) in [([0, 1, 2, 0], 7), ([0, 1, 2, 3], 6)] {
        let mut bits = vec![true; 4];
        for ((u, v), parent) in cycle(2).inner().graph().edges().into_iter().zip(parents) {
            bits.extend([parent == u, parent == v]);
        }
        bits.extend([true, true]);
        let strict = reduce(&cycle(2));
        assert_eq!(
            degrees(strict.target_problem(), &bits)
                .iter()
                .filter(|&&d| d == 1)
                .count(),
            leaves
        );
        assert!(strict.target_problem().inner().is_valid_solution(&bits));
        assert!(strict.extract_solution(&bits).is_err());
        let loose_source = cycle(i64::try_from(10 - leaves).unwrap());
        let loose = reduce(&loose_source);
        let cover = loose.extract_solution(&bits).unwrap();
        assert_eq!(loose_source.evaluate(&cover).unwrap(), Or(true));
    }
}

#[test]
fn test_empty_edgeless_disconnected_and_signed_bounds() {
    for (n, edges, minimum) in [
        (0, vec![], 0),
        (1, vec![], 0),
        (5, vec![], 0),
        (5, vec![(0, 1), (2, 3)], 2),
    ] {
        for k in [i64::MIN, -1, 0, 1, 2, 5, 6, i64::MAX] {
            let source = source(n, edges.clone(), k);
            let reduction = reduce(&source);
            let target = reduction.target_problem();
            assert_eq!(target.num_vertices(), n + edges.len() + 3);
            assert_eq!(target.num_edges(), n + 2 * edges.len() + 2);
            let solution = BruteForce::new().solve(target).unwrap();
            assert_eq!(solution.is_some(), k >= minimum);
            if let Some(bits) = solution {
                let extracted = reduction.extract_solution(&bits).unwrap();
                assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
                for (v, &selected) in extracted.iter().enumerate() {
                    if !edges.iter().any(|&(u, w)| u == v || w == v) {
                        assert!(!selected);
                    }
                }
            }
            crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
            if n == 0 {
                assert_eq!(target.inner().graph().edges(), vec![(0, 1), (0, 2)]);
            }
        }
    }
}

#[test]
fn test_size_and_signed_arithmetic_boundaries() {
    let max = usize::MAX;
    for (n, m, expected) in [
        (max - 3, 0, Some((max, max - 1))),
        (max - 2, 0, None),
        (max, 1, None),
        (0, max / 2 - 1, Some((max / 2 + 2, max - 1))),
        (0, max / 2, None),
        (0, max / 2 + 1, None),
        (3, max / 2 - 2, Some((max / 2 + 4, max))),
        (4, max / 2 - 2, None),
        (5, max / 2 - 2, None),
        (6, max / 2 - 2, None),
    ] {
        assert_eq!(target_sizes(n, m).ok(), expected, "n={n},m={m}");
    }
    let signed_max = usize::try_from(i64::MAX).unwrap();
    assert_eq!(leaf_bound(0, signed_max, -1).unwrap(), i64::MAX);
    assert_eq!(leaf_bound(0, signed_max + 1, 0).unwrap(), i64::MAX);
    assert!(leaf_bound(0, signed_max + 1, -1).is_err());
    assert!(leaf_bound(0, signed_max + 2, 0).is_err());
    assert!(leaf_bound(signed_max + 1, 3, 0).is_err());
    for k in [1, 2, 3] {
        assert_eq!(leaf_bound(10, 3, k).unwrap(), 2 - k);
    }
    for k in [i64::MIN, -1, 0, 2, 3, i64::MAX] {
        assert_eq!(leaf_bound(2, 6, k).unwrap(), 5 - k.clamp(-1, 2));
    }
}

// Grow arbitrary trees using the actual target adjacency, with no gadget assumptions.
fn spanning_trees(edges: &[(usize, usize)], n: usize, visit: &mut impl FnMut(&[bool])) {
    assert!(edges.len() < 64 && n < 64);
    let mut incident = vec![0_u64; n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        incident[u] |= 1 << i;
        incident[v] |= 1 << i;
    }
    fn grow(
        edges: &[(usize, usize)],
        incident: &[u64],
        seen: u64,
        touching_seen: u64,
        frontier: u64,
        chosen: &mut [bool],
        visit: &mut impl FnMut(&[bool]),
    ) {
        if seen.count_ones() == u32::try_from(incident.len()).unwrap() {
            visit(chosen);
            return;
        }
        if frontier == 0 {
            return;
        }
        let edge = usize::try_from(frontier.trailing_zeros()).unwrap();
        let rest = frontier & !(1 << edge);
        let (u, v) = edges[edge];
        let new_vertex = if seen >> u & 1 == 1 { v } else { u };
        let next_seen = seen | (1 << new_vertex);
        // Remove edges closing a cycle; add edges from the new vertex to unseen ones.
        let next = (rest & !incident[new_vertex]) | (incident[new_vertex] & !touching_seen);
        chosen[edge] = true;
        grow(
            edges,
            incident,
            next_seen,
            touching_seen | incident[new_vertex],
            next,
            chosen,
            visit,
        );
        chosen[edge] = false;
        grow(edges, incident, seen, touching_seen, rest, chosen, visit);
    }
    grow(
        edges,
        &incident,
        1,
        incident[0],
        incident[0],
        &mut vec![false; edges.len()],
        visit,
    );
}

fn exhaustive(n: usize, edge_count: Option<usize>) {
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
        .collect();
    for mask in 0_usize..1 << pairs.len() {
        if edge_count.is_some_and(|m| usize::try_from(mask.count_ones()).unwrap() != m) {
            continue;
        }
        let edges: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, &e)| (mask >> i & 1 == 1).then_some(e))
            .collect();
        let source = source(n, edges.clone(), i64::try_from(n).unwrap());
        let covers = BruteForce::new().find_all_witnesses(&source).unwrap();
        let minimum = covers
            .iter()
            .map(|c| c.iter().filter(|&&x| x).count())
            .min()
            .unwrap();
        let sources: Vec<_> = (-1..=i64::try_from(n).unwrap())
            .map(|k| Decision::new(source.inner().clone(), k))
            .collect();
        let reductions: Vec<_> = sources.iter().map(reduce).collect();
        let target = reductions.last().unwrap().target_problem();
        assert_eq!(target.num_vertices(), n + edges.len() + 3);
        assert_eq!(target.num_edges(), n + 2 * edges.len() + 2);
        for cover in covers {
            let mut bits = vec![true; n];
            for &(u, _) in &edges {
                bits.extend([cover[u], !cover[u]]);
            }
            bits.extend([true, true]);
            let cost = cover.iter().filter(|&&x| x).count();
            assert_eq!(
                reductions[cost + 1]
                    .target_problem()
                    .evaluate(&bits)
                    .unwrap(),
                Or(true)
            );
        }
        let mut maximum = 0;
        let target_edges = target.inner().graph().edges();
        spanning_trees(&target_edges, target.num_vertices(), &mut |bits| {
            let mut degree = vec![0; target.num_vertices()];
            for (&(u, v), &selected) in target_edges.iter().zip(bits) {
                if selected {
                    degree[u] += 1;
                    degree[v] += 1;
                }
            }
            let leaves = degree.iter().filter(|&&d| d == 1).count();
            maximum = maximum.max(leaves);
            if i64::try_from(leaves).unwrap() >= *target.bound() {
                let k = target.num_vertices() - 1 - leaves;
                let cover = reductions[k + 1].extract_solution(&bits.to_vec()).unwrap();
                assert_eq!(
                    cover,
                    degree[..n].iter().map(|&d| d >= 2).collect::<Vec<_>>()
                );
                assert_eq!(sources[k + 1].evaluate(&cover).unwrap(), Or(true));
            }
        });
        assert_eq!(maximum, target.num_vertices() - 1 - minimum);
        for (i, reduction) in reductions.iter().enumerate() {
            assert_eq!(
                i64::try_from(maximum).unwrap() >= *reduction.target_problem().bound(),
                i > minimum
            );
        }
    }
}

#[test]
fn test_exhaustive_graphs_through_four_vertices() {
    for n in 0..=4 {
        exhaustive(n, None);
    }
}

macro_rules! exhaustive_five_vertices {
    ($($name:ident: $edges:literal),* $(,)?) => {$(
        #[test]
        fn $name() {exhaustive(5,Some($edges));}
    )*};
}
exhaustive_five_vertices! {
    test_five_vertices_edgeless: 0, test_five_vertices_one_edge: 1,
    test_five_vertices_two_edges: 2, test_five_vertices_three_edges: 3,
    test_five_vertices_four_edges: 4, test_five_vertices_five_edges: 5,
    test_five_vertices_six_edges: 6, test_five_vertices_seven_edges: 7,
    test_five_vertices_eight_edges: 8, test_five_vertices_nine_edges: 9,
    test_five_vertices_complete: 10,
}

#[test]
fn test_registered_cycle_target() {
    let source = Decision::new(
        MinimumVertexCover::new(SimpleGraph::cycle(4), vec![One; 4]),
        2,
    );
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == "DecisionMinimumVertexCover"
                && entry.source_variant() == vec![("graph", "SimpleGraph"), ("weight", "One")]
                && entry.target_name == "DecisionMaximumLeafSpanningTree"
        })
        .expect("direct decision reduction must be registered");
    assert!(entry.reduce_aggregate_fn.is_some());
    let reduced = entry.reduce_fn.unwrap()(&source).unwrap();
    let target = reduced
        .target_problem_any()
        .downcast_ref::<Decision<MaximumLeafSpanningTree<SimpleGraph>>>()
        .unwrap();
    assert_eq!(*target.bound(), 8);
    assert_eq!(target.inner().num_vertices(), 11);
    assert_eq!(
        target.inner().graph().edges(),
        vec![
            (8, 0),
            (8, 1),
            (8, 2),
            (8, 3),
            (0, 4),
            (1, 4),
            (1, 5),
            (2, 5),
            (2, 6),
            (3, 6),
            (3, 7),
            (0, 7),
            (8, 9),
            (8, 10),
        ]
    );
}
