use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianPathBetweenTwoVertices, LongestPath};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::SimpleGraph;
use crate::traits::EvaluationError::InvalidConfiguration;
use crate::types::One;
use crate::types::OptimizationValue;

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_closed_loop() {
    // Graph with a known Hamiltonian 0-4 path: 0-1-2-3-4 plus extra edges
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 3), (1, 4)]),
        0,
        4,
    );
    let result = ReduceTo::<Decision<LongestPath<SimpleGraph, One>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = result.target_problem();

    assert_eq!(target.inner().num_vertices(), 5);
    assert_eq!(target.inner().num_edges(), 6);
    assert_eq!(target.inner().source_vertex(), 0);
    assert_eq!(target.inner().target_vertex(), 4);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath closed loop",
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_path_graph() {
    // Simple path graph: 0-1-2-3 with s=0, t=3 (trivially has a Hamiltonian path)
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        0,
        3,
    );
    let result = ReduceTo::<Decision<LongestPath<SimpleGraph, One>>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath path graph",
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_no_hamiltonian_path() {
    // Star graph K_{1,4}: vertex 0 connected to 1,2,3,4.
    // No Hamiltonian path from 1 to 2 exists (vertices 3,4 are leaves
    // connected only to 0, so no path can visit all without revisiting 0).
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]),
        1,
        2,
    );
    let result = ReduceTo::<Decision<LongestPath<SimpleGraph, One>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let solver = BruteForce::new();
    assert!(solver.solve(result.target_problem()).unwrap().is_none());
    let target_best = solver
        .solve(result.target_problem().inner())
        .unwrap()
        .expect("LongestPath should have some valid path");

    // The best path has fewer than n-1 = 4 edges (it's not Hamiltonian)
    let selected_edges: usize = target_best.iter().filter(|&&selected| selected).count();
    assert!(
        selected_edges < 4,
        "Best path should have fewer than n-1 edges since no Hamiltonian s-t path exists"
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_complete_graph() {
    // Complete graph K4 with s=0, t=3: many Hamiltonian 0-3 paths exist
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        0,
        3,
    );
    let result = ReduceTo::<Decision<LongestPath<SimpleGraph, One>>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath complete K4",
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_triangle() {
    // Triangle: 0-1-2-0, with s=0, t=2
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        0,
        2,
    );
    let result = ReduceTo::<Decision<LongestPath<SimpleGraph, One>>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = result.target_problem();

    assert_eq!(target.inner().num_vertices(), 3);
    assert_eq!(target.inner().num_edges(), 3);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath triangle",
    );
}

#[test]
fn test_hamiltonian_path_extraction_for_all_small_graphs_and_endpoints() {
    use crate::Problem;
    for n in 2..=4 {
        let possible: Vec<_> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        for graph_mask in 0usize..(1 << possible.len()) {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| ((graph_mask >> i) & 1 == 1).then_some(e))
                .collect();
            for start in 0..n {
                for end in 0..n {
                    if start == end {
                        continue;
                    }
                    let source = HamiltonianPathBetweenTwoVertices::new(
                        SimpleGraph::new(n, edges.clone()),
                        start,
                        end,
                    );
                    let reduction =
                        ReduceTo::<Decision<LongestPath<SimpleGraph, One>>>::reduce_to(&source)
                            .unwrap();
                    let target = crate::rules::ReductionResult::target_problem(&reduction);
                    for mask in 0usize..(1 << edges.len()) {
                        let config: Vec<_> =
                            (0..edges.len()).map(|i| (mask >> i) & 1 == 1).collect();
                        let value = target.inner().evaluate(&config).unwrap();
                        let expected = value.0 == Some(n as i64 - 1);
                        assert_eq!(
                            crate::types::Or(OptimizationValue::meets_bound(
                                &(value),
                                crate::rules::ReductionResult::target_problem(&reduction).bound()
                            ))
                            .0,
                            expected
                        );
                        if expected {
                            let order = reduction
                                .recover_result(
                                    &source,
                                    SolveOutcome::optimal(
                                        reduction.target_problem(),
                                        config.clone(),
                                    )
                                    .unwrap(),
                                )
                                .map(|result| {
                                    result.into_solution().expect(
                                        "qualifying target result must recover a source solution",
                                    )
                                })
                                .unwrap();
                            assert!(source.evaluate(&order).unwrap().0);
                        }
                    }
                    assert!(matches!(
                        ReductionResult::target_problem(&reduction)
                            .inner()
                            .evaluate(&vec![false; edges.len() + 1]),
                        Err(InvalidConfiguration(_))
                    ));
                }
            }
        }
    }
}
