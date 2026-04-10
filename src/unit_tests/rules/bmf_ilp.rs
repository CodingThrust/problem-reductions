use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReduceTo, ReductionResult};

#[test]
fn test_bmf_to_ilp_structure() {
    // 2x2 identity matrix, rank 1
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 1);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // b: 2*1=2, c: 1*2=2, p: 2*1*2=4, w: 2*2=4, e: 2*2=4 => 16
    assert_eq!(ilp.num_vars, 16);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_bmf_to_ilp_closed_loop() {
    // 2x2 identity, rank 2 — exact factorization exists.
    // Use ILP solver on target (fast) + brute force on source (tiny 2x2).
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_bmf_to_ilp_bf_vs_ilp() {
    let problem = BMF::new(vec![vec![true, true], vec![true, false]], 1);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_bmf_to_ilp_trivial() {
    // 1x1 matrix, rank 1
    let problem = BMF::new(vec![vec![true]], 1);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // b: 1, c: 1, p: 1, w: 1, e: 1 => 5
    assert_eq!(ilp.num_vars, 5);
}
