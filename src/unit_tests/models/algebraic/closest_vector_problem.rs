use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_cvp_creation() {
    // 3D integer lattice: b1=(2,0,0), b2=(1,2,0), b3=(0,1,2)
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(cvp.num_variables(), 3);
    assert_eq!(cvp.ambient_dimension(), 3);
    assert_eq!(cvp.num_basis_vectors(), 3);
}

#[test]
fn test_cvp_evaluate() {
    // b1=(2,0,0), b2=(1,2,0), b3=(0,1,2), target=(3,3,3)
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    // x=(1,1,1) -> Bx=(3,3,2), distance=1.0
    // config offset: x_i - lower = 1 - (-2) = 3
    let config_111 = vec![3, 3, 3]; // maps to x=(1,1,1)
    let result = Problem::evaluate(&cvp, &config_111);
    assert_eq!(result, SolutionSize::Valid(1.0));
}

#[test]
fn test_cvp_direction() {
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 2), VarBounds::bounded(0, 2)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(cvp.direction(), Direction::Minimize);
}

#[test]
fn test_cvp_dims() {
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(-1, 3), VarBounds::bounded(0, 5)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);
    assert_eq!(cvp.dims(), vec![5, 6]); // (-1..3)=5 values, (0..5)=6 values
}

#[test]
fn test_cvp_num_encoding_bits_uses_var_bound_ranges() {
    let basis = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let target = vec![0.0, 0.0, 0.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(0, 5),
        VarBounds::bounded(3, 3),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    assert_eq!(cvp.num_encoding_bits(), 6);
}

#[test]
fn test_var_bounds_exact_encoding_weights_cap_non_power_of_two_ranges() {
    let weights = VarBounds::bounded(0, 5).exact_encoding_weights();
    let represented_offsets: std::collections::BTreeSet<i64> = (0..(1usize << weights.len()))
        .map(|mask| {
            weights
                .iter()
                .enumerate()
                .filter(|(bit, _)| ((mask >> bit) & 1) == 1)
                .map(|(_, &weight)| weight)
                .sum()
        })
        .collect();

    assert_eq!(weights, vec![1, 2, 2]);
    assert_eq!(represented_offsets, (0..=5).collect());
    assert!(VarBounds::bounded(4, 4).exact_encoding_weights().is_empty());
}

#[test]
fn test_cvp_brute_force() {
    // b1=(2,0,0), b2=(1,2,0), b3=(0,1,2), target=(3,3,3)
    // Optimal: x=(1,1,1), Bx=(3,3,2), distance=1.0
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-1, 3),
        VarBounds::bounded(-1, 3),
        VarBounds::bounded(-1, 3),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let solver = BruteForce::new();
    let solution = solver.find_best(&cvp).expect("should find a solution");
    let values: Vec<i64> = solution
        .iter()
        .enumerate()
        .map(|(i, &c)| cvp.bounds()[i].lower.unwrap() + c as i64)
        .collect();
    assert_eq!(values, vec![1, 1, 1]);
    assert_eq!(Problem::evaluate(&cvp, &solution), SolutionSize::Valid(1.0));
}

#[test]
fn test_cvp_serialization() {
    let basis = vec![vec![2, 0, 0], vec![1, 2, 0], vec![0, 1, 2]];
    let target = vec![3.0, 3.0, 3.0];
    let bounds = vec![
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
        VarBounds::bounded(-2, 4),
    ];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let json = serde_json::to_string(&cvp).expect("serialize");
    let cvp2: ClosestVectorProblem<i32> = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(cvp2.num_basis_vectors(), 3);
    assert_eq!(cvp2.ambient_dimension(), 3);
    // Verify functional equivalence after round-trip
    let config = vec![3, 3, 3];
    assert_eq!(
        Problem::evaluate(&cvp, &config),
        Problem::evaluate(&cvp2, &config)
    );
}

#[test]
fn test_cvp_f64_basis() {
    // Non-integer basis to exercise the f64 variant
    let basis: Vec<Vec<f64>> = vec![vec![1.5, 0.0], vec![0.0, 2.0]];
    let target = vec![1.0, 1.0];
    let bounds = vec![VarBounds::bounded(-2, 2), VarBounds::bounded(-2, 2)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let solver = BruteForce::new();
    let solution = solver.find_best(&cvp).expect("should find a solution");
    let values: Vec<i64> = solution
        .iter()
        .enumerate()
        .map(|(i, &c)| cvp.bounds()[i].lower.unwrap() + c as i64)
        .collect();
    // x=(1,1): Bx=(1.5, 2.0), dist=sqrt(0.25+1.0)=sqrt(1.25)≈1.118
    // x=(1,0): Bx=(1.5, 0.0), dist=sqrt(0.25+1.0)=sqrt(1.25)≈1.118
    // x=(0,1): Bx=(0.0, 2.0), dist=sqrt(1.0+1.0)=sqrt(2.0)≈1.414
    // x=(0,0): Bx=(0.0, 0.0), dist=sqrt(1.0+1.0)=sqrt(2.0)≈1.414
    // Both (1,0) and (1,1) tie at sqrt(1.25); brute force returns first found
    assert!(values == vec![1, 0] || values == vec![1, 1]);
}

#[test]
fn test_cvp_2d_identity() {
    // Identity basis in 2D, target=(0.3, 0.7)
    // Closest: x=(0,1), Bx=(0,1), distance=sqrt(0.09+0.09)=0.3*sqrt(2)
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.3, 0.7];
    let bounds = vec![VarBounds::bounded(-2, 2), VarBounds::bounded(-2, 2)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    let solver = BruteForce::new();
    let solution = solver.find_best(&cvp).expect("should find a solution");
    let values: Vec<i64> = solution
        .iter()
        .enumerate()
        .map(|(i, &c)| cvp.bounds()[i].lower.unwrap() + c as i64)
        .collect();
    assert_eq!(values, vec![0, 1]);
}

#[test]
fn test_cvp_evaluate_exact_solution() {
    // Target is exactly a lattice point: t = (2, 2), basis = identity
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![2.0, 2.0];
    let bounds = vec![VarBounds::bounded(0, 4), VarBounds::bounded(0, 4)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    // x=(2,2), Bx=(2,2), distance=0
    let config = vec![2, 2]; // offset from lower=0
    let result = Problem::evaluate(&cvp, &config);
    assert_eq!(result, SolutionSize::Valid(0.0));
}

#[test]
#[should_panic(expected = "bounds length must match")]
fn test_cvp_mismatched_bounds() {
    let basis = vec![vec![1, 0], vec![0, 1]];
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 1)]; // only 1 bound for 2 vars
    ClosestVectorProblem::new(basis, target, bounds);
}

#[test]
#[should_panic(expected = "basis vector")]
fn test_cvp_inconsistent_dimensions() {
    let basis = vec![vec![1, 0], vec![0]]; // second vector has wrong dim
    let target = vec![0.5, 0.5];
    let bounds = vec![VarBounds::bounded(0, 1), VarBounds::bounded(0, 1)];
    ClosestVectorProblem::new(basis, target, bounds);
}

#[test]
fn test_cvp_paper_example() {
    // Paper: basis (2,0),(1,2), target (2.8,1.5), closest (3,2) at x=(1,1)
    let basis = vec![vec![2, 0], vec![1, 2]];
    let target = vec![2.8, 1.5];
    let bounds = vec![VarBounds::bounded(-2, 4), VarBounds::bounded(-2, 4)];
    let cvp = ClosestVectorProblem::new(basis, target, bounds);

    // x=(1,1): Bx = 2*1+1*1=3, 0*1+2*1=2 -> point (3,2)
    // distance = sqrt((2.8-3)^2 + (1.5-2)^2) = sqrt(0.04+0.25) = sqrt(0.29)
    // config offset: x_i - lower = 1 - (-2) = 3
    let config = vec![3, 3]; // maps to x=(1,1)
    let result = Problem::evaluate(&cvp, &config);
    assert!(result.is_valid());
    let dist = result.unwrap();
    assert!((dist - 0.29_f64.sqrt()).abs() < 1e-10);

    let solver = BruteForce::new();
    let best = solver.find_best(&cvp).unwrap();
    let best_dist = Problem::evaluate(&cvp, &best).unwrap();
    assert!((best_dist - 0.29_f64.sqrt()).abs() < 1e-10);
}
