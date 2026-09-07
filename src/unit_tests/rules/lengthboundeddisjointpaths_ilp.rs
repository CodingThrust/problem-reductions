use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_closed_loop() {
    // Diamond graph: 4 vertices, s=0, t=3, K=2
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_bf_vs_ilp() {
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_triangle_subgraphs() {
    let edges = [(0, 1), (1, 2), (0, 2)];
    for mask in 0..8 {
        for bound in 1..=2 {
            let graph = SimpleGraph::new(
                3,
                edges
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &edge)| (mask & (1 << i) != 0).then_some(edge))
                    .collect(),
            );
            let source = LengthBoundedDisjointPaths::new(graph, 0, 2, bound);
            let expected = i64::from(mask & 4 != 0) + i64::from(bound == 2 && mask & 3 == 3);
            let reference = BruteForce::new().solve(&source).unwrap().unwrap();
            assert_eq!(source.evaluate(&reference).unwrap(), Max(Some(expected)));
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
            let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
            let extracted = reduction.extract_solution(&target_solution).unwrap();
            assert_eq!(source.evaluate(&extracted).unwrap(), Max(Some(expected)));
        }
    }
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_preserves_edge_order() {
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(2, 0), (3, 1), (2, 1), (1, 0)]),
        0,
        2,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    // Reverse orientations of edge 0 (0->2) and edges 3,2 (0->1->2).
    let mut target_solution = vec![0; 18];
    for index in [1, 13, 15, 16, 17] {
        target_solution[index] = 1;
    }
    assert!(reduction
        .target_problem()
        .is_feasible(&target_solution)
        .unwrap());
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(
        extracted,
        vec![
            vec![true, false, false, false],
            vec![false, false, true, true]
        ]
    );
    assert_eq!(source.evaluate(&extracted).unwrap(), Max(Some(2)));
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_extracts_path_from_circulation() {
    for cycle in [
        vec![(0, 2), (2, 3), (3, 0)],
        vec![(1, 2), (2, 3), (3, 1)],
        vec![(2, 3), (3, 4), (4, 2)],
    ] {
        let mut edges = vec![(0, 1)];
        edges.extend(cycle);
        let source = LengthBoundedDisjointPaths::new(SimpleGraph::new(5, edges), 0, 1, 4);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let target_solution = vec![1, 0, 1, 0, 1, 0, 1, 0, 1];
        assert!(reduction
            .target_problem()
            .is_feasible(&target_solution)
            .unwrap());
        let extracted = reduction.extract_solution(&target_solution).unwrap();
        assert_eq!(extracted, vec![vec![true, false, false, false]]);
        assert_eq!(source.evaluate(&extracted).unwrap(), Max(Some(1)));
    }
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_rejects_invalid_target_solutions() {
    let source = LengthBoundedDisjointPaths::new(SimpleGraph::new(2, vec![(0, 1)]), 0, 1, 1);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    for solution in [vec![], vec![2, 0, 1], vec![0, 0, 1], vec![1, 0, 0]] {
        assert!(reduction.extract_solution(&solution).is_err());
    }
}
