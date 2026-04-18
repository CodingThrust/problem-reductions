use super::*;
use crate::models::algebraic::BMF;
use crate::models::graph::BicliqueCover;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, Solver};
use crate::topology::BipartiteGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_bicliquecover_to_bmf_structure() {
    // Graph with edges (0,0) and (1,1), k=2 → BMF target is 2x2 identity, rank 2.
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]), 2);
    let reduction: ReductionBicliqueCoverToBMF = ReduceTo::<BMF>::reduce_to(&problem);
    let target = reduction.target_problem();
    assert_eq!(target.rows(), 2);
    assert_eq!(target.cols(), 2);
    assert_eq!(target.rank(), 2);
    assert_eq!(target.matrix(), &[vec![true, false], vec![false, true]][..]);
}

#[test]
fn test_bicliquecover_to_bmf_closed_loop_full_biclique() {
    // K_{2,2} at rank 1 — single biclique covers all 4 edges.
    let problem = BicliqueCover::new(
        BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        1,
    );
    let reduction: ReductionBicliqueCoverToBMF = ReduceTo::<BMF>::reduce_to(&problem);
    let target = reduction.target_problem();

    let bf_source = BruteForce::new().solve(&problem);
    let target_witness = BruteForce::new()
        .find_witness(target)
        .expect("target must be feasible");
    let extracted = reduction.extract_solution(&target_witness);
    assert_eq!(problem.evaluate(&extracted), bf_source);
}

#[test]
fn test_bicliquecover_to_bmf_closed_loop_identity_rank2() {
    // Identity-biadjacency at rank 2 — exact factorization needs two singleton bicliques.
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]), 2);
    let reduction: ReductionBicliqueCoverToBMF = ReduceTo::<BMF>::reduce_to(&problem);
    let target = reduction.target_problem();

    let bf_source = BruteForce::new().solve(&problem);
    let target_witness = BruteForce::new()
        .find_witness(target)
        .expect("target must be feasible");
    let extracted = reduction.extract_solution(&target_witness);
    assert_eq!(problem.evaluate(&extracted), bf_source);
}

#[test]
fn test_bicliquecover_to_bmf_insufficient_rank() {
    // Identity biadjacency at rank 1 — infeasible for both problems.
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]), 1);
    let reduction: ReductionBicliqueCoverToBMF = ReduceTo::<BMF>::reduce_to(&problem);
    let target = reduction.target_problem();
    assert_eq!(BruteForce::new().solve(&problem), Min(None));
    assert_eq!(BruteForce::new().solve(target), Min(None));
}

#[test]
fn test_config_roundtrip_bc_bmf() {
    // The transpose helpers must invert each other.
    use crate::rules::bmf_bicliquecover::{config_bc_to_bmf, config_bmf_to_bc};
    let (m, n, k) = (2, 3, 2);
    let bc = vec![1, 0, 0, 1, 1, 0, 1, 1, 0, 1]; // length (m+n)*k = 10
    let bmf = config_bc_to_bmf(&bc, m, n, k);
    assert_eq!(bmf.len(), m * k + k * n);
    let bc_back = config_bmf_to_bc(&bmf, m, n, k);
    assert_eq!(bc_back, bc);
}
