use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

/// Check if a configuration is a valid maximal independent set.
fn is_valid_maximal_is(problem: &MaximalIS<SimpleGraph, i32>, config: &[usize]) -> bool {
    problem.evaluate(config).is_valid()
}

/// Compute the weight of a configuration (sum of selected vertex weights).
fn config_weight(problem: &MaximalIS<SimpleGraph, i32>, config: &[usize]) -> i32 {
    config
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| problem.weights()[i])
        .sum()
}

#[test]
fn test_reduction_creates_valid_ilp() {
    // Path graph P3: 0-1-2 (edges: (0,1), (1,2))
    // Independence constraints: 2 (one per edge)
    // Maximality constraints: 3 (one per vertex)
    // Total constraints: 5
    let problem = MaximalIS::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
    );
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 3, "Should have one variable per vertex");
    assert_eq!(
        ilp.constraints.len(),
        5,
        "Should have 2 independence + 3 maximality constraints"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");
}

#[test]
fn test_maximalis_to_ilp_bf_vs_ilp() {
    // Path graph P3: 0-1-2
    // Maximal independent sets: {0,2} (weight 2) and {1} (weight 1)
    // Maximum weight maximal IS: {0,2} with Max(Some(2))
    let problem = MaximalIS::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
    );
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_best = bf_solutions
        .iter()
        .map(|s| problem.evaluate(s))
        .max()
        .expect("BruteForce should find at least one solution");

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_best, Max(Some(2)));
    assert_eq!(ilp_value, Max(Some(2)));

    // Verify solution is a valid maximal IS
    assert!(
        is_valid_maximal_is(&problem, &extracted),
        "Extracted solution should be a valid maximal IS"
    );
}

#[test]
fn test_solution_extraction() {
    // Path graph P3: 0-1-2
    let problem = MaximalIS::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
    );
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Test 1:1 extraction: {0, 2} is a valid maximal IS
    let ilp_solution = vec![1, 0, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 1]);
    assert!(is_valid_maximal_is(&problem, &extracted));

    // Test extraction of single-vertex IS: {1} is also a valid maximal IS
    let ilp_solution2 = vec![0, 1, 0];
    let extracted2 = reduction.extract_solution(&ilp_solution2);
    assert_eq!(extracted2, vec![0, 1, 0]);
    assert!(is_valid_maximal_is(&problem, &extracted2));
}

#[test]
fn test_maximalis_to_ilp_trivial() {
    // Single vertex with no edges: the only maximal IS is {0}
    let problem = MaximalIS::new(SimpleGraph::new(1, vec![]), vec![5i32]);
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 1);
    // No independence constraints (no edges), 1 maximality constraint
    assert_eq!(ilp.constraints.len(), 1);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1], "Single vertex must be selected");
    assert!(is_valid_maximal_is(&problem, &extracted));
    assert_eq!(problem.evaluate(&extracted), Max(Some(5)));
}

#[test]
fn test_maximalis_to_ilp_star_graph() {
    // Star graph: center 0 connected to leaves 1, 2, 3
    // Maximal IS options: {1,2,3} (leaves, weight 3) or {0} (center, weight 1)
    // Maximum weight maximal IS: {1,2,3} with weight 3
    let problem = MaximalIS::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i32; 4],
    );
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3 independence constraints (edges), 4 maximality constraints
    assert_eq!(ilp.num_vars, 4);
    assert_eq!(ilp.constraints.len(), 7);

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_best = bf_solutions
        .iter()
        .map(|s| problem.evaluate(s))
        .max()
        .expect("BruteForce should find solutions");

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_best, ilp_value);
    assert!(is_valid_maximal_is(&problem, &extracted));
}

#[test]
fn test_maximalis_to_ilp_weighted() {
    // Triangle graph K3: only maximal IS are single vertices
    // Weights [1, 10, 1]: best IS is {1} with weight 10
    let problem = MaximalIS::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 10, 1],
    );
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(is_valid_maximal_is(&problem, &extracted));
    assert_eq!(config_weight(&problem, &extracted), 10);
    assert_eq!(problem.evaluate(&extracted), Max(Some(10)));

    // Vertex 1 (weight 10) should be selected
    assert_eq!(extracted[1], 1);
}

#[test]
fn test_maximalis_to_ilp_path_p5() {
    // Path graph P5: 0-1-2-3-4
    // Optimal maximal IS: {0,2,4} with weight 3
    let problem = MaximalIS::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction: ReductionMaximalISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 4 independence constraints + 5 maximality constraints = 9
    assert_eq!(ilp.num_vars, 5);
    assert_eq!(ilp.constraints.len(), 9);

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_best = bf_solutions
        .iter()
        .map(|s| problem.evaluate(s))
        .max()
        .expect("BruteForce should find solutions");

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_best, Max(Some(3)));
    assert_eq!(ilp_value, Max(Some(3)));

    assert!(is_valid_maximal_is(&problem, &extracted));
}
