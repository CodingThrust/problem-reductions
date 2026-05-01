use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::{Graph, SimpleGraph, TriangularSubgraph};
use crate::types::One;

#[test]
fn test_mis_simple_one_to_triangular_is_deterministic_on_large_graph() {
    // Regression for #1061: the triangular mapping shares the same greedy
    // path decomposition as the KSG mapping, so it had the same non-determinism
    // (unseeded RNG + HashSet iteration order in `adj`). A 64-vertex graph
    // forces `pathwidth` to pick the Greedy branch.
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
    let first = ReduceTo::<MaximumIndependentSet<TriangularSubgraph, i32>>::reduce_to(&problem);
    let baseline_atoms = first.target_problem().graph().num_vertices();
    let baseline_edges = first.target_problem().graph().edges().len();

    for _ in 0..3 {
        let again = ReduceTo::<MaximumIndependentSet<TriangularSubgraph, i32>>::reduce_to(&problem);
        assert_eq!(
            again.target_problem().graph().num_vertices(),
            baseline_atoms,
        );
        assert_eq!(again.target_problem().graph().edges().len(), baseline_edges);
    }
}

#[test]
fn test_mis_simple_one_to_triangular_closed_loop() {
    // Path graph: 0-1-2
    let problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![One; 3]);
    let result = ReduceTo::<MaximumIndependentSet<TriangularSubgraph, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    // The triangular graph should have more vertices than the original
    assert!(target.graph().num_vertices() > 3);

    // Map a trivial zero solution back to verify dimensions
    let zero_config = vec![0; target.graph().num_vertices()];
    let original_solution = result.extract_solution(&zero_config);
    assert_eq!(original_solution.len(), 3);
}

#[test]
fn test_mis_simple_one_to_triangular_graph_methods() {
    // Single edge graph: 0-1
    let problem = MaximumIndependentSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![One; 2]);
    let result = ReduceTo::<MaximumIndependentSet<TriangularSubgraph, i32>>::reduce_to(&problem);
    let target = result.target_problem();
    let graph = target.graph();

    // Exercise all Graph trait methods on the TriangularSubgraph type
    let n = graph.num_vertices();
    assert!(n > 2);

    let m = graph.num_edges();
    assert!(m > 0);

    let edges = graph.edges();
    assert_eq!(edges.len(), m);

    // Check edges are consistent with has_edge
    for &(u, v) in &edges {
        assert!(graph.has_edge(u, v));
        assert!(graph.has_edge(v, u)); // symmetric
    }

    // Check neighbors are consistent with edges
    for v in 0..n {
        let nbrs = graph.neighbors(v);
        for &u in &nbrs {
            assert!(graph.has_edge(v, u));
        }
    }

    // Exercise TriangularSubgraph-specific methods
    let positions = graph.positions();
    assert_eq!(positions.len(), n);
    assert_eq!(graph.num_positions(), n);
}
