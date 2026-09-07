use super::*;
use crate::models::graph::KColoring;
use crate::models::set::TwoDimensionalConsecutiveSets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_kcoloring_to_twodimensionalconsecutivesets_closed_loop() {
    // Triangle graph: 3-colorable
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "K3-coloring triangle -> TDCS",
    );
}

#[test]
fn test_kcoloring_to_tdcs_target_structure() {
    // Graph with 4 vertices and 3 edges: path 0-1-2-3
    let source = KColoring::<K3, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // Alphabet: 4 vertices + 3 edges = 7
    assert_eq!(target.alphabet_size(), 7);
    // One subset per edge
    assert_eq!(target.num_subsets(), 3);
    // Each subset has size 3
    for subset in target.subsets() {
        assert_eq!(subset.len(), 3);
    }
}

#[test]
fn test_kcoloring_to_tdcs_non_3colorable() {
    // K3 with an extra vertex connected to all 3: K4 restricted to 3 vertices + 1
    // Use K_3 + edge to make a non-3-colorable subgraph: vertex 0 connected to 1, 2;
    // vertex 1 connected to 2; all three connected to vertex 3
    // This is K4 but we only check source side (target brute-force too slow).
    let source = KColoring::<K3, _>::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));

    let solver = BruteForce::new();
    let source_solutions = solver.find_all_witnesses(&source).unwrap();
    assert!(source_solutions.is_empty(), "K4 is not 3-colorable");

    // Verify the reduction produces the correct structure
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert_eq!(target.alphabet_size(), 10); // 4 vertices + 6 edges
    assert_eq!(target.num_subsets(), 6);
}

#[test]
fn test_kcoloring_to_tdcs_bipartite() {
    // Path 0-1-2: bipartite, 2-colorable (hence 3-colorable)
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "K3-coloring path -> TDCS",
    );
}

#[test]
fn test_kcoloring_to_tdcs_single_edge() {
    // Single edge: trivially 3-colorable
    let source = KColoring::<K3, _>::new(SimpleGraph::new(2, vec![(0, 1)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.alphabet_size(), 3); // 2 vertices + 1 edge
    assert_eq!(target.num_subsets(), 1);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "K3-coloring single edge -> TDCS",
    );
}

#[test]
fn test_kcoloring_to_tdcs_extract_solution_valid() {
    // Triangle: verify extracted coloring is valid
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source)
        .expect("reduction should succeed");

    let solver = BruteForce::new();
    let target_solutions = solver
        .find_all_witnesses(reduction.target_problem())
        .unwrap();

    for target_sol in &target_solutions {
        let source_sol = reduction.extract_solution(target_sol).unwrap();
        assert_eq!(source_sol.len(), 3);
        // Verify it is a valid coloring
        assert!(
            source.evaluate(&source_sol).unwrap().0,
            "Extracted coloring must be valid: {:?}",
            source_sol
        );
    }
}

#[test]
fn test_kcoloring_to_tdcs_empty_graph_has_a_target_witness() {
    let source = KColoring::<K3, _>::new(SimpleGraph::empty(0));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert_eq!(target.alphabet_size(), 1);
    assert_eq!(target.num_subsets(), 0);
    let witness = BruteForce::new().solve(target).unwrap().unwrap();
    let coloring = reduction.extract_solution(&witness).unwrap();
    assert!(coloring.is_empty());
    assert!(source.evaluate(&coloring).unwrap().0);
}

#[test]
fn test_kcoloring_to_tdcs_native_loops_are_no() {
    for n in [1, 5] {
        let source = KColoring::<K3, _>::new(SimpleGraph::new(n, vec![(0, 0)]));
        let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert_eq!(target.alphabet_size(), 3);
        assert_eq!(target.num_subsets(), 3);
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        assert!(BruteForce::new().solve(target).unwrap().is_none());
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    assert!(reduction.extract_solution(&vec![a, b, c]).is_err());
                }
            }
        }
    }
}

#[test]
fn test_kcoloring_to_tdcs_rejects_noncertificates() {
    let source = KColoring::<K3, _>::new(SimpleGraph::new(2, vec![(0, 1)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source).unwrap();
    for config in [vec![], vec![0, 1], vec![0, 1, 3], vec![0, 0, 0]] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn test_kcoloring_to_tdcs_many_groups_gaps_and_repeated_edges() {
    for (n, edges, grouping, expected) in [
        (
            5,
            vec![(0, 1), (1, 2), (2, 3), (3, 4)],
            vec![0, 2, 4, 6, 8, 1, 3, 5, 7],
            vec![0, 1, 2, 0, 1],
        ),
        (4, vec![(0, 3)], vec![0, 0, 4, 4, 2], vec![0, 0, 1, 1]),
        (
            2,
            vec![(0, 1), (1, 0), (0, 1)],
            vec![0, 1, 2, 2, 2],
            vec![0, 1],
        ),
    ] {
        let source = KColoring::<K3, _>::new(SimpleGraph::new(n, edges));
        let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source).unwrap();
        assert!(reduction.target_problem().evaluate(&grouping).unwrap().0);
        let coloring = reduction.extract_solution(&grouping).unwrap();
        assert_eq!(coloring, expected);
        assert!(source.evaluate(&coloring).unwrap().0);
    }
}

#[test]
fn test_kcoloring_to_tdcs_all_tiny_graphs_and_target_assignments() {
    for n in 0..=3 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..1usize << pairs.len() {
            let edges = pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, &e)| e)
                .collect();
            let source = KColoring::<K3, _>::new(SimpleGraph::new(n, edges));
            let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source).unwrap();
            let target = reduction.target_problem();
            let size = target.alphabet_size();
            let mut target_yes = false;
            for mut encoded in 0..size.pow(u32::try_from(size).unwrap()) {
                let grouping = (0..size)
                    .map(|_| {
                        let group = encoded % size;
                        encoded /= size;
                        group
                    })
                    .collect();
                let feasible = target.evaluate(&grouping).unwrap().0;
                let extracted = reduction.extract_solution(&grouping);
                assert_eq!(extracted.is_ok(), feasible);
                if let Ok(coloring) = extracted {
                    assert!(source.evaluate(&coloring).unwrap().0);
                    target_yes = true;
                }
            }
            assert_eq!(
                target_yes,
                BruteForce::new().solve(&source).unwrap().is_some()
            );
        }
    }
}
