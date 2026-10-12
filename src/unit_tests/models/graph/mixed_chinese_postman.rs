use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_spec_infers_graph_and_default_weights() {
    let problem = MixedChinesePostman::<i64>::try_from(MixedChinesePostmanI64CreateSpec {
        graph: vec![(0, 1)],
        arcs: vec![(1, 0)],
        num_vertices: None,
        arc_weights: None,
        edge_weights: None,
    })
    .unwrap();
    assert_eq!(problem.num_vertices(), 2);
    assert_eq!(problem.arc_weights(), &[1]);
    assert_eq!(problem.edge_weights(), &[1]);
}
use crate::solvers::BruteForce;
use crate::topology::MultiMixedGraph;
use crate::traits::Problem;
use crate::types::Min;

fn sample_instance() -> MixedChinesePostman<i64> {
    MixedChinesePostman::new(
        MultiMixedGraph::new(
            5,
            vec![(0, 1), (1, 2), (2, 3), (3, 0)],
            vec![(0, 2), (1, 3), (0, 4), (4, 2)],
        ),
        vec![2, 3, 1, 4],
        vec![2, 3, 1, 2],
    )
}

fn disconnected_instance() -> MixedChinesePostman<i64> {
    MixedChinesePostman::new(
        MultiMixedGraph::new(
            6,
            vec![(0, 1), (1, 0), (2, 3)],
            vec![(0, 2), (1, 3), (3, 4), (4, 5), (5, 2)],
        ),
        vec![1, 1, 1],
        vec![1, 1, 5, 5, 5],
    )
}

#[test]
fn test_mixed_chinese_postman_creation_and_accessors() {
    let problem = sample_instance();

    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.dimensions(), vec![2, 2, 2, 2]);
    assert_eq!(problem.arc_weights(), &[2, 3, 1, 4]);
    assert_eq!(problem.edge_weights(), &[2, 3, 1, 2]);
}

#[test]
fn test_mixed_chinese_postman_evaluate_optimal() {
    let problem = sample_instance();

    // Reverse (0,2) and (1,3), keep (0,4) and (4,2) forward.
    assert_eq!(
        problem.evaluate(&vec![true, true, false, false]).unwrap(),
        Min(Some(21))
    );
}

#[test]
fn test_mixed_chinese_postman_evaluate_connected_instance() {
    let problem = disconnected_instance();

    // The available graph is strongly connected, so valid orientations
    // should return Some(cost).
    let val = problem
        .evaluate(&vec![false, false, false, false, false])
        .unwrap();
    assert!(val.0.is_some());
}

#[test]
fn test_mixed_chinese_postman_single_edge_walk() {
    // V={0,1}, A=∅, E={{0,1}}, weight=1.
    // Walk 0→1→0: base cost 1, needs to balance so total cost is 2.
    let problem = MixedChinesePostman::new(
        MultiMixedGraph::new(2, vec![], vec![(0, 1)]),
        vec![],
        vec![1],
    );

    assert_eq!(problem.evaluate(&vec![false]).unwrap(), Min(Some(2)));
    assert_eq!(problem.evaluate(&vec![true]).unwrap(), Min(Some(2)));

    let solver = BruteForce::new();
    assert!(solver.solve(&problem).unwrap().is_some());
}

#[test]
fn test_mixed_chinese_postman_rejects_disconnected_graph() {
    // Two disconnected components {0,1} and {2,3}: no closed walk can cover all edges.
    let problem = MixedChinesePostman::new(
        MultiMixedGraph::new(4, vec![], vec![(0, 1), (2, 3)]),
        vec![],
        vec![1, 1],
    );

    assert_eq!(problem.evaluate(&vec![false, false]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&vec![false, true]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&vec![true, false]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&vec![true, true]).unwrap(), Min(None));
}

#[test]
fn test_mixed_chinese_postman_rejects_wrong_config_length() {
    let problem = sample_instance();

    assert!(matches!(
        problem.evaluate(&vec![]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![true, true, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![true, true, false, false, true]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_mixed_chinese_postman_solver_finds_optimal() {
    let problem = sample_instance();
    let solver = BruteForce::new();

    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("expected an optimal orientation");
    assert!(problem.is_valid_solution(&solution).unwrap());
    // The optimal cost should be 21.
    assert_eq!(problem.evaluate(&solution).unwrap(), Min(Some(21)));
}

#[test]
fn test_mixed_chinese_postman_serialization_roundtrip() {
    let problem = sample_instance();

    let json = serde_json::to_string(&problem).unwrap();
    let restored: MixedChinesePostman<i64> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.num_vertices(), 5);
    assert_eq!(restored.num_arcs(), 4);
    assert_eq!(restored.num_edges(), 4);
    assert_eq!(restored.arc_weights(), &[2, 3, 1, 4]);
    assert_eq!(restored.edge_weights(), &[2, 3, 1, 2]);
}

#[test]
fn test_mixed_chinese_postman_problem_name() {
    assert_eq!(
        <MixedChinesePostman<i64> as Problem>::NAME,
        "MixedChinesePostman"
    );
}

#[test]
fn test_mixed_chinese_postman_ignores_isolated_vertices() {
    let problem = MixedChinesePostman::new(
        MultiMixedGraph::new(
            8,
            vec![(5, 3), (1, 4), (0, 1), (2, 4), (0, 5)],
            vec![(4, 2), (0, 4), (0, 2), (1, 3)],
        ),
        vec![4, 5, 1, 12, 9],
        vec![6, 1, 13, 7],
    );

    assert_eq!(
        problem.evaluate(&vec![false, true, true, true]).unwrap(),
        Min(Some(69))
    );
}

#[test]
fn test_mixed_chinese_postman_deserialization_rejects_invalid_weights() {
    let valid = serde_json::to_value(sample_instance()).unwrap();
    let restored: MixedChinesePostman<i64> = serde_json::from_value(valid.clone()).unwrap();
    assert_eq!(serde_json::to_value(&restored).unwrap(), valid);

    let cases = [
        (
            "arc_weights",
            serde_json::json!([2, 3, 1]),
            "arc_weights has length 3, expected 4",
        ),
        (
            "edge_weights",
            serde_json::json!([2, 3, 1, 2, 7]),
            "edge_weights has length 5, expected 4",
        ),
        (
            "arc_weights",
            serde_json::json!([2, 3, -1, 4]),
            "arc weight at index 2 must be nonnegative",
        ),
        (
            "edge_weights",
            serde_json::json!([2, -3, 1, 2]),
            "edge weight at index 1 must be nonnegative",
        ),
    ];
    for (field, value, expected) in cases {
        let mut json = valid.clone();
        json[field] = value;
        let error = serde_json::from_value::<MixedChinesePostman<i64>>(json)
            .unwrap_err()
            .to_string();
        assert_eq!(error, format!("problem construction failed: {expected}"));
    }
}

#[test]
fn mixed_chinese_postman_loop_has_the_same_value_after_serde() {
    let problem = MixedChinesePostman::new(
        MultiMixedGraph::new(1, vec![(0, 0)], vec![(0, 0)]),
        vec![2i64],
        vec![3i64],
    );
    let restored: MixedChinesePostman<i64> =
        serde_json::from_value(serde_json::to_value(&problem).unwrap()).unwrap();
    assert_eq!(
        problem.evaluate(&vec![false]).unwrap(),
        crate::types::Min(Some(5))
    );
    assert_eq!(
        restored.evaluate(&vec![false]).unwrap(),
        problem.evaluate(&vec![false]).unwrap()
    );
}

#[test]
fn mixed_postman_create_preserves_parallel_loops_for_both_weight_variants() {
    let weighted = MixedChinesePostman::<i64>::try_from(MixedChinesePostmanI64CreateSpec {
        graph: vec![(0, 0), (0, 0)],
        arcs: vec![(0, 0), (0, 0)],
        num_vertices: Some(1),
        arc_weights: Some(vec![2, 3]),
        edge_weights: Some(vec![5, 7]),
    })
    .unwrap();
    let restored: MixedChinesePostman<i64> =
        serde_json::from_value(serde_json::to_value(&weighted).unwrap()).unwrap();
    assert_eq!(restored.graph(), weighted.graph());
    assert_eq!(restored.arc_weights(), &[2, 3]);
    assert_eq!(restored.edge_weights(), &[5, 7]);
    assert_eq!(
        restored.evaluate(&vec![false, true]).unwrap(),
        Min(Some(17))
    );

    let unweighted = MixedChinesePostman::<One>::try_from(MixedChinesePostmanOneCreateSpec {
        graph: vec![(0, 0), (0, 0)],
        arcs: vec![(0, 0), (0, 0)],
        num_vertices: Some(1),
    })
    .unwrap();
    assert_eq!(unweighted.graph(), weighted.graph());
    assert_eq!(
        unweighted.evaluate(&vec![false, true]).unwrap(),
        Min(Some(4))
    );
}

#[test]
fn mixed_postman_create_rejects_missing_topology_and_invalid_endpoints() {
    for (graph, arcs, num_vertices, expected) in [
        (vec![], vec![(0, 0)], None, "num_vertices is required"),
        (vec![(0, 0)], vec![], Some(1), "arcs must be non-empty"),
        (vec![(0, 1)], vec![(0, 0)], Some(1), "too small"),
        (
            vec![(0, 0)],
            vec![(0, 1)],
            Some(1),
            "arc 0 endpoint is out of range",
        ),
    ] {
        let error = MixedChinesePostman::<i64>::try_from(MixedChinesePostmanI64CreateSpec {
            graph: graph.clone(),
            arcs: arcs.clone(),
            num_vertices,
            arc_weights: None,
            edge_weights: None,
        })
        .unwrap_err();
        assert!(matches!(
            error,
            crate::registry::ConstructionError::Conversion(_)
        ));
        assert!(error.to_string().contains(expected), "{error}");
        let error = MixedChinesePostman::<One>::try_from(MixedChinesePostmanOneCreateSpec {
            graph,
            arcs,
            num_vertices,
        })
        .unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
    }
}
