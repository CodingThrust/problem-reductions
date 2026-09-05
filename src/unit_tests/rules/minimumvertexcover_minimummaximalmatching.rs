use crate::models::graph::{MinimumMaximalMatching, MinimumVertexCover};
use crate::rules::{ReductionGraph, ReductionMode};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Min, One};

fn graph_from_mask(n: usize, mask: usize) -> SimpleGraph {
    let mut edges = Vec::new();
    let mut bit = 0usize;
    for u in 0..n {
        for v in (u + 1)..n {
            if (mask >> bit) & 1 == 1 {
                edges.push((u, v));
            }
            bit += 1;
        }
    }
    SimpleGraph::new(n, edges)
}

#[test]
fn test_minimumvertexcover_to_minimummaximalmatching_c5_gap() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let mvc = MinimumVertexCover::new(graph.clone(), vec![One; 5]);
    let mmm = MinimumMaximalMatching::new(graph);
    let solver = BruteForce::new();

    assert_eq!(
        mvc.evaluate(&solver.solve(&mvc).unwrap().unwrap()).unwrap(),
        Min(Some(3))
    );
    assert_eq!(
        mmm.evaluate(&solver.solve(&mmm).unwrap().unwrap()).unwrap(),
        Min(Some(2))
    );
}

#[test]
fn test_minimumvertexcover_to_minimummaximalmatching_forward_bound_on_small_graphs() {
    let solver = BruteForce::new();

    for n in 0usize..=5 {
        let num_possible_edges = n * (n.saturating_sub(1)) / 2;
        for mask in 0usize..(1usize << num_possible_edges) {
            let graph = graph_from_mask(n, mask);
            let mvc = MinimumVertexCover::new(graph.clone(), vec![One; n]);
            let mmm = MinimumMaximalMatching::new(graph);
            let mvc_value_solution = solver.solve(&mvc).unwrap().unwrap();
            let mvc_value = mvc.evaluate(&mvc_value_solution).unwrap();
            let mmm_value_solution = solver.solve(&mmm).unwrap().unwrap();
            let mmm_value = mmm.evaluate(&mmm_value_solution).unwrap();

            let Min(Some(mvc_size)) = mvc_value else {
                panic!("MinimumVertexCover should always have an optimal solution");
            };
            let Min(Some(mmm_size)) = mmm_value else {
                panic!("MinimumMaximalMatching should always have an optimal solution");
            };
            assert!(
                mmm_size <= mvc_size,
                "expected mmm(G) <= mvc(G) for n={n}, mask={mask:#b}, got {mmm_size} > {mvc_size}",
            );
        }
    }
}

#[test]
fn test_minimumvertexcover_to_minimummaximalmatching_has_no_runtime_modes() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name("MinimumVertexCover", "MinimumMaximalMatching",));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "MinimumMaximalMatching",
        ReductionMode::Witness,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "MinimumMaximalMatching",
        ReductionMode::Aggregate,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "MinimumMaximalMatching",
        ReductionMode::Turing,
    ));
}
