use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

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
fn test_direction() {
    let problem = QUBO::<f64>::from_matrix(vec![vec![1.0]]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_num_variables() {
    let problem = QUBO::<f64>::from_matrix(vec![vec![0.0; 5]; 5]);
    assert_eq!(problem.num_variables(), 5);
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
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0.0));
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
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/qubo.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let jl_matrix: Vec<Vec<f64>> = instance["instance"]["matrix"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                row.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect()
            })
            .collect();
        let n = jl_matrix.len();
        let mut rust_matrix = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            rust_matrix[i][i] = jl_matrix[i][i];
            for j in (i + 1)..n {
                rust_matrix[i][j] = jl_matrix[i][j] + jl_matrix[j][i];
            }
        }
        let problem = QUBO::from_matrix(rust_matrix);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result: SolutionSize<f64> = Problem::evaluate(&problem, &config);
            let jl_size = eval["size"].as_f64().unwrap();
            assert!(result.is_valid(), "QUBO should always be valid");
            assert!(
                (result.unwrap() - jl_size).abs() < 1e-10,
                "QUBO value mismatch for config {:?}",
                config
            );
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "QUBO best solutions mismatch");
    }
}
