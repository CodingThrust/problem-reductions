use super::*;
use crate::models::algebraic::ObjectiveSense;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_maximumdomaticnumber_to_ilp_closed_loop() {
    // Path P3: 0-1-2, domatic number = 2
    let problem = MaximumDomaticNumber::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionDomaticNumberToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_witness = bf.find_witness(&problem).unwrap();
    let bf_value = problem.evaluate(&bf_witness);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    // Both should find domatic number = 2
    assert_eq!(bf_value, Max(Some(2)));
    assert_eq!(ilp_value, Max(Some(2)));

    // Verify the ILP solution is valid for the original problem
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_maximumdomaticnumber_to_ilp_structure() {
    // P3: 3 vertices
    let problem = MaximumDomaticNumber::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionDomaticNumberToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3: n²+n = 12 variables
    assert_eq!(ilp.num_vars, 12);

    // Constraints: n + n² + n = 3 + 9 + 3 = 15
    assert_eq!(ilp.constraints.len(), 15);

    // Objective should be maximize
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);

    // Objective should have 3 terms (y_0, y_1, y_2)
    assert_eq!(ilp.objective.len(), 3);
    for &(var, coef) in &ilp.objective {
        assert!(var >= 9); // y_i at indices 9, 10, 11
        assert!((coef - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_maximumdomaticnumber_to_ilp_bf_vs_ilp() {
    // P3: 3 vertices, domatic number = 2
    let problem = MaximumDomaticNumber::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionDomaticNumberToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_maximumdomaticnumber_to_ilp_complete_graph() {
    // K3: domatic number = 3 (each vertex is its own dominating set)
    let problem = MaximumDomaticNumber::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]));
    let reduction: ReductionDomaticNumberToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);

    assert_eq!(value, Max(Some(3)));
}

#[test]
fn test_maximumdomaticnumber_to_ilp_single_vertex() {
    // Single vertex: domatic number = 1
    let problem = MaximumDomaticNumber::new(SimpleGraph::new(1, vec![]));
    let reduction: ReductionDomaticNumberToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);

    assert_eq!(value, Max(Some(1)));
}

#[test]
fn test_maximumdomaticnumber_to_ilp_solution_extraction() {
    // P3: 0-1-2
    let problem = MaximumDomaticNumber::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionDomaticNumberToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Manually construct an ILP solution: vertices 0,2 in set 0, vertex 1 in set 1
    // x_{0,0}=1, x_{0,1}=0, x_{0,2}=0,
    // x_{1,0}=0, x_{1,1}=1, x_{1,2}=0,
    // x_{2,0}=1, x_{2,1}=0, x_{2,2}=0,
    // y_0=1, y_1=1, y_2=0
    let ilp_solution = vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 0]);

    // Verify this is a valid partition with 2 dominating sets
    let value = problem.evaluate(&extracted);
    assert_eq!(value, Max(Some(2)));
}
