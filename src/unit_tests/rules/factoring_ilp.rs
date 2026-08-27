use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use num_bigint::BigUint;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Factor 6 with 2-bit factors
    let problem = Factoring::new(2, 2, 6);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // Check variable count: m + n + m*n + (m+n) = 2 + 2 + 4 + 4 = 12
    assert_eq!(ilp.num_vars(), 12);

    // Check constraint count: 3*m*n + 4*m + 4*n + 1 = 12 + 8 + 8 + 1 = 29
    assert_eq!(ilp.constraints().len(), 29);

    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_variable_layout() {
    let problem = Factoring::new(3, 2, 6);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    // p variables: [0, 1, 2]
    assert_eq!(reduction.p_var(0), 0);
    assert_eq!(reduction.p_var(2), 2);

    // q variables: [3, 4]
    assert_eq!(reduction.q_var(0), 3);
    assert_eq!(reduction.q_var(1), 4);

    // z variables: [5, 6, 7, 8, 9, 10] (3x2 = 6)
    assert_eq!(reduction.z_var(0, 0), 5);
    assert_eq!(reduction.z_var(0, 1), 6);
    assert_eq!(reduction.z_var(1, 0), 7);
    assert_eq!(reduction.z_var(2, 1), 10);

    // carry variables: [11, 12, 13, 14, 15] (m+n = 5)
    assert_eq!(reduction.carry_var(0), 11);
    assert_eq!(reduction.carry_var(4), 15);
}

#[test]
fn test_factor_6() {
    // 6 = 2 × 3 or 3 × 2
    let problem = Factoring::new(2, 2, 6);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // Verify it's a valid factorization
    assert!(problem.is_valid_factorization(&extracted));

    let (a, b) = extracted.clone();
    assert_eq!(a * b, BigUint::from(6u32));
}

#[test]
fn test_factor_15() {
    // Closed-loop test for factoring 15 = 3 × 5 (or 5 × 3, 1 × 15, 15 × 1)

    // 1. Create factoring instance: find p (4-bit) × q (4-bit) = 15
    let problem = Factoring::new(4, 4, 15);

    // 2. Reduce to ILP
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // 3. Solve ILP
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");

    // 4. Extract factoring solution
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // 5. Verify: solution is valid and p × q = 15
    assert!(problem.is_valid_factorization(&extracted));
    let (p, q) = extracted.clone();
    assert_eq!(p * q, BigUint::from(15u32)); // e.g., (3, 5) or (5, 3)
}

#[test]
fn test_factor_35() {
    // 35 = 5 × 7 or 7 × 5
    let problem = Factoring::new(3, 3, 35);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.is_valid_factorization(&extracted));

    let (a, b) = extracted.clone();
    assert_eq!(a * b, BigUint::from(35u32));
}

#[test]
fn test_factor_one() {
    // 1 = 1 × 1
    let problem = Factoring::new(2, 2, 1);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.is_valid_factorization(&extracted));

    let (a, b) = extracted.clone();
    assert_eq!(a * b, BigUint::from(1u32));
}

#[test]
fn test_factor_prime() {
    // 7 is prime: 7 = 1 × 7 or 7 × 1
    let problem = Factoring::new(3, 3, 7);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.is_valid_factorization(&extracted));

    let (a, b) = extracted.clone();
    assert_eq!(a * b, BigUint::from(7u32));
}

#[test]
fn test_factor_square() {
    // 9 = 3 × 3
    let problem = Factoring::new(3, 3, 9);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.is_valid_factorization(&extracted));

    let (a, b) = extracted.clone();
    assert_eq!(a * b, BigUint::from(9u32));
}

#[test]
fn test_infeasible_target_too_large() {
    // Target 100 with 2-bit factors (max product is 3 × 3 = 9)
    let problem = Factoring::new(2, 2, 100);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(ilp);

    assert!(result.is_err(), "Should be infeasible");
}

#[test]
fn test_factoring_to_ilp_closed_loop() {
    let problem = Factoring::new(2, 2, 6);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // Get ILP solution
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let ilp_factors = reduction.extract_solution(&ilp_solution).unwrap();

    // Get brute force solutions
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();

    // ILP solution should be among brute force solutions
    let (a, b) = ilp_factors.clone();
    let bf_pairs: Vec<(BigUint, BigUint)> = bf_solutions.to_vec();

    assert!(
        bf_pairs.contains(&(a.clone(), b.clone())),
        "ILP solution ({}, {}) should be in brute force solutions {:?}",
        a,
        b,
        bf_pairs
    );
}

#[test]
fn test_solution_extraction() {
    let problem = Factoring::new(2, 2, 6);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    // Manually construct ILP solution for 2 × 3 = 6
    // p = 2 = binary 10 -> p_0=0, p_1=1
    // q = 3 = binary 11 -> q_0=1, q_1=1
    // z_00 = p_0 * q_0 = 0, z_01 = p_0 * q_1 = 0
    // z_10 = p_1 * q_0 = 1, z_11 = p_1 * q_1 = 1
    // Variables: [p0, p1, q0, q1, z00, z01, z10, z11, c0, c1, c2, c3]
    let ilp_solution = vec![0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0];
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(extracted, (BigUint::from(2u32), BigUint::from(3u32)));

    let (a, b) = extracted.clone();
    assert_eq!(a, BigUint::from(2u32));
    assert_eq!(b, BigUint::from(3u32));
    assert_eq!(a * b, BigUint::from(6u32));
}

#[test]
fn test_target_ilp_structure() {
    let problem = Factoring::new(3, 4, 12);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // num_vars = 3 + 4 + 12 + 7 = 26
    assert_eq!(ilp.num_vars(), 26);

    // num_constraints = 3*12 + 4*3 + 4*4 + 1 = 36 + 12 + 16 + 1 = 65
    assert_eq!(ilp.constraints().len(), 65);
}

#[test]
fn test_solve_reduced() {
    let problem = Factoring::new(2, 2, 6);

    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let solution = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.is_valid_factorization(&solution));
}

#[test]
fn test_asymmetric_bit_widths() {
    // 12 = 3 × 4 or 4 × 3 or 2 × 6 or 6 × 2 or 1 × 12 or 12 × 1
    let problem = Factoring::new(2, 4, 12);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(problem.is_valid_factorization(&extracted));

    let (a, b) = extracted.clone();
    assert_eq!(a * b, BigUint::from(12u32));
}

#[test]
fn test_oversized_biguint_target_makes_ilp_infeasible() {
    let target = BigUint::from(1u32) << 70;
    let problem = Factoring::new(2, 2, target);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_constraint_count_formula() {
    // Verify constraint count matches formula: 3*m*n + 4*m + 4*n + 1
    // (3*m*n McCormick + (m+n) bit equations + 1 final carry + (m+n) binary bounds + 2*(m+n) carry bounds)
    for (m, n) in [(2, 2), (3, 3), (2, 4), (4, 2)] {
        let problem = Factoring::new(m, n, 1);
        let reduction: ReductionFactoringToILP =
            ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
        let ilp = reduction.target_problem();

        let expected = 3 * m * n + 4 * m + 4 * n + 1;
        assert_eq!(
            ilp.constraints().len(),
            expected,
            "Constraint count mismatch for m={}, n={}",
            m,
            n
        );
    }
}

#[test]
fn test_variable_count_formula() {
    // Verify variable count matches formula: m + n + m*n + (m+n)
    for (m, n) in [(2, 2), (3, 3), (2, 4), (4, 2)] {
        let problem = Factoring::new(m, n, 1);
        let reduction: ReductionFactoringToILP =
            ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
        let ilp = reduction.target_problem();

        let expected = m + n + m * n + (m + n);
        assert_eq!(
            ilp.num_vars(),
            expected,
            "Variable count mismatch for m={}, n={}",
            m,
            n
        );
    }
}

#[test]
fn test_factoring_to_ilp_bf_vs_ilp() {
    let problem = Factoring::new(2, 2, 6);
    let reduction: ReductionFactoringToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
