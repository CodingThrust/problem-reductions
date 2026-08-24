use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::rules::unitdiskmapping::ksg;
use crate::solvers::BruteForce;
use crate::topology::{Graph, KingsSubgraph, SimpleGraph};
use crate::types::One;

#[test]
fn test_map_unweighted_produces_uniform_weights() {
    // Triangle graph
    let result = ksg::map_unweighted(3, &[(0, 1), (1, 2), (0, 2)]).unwrap();
    assert!(
        result.node_weights.iter().all(|&w| w == 1),
        "map_unweighted triangle should produce uniform weights, got: {:?}",
        result.node_weights
    );

    // Path graph
    let result2 = ksg::map_unweighted(3, &[(0, 1), (1, 2)]).unwrap();
    assert!(
        result2.node_weights.iter().all(|&w| w == 1),
        "map_unweighted path should produce uniform weights, got: {:?}",
        result2.node_weights
    );

    // Cycle-5
    let result3 = ksg::map_unweighted(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)]).unwrap();
    assert!(
        result3.node_weights.iter().all(|&w| w == 1),
        "map_unweighted cycle5 should produce uniform weights, got: {:?}",
        result3.node_weights
    );
}

#[test]
fn test_mis_simple_one_to_kings_one_is_deterministic_on_large_graph() {
    // Regression for #1061: `pred reduce` output was non-deterministic because the
    // greedy path decomposition used an unseeded thread RNG for tie-breaking and
    // the adjacency list used HashSet iteration order. A 64-vertex graph matches
    // the reporter's scenario (10x10 kings, p=0.3) and forces the Auto path to
    // pick the Greedy branch (>30 vertices).
    let n = 64;
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for r in 0..8 {
        for c in 0..8 {
            let v = r * 8 + c;
            if c + 1 < 8 {
                edges.push((v, r * 8 + c + 1));
            }
            if r + 1 < 8 {
                edges.push((v, (r + 1) * 8 + c));
            }
            if r + 1 < 8 && c + 1 < 8 {
                edges.push((v, (r + 1) * 8 + c + 1));
            }
        }
    }

    let problem = MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![One; n]);

    let first = ReduceTo::<MaximumIndependentSet<KingsSubgraph, One>>::reduce_to(&problem)
        .expect("reduction should succeed");
    let baseline_atoms = first.target_problem().graph().num_vertices();
    let baseline_edges = first.target_problem().graph().edges().len();

    for _ in 0..3 {
        let again = ReduceTo::<MaximumIndependentSet<KingsSubgraph, One>>::reduce_to(&problem)
            .expect("reduction should succeed");
        assert_eq!(
            again.target_problem().graph().num_vertices(),
            baseline_atoms,
        );
        assert_eq!(again.target_problem().graph().edges().len(), baseline_edges);
    }
}

#[test]
fn test_mis_simple_one_to_kings_one_closed_loop() {
    // Path graph: 0-1-2-3-4 (MIS = 3: select vertices 0, 2, 4)
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![One; 5],
    );
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, One>>::reduce_to(&problem)
        .expect("reduction should succeed");
    let target = result.target_problem();
    assert!(target.graph().num_vertices() > 5);

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_witnesses(target).unwrap();
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]).unwrap();
    assert_eq!(original_solution.len(), 5);
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 3, "Max IS in path of 5 should be 3");
}

#[test]
fn test_mis_simple_one_to_kings_one_all_four_vertex_graphs() {
    use crate::test_unitdiskmapping_algorithms::common::{
        is_independent_set, solve_mis, solve_mis_config,
    };

    let possible_edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for mask in 0..(1_usize << possible_edges.len()) {
        let edges = possible_edges
            .iter()
            .enumerate()
            .filter(|(index, _)| mask & (1 << index) != 0)
            .map(|(_, &edge)| edge)
            .collect::<Vec<_>>();
        let source = MaximumIndependentSet::new(SimpleGraph::new(4, edges.clone()), vec![One; 4]);
        let reduction =
            ReduceTo::<MaximumIndependentSet<KingsSubgraph, One>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let target_solution =
            solve_mis_config(target.graph().num_vertices(), &target.graph().edges());
        let source_solution = reduction.extract_solution(&target_solution).unwrap();

        assert!(
            is_independent_set(&edges, &source_solution),
            "graph mask {mask:#08b} extracted a non-independent set"
        );
        assert_eq!(
            source_solution.iter().filter(|&&value| value > 0).count(),
            solve_mis(4, &edges),
            "graph mask {mask:#08b} did not preserve the optimum"
        );
    }
}
