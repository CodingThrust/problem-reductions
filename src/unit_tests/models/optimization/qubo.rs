use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_qubo_from_matrix() {
    let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);
    assert_eq!(problem.num_vars(), 2);
    assert_eq!(problem.get(0, 0), Some(&1.0));
    assert_eq!(problem.get(0, 1), Some(&2.0));
    assert_eq!(problem.get(1, 1), Some(&3.0));
}

#[test]
fn test_qubo_new() {
    let problem = QUBO::new(vec![1.0, 2.0], vec![((0, 1), 3.0)]);
    assert_eq!(problem.get(0, 0), Some(&1.0));
    assert_eq!(problem.get(1, 1), Some(&2.0));
    assert_eq!(problem.get(0, 1), Some(&3.0));
}

#[test]
fn test_evaluate() {
    // Q = [[1, 2], [0, 3]]
    // f(x) = x0 + 3*x1 + 2*x0*x1
    let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);

    assert_eq!(problem.evaluate(&[0, 0]), 0.0);
    assert_eq!(problem.evaluate(&[1, 0]), 1.0);
    assert_eq!(problem.evaluate(&[0, 1]), 3.0);
    assert_eq!(problem.evaluate(&[1, 1]), 6.0); // 1 + 3 + 2 = 6
}

#[test]
fn test_solution_size() {
    let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);

    let sol = problem.solution_size(&[0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0.0);

    let sol = problem.solution_size(&[1, 1]);
    assert_eq!(sol.size, 6.0);
}

#[test]
fn test_brute_force_minimize() {
    // Q = [[1, 0], [0, -2]]
    // f(x) = x0 - 2*x1
    // Minimum at x = [0, 1] with value -2
    let problem = QUBO::from_matrix(vec![vec![1.0, 0.0], vec![0.0, -2.0]]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1]);
    assert_eq!(problem.solution_size(&solutions[0]).size, -2.0);
}

#[test]
fn test_brute_force_with_interaction() {
    // Q = [[-1, 2], [0, -1]]
    // f(x) = -x0 - x1 + 2*x0*x1
    // x=[0,0] -> 0, x=[1,0] -> -1, x=[0,1] -> -1, x=[1,1] -> 0
    let problem = QUBO::from_matrix(vec![vec![-1.0, 2.0], vec![0.0, -1.0]]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Minimum is -1 at [1,0] or [0,1]
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, -1.0);
    }
}

#[test]
fn test_energy_mode() {
    let problem = QUBO::<f64>::from_matrix(vec![vec![1.0]]);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_num_variables_flavors() {
    let problem = QUBO::<f64>::from_matrix(vec![vec![0.0; 5]; 5]);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_problem_size() {
    let problem = QUBO::<f64>::from_matrix(vec![vec![0.0; 3]; 3]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
}

#[test]
fn test_matrix_access() {
    let problem = QUBO::from_matrix(vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 4.0, 5.0],
        vec![0.0, 0.0, 6.0],
    ]);
    let matrix = problem.matrix();
    assert_eq!(matrix.len(), 3);
    assert_eq!(matrix[0], vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_empty_qubo() {
    let problem = QUBO::<f64>::from_matrix(vec![]);
    assert_eq!(problem.num_vars(), 0);
    assert_eq!(problem.evaluate(&[]), 0.0);
}

#[test]
fn test_single_variable() {
    let problem = QUBO::from_matrix(vec![vec![-5.0]]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1]); // x=1 gives -5, x=0 gives 0
}

#[test]
fn test_qubo_new_reverse_indices() {
    // Test the case where (j, i) is provided with i < j
    let problem = QUBO::new(vec![1.0, 2.0], vec![((1, 0), 3.0)]); // j > i
    assert_eq!(problem.get(0, 1), Some(&3.0)); // Should be stored at (0, 1)
}

#[test]
fn test_get_out_of_bounds() {
    let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);
    assert_eq!(problem.get(5, 5), None);
    assert_eq!(problem.get(0, 5), None);
}

#[test]
fn test_qubo_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Simple 2-variable QUBO: Q = [[1, -2], [0, 1]]
    // f(x) = x0 - 2*x0*x1 + x1
    let q = vec![vec![1.0, -2.0], vec![0.0, 1.0]];
    let p = QUBO::<f64>::from_matrix(q);
    assert_eq!(p.dims(), vec![2, 2]);
    // x = [0, 0]: f = 0
    assert_eq!(ProblemV2::evaluate(&p, &[0, 0]), 0.0);
    // x = [1, 1]: f = 1 - 2 + 1 = 0
    assert_eq!(ProblemV2::evaluate(&p, &[1, 1]), 0.0);
    // x = [1, 0]: f = 1
    assert_eq!(ProblemV2::evaluate(&p, &[1, 0]), 1.0);
    assert_eq!(p.direction(), Direction::Minimize);
}
