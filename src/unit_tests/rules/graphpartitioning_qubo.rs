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
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "GraphPartitioning->QUBO closed loop",
    );
}

#[test]
fn test_graphpartitioning_to_qubo_matrix_matches_issue_example() {
    let source = example_problem();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 6);

    let expected_diagonal = [-48, -47, -46, -46, -47, -48];
    for (index, expected) in expected_diagonal.into_iter().enumerate() {
        assert_eq!(qubo.get(index, index), Some(expected));
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
        assert_eq!(qubo.get(u, v), Some(18), "edge ({u}, {v})");
    }

    let non_edge_pairs = [(0, 3), (0, 4), (0, 5), (1, 4), (1, 5), (2, 5)];
    for &(u, v) in &non_edge_pairs {
        assert_eq!(qubo.get(u, v), Some(20), "non-edge ({u}, {v})");
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

#[test]
fn odd_partition_recovers_infeasibility_from_every_qubo_optimum() {
    use crate::solvers::{BruteForce, SolveOutcome};
    let source = GraphPartitioning::new(SimpleGraph::new(1, vec![]));
    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    let optima = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    assert_eq!(optima.len(), 2);
    for solution in optima {
        assert_eq!(
            reduction
                .recover_result(
                    &source,
                    SolveOutcome::optimal(reduction.target_problem(), solution).unwrap()
                )
                .unwrap(),
            SolveOutcome::Infeasible
        );
    }
}
