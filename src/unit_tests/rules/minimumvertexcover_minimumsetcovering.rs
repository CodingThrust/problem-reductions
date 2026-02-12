use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_vc_to_sc_basic() {
    // Path graph 0-1-2 with edges (0,1) and (1,2)
    // Vertex 0 covers edge 0
    // Vertex 1 covers edges 0 and 1
    // Vertex 2 covers edge 1
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    // Check the sets are constructed correctly
    assert_eq!(sc_problem.universe_size(), 2); // 2 edges
    assert_eq!(sc_problem.num_sets(), 3); // 3 vertices

    // Set 0 (vertex 0): should contain edge 0
    assert_eq!(sc_problem.get_set(0), Some(&vec![0]));
    // Set 1 (vertex 1): should contain edges 0 and 1
    assert_eq!(sc_problem.get_set(1), Some(&vec![0, 1]));
    // Set 2 (vertex 2): should contain edge 1
    assert_eq!(sc_problem.get_set(2), Some(&vec![1]));
}

#[test]
fn test_vc_to_sc_triangle() {
    // Triangle graph: 3 vertices, 3 edges
    // Edge indices: (0,1)->0, (1,2)->1, (0,2)->2
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    assert_eq!(sc_problem.universe_size(), 3);
    assert_eq!(sc_problem.num_sets(), 3);

    // Verify each vertex covers exactly 2 edges
    for i in 0..3 {
        let set = sc_problem.get_set(i).unwrap();
        assert_eq!(set.len(), 2);
    }
}

#[test]
fn test_vc_to_sc_solution_extraction() {
    use crate::traits::Problem;

    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    // Solve the MinimumSetCovering problem
    let solver = BruteForce::new();
    let sc_solutions = solver.find_best(sc_problem);

    // Extract solutions back to MinimumVertexCover
    let vc_solutions: Vec<_> = sc_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify extracted solutions are valid vertex covers
    for sol in &vc_solutions {
        // Check that the solution evaluates to a valid value (not i32::MAX for invalid)
        let eval = vc_problem.evaluate(sol);
        assert!(eval.is_valid());
    }

    // The minimum should be selecting just vertex 1 (covers both edges)
    let min_size: usize = vc_solutions[0].iter().sum();
    assert_eq!(min_size, 1);
}

#[test]
fn test_vc_to_sc_optimality_preservation() {
    // Test that optimal solutions are preserved through reduction
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();

    // Solve VC directly
    let direct_solutions = solver.find_best(&vc_problem);
    let direct_size = direct_solutions[0].iter().sum::<usize>();

    // Solve via reduction
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_solutions = solver.find_best(reduction.target_problem());
    let reduced_solutions: Vec<_> = sc_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();
    let reduced_size = reduced_solutions[0].iter().sum::<usize>();

    // Optimal sizes should match
    assert_eq!(direct_size, reduced_size);
}

#[test]
fn test_vc_to_sc_weighted() {
    // Weighted problem: weights should be preserved
    let vc_problem = MinimumVertexCover::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 1, 10]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    // Weights should be preserved - access via weights_ref method on the problem
    assert_eq!(*sc_problem.weights_ref(), vec![10, 1, 10]);

    // Solve both ways
    let solver = BruteForce::new();
    let vc_solutions = solver.find_best(&vc_problem);
    let sc_solutions = solver.find_best(sc_problem);

    // Both should select vertex 1 (weight 1)
    assert_eq!(vc_solutions[0], vec![0, 1, 0]);
    assert_eq!(sc_solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_vc_to_sc_empty_graph() {
    // Graph with no edges
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    assert_eq!(sc_problem.universe_size(), 0);
    assert_eq!(sc_problem.num_sets(), 3);

    // All sets should be empty
    for i in 0..3 {
        assert!(sc_problem.get_set(i).unwrap().is_empty());
    }
}

#[test]
fn test_vc_to_sc_star_graph() {
    // Star graph: center vertex 0 connected to all others
    // Edges: (0,1), (0,2), (0,3)
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    // Vertex 0 should cover all 3 edges
    assert_eq!(sc_problem.get_set(0), Some(&vec![0, 1, 2]));
    // Other vertices cover only 1 edge each
    assert_eq!(sc_problem.get_set(1), Some(&vec![0]));
    assert_eq!(sc_problem.get_set(2), Some(&vec![1]));
    assert_eq!(sc_problem.get_set(3), Some(&vec![2]));

    // Minimum cover should be just vertex 0
    let solver = BruteForce::new();
    let solutions = solver.find_best(&vc_problem);
    assert_eq!(solutions[0], vec![1, 0, 0, 0]);
}

#[test]
fn test_vc_to_sc_all_solutions_valid() {
    use crate::traits::Problem;

    // Ensure all solutions extracted from SC are valid VC solutions
    let vc_problem =
        MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&vc_problem);
    let sc_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let sc_solutions = solver.find_best(sc_problem);

    for sc_sol in &sc_solutions {
        let vc_sol = reduction.extract_solution(sc_sol);
        let eval = vc_problem.evaluate(&vc_sol);
        assert!(
            eval .is_valid(),
            "Extracted solution {:?} should be valid",
            vc_sol
        );
    }
}
