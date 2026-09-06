use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::topology::Graph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_partitionintocliques_target_bound_rejects_overflow() {
    assert_eq!(target_clique_bound(1, i64::MAX - 3), Ok(i64::MAX));
    for (cliques, edges) in [(2, i64::MAX - 3), (1, i64::MAX - 2), (1, i64::MAX)] {
        assert!(matches!(
            target_clique_bound(cliques, edges),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn test_partitionintocliques_aggregate_applies_gadget_offset() {
    let source = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source).unwrap();
    // K + 2m + 2 = 6, including both directed-edge gadgets and the side cliques.
    for (value, expected) in [
        (Min(None), false),
        (Min(Some(5)), true),
        (Min(Some(6)), true),
        (Min(Some(7)), false),
    ] {
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&reduction, value),
            crate::types::Or(expected),
        );
    }
}

#[test]
fn test_partitionintocliques_to_minimumcoveringbycliques_closed_loop() {
    let source: PartitionIntoCliques<SimpleGraph> = serde_json::from_value(serde_json::json!({
        "graph": {"num_vertices": 0, "edges": []}, "num_cliques": 0
    }))
    .unwrap();
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PartitionIntoCliques -> MinimumCoveringByCliques closed loop",
    );
}

#[test]
fn test_partitionintocliques_to_minimumcoveringbycliques_orlin_example_structure() {
    let source = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let layout = OrlinLayout::new(source.graph());

    assert_eq!(target.graph().num_vertices(), 14);
    assert_eq!(target.graph().num_edges(), 53);

    // Left clique on x_0, x_1, x_2, a_(0,1), a_(1,0)
    assert!(target.graph().has_edge(0, 1));
    assert!(target.graph().has_edge(0, 2));
    assert!(target.graph().has_edge(1, 2));
    assert!(target.graph().has_edge(0, 6));
    assert!(target.graph().has_edge(1, 7));

    // Right clique on y_0, y_1, y_2, b_(0,1), b_(1,0)
    assert!(target.graph().has_edge(3, 4));
    assert!(target.graph().has_edge(3, 5));
    assert!(target.graph().has_edge(4, 5));
    assert!(target.graph().has_edge(3, 8));
    assert!(target.graph().has_edge(4, 9));

    // Matching and gadget cross edges from the issue body
    assert!(target.graph().has_edge(0, 3));
    assert!(target.graph().has_edge(1, 4));
    assert!(target.graph().has_edge(0, 4));
    assert!(target.graph().has_edge(0, 8));
    assert!(target.graph().has_edge(6, 4));
    assert!(target.graph().has_edge(6, 8));

    let target_solution = edge_labels_from_clique_cover(
        target.graph(),
        &[
            vec![layout.x(0), layout.x(1), layout.y(0), layout.y(1)],
            vec![layout.x(2), layout.y(2)],
            vec![layout.x(0), layout.a(0), layout.b(0), layout.y(1)],
            vec![layout.x(1), layout.a(1), layout.b(1), layout.y(0)],
            {
                let mut clique = layout.left_vertices();
                clique.push(layout.z_left());
                clique
            },
            {
                let mut clique = layout.right_vertices();
                clique.push(layout.z_right());
                clique
            },
        ],
    );
    assert_eq!(target.evaluate(&target_solution).unwrap(), Min(Some(6)));
    assert_eq!(
        reduction.extract_solution(&target_solution).unwrap(),
        vec![0, 0, 1]
    );
}

#[test]
fn test_partitionintocliques_to_minimumcoveringbycliques_unsat_extracts_invalid_source() {
    let source = PartitionIntoCliques::new(SimpleGraph::new(2, vec![]), 1);
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let layout = OrlinLayout::new(source.graph());

    let target_solution = edge_labels_from_clique_cover(
        target.graph(),
        &[
            {
                let mut clique = layout.left_vertices();
                clique.push(layout.z_left());
                clique
            },
            {
                let mut clique = layout.right_vertices();
                clique.push(layout.z_right());
                clique
            },
            vec![layout.x(0), layout.y(0)],
            vec![layout.x(1), layout.y(1)],
        ],
    );
    assert_eq!(target.evaluate(&target_solution).unwrap(), Min(Some(4)));

    assert_eq!(
        reduction
            .extract_solution(&target_solution)
            .unwrap_err()
            .to_string(),
        "target cover does not certify the source clique bound"
    );
}

#[test]
fn test_partitionintocliques_native_bounds_and_adjacency_semantics() {
    use crate::rules::AggregateReductionResult;
    for (n, edges) in [
        (0, vec![]),
        (1, vec![(0, 0)]),
        (2, vec![(0, 0)]),
        (3, vec![(0, 1), (1, 0), (0, 0)]),
    ] {
        for bound in [0, 1, n, n + 1, usize::MAX] {
            let source: PartitionIntoCliques<SimpleGraph> =
                serde_json::from_value(serde_json::json!({
                    "graph": {"num_vertices": n, "edges": edges}, "num_cliques": bound
                }))
                .unwrap();
            let reduction =
                ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source).unwrap();
            let target = ReductionResult::target_problem(&reduction);
            let layout = OrlinLayout::new(source.graph());
            let mut cliques: Vec<Vec<usize>> =
                (0..n).map(|i| vec![layout.x(i), layout.y(i)]).collect();
            for (idx, &(i, j)) in layout.directed_pairs.iter().enumerate() {
                cliques.push(vec![layout.x(i), layout.a(idx), layout.b(idx), layout.y(j)]);
            }
            let mut left = layout.left_vertices();
            left.push(layout.z_left());
            cliques.push(left);
            let mut right = layout.right_vertices();
            right.push(layout.z_right());
            cliques.push(right);
            let witness = edge_labels_from_clique_cover(target.graph(), &cliques);
            let value = target.evaluate(&witness).unwrap();
            assert_eq!(
                value,
                Min(Some((n + layout.num_directed_pairs() + 2) as i64))
            );
            assert_eq!(
                AggregateReductionResult::extract_value(&reduction, value).0,
                n <= bound
            );
            if n <= bound {
                let decoded = reduction.extract_solution(&witness).unwrap();
                assert_eq!(decoded, (0..n).collect::<Vec<_>>());
                if bound <= n + 1 {
                    assert!(source.evaluate(&decoded).unwrap().0);
                }
            } else {
                assert!(reduction.extract_solution(&witness).is_err());
            }
            assert!(reduction.extract_solution(&vec![0; witness.len()]).is_err());
            assert!(reduction
                .extract_solution(&vec![0; witness.len() + 1])
                .is_err());
            let q = layout.num_directed_pairs();
            assert_eq!(target.num_vertices(), 2 * n + 2 * q + 4);
            assert_eq!(target.num_edges(), (n + q) * (n + q) + 4 * n + 7 * q + 2);
        }
    }
}

#[test]
fn test_partitionintocliques_layout_dimensions_numeric_domain() {
    // Dimension arithmetic can be checked without allocating the represented graph.
    for (n, expected) in [(0, (4, 2)), (1, (6, 7)), (3, (10, 23))] {
        let layout = OrlinLayout {
            num_source_vertices: n,
            directed_pairs: vec![],
        };
        assert_eq!(layout.dimensions().unwrap(), expected);
    }
    for n in [usize::MAX, usize::MAX / 2, 1usize << (usize::BITS / 2)] {
        let layout = OrlinLayout {
            num_source_vertices: n,
            directed_pairs: vec![],
        };
        assert!(matches!(
            layout.dimensions(),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
    #[cfg(target_pointer_width = "64")]
    {
        // Vertex and edge indices fit usize, but not every cover value fits i64.
        let layout = OrlinLayout {
            num_source_vertices: 3_037_000_500,
            directed_pairs: vec![],
        };
        assert!(matches!(
            layout.dimensions(),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
