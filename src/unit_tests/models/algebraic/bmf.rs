use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_bmf_creation() {
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);
    assert_eq!(problem.rows(), 2);
    assert_eq!(problem.cols(), 2);
    assert_eq!(problem.rank(), 2);
    assert_eq!(problem.num_variables(), 8); // 2*2 + 2*2
}

#[test]
fn test_extract_factors() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    // Config: [b00, c00] = [1, 1]
    let solution = (vec![vec![true]], vec![vec![true]]);
    let (b, c) = problem.extract_factors(&solution);
    assert_eq!(b, vec![vec![true]]);
    assert_eq!(c, vec![vec![true]]);
}

#[test]
fn test_extract_factors_larger() {
    // 2x2 matrix with rank 1
    let matrix = vec![vec![true, true], vec![true, true]];
    let problem = BMF::new(matrix, 1);
    // B: 2x1, C: 1x2
    // Config: [b00, b10, c00, c01] = [1, 1, 1, 1]
    let solution = (vec![vec![true], vec![true]], vec![vec![true, true]]);
    let (b, c) = problem.extract_factors(&solution);
    assert_eq!(b, vec![vec![true], vec![true]]);
    assert_eq!(c, vec![vec![true, true]]);
}

#[test]
fn test_boolean_product() {
    // B = [[1], [1]], C = [[1, 1]]
    // B ⊙ C = [[1,1], [1,1]]
    let b = vec![vec![true], vec![true]];
    let c = vec![vec![true, true]];
    let product = BMF::boolean_product(&b, &c);
    assert_eq!(product, vec![vec![true, true], vec![true, true]]);
}

#[test]
fn test_boolean_product_rank2() {
    // B = [[1,0], [0,1]], C = [[1,0], [0,1]]
    // B ⊙ C = [[1,0], [0,1]] (identity)
    let b = vec![vec![true, false], vec![false, true]];
    let c = vec![vec![true, false], vec![false, true]];
    let product = BMF::boolean_product(&b, &c);
    assert_eq!(product, vec![vec![true, false], vec![false, true]]);
}

#[test]
fn test_hamming_distance() {
    // Target: [[1,0], [0,1]]
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // B = [[1,0], [0,1]], C = [[1,0], [0,1]] -> exact match
    // Config: [1,0,0,1, 1,0,0,1]
    let config = (
        vec![vec![true, false], vec![false, true]],
        vec![vec![true, false], vec![false, true]],
    );
    assert_eq!(problem.hamming_distance(&config).unwrap(), 0);

    // All zeros -> product is all zeros, distance = 2
    let config = (vec![vec![false; 2]; 2], vec![vec![false; 2]; 2]);
    assert_eq!(problem.hamming_distance(&config).unwrap(), 2);
}

#[test]
fn test_evaluate() {
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // Exact factorization -> Min(Some(total_factor_size)) = 4 (two 1s in B, two in C)
    let config = (
        vec![vec![true, false], vec![false, true]],
        vec![vec![true, false], vec![false, true]],
    );
    assert_eq!(Problem::evaluate(&problem, &config).unwrap(), Min(Some(4)));

    // Non-exact -> Min(None)
    let config = (vec![vec![false; 2]; 2], vec![vec![false; 2]; 2]);
    assert_eq!(Problem::evaluate(&problem, &config).unwrap(), Min(None));
}

#[test]
fn test_evaluate_rejects_invalid_configurations() {
    let problem = BMF::new(vec![vec![true]], 1);
    assert!(Problem::evaluate(&problem, &(vec![], vec![vec![true]])).is_err());
    assert!(Problem::evaluate(&problem, &(vec![vec![true]], vec![])).is_err());
    assert!(Problem::evaluate(&problem, &(vec![vec![true, false]], vec![vec![true]])).is_err());
}

#[test]
fn test_brute_force_ones() {
    // All-ones 2x2 factors exactly at rank 1: optimal total_factor_size = 4
    // (B = [[1],[1]] has two 1s, C = [[1,1]] has two 1s).
    let matrix = vec![vec![true, true], vec![true, true]];
    let problem = BMF::new(matrix, 1);
    let solver = BruteForce::new();

    let witnesses = solver.find_all_witnesses(&problem).unwrap();
    assert!(!witnesses.is_empty());
    for sol in &witnesses {
        assert!(problem.is_exact(sol).unwrap());
        assert_eq!(Problem::evaluate(&problem, sol).unwrap(), Min(Some(4)));
    }
}

#[test]
fn test_brute_force_identity() {
    // Identity matrix factors exactly at rank 2.
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);
    let solver = BruteForce::new();

    let witnesses = solver.find_all_witnesses(&problem).unwrap();
    for sol in &witnesses {
        assert!(problem.is_exact(sol).unwrap());
    }
}

#[test]
fn test_brute_force_insufficient_rank() {
    // Rank-1 over the 2x2 identity admits no exact factorization,
    // so every config evaluates to Min(None).
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 1);
    let solver = BruteForce::new();

    let witness = solver.solve(&problem).unwrap();
    assert!(
        witness.is_none()
            || Problem::evaluate(&problem, witness.as_ref().unwrap()).unwrap() == Min(None)
    );
}

#[test]
fn test_boolean_matrix_product_function() {
    let b = vec![vec![true], vec![true]];
    let c = vec![vec![true, true]];
    let product = boolean_matrix_product(&b, &c);
    assert_eq!(product, vec![vec![true, true], vec![true, true]]);
}

#[test]
fn test_matrix_hamming_distance_function() {
    let a = vec![vec![true, false], vec![false, true]];
    let b = vec![vec![true, true], vec![true, true]];
    assert_eq!(matrix_hamming_distance(&a, &b), 2);

    let c = vec![vec![true, false], vec![false, true]];
    assert_eq!(matrix_hamming_distance(&a, &c), 0);
}

#[test]
fn test_empty_matrix() {
    let matrix: Vec<Vec<bool>> = vec![];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.num_variables(), 0);
    // Empty matrix factors exactly with zero factor size.
    assert_eq!(
        Problem::evaluate(&problem, &(vec![], vec![vec![]])).unwrap(),
        Min(Some(0))
    );
}

#[test]
fn test_rank_zero_exactness() {
    let nonzero = BMF::new(vec![vec![true, false]], 0);
    assert_eq!(nonzero.dimensions(), Vec::<usize>::new());
    let empty_factors = (vec![vec![]], vec![]);
    assert_eq!(nonzero.hamming_distance(&empty_factors).unwrap(), 1);
    assert!(!nonzero.is_exact(&empty_factors).unwrap());
    assert_eq!(
        Problem::evaluate(&nonzero, &empty_factors).unwrap(),
        Min(None)
    );

    let zero = BMF::new(vec![vec![false, false]], 0);
    assert_eq!(zero.hamming_distance(&empty_factors).unwrap(), 0);
    assert!(zero.is_exact(&empty_factors).unwrap());
    assert_eq!(
        Problem::evaluate(&zero, &empty_factors).unwrap(),
        Min(Some(0))
    );
}

#[test]
fn test_is_exact() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert!(problem
        .is_exact(&(vec![vec![true]], vec![vec![true]]))
        .unwrap());
    assert!(!problem
        .is_exact(&(vec![vec![false]], vec![vec![false]]))
        .unwrap());
}

#[test]
fn test_bmf_problem() {
    use crate::traits::Problem;

    // 2x2 identity matrix with rank 2
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // dims: B(2*2) + C(2*2) = 8 binary variables
    assert_eq!(problem.dimensions(), vec![2; 8]);

    // Exact factorization: B = I, C = I — total factor size = 4
    assert_eq!(
        Problem::evaluate(
            &problem,
            &(
                vec![vec![true, false], vec![false, true]],
                vec![vec![true, false], vec![false, true]],
            ),
        )
        .unwrap(),
        Min(Some(4))
    );

    // All zeros -> product is all zeros, not equal to A -> infeasible
    assert_eq!(
        Problem::evaluate(
            &problem,
            &(vec![vec![false; 2]; 2], vec![vec![false; 2]; 2])
        )
        .unwrap(),
        Min(None)
    );

    // 1x1 matrix
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.dimensions(), vec![2; 2]); // B(1*1) + C(1*1)
    assert_eq!(
        Problem::evaluate(&problem, &(vec![vec![true]], vec![vec![true]])).unwrap(),
        Min(Some(2))
    );
    assert_eq!(
        Problem::evaluate(&problem, &(vec![vec![false]], vec![vec![false]])).unwrap(),
        Min(None)
    );
}

#[test]
fn test_size_getters() {
    let problem = BMF::new(
        vec![vec![true, false], vec![false, true], vec![true, true]],
        1,
    );
    assert_eq!(problem.m(), 3); // rows
    assert_eq!(problem.n(), 2); // cols
}

#[test]
fn test_bmf_paper_example() {
    // Paper: A=[[1,1,0],[1,1,1],[0,1,1]], k=2, exact factorization
    let matrix = vec![
        vec![true, true, false],
        vec![true, true, true],
        vec![false, true, true],
    ];
    let problem = BMF::new(matrix, 2);
    // B (3x2): [[1,0],[1,1],[0,1]], C (2x3): [[1,1,0],[0,1,1]]
    // Config: B row-major then C row-major
    // Eight 1s total -> optimal total factor size = 8.
    let config = (
        vec![vec![true, false], vec![true, true], vec![false, true]],
        vec![vec![true, true, false], vec![false, true, true]],
    );
    assert!(problem.is_exact(&config).unwrap());
    assert_eq!(Problem::evaluate(&problem, &config).unwrap(), Min(Some(8)));

    let solver = BruteForce::new();
    let best = solver.solve(&problem).unwrap().unwrap();
    assert!(problem.is_exact(&best).unwrap());
}
