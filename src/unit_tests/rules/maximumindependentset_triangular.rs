use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::{Graph, SimpleGraph, TriangularSubgraph};

#[test]
fn test_mis_simple_to_triangular_closed_loop() {
    // Path graph: 0-1-2
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
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
fn test_mis_simple_to_triangular_graph_methods() {
    // Single edge graph: 0-1
    let problem = MaximumIndependentSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]);
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
