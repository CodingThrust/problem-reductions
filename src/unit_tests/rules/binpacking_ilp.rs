use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3 items with weights [3, 3, 2], capacity 5
    let problem = BinPacking::new(vec![3, 3, 2], 5);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3: 9 assignment vars + 3 bin vars = 12
    assert_eq!(ilp.num_vars, 12, "Should have n^2 + n variables");
    // 3 assignment + 3 capacity = 6
    assert_eq!(ilp.constraints.len(), 6, "Should have 2n constraints");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");
}

#[test]
fn test_binpacking_to_ilp_closed_loop() {
    // 4 items with weights [3, 3, 2, 2], capacity 5
    // Optimal: 2 bins, e.g. {3,2} and {3,2}
    let problem = BinPacking::new(vec![3, 3, 2, 2], 5);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve original with brute force
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    assert_eq!(bf_obj, Min(Some(2)));
    assert_eq!(ilp_obj, Min(Some(2)));
}

#[test]
fn test_single_item() {
    let problem = BinPacking::new(vec![5], 10);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 2); // 1 assignment + 1 bin var
    assert_eq!(ilp.constraints.len(), 2); // 1 assignment + 1 capacity

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_same_weight_items() {
    // 4 items all weight 3, capacity 6 -> 2 items per bin -> 2 bins needed
    let problem = BinPacking::new(vec![3, 3, 3, 3], 6);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), Min(Some(2)));
}

#[test]
fn test_exact_fill() {
    // 2 items, weights [5, 5], capacity 10 -> fit in 1 bin
    let problem = BinPacking::new(vec![5, 5], 10);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_solution_extraction() {
    let problem = BinPacking::new(vec![3, 3, 2], 5);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Manually construct an ILP solution:
    // n=3, x_{00}=1 (item 0 in bin 0), x_{11}=1 (item 1 in bin 1), x_{20}=1 (item 2 in bin 0)
    // y_0=1, y_1=1, y_2=0
    let mut ilp_solution = vec![0usize; 12];
    ilp_solution[0] = 1; // x_{0,0} = 1
    ilp_solution[4] = 1; // x_{1,1} = 1
    ilp_solution[6] = 1; // x_{2,0} = 1
    ilp_solution[9] = 1; // y_0 = 1
    ilp_solution[10] = 1; // y_1 = 1

    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_structure_constraints() {
    // 2 items, weights [3, 4], capacity 5
    let problem = BinPacking::new(vec![3, 4], 5);
    let reduction: ReductionBPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 4 assignment vars + 2 bin vars = 6
    assert_eq!(ilp.num_vars, 6);
    // 2 assignment + 2 capacity = 4
    assert_eq!(ilp.constraints.len(), 4);

    // Check objective: minimize y_0 + y_1 (vars at indices 4 and 5)
    let obj_vars: Vec<usize> = ilp.objective.iter().map(|&(v, _)| v).collect();
    assert!(obj_vars.contains(&4));
    assert!(obj_vars.contains(&5));
    for &(_, coef) in &ilp.objective {
        assert!((coef - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_solve_reduced() {
    let problem = BinPacking::new(vec![6, 5, 5, 4, 3], 10);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert!(problem.evaluate(&solution).is_valid());
    assert_eq!(problem.evaluate(&solution), Min(Some(3)));
}
