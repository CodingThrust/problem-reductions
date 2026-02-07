use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;

#[test]
fn test_matching_to_setpacking_structure() {
    // Path graph 0-1-2
    let matching = Matching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    // Should have 2 sets (one for each edge)
    assert_eq!(sp.num_sets(), 2);

    // Sets should contain edge endpoints
    let sets = sp.sets();
    assert_eq!(sets[0], vec![0, 1]);
    assert_eq!(sets[1], vec![1, 2]);
}

#[test]
fn test_matching_to_setpacking_path() {
    // Path 0-1-2-3 with unit weights
    let matching = Matching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);

    // Extract back to Matching solutions
    let _matching_solutions: Vec<_> = sp_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify against direct Matching solution
    let direct_solutions = solver.find_best(&matching);

    // Solutions should have same objective value
    let sp_size: usize = sp_solutions[0].iter().sum();
    let direct_size: usize = direct_solutions[0].iter().sum();
    assert_eq!(sp_size, direct_size);
    assert_eq!(sp_size, 2); // Max matching in path graph has 2 edges
}

#[test]
fn test_matching_to_setpacking_triangle() {
    // Triangle graph
    let matching = Matching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);

    // Max matching in triangle = 1 (any single edge)
    for sol in &sp_solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }

    // Should have 3 optimal solutions (one for each edge)
    assert_eq!(sp_solutions.len(), 3);
}

#[test]
fn test_matching_to_setpacking_weighted() {
    // Weighted edges: heavy edge should win over multiple light edges
    let matching =
        Matching::<SimpleGraph, i32>::new(4, vec![(0, 1, 100), (0, 2, 1), (1, 3, 1)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(sp.weights_ref(), &vec![100, 1, 1]);

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);

    // Edge 0-1 (weight 100) alone beats edges 0-2 + 1-3 (weight 2)
    assert!(sp_solutions.contains(&vec![1, 0, 0]));

    // Verify through direct Matching solution
    let direct_solutions = solver.find_best(&matching);
    assert_eq!(matching.solution_size(&sp_solutions[0]).size, 100);
    assert_eq!(matching.solution_size(&direct_solutions[0]).size, 100);
}

#[test]
fn test_matching_to_setpacking_solution_extraction() {
    let matching = Matching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);

    // Test solution extraction is 1:1
    let sp_solution = vec![1, 0, 1];
    let matching_solution = reduction.extract_solution(&sp_solution);
    assert_eq!(matching_solution, vec![1, 0, 1]);

    // Verify the extracted solution is valid for original Matching
    assert!(matching.solution_size(&matching_solution).is_valid);
}

#[test]
fn test_matching_to_setpacking_k4() {
    // Complete graph K4: can have perfect matching (2 edges covering all 4 vertices)
    let matching = Matching::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    );
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);
    let direct_solutions = solver.find_best(&matching);

    // Both should find matchings of size 2
    let sp_size: usize = sp_solutions[0].iter().sum();
    let direct_size: usize = direct_solutions[0].iter().sum();
    assert_eq!(sp_size, 2);
    assert_eq!(direct_size, 2);
}

#[test]
fn test_matching_to_setpacking_empty() {
    // Graph with no edges
    let matching = Matching::<SimpleGraph, i32>::unweighted(3, vec![]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    assert_eq!(sp.num_sets(), 0);
}

#[test]
fn test_matching_to_setpacking_single_edge() {
    let matching = Matching::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    assert_eq!(sp.num_sets(), 1);
    assert_eq!(sp.sets()[0], vec![0, 1]);

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);

    // Should select the only set
    assert_eq!(sp_solutions, vec![vec![1]]);
}

#[test]
fn test_matching_to_setpacking_disjoint_edges() {
    // Two disjoint edges: 0-1 and 2-3
    let matching = Matching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (2, 3)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);

    // Both edges can be selected (they don't share vertices)
    assert_eq!(sp_solutions, vec![vec![1, 1]]);
}

#[test]
fn test_reduction_sizes() {
    let matching = Matching::<SimpleGraph, i32>::unweighted(5, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();

    assert_eq!(source_size.get("num_vertices"), Some(5));
    assert_eq!(source_size.get("num_edges"), Some(3));
    assert_eq!(target_size.get("num_sets"), Some(3));
}

#[test]
fn test_matching_to_setpacking_star() {
    // Star graph: center vertex 0 connected to 1, 2, 3
    let matching = Matching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (0, 2), (0, 3)]);
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp);

    // All edges share vertex 0, so max matching = 1
    for sol in &sp_solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
    // Should have 3 optimal solutions
    assert_eq!(sp_solutions.len(), 3);
}
