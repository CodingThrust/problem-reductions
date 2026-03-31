use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_covering_by_cliques_creation() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumCoveringByCliques::new(graph);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 3);
    // Each edge can be assigned to one of 3 groups
    assert_eq!(problem.dims(), vec![3; 3]);
}

#[test]
fn test_minimum_covering_by_cliques_triangle() {
    // Triangle: all 3 edges form a single clique -> 1 group suffices
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumCoveringByCliques::new(graph);

    // All edges in group 0 -> valid, 1 clique
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(Some(1)));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(1)));
}

#[test]
fn test_minimum_covering_by_cliques_path() {
    // Path 0-1-2: edges (0,1) and (1,2) are each individual cliques (K2)
    // but cannot be combined into one clique since 0 and 2 are not adjacent.
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumCoveringByCliques::new(graph);

    // Both edges in the same group -> invalid (0 and 2 not adjacent)
    assert_eq!(problem.evaluate(&[0, 0]), Min(None));

    // Two separate groups -> valid, 2 cliques
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(2)));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(2)));
}

#[test]
fn test_minimum_covering_by_cliques_invalid_group() {
    // Square: 0-1-2-3-0, edges (0,1),(1,2),(2,3),(3,0)
    // Putting non-adjacent-endpoint edges in same group is invalid
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let problem = MinimumCoveringByCliques::new(graph);

    // Edges (0,1) and (2,3) in same group: vertices {0,1,2,3}, not a clique
    assert_eq!(problem.evaluate(&[0, 1, 0, 1]), Min(None));

    // Each edge in its own group -> valid, 4 cliques
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Min(Some(4)));
}

#[test]
fn test_minimum_covering_by_cliques_empty_graph() {
    // No edges: 0 cliques needed
    let graph = SimpleGraph::new(3, vec![]);
    let problem = MinimumCoveringByCliques::new(graph);
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_covering_by_cliques_wrong_length() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumCoveringByCliques::new(graph);
    assert_eq!(problem.evaluate(&[0]), Min(None));
}

#[test]
fn test_minimum_covering_by_cliques_solver() {
    // K4 minus one edge: 4 vertices, 5 edges
    // 0-1, 0-2, 0-3, 1-2, 2-3  (missing 1-3)
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)]);
    let problem = MinimumCoveringByCliques::new(graph);

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&solution);
    // Two triangles: {0,1,2} and {0,2,3} cover all 5 edges
    assert_eq!(value, Min(Some(2)));
}

#[test]
fn test_minimum_covering_by_cliques_is_valid_cover() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumCoveringByCliques::new(graph);

    // All in one group (triangle) -> valid
    assert!(problem.is_valid_cover(&[0, 0, 0]));

    // Path 0-1 and 1-2 in same group, 0-2 separate:
    // Group 0 has vertices {0,1,2} from edges (0,1) and (1,2).
    // But 0 and 2 are adjacent in this graph, so {0,1,2} is a clique -> valid
    assert!(problem.is_valid_cover(&[0, 0, 1]));
}

#[test]
fn test_minimum_covering_by_cliques_paper_example() {
    // 6 vertices, 9 edges from the canonical example
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (0, 2),
            (4, 0),
            (4, 1),
            (5, 2),
            (5, 3),
        ],
    );
    let problem = MinimumCoveringByCliques::new(graph);

    // The given optimal config
    let config = vec![0, 0, 1, 1, 0, 2, 2, 3, 3];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 4);
}
