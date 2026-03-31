use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

fn issue_instance() -> MinimumWeightDecoding {
    MinimumWeightDecoding::new(
        vec![
            vec![true, false, true, true],
            vec![false, true, true, false],
            vec![true, true, false, true],
        ],
        vec![true, true, false],
    )
}

fn small_instance() -> MinimumWeightDecoding {
    // 2×3 matrix, s = [1, 0]
    // H = [[1,1,0],[0,1,1]], s = [true, false]
    // x=[1,0,0]: row0=1 mod2=1=s[0] ✓, row1=0 mod2=0=s[1] ✓ → weight 1
    MinimumWeightDecoding::new(
        vec![vec![true, true, false], vec![false, true, true]],
        vec![true, false],
    )
}

fn infeasible_instance() -> MinimumWeightDecoding {
    // H = [[1,1],[1,1]], s = [true, false]
    // For any x, row0 and row1 have identical dot products → s[0] ≠ s[1] means infeasible
    MinimumWeightDecoding::new(vec![vec![true, true], vec![true, true]], vec![true, false])
}

#[test]
fn test_minimumweightdecoding_to_ilp_structure() {
    let problem = issue_instance();
    let reduction: ReductionMinimumWeightDecodingToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 4 cols + 3 rows = 7 variables
    assert_eq!(ilp.num_vars, 7);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // Objective: 4 terms (one per x_j)
    assert_eq!(ilp.objective.len(), 4);

    // Constraints: 3 equality + 4 binary bounds = 7
    assert_eq!(ilp.constraints.len(), 7);
}

#[test]
fn test_minimumweightdecoding_to_ilp_closed_loop() {
    let problem = issue_instance();
    let bf = BruteForce::new();
    let bf_witness = bf
        .find_witness(&problem)
        .expect("issue instance has optimal");
    let bf_value = problem.evaluate(&bf_witness);
    assert_eq!(bf_value, Min(Some(1)));

    let reduction: ReductionMinimumWeightDecodingToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    let ilp_value = problem.evaluate(&extracted);
    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_minimumweightdecoding_to_ilp_small_closed_loop() {
    let problem = small_instance();
    let bf = BruteForce::new();
    let bf_witness = bf
        .find_witness(&problem)
        .expect("small instance has optimal");
    let bf_value = problem.evaluate(&bf_witness);
    assert_eq!(bf_value, Min(Some(1)));

    let reduction: ReductionMinimumWeightDecodingToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), bf_value);
}

#[test]
fn test_minimumweightdecoding_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionMinimumWeightDecodingToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible instance should produce infeasible ILP"
    );
}

#[test]
fn test_minimumweightdecoding_to_ilp_bf_vs_ilp() {
    let problem = issue_instance();
    let reduction: ReductionMinimumWeightDecodingToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_minimumweightdecoding_to_ilp_extract_solution() {
    let problem = issue_instance();
    let reduction: ReductionMinimumWeightDecodingToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Manually construct a valid target solution: x=[0,0,1,0], k=[0,0,0]
    // (k_i values are the integer slack from mod-2)
    // Row 0: H[0][2]=1 → sum=1, s=1 → 1-1=0 → k_0=0 ✓
    // Row 1: H[1][2]=1 → sum=1, s=1 → 1-1=0 → k_1=0 ✓
    // Row 2: H[2][2]=0 → sum=0, s=0 → 0-0=0 → k_2=0 ✓
    let target_solution = vec![0, 0, 1, 0, 0, 0, 0];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted.len(), 4);
    assert_eq!(extracted, vec![0, 0, 1, 0]);
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}
