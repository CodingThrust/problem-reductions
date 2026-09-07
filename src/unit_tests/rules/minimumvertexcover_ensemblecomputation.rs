use crate::models::graph::MinimumVertexCover;
use crate::models::misc::EnsembleComputation;
use crate::rules::traits::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One};

/// Verify that a configuration is a valid vertex cover.
fn is_valid_cover(graph: &SimpleGraph, config: &[bool]) -> bool {
    for (u, v) in graph.edges() {
        if !config[u] && !config[v] {
            return false;
        }
    }
    true
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_closed_loop() {
    // Single edge: 2 vertices, 1 edge (0,1)
    // K* = 1, optimal EC length = K* + |E| = 2
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 2]);
    let reduction =
        ReduceTo::<EnsembleComputation>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Verify target structure
    assert_eq!(target.universe_size(), 3); // |V| + 1
    assert_eq!(target.num_subsets(), 1); // |E|
    assert_eq!(target.budget(), 3); // |V| + |E|

    // Solve target with brute force — optimal value should be 2 (K*=1 + |E|=1)
    let solver = BruteForce::new();
    let optimal_solution = solver.solve(target).unwrap().unwrap();
    let optimal = target.evaluate(&optimal_solution).unwrap();
    assert_eq!(optimal, Min(Some(2)));

    // Every extracted solution must be a valid vertex cover
    let witnesses = solver.find_all_witnesses(target).unwrap();
    for witness in &witnesses {
        let source_config = reduction.extract_solution(witness).unwrap();
        assert_eq!(source_config.len(), 2);
        assert_eq!(source.evaluate(&source_config).unwrap(), Min(Some(1)));
        assert!(
            is_valid_cover(&graph, &source_config),
            "Extracted config {:?} is not a valid vertex cover (from target witness {:?})",
            source_config,
            witness
        );
    }
}

#[test]
fn test_reduction_structure_triangle() {
    // Triangle K₃: 3 vertices, 3 edges
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let source = MinimumVertexCover::new(graph, vec![One; 3]);
    let reduction =
        ReduceTo::<EnsembleComputation>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Verify sizes
    assert_eq!(target.universe_size(), 4); // 3 + 1
    assert_eq!(target.num_subsets(), 3); // 3 edges
    assert_eq!(target.budget(), 6); // 3 + 3

    // Verify subsets: each edge {u,v} maps to {a₀=3, u, v}
    let subsets = target.subsets();
    assert_eq!(subsets.len(), 3);
    assert!(subsets.contains(&vec![0, 1, 3]));
    assert!(subsets.contains(&vec![1, 2, 3]));
    assert!(subsets.contains(&vec![0, 2, 3]));
}

#[test]
fn test_reduction_structure_path() {
    // Path P₃: 3 vertices {0,1,2}, 2 edges
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let source = MinimumVertexCover::new(graph, vec![One; 3]);
    let reduction =
        ReduceTo::<EnsembleComputation>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    assert_eq!(target.num_subsets(), 2);
    assert_eq!(target.budget(), 5); // 3 + 2
}

#[test]
fn test_extract_solution_correctness() {
    // Single edge: vertices {0,1}, edge (0,1), a₀ = 2
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 2]);
    let reduction =
        ReduceTo::<EnsembleComputation>::reduce_to(&source).expect("reduction should succeed");

    // Step 0: {a₀=2} ∪ {0} → z₀ = {0,2}   operands: (2, 0)
    // Step 1: {1} ∪ z₀ → z₁ = {0,1,2}      operands: (1, 3)
    // Step 2: padding {a₀=2} ∪ {1}           operands: (2, 1)
    let config = vec![2, 0, 1, 3, 2, 1];

    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&config).unwrap(), Min(Some(2)));

    let cover = reduction.extract_solution(&config).unwrap();
    assert_eq!(cover, vec![true, false]);
    assert!(is_valid_cover(&graph, &cover));
}

#[test]
fn test_extract_from_non_normalized_witness() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 2]);
    let reduction =
        ReduceTo::<EnsembleComputation>::reduce_to(&source).expect("reduction should succeed");

    // Non-normalized: {0} ∪ {1} first, then {a₀} ∪ z₀
    let config = vec![0, 1, 2, 3, 2, 0];

    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&config).unwrap(), Min(Some(2)));

    let cover = reduction.extract_solution(&config).unwrap();
    assert_eq!(cover, vec![true, false]);
    assert!(is_valid_cover(&graph, &cover));
}

#[test]
fn test_empty_graph() {
    let graph = SimpleGraph::new(3, vec![]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 3]);
    let reduction =
        ReduceTo::<EnsembleComputation>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    assert_eq!(target.num_subsets(), 0);
    assert_eq!(target.budget(), 3);

    // No subsets → optimal value is 0
    let solver = BruteForce::new();
    let optimal_solution = solver.solve(target).unwrap().unwrap();
    let optimal = target.evaluate(&optimal_solution).unwrap();
    assert_eq!(optimal, Min(Some(0)));
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_zero_vertices() {
    let source = MinimumVertexCover::new(SimpleGraph::new(0, vec![]), vec![]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().universe_size(), 1);
    assert_eq!(reduction.target_problem().budget(), 1);
    // No targets: even these out-of-range suffix operands have no semantics.
    assert_eq!(
        reduction.extract_solution(&vec![usize::MAX; 2]).unwrap(),
        Vec::<bool>::new()
    );
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_rejects_invalid_programs() {
    let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![One; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source).unwrap();
    for program in [vec![], vec![0; 6], vec![3, 0, 1, 2, 0, 1]] {
        assert!(reduction.extract_solution(&program).is_err());
    }
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_unused_and_repeated_operations() {
    let source = MinimumVertexCover::new(SimpleGraph::new(5, vec![(1, 2), (1, 3)]), vec![One; 5]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source).unwrap();
    // Two-atom pair, useful pair, duplicate pair, unused four-atom result,
    // then the required triples. The final suffix is intentionally invalid.
    let program = vec![0, 4, 1, 5, 1, 5, 6, 7, 2, 7, 3, 7, usize::MAX, usize::MAX];
    assert_eq!(
        reduction.target_problem().evaluate(&program).unwrap(),
        Min(Some(6))
    );
    let cover = reduction.extract_solution(&program).unwrap();
    assert_eq!(cover, vec![true, true, false, false, false]);
    assert!(is_valid_cover(source.graph(), &cover));
    assert!(cover.iter().filter(|&&v| v).count() <= 6 - 2);
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_all_small_pair_families() {
    // All graphs on four vertices, and all pair families that fit the budget.
    // A triple necessarily uses one such pair, so these include both normal
    // and non-normal programs; compare optimum against independent cover search.
    let pairs: Vec<_> = (0..5)
        .flat_map(|u| (u + 1..5).map(move |v| (u, v)))
        .collect();
    let graph_edges: Vec<_> = pairs.iter().copied().filter(|&(_, v)| v < 4).collect();
    for graph_mask in 0..(1 << graph_edges.len()) {
        let edges: Vec<_> = graph_edges
            .iter()
            .enumerate()
            .filter_map(|(i, &edge)| (graph_mask & (1 << i) != 0).then_some(edge))
            .collect();
        let source = MinimumVertexCover::new(SimpleGraph::new(4, edges.clone()), vec![One; 4]);
        let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source).unwrap();
        let optimum = (0u32..16)
            .filter(|bits| {
                edges
                    .iter()
                    .all(|&(u, v)| bits & ((1 << u) | (1 << v)) != 0)
            })
            .map(u32::count_ones)
            .min()
            .unwrap() as usize;
        for pair_mask in 0u32..(1 << pairs.len()) {
            if pair_mask.count_ones() > 4 {
                continue;
            }
            let family: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &pair)| (pair_mask & (1 << i) != 0).then_some(pair))
                .collect();
            let mut program: Vec<_> = family.iter().flat_map(|&(u, v)| [u, v]).collect();
            let mut supported = true;
            for &(u, v) in &edges {
                let triple = [u, v, 4];
                let Some((i, &(a, b))) = family
                    .iter()
                    .enumerate()
                    .find(|(_, (a, b))| triple.contains(a) && triple.contains(b))
                else {
                    supported = false;
                    break;
                };
                let singleton = *triple.iter().find(|&&x| x != a && x != b).unwrap();
                program.extend([singleton, 5 + i]);
            }
            if !supported {
                continue;
            }
            program.resize(2 * reduction.target_problem().budget(), usize::MAX);
            let Min(Some(length)) = reduction.target_problem().evaluate(&program).unwrap() else {
                panic!("pair-family program must compute every triple");
            };
            let cover = reduction.extract_solution(&program).unwrap();
            let size = cover.iter().filter(|&&v| v).count();
            assert!(is_valid_cover(source.graph(), &cover));
            assert!(size <= length as usize - edges.len());
            if length as usize == edges.len() + optimum {
                assert_eq!(size, optimum);
            }
        }
    }
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_loops_and_parallel_edges() {
    // SimpleGraph's native constructor permits these representations. Duplicate
    // required sets need no extra operations; a loop requires its endpoint pair.
    let source = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 0), (0, 1), (1, 0), (1, 2), (1, 2)]),
        vec![One; 3],
    );
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source).unwrap();
    let mut program = vec![3, 0, 3, 1, 1, 4, 2, 5];
    program.resize(2 * reduction.target_problem().budget(), usize::MAX);
    assert_eq!(
        reduction.target_problem().evaluate(&program).unwrap(),
        Min(Some(4))
    );
    let cover = reduction.extract_solution(&program).unwrap();
    assert_eq!(cover, vec![true, true, false]);
    assert_eq!(source.evaluate(&cover).unwrap(), Min(Some(2)));

    let source = MinimumVertexCover::new(SimpleGraph::new(1, vec![(0, 0)]), vec![One]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source).unwrap();
    assert_eq!(
        reduction
            .extract_solution(&vec![1, 0, usize::MAX, usize::MAX])
            .unwrap(),
        vec![true]
    );
}
