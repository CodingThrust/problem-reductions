use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_maximum_achromatic_number_c6() {
    // C6 (6-cycle): achromatic number is 3
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
    let problem = MaximumAchromaticNumber::new(graph);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.dims(), vec![6; 6]);

    // [0,1,2,0,1,2] is a valid complete proper 3-coloring
    let config = vec![0, 1, 2, 0, 1, 2];
    assert_eq!(problem.evaluate(&config), Max(Some(3)));
}

#[test]
fn test_maximum_achromatic_number_improper_coloring() {
    // Adjacent vertices with the same color -> invalid
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MaximumAchromaticNumber::new(graph);

    // Vertices 0 and 1 are adjacent and share color 0
    assert_eq!(problem.evaluate(&[0, 0, 1, 2]), Max(None));
}

#[test]
fn test_maximum_achromatic_number_incomplete_coloring() {
    // Proper but not complete: color pair with no connecting edge -> invalid
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MaximumAchromaticNumber::new(graph);

    // Colors: 0->0, 1->1, 2->2, 3->3 — proper (no adjacent same color)
    // But colors 0 and 2 have no edge between them, etc. -> incomplete
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Max(None));

    // Colors: 0->0, 1->1, 2->0, 3->1 — proper (edges 0-1 and 2-3 have different colors)
    // Colors 0 and 1 have edges (0,1) and (2,3) -> complete
    assert_eq!(problem.evaluate(&[0, 1, 0, 1]), Max(Some(2)));
}

#[test]
fn test_maximum_achromatic_number_solver() {
    // Small graph: path P3 (0-1-2)
    // Possible colorings:
    // [0,1,0] -> 2 colors, proper, complete (edge between 0 and 1 classes) -> Max(2)
    // [0,1,2] -> 3 colors, proper, but colors 0 and 2 have no edge -> incomplete
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumAchromaticNumber::new(graph);

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert_eq!(value, Max(Some(2)));
}

#[test]
fn test_maximum_achromatic_number_wrong_length() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumAchromaticNumber::new(graph);
    assert_eq!(problem.evaluate(&[0, 1]), Max(None));
}

#[test]
fn test_maximum_achromatic_number_empty_graph() {
    // No vertices, no edges
    let graph = SimpleGraph::new(0, vec![]);
    let problem = MaximumAchromaticNumber::new(graph);
    assert_eq!(problem.evaluate(&[]), Max(Some(0)));
}

#[test]
fn test_maximum_achromatic_number_single_vertex() {
    // Single vertex, no edges: 1 color, trivially complete
    let graph = SimpleGraph::new(1, vec![]);
    let problem = MaximumAchromaticNumber::new(graph);
    assert_eq!(problem.evaluate(&[0]), Max(Some(1)));
}

#[test]
fn test_maximum_achromatic_number_complete_graph_k3() {
    // K3: achromatic number = 3 (each vertex gets its own color)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MaximumAchromaticNumber::new(graph);

    // 3 colors: proper and complete (every color pair has an edge)
    assert_eq!(problem.evaluate(&[0, 1, 2]), Max(Some(3)));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Max(Some(3)));
}
