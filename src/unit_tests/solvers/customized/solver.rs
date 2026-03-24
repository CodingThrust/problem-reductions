use crate::solvers::CustomizedSolver;

#[test]
fn test_customized_solver_returns_none_for_unsupported_problem() {
    let problem = crate::models::misc::GroupingBySwapping::new(3, vec![0, 1, 2, 0, 1, 2], 2);
    let solver = CustomizedSolver::new();
    assert!(solver.solve_dyn(&problem).is_none());
}
