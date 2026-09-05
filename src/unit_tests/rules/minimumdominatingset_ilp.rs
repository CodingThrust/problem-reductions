use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i64; 3],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars(), 3, "Should have one variable per vertex");
    assert_eq!(
        ilp.constraints().len(),
        3,
        "Should have one constraint per vertex"
    );
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize, "Should minimize");

    // Each constraint should be x_v + sum_{u in N(v)} x_u >= 1
    for constraint in ilp.constraints() {
        assert!(!constraint.terms().is_empty());
        assert_eq!(constraint.rhs(), 1);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // Check that weights are correctly transferred to objective
    let mut coeffs: Vec<i64> = vec![0; 3];
    for &(var, coef) in ilp.objective() {
        coeffs[var] = coef;
    }
    assert_eq!(coeffs, vec![5, 10, 15]);
}

#[test]
fn test_minimumdominatingset_to_ilp_closed_loop() {
    // Star graph: center vertex 0 connected to all others
    // Minimum dominating set is just the center (weight 1)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i64; 4],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let bf_size = problem.evaluate(&bf_solutions[0]).unwrap();

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_size = problem.evaluate(&extracted).unwrap();

    // Both should find optimal size = 1 (just the center)
    assert_eq!(bf_size, Min(Some(1)));
    assert_eq!(ilp_size, Min(Some(1)));

    // Verify the ILP solution is valid for the original problem
    assert!(
        problem.evaluate(&extracted).unwrap().is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_ilp_solution_equals_brute_force_path() {
    // Path graph 0-1-2-3-4: min DS = 2 (e.g., vertices 1 and 3)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i64; 5],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force
    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let bf_size = problem.evaluate(&bf_solutions[0]).unwrap();

    // Solve via ILP
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_size = problem.evaluate(&extracted).unwrap();

    assert_eq!(bf_size, Min(Some(2)));
    assert_eq!(ilp_size, Min(Some(2)));

    // Verify validity
    assert!(problem.evaluate(&extracted).unwrap().is_valid());
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    // Star with heavy center: prefer selecting all leaves (total weight 3)
    // over center (weight 100)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![100, 1, 1, 1],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let bf_obj = problem.evaluate(&bf_solutions[0]).unwrap();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_obj = problem.evaluate(&extracted).unwrap();

    assert_eq!(bf_obj, Min(Some(3)));
    assert_eq!(ilp_obj, Min(Some(3)));

    // Verify the solution selects all leaves
    assert_eq!(extracted, vec![false, true, true, true]);
}

#[test]
fn test_solution_extraction() {
    let problem =
        MinimumDominatingSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i64; 4]);
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![true, false, true, false]);

    // Verify this is a valid DS (0 dominates 0,1 and 2 dominates 2,3)
    assert!(problem.evaluate(&extracted).unwrap().is_valid());
}

#[test]
fn test_ilp_structure() {
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i64; 5],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 5);
    assert_eq!(ilp.constraints().len(), 5); // one per vertex
}

#[test]
fn test_isolated_vertices() {
    // Graph with isolated vertex 2: it must be in the dominating set
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // Vertex 2 must be selected (isolated)
    assert!(extracted[2]);

    assert!(problem.evaluate(&extracted).unwrap().is_valid());
}

#[test]
fn test_complete_graph() {
    // Complete graph K4: min DS = 1 (any vertex dominates all)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1i64; 4],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.evaluate(&extracted).unwrap().is_valid());
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(1)));
}

#[test]
fn test_single_vertex() {
    // Single vertex with no edges: must be in dominating set
    let problem = MinimumDominatingSet::new(SimpleGraph::new(1, vec![]), vec![1i64; 1]);
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(extracted, vec![true]);

    assert!(problem.evaluate(&extracted).unwrap().is_valid());
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(1)));
}

#[test]
fn test_cycle_graph() {
    // Cycle C5: 0-1-2-3-4-0
    // Minimum dominating set size = 2
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
        vec![1i64; 5],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let bf_size = problem.evaluate(&bf_solutions[0]).unwrap();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_size = problem.evaluate(&extracted).unwrap();

    assert_eq!(bf_size, ilp_size);

    assert!(problem.evaluate(&extracted).unwrap().is_valid());
}

#[test]
fn test_minimumdominatingset_to_ilp_bf_vs_ilp() {
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i64; 4],
    );
    let reduction: ReductionDSToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
