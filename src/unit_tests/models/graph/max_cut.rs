use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
include!("../../jl_helpers.rs");

#[test]
fn test_maxcut_creation() {
    use crate::traits::Problem;

    let problem = MaxCut::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 2, 3],
    );
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_maxcut_unweighted() {
    let problem = MaxCut::<_, i32>::unweighted(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert_eq!(problem.graph().num_edges(), 2);
}

#[test]
fn test_cut_size_function() {
    use crate::topology::SimpleGraph;
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let weights = vec![1, 2, 3];

    // Partition {0} vs {1, 2}
    assert_eq!(cut_size(&graph, &weights, &[false, true, true]), 4); // 1 + 3

    // Partition {0, 1} vs {2}
    assert_eq!(cut_size(&graph, &weights, &[false, false, true]), 5); // 2 + 3

    // All same partition
    assert_eq!(cut_size(&graph, &weights, &[false, false, false]), 0);
}

#[test]
fn test_edge_weight() {
    let problem = MaxCut::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![5, 10]);
    assert_eq!(problem.edge_weight(0, 1), Some(&5));
    assert_eq!(problem.edge_weight(1, 2), Some(&10));
    assert_eq!(problem.edge_weight(0, 2), None);
}

#[test]
fn test_edges() {
    let problem = MaxCut::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 2]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    let problem = MaxCut::<_, i32>::unweighted(SimpleGraph::new(2, vec![(0, 1)]));
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_new() {
    let problem = MaxCut::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![5, 10]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
    assert_eq!(problem.edge_weights(), vec![5, 10]);
}

#[test]
fn test_unweighted() {
    let problem = MaxCut::<_, i32>::unweighted(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.graph().num_edges(), 2);
    assert_eq!(problem.edge_weights(), vec![1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem = MaxCut::<_, i32>::unweighted(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_new_with_separate_weights() {
    let problem = MaxCut::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![7, 3]);
    assert_eq!(problem.edge_weights(), vec![7, 3]);
}

#[test]
fn test_edge_weight_by_index() {
    let problem = MaxCut::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![5, 10]);
    assert_eq!(problem.edge_weight_by_index(0), Some(&5));
    assert_eq!(problem.edge_weight_by_index(1), Some(&10));
    assert_eq!(problem.edge_weight_by_index(2), None);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/maxcut.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let weighted_edges = jl_parse_weighted_edges(&instance["instance"]);
        let edges: Vec<(usize, usize)> = weighted_edges.iter().map(|&(u, v, _)| (u, v)).collect();
        let weights: Vec<i32> = weighted_edges.into_iter().map(|(_, _, w)| w).collect();
        let problem = MaxCut::new(SimpleGraph::new(nv, edges), weights);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_size = eval["size"].as_i64().unwrap() as i32;
            assert!(result.is_valid(), "MaxCut should always be valid");
            assert_eq!(
                result.unwrap(),
                jl_size,
                "MaxCut size mismatch for config {:?}",
                config
            );
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "MaxCut best solutions mismatch");
    }
}

#[test]
fn test_cut_size_method() {
    let problem = MaxCut::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![1, 2, 3]);
    // Partition {0} vs {1, 2}: cuts edges (0,1)=1 and (0,2)=3
    assert_eq!(problem.cut_size(&[0, 1, 1]), 4);
    // All same partition: no edges cut
    assert_eq!(problem.cut_size(&[0, 0, 0]), 0);
}
