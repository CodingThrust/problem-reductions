use crate::models::decision::Decision;
use crate::models::graph::{MinimumDominatingSet, MinimumSumMulticenter};
use crate::rules::{ReduceTo, ReductionResult};
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
fn test_decisionminimumdominatingset_to_minimumsummulticenter_structure() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        2,
    );
    let reduction = ReduceTo::<MinimumSumMulticenter<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(
        target.graph().num_vertices(),
        source.inner().graph().num_vertices()
    );
    assert_eq!(target.graph().edges(), source.inner().graph().edges());
    assert_eq!(target.vertex_weights(), vec![1i32; 6].as_slice());
    assert_eq!(target.edge_lengths(), vec![1i32; 7].as_slice());
    assert_eq!(target.k(), 2);
}

#[test]
fn test_decisionminimumdominatingset_to_minimumsummulticenter_closed_loop_yes_instance() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        2,
    );
    let reduction = ReduceTo::<MinimumSumMulticenter<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let target_solutions = BruteForce::new().find_all_witnesses(target);
    assert!(
        !target_solutions.is_empty(),
        "target should have optimal K-center placements"
    );

    for target_solution in target_solutions {
        assert_eq!(target.evaluate(&target_solution).unwrap(), 4);
        let extracted = reduction.extract_solution(&target_solution);
        assert_eq!(extracted, target_solution);
        assert_eq!(source.evaluate(&extracted), Or(true));
    }
}

#[test]
fn test_decisionminimumdominatingset_to_minimumsummulticenter_closed_loop_no_instance() {
    let source = decision_mds(
        6,
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
        1,
    );
    let reduction = ReduceTo::<MinimumSumMulticenter<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let target_solutions = BruteForce::new().find_all_witnesses(target);
    assert!(
        !target_solutions.is_empty(),
        "target should still have optimal K-center placements"
    );

    let threshold = source.inner().graph().num_vertices() as i32 - source.k() as i32;
    for target_solution in target_solutions {
        let target_value = target.evaluate(&target_solution).unwrap();
        assert_eq!(target_value, 6);
        assert!(target_value > threshold);

        let extracted = reduction.extract_solution(&target_solution);
        assert_eq!(extracted, target_solution);
        assert_eq!(source.evaluate(&extracted), Or(false));
    }
}
