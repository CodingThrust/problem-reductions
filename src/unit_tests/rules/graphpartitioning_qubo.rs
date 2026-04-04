use super::*;
use crate::models::algebraic::QUBO;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::topology::SimpleGraph;

fn example_problem() -> GraphPartitioning<SimpleGraph> {
    GraphPartitioning::new(SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
            (3, 5),
            (4, 5),
        ],
    ))
}

#[test]
fn test_graphpartitioning_to_qubo_closed_loop() {
    let source = example_problem();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "GraphPartitioning->QUBO closed loop",
    );
}

#[test]
fn test_graphpartitioning_to_qubo_matrix_matches_issue_example() {
    let source = example_problem();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 6);

    let expected_diagonal = [-48.0, -47.0, -46.0, -46.0, -47.0, -48.0];
    for (index, expected) in expected_diagonal.into_iter().enumerate() {
        assert_eq!(qubo.get(index, index), Some(&expected));
    }

    let edge_pairs = [
        (0, 1),
        (0, 2),
        (1, 2),
        (1, 3),
        (2, 3),
        (2, 4),
        (3, 4),
        (3, 5),
        (4, 5),
    ];
    for &(u, v) in &edge_pairs {
        assert_eq!(qubo.get(u, v), Some(&18.0), "edge ({u}, {v})");
    }

    let non_edge_pairs = [(0, 3), (0, 4), (0, 5), (1, 4), (1, 5), (2, 5)];
    for &(u, v) in &non_edge_pairs {
        assert_eq!(qubo.get(u, v), Some(&20.0), "non-edge ({u}, {v})");
    }
}

#[cfg(feature = "example-db")]
#[test]
fn test_graphpartitioning_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "graphpartitioning_to_qubo")
        .expect("missing canonical GraphPartitioning -> QUBO example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "GraphPartitioning");
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.target.instance["num_vars"], 6);
    assert!(!example.solutions.is_empty());
}
