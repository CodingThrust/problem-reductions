use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;

#[test]
fn test_lcs_to_ilp_issue_example() {
    // From issue #110: s1 = "ABAC", s2 = "BACA"
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A'],
    ]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 6 match pairs as described in the issue
    assert_eq!(ilp.num_vars, 6);

    // Solve ILP
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");

    // Extract solution back to LCS config
    let extracted = reduction.extract_solution(&ilp_solution);

    // Verify the solution is valid and optimal (length 3)
    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_to_ilp_closed_loop() {
    // Compare brute force LCS with ILP-based solution
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A'],
    ]);

    // Brute force optimal
    let bf = BruteForce::new();
    let bf_solution = bf.find_best(&problem).expect("should find a solution");
    let bf_metric = problem.evaluate(&bf_solution);
    assert!(bf_metric.is_valid());
    let bf_value = bf_metric.unwrap();

    // ILP optimal
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_metric = problem.evaluate(&extracted);
    assert!(ilp_metric.is_valid());
    let ilp_value = ilp_metric.unwrap();

    // Both should give the same optimal value
    assert_eq!(
        bf_value, ilp_value,
        "BF optimal {} != ILP optimal {}",
        bf_value, ilp_value
    );
}

#[test]
fn test_lcs_to_ilp_identical_strings() {
    // LCS of identical strings = the string itself
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'A', b'B', b'C']]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_to_ilp_no_common_chars() {
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'C', b'D']]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // No match pairs → 0 variables
    assert_eq!(ilp.num_vars, 0);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 0);
}

#[test]
fn test_lcs_to_ilp_single_char_alphabet() {
    // All same chars → LCS = min length
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'A', b'A'], vec![b'A', b'A']]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 2);
}

#[test]
fn test_lcs_to_ilp_asymmetric_lengths() {
    // s1 = "AB", s2 = "AABB"
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'A', b'A', b'B', b'B']]);

    // BF optimal
    let bf = BruteForce::new();
    let bf_solution = bf.find_best(&problem).unwrap();
    let bf_value = problem.evaluate(&bf_solution).unwrap();

    // ILP optimal
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).unwrap();
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted).unwrap();

    assert_eq!(bf_value, ilp_value);
    assert_eq!(ilp_value, 2); // "AB" is subseq of both
}

#[test]
fn test_lcs_to_ilp_constraint_structure() {
    // Verify basic ILP structure for a small example
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'B', b'A']]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Match pairs: (0,1)=A, (1,0)=B → 2 variables
    assert_eq!(ilp.num_vars, 2);

    // Constraints:
    // - s1 pos 0: m_{0,1} <= 1 (1 constraint)
    // - s1 pos 1: m_{1,0} <= 1 (1 constraint)
    // - s2 pos 0: m_{1,0} <= 1 (1 constraint)
    // - s2 pos 1: m_{0,1} <= 1 (1 constraint)
    // - Crossing: j1=0 < j1'=1 and j2=1 > j2'=0 → m_{0,1} + m_{1,0} <= 1 (1 constraint)
    // Total: 5
    assert_eq!(ilp.constraints.len(), 5);

    // Optimal: can only pick one of the two → LCS length 1
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).unwrap();
    let extracted = reduction.extract_solution(&ilp_solution);
    let metric = problem.evaluate(&extracted);
    assert_eq!(metric.unwrap(), 1);
}
