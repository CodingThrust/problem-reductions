use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_biclique_cover_creation() {
    let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2)], 2);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.k(), 2);
    assert_eq!(problem.num_variables(), 8); // 4 vertices * 2 bicliques
}

#[test]
fn test_from_matrix() {
    // Matrix:
    // [[1, 1],
    //  [1, 0]]
    // Edges: (0,2), (0,3), (1,2)
    let matrix = vec![vec![1, 1], vec![1, 0]];
    let problem = BicliqueCover::from_matrix(&matrix, 2);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_get_biclique_memberships() {
    let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);
    // Config: vertex 0 in biclique 0, vertex 2 in biclique 0
    // Variables: [v0_b0, v1_b0, v2_b0, v3_b0]
    let config = vec![1, 0, 1, 0];
    let (left, right) = problem.get_biclique_memberships(&config);
    assert!(left[0].contains(&0));
    assert!(!left[0].contains(&1));
    assert!(right[0].contains(&2));
    assert!(!right[0].contains(&3));
}

#[test]
fn test_is_edge_covered() {
    let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);
    // Put vertex 0 and 2 in biclique 0
    let config = vec![1, 0, 1, 0];
    assert!(problem.is_edge_covered(0, 2, &config));

    // Don't put vertex 2 in biclique
    let config = vec![1, 0, 0, 0];
    assert!(!problem.is_edge_covered(0, 2, &config));
}

#[test]
fn test_is_valid_cover() {
    let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3)], 1);
    // Put 0, 2, 3 in biclique 0 -> covers both edges
    let config = vec![1, 0, 1, 1];
    assert!(problem.is_valid_cover(&config));

    // Only put 0, 2 -> doesn't cover (0,3)
    let config = vec![1, 0, 1, 0];
    assert!(!problem.is_valid_cover(&config));
}

#[test]
fn test_evaluate() {
    let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);

    // Valid cover with size 2
    assert_eq!(problem.evaluate(&[1, 0, 1, 0]), SolutionSize::Valid(2));

    // Invalid cover returns Invalid
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_brute_force_simple() {
    // Single edge (0, 2) with k=1
    let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        assert!(problem.is_valid_cover(sol));
        // Minimum size is 2 (one left, one right vertex)
        assert_eq!(problem.total_biclique_size(sol), 2);
    }
}

#[test]
fn test_brute_force_two_bicliques() {
    // Edges that need 2 bicliques to cover efficiently
    // (0,2), (1,3) - these don't share vertices
    let problem = BicliqueCover::new(2, 2, vec![(0, 2), (1, 3)], 2);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        assert!(problem.is_valid_cover(sol));
    }
}

#[test]
fn test_count_covered_edges() {
    let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2)], 1);
    // Cover only (0,2): put 0 and 2 in biclique
    let config = vec![1, 0, 1, 0];
    assert_eq!(problem.count_covered_edges(&config), 1);

    // Cover (0,2) and (0,3): put 0, 2, 3 in biclique
    let config = vec![1, 0, 1, 1];
    assert_eq!(problem.count_covered_edges(&config), 2);
}

#[test]
fn test_is_biclique_cover_function() {
    let edges = vec![(0, 2), (1, 3)];
    let left = vec![vec![0].into_iter().collect(), vec![1].into_iter().collect()];
    let right = vec![vec![2].into_iter().collect(), vec![3].into_iter().collect()];
    assert!(is_biclique_cover(&edges, &left, &right));

    // Missing coverage
    let left = vec![vec![0].into_iter().collect()];
    let right = vec![vec![2].into_iter().collect()];
    assert!(!is_biclique_cover(&edges, &left, &right));
}

#[test]
fn test_direction() {
    let problem = BicliqueCover::new(1, 1, vec![(0, 1)], 1);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_empty_edges() {
    let problem = BicliqueCover::new(2, 2, vec![], 1);
    // No edges to cover -> valid with size 0
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_biclique_problem() {
    use crate::traits::{OptimizationProblem, Problem};
    use crate::types::Direction;

    // Single edge (0, 2) with k=1, 2 left + 2 right vertices
    let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);

    // dims: 4 vertices * 1 biclique = 4 binary variables
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);

    // Valid cover: vertex 0 and vertex 2 in biclique 0
    // Config: [v0_b0=1, v1_b0=0, v2_b0=1, v3_b0=0]
    assert_eq!(problem.evaluate(&[1, 0, 1, 0]), SolutionSize::Valid(2));

    // Invalid cover: only vertex 0, edge (0,2) not covered
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), SolutionSize::Invalid);

    // Valid cover with all vertices -> size 4
    assert_eq!(problem.evaluate(&[1, 1, 1, 1]), SolutionSize::Valid(4));

    // Empty config: no vertices in biclique, edge not covered
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), SolutionSize::Invalid);

    // Direction is minimize
    assert_eq!(problem.direction(), Direction::Minimize);

    // Test with no edges: any config is valid
    let empty_problem = BicliqueCover::new(2, 2, vec![], 1);
    assert_eq!(empty_problem.evaluate(&[0, 0, 0, 0]), SolutionSize::Valid(0));
}
