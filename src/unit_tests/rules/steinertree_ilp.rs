use super::*;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;

// Construct the proof's path witness for a source tree, using only source edges.
fn lift(source: &SteinerTree<SimpleGraph, i64>, chosen: &[bool]) -> Vec<i64> {
    let n = source.num_vertices();
    let m = source.num_edges();
    let root = source.terminals()[0];
    let edges = source.graph().edges();
    let mut witness = vec![0; tree_ilp_sizes(n, m, source.terminals().len()).unwrap().0];
    let mut adj = vec![vec![]; n];
    for (e, &(u, v)) in edges.iter().enumerate() {
        if chosen[e] {
            witness[e] = 1;
            witness[m + u] = 1;
            witness[m + v] = 1;
            adj[u].push((v, e, 0));
            adj[v].push((u, e, 1));
        }
    }
    let mut parent = vec![None; n];
    let mut stack = vec![root];
    parent[root] = Some((root, 0, 0));
    while let Some(u) = stack.pop() {
        for &(v, e, dir) in &adj[u] {
            if parent[v].is_none() {
                parent[v] = Some((u, e, dir));
                stack.push(v);
            }
        }
    }
    for (commodity, sink) in (0..n).filter(|&v| v != root).enumerate() {
        if witness[m + sink] == 0 {
            continue;
        }
        let mut v = sink;
        while v != root {
            let (u, e, dir) = parent[v].unwrap();
            witness[m + n + commodity * 2 * m + 2 * e + dir] = 1;
            v = u;
        }
    }
    witness
}

#[test]
fn test_steinertree_to_ilp_closed_loop() {
    for (n, edges, weights, terminals, optimum) in [
        (
            3,
            vec![(0, 1), (1, 2), (0, 2)],
            vec![-1, -1, -1],
            vec![0, 1],
            -2,
        ),
        (
            3,
            vec![(0, 1), (1, 2), (0, 2)],
            vec![0, 0, 0],
            vec![0, 1],
            0,
        ),
        (4, vec![(0, 1), (2, 3)], vec![1, -10], vec![0, 1], 1),
        (3, vec![(0, 1), (1, 2)], vec![1, -5], vec![0, 1], -4),
        (3, vec![(0, 1), (1, 2)], vec![2, 3], vec![2, 0], 5),
    ] {
        let source = SteinerTree::new(SimpleGraph::new(n, edges), weights, terminals);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let witness = ILPSolver::new().solve(reduction.target_problem()).unwrap();
        let decoded = reduction.extract_solution(&witness).unwrap();
        assert_eq!(source.evaluate(&decoded).unwrap(), Min(Some(optimum)));
        assert_eq!(
            reduction.target_problem().evaluate(&witness).unwrap().value,
            Some(optimum)
        );
        crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
    }
    let source = SteinerTree::new(SimpleGraph::new(3, vec![(0, 1)]), vec![-1], vec![0, 2]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(matches!(
        ILPSolver::new().solve(reduction.target_problem()),
        Err(crate::solvers::ILPSolveError::Infeasible)
    ));
}

#[test]
fn test_steiner_all_source_trees_lift_and_preserve_objective() {
    let source = SteinerTree::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![-2, 0, 3, -1, 2, 0],
        vec![2, 0],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert_eq!((target.num_vars(), target.constraints().len()), (46, 63));
    for mask in 0..64 {
        let selected: Vec<_> = (0..6).map(|e| mask & (1 << e) != 0).collect();
        if source.is_valid_solution(&selected) {
            let witness = lift(&source, &selected);
            assert_eq!(
                target.evaluate(&witness).unwrap().value,
                source.evaluate(&selected).unwrap().0
            );
            assert_eq!(reduction.extract_solution(&witness).unwrap(), selected);
        }
    }
}

#[test]
fn test_steiner_every_small_raw_target_and_malformed_witness() {
    let source = SteinerTree::new(SimpleGraph::new(2, vec![(0, 1)]), vec![-3], vec![1, 0]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let mut feasible_count = 0;
    for mask in 0..(1 << target.num_vars()) {
        let witness: Vec<_> = (0..target.num_vars()).map(|v| (mask >> v) & 1).collect();
        if target.evaluate(&witness).unwrap().is_valid() {
            feasible_count += 1;
            let decoded = reduction.extract_solution(&witness).unwrap();
            assert_eq!(source.evaluate(&decoded).unwrap(), Min(Some(-3)));
        } else {
            assert!(reduction.extract_solution(&witness).is_err());
        }
    }
    for bad in [
        vec![],
        vec![1; target.num_vars() + 1],
        vec![2; target.num_vars()],
    ] {
        assert!(reduction.extract_solution(&bad).is_err());
    }
    assert_eq!(feasible_count, 1);
}

#[test]
fn test_steiner_count_boundaries() {
    assert_eq!(tree_ilp_sizes(5, 7, 3).unwrap(), (68, 94));
    assert_eq!(tree_ilp_sizes(2, 0, 2).unwrap(), (2, 5));
    for (n, m, k) in [
        (0, 0, 0),
        (2, usize::MAX, 2),
        (usize::MAX, 1, 2),
        (usize::MAX, 0, 2),
        (2, 0, usize::MAX),
    ] {
        assert!(matches!(
            tree_ilp_sizes(n, m, k),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
