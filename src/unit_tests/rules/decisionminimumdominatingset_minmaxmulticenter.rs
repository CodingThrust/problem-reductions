use super::*;
use crate::models::decision::Decision;
use crate::models::graph::{MinMaxMulticenter, MinimumDominatingSet};
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{One, Or};

fn decision_mds(
    num_vertices: usize,
    edges: &[(usize, usize)],
    k: i32,
) -> Decision<MinimumDominatingSet<SimpleGraph, One>> {
    Decision::new(
        MinimumDominatingSet::new(
            SimpleGraph::new(num_vertices, edges.to_vec()),
            vec![One; num_vertices],
        ),
        k,
    )
}

#[test]
fn test_decisionminimumdominatingset_to_minmaxmulticenter_structure() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        2,
    );
    let reduction = ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(
        target.graph().num_vertices(),
        source.inner().graph().num_vertices()
    );
    assert_eq!(target.graph().edges(), source.inner().graph().edges());
    assert_eq!(target.vertex_weights(), vec![One; 6].as_slice());
    assert_eq!(target.edge_lengths(), vec![One; 7].as_slice());
    assert_eq!(target.k(), 2);
}

#[test]
fn test_decisionminimumdominatingset_to_minmaxmulticenter_closed_loop() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        2,
    );
    let reduction = ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    let target_solutions = BruteForce::new().find_all_witnesses(target);
    assert!(
        !target_solutions.is_empty(),
        "target should have feasible K-center placements"
    );

    for target_solution in target_solutions {
        let extracted = reduction.extract_solution(&target_solution);
        assert_eq!(extracted, target_solution);
        assert_eq!(source.evaluate(&extracted), Or(true));
    }
}

#[test]
fn test_decisionminimumdominatingset_to_minmaxmulticenter_no_witness_when_bound_too_small() {
    let source = decision_mds(4, &[(0, 1), (2, 3)], 1);
    let reduction = ReduceTo::<MinMaxMulticenter<SimpleGraph, One>>::reduce_to(&source);

    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}
