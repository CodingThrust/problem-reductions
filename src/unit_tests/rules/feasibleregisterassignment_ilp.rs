use super::*;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;

fn feasible_example() -> FeasibleRegisterAssignment {
    FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0])
}

#[test]
fn test_feasible_register_assignment_to_ilp_structure() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 14);
    assert_eq!(ilp.constraints.len(), 42);
    assert_eq!(ilp.objective, vec![]);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_feasible_register_assignment_to_ilp_closed_loop() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("feasible source instance should yield a feasible ILP");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(source.evaluate(&extracted), Or(true));
    let mut sorted = extracted.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1, 2, 3]);
}

#[test]
fn test_feasible_register_assignment_to_ilp_infeasible() {
    let source = FeasibleRegisterAssignment::new(3, vec![(0, 1), (0, 2), (1, 2)], 1, vec![0, 0, 0]);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);

    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "register-conflict source instance should reduce to an infeasible ILP"
    );
}

#[test]
fn test_feasible_register_assignment_to_ilp_bf_vs_ilp() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
