use super::*;
use crate::models::algebraic::BMF;
use crate::models::graph::BicliqueCover;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::BipartiteGraph;
use crate::traits::Problem;

#[test]
fn test_bicliquecover_to_bmf_structure() {
    // Graph with edges (0,0) and (1,1), k=2 → BMF target is 2x2 identity, rank 2.
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]), 2);
    let reduction: ReductionBicliqueCoverToBMF =
        ReduceTo::<BMF>::reduce_to(&problem).expect("reduction should succeed");
    let target = reduction.target_problem();
    assert_eq!(target.rows(), 2);
    assert_eq!(target.cols(), 2);
    assert_eq!(target.rank(), 2);
    assert_eq!(target.matrix(), &[vec![true, false], vec![false, true]][..]);
}

#[test]
fn test_bicliquecover_to_bmf_overhead_matches_target_shape() {
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (1, 2)]), 2);
    let reduction: ReductionBicliqueCoverToBMF =
        ReduceTo::<BMF>::reduce_to(&problem).expect("reduction should succeed");
    let target = reduction.target_problem();

    let entry = inventory::iter::<crate::rules::ReductionEntry>()
        .find(|entry| entry.source_name == "BicliqueCover" && entry.target_name == "BMF")
        .expect("BicliqueCover -> BMF reduction should be registered");
    let source_size = problem.parameters();
    let predicted = entry
        .parameter_contract()
        .unwrap()
        .transform()
        .unwrap()
        .evaluate(&source_size)
        .unwrap();

    assert_eq!(
        predicted.get("rows"),
        Some(target.rows().try_into().unwrap())
    );
    assert_eq!(
        predicted.get("cols"),
        Some(target.cols().try_into().unwrap())
    );
    assert_eq!(
        predicted.get("rank"),
        Some(target.rank().try_into().unwrap())
    );
}

#[test]
fn test_bicliquecover_to_bmf_closed_loop_full_biclique() {
    // K_{2,2} at rank 1 — single biclique covers all 4 edges.
    let problem = BicliqueCover::new(
        BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        1,
    );
    let reduction: ReductionBicliqueCoverToBMF =
        ReduceTo::<BMF>::reduce_to(&problem).expect("reduction should succeed");
    let target = reduction.target_problem();

    let bf_source_solution = BruteForce::new().solve(&problem).unwrap().unwrap();

    let bf_source = problem.evaluate(&bf_source_solution).unwrap();
    let target_witness = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("target must be feasible");
    let extracted = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), bf_source);
}

#[test]
fn test_bicliquecover_to_bmf_closed_loop_identity_rank2() {
    // Identity-biadjacency at rank 2 — exact factorization needs two singleton bicliques.
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]), 2);
    let reduction: ReductionBicliqueCoverToBMF =
        ReduceTo::<BMF>::reduce_to(&problem).expect("reduction should succeed");
    let target = reduction.target_problem();

    let bf_source_solution = BruteForce::new().solve(&problem).unwrap().unwrap();

    let bf_source = problem.evaluate(&bf_source_solution).unwrap();
    let target_witness = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("target must be feasible");
    let extracted = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), bf_source);
}

#[test]
fn test_bicliquecover_to_bmf_insufficient_rank() {
    // Identity biadjacency at rank 1 — infeasible for both problems.
    let problem = BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]), 1);
    let reduction: ReductionBicliqueCoverToBMF =
        ReduceTo::<BMF>::reduce_to(&problem).expect("reduction should succeed");
    let target = reduction.target_problem();
    assert!(BruteForce::new().solve(&problem).unwrap().is_none());
    assert!(BruteForce::new().solve(target).unwrap().is_none());
}

#[test]
fn test_config_roundtrip_bc_bmf() {
    // The transpose helpers must invert each other.
    use crate::rules::bmf_bicliquecover::{config_bc_to_bmf, config_bmf_to_bc};
    let (m, n, k) = (2, 3, 2);
    let bc = vec![
        vec![true, false, true, true, false],
        vec![false, true, false, true, true],
    ];
    let bmf = config_bc_to_bmf(&bc, m, n, k);
    assert_eq!(bmf.0.len(), m);
    assert_eq!(bmf.1.len(), k);
    let bc_back = config_bmf_to_bc(&bmf, m, n, k);
    assert_eq!(bc_back, bc);
}
