use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::topology::Graph;
use crate::traits::Problem;
use crate::types::{Min, Or};

#[test]
fn test_partitionintocliques_to_minimumcoveringbycliques_closed_loop() {
    let source = PartitionIntoCliques::new(SimpleGraph::new(1, vec![]), 1);
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PartitionIntoCliques -> MinimumCoveringByCliques closed loop",
    );
}

#[test]
fn test_partitionintocliques_to_minimumcoveringbycliques_orlin_example_structure() {
    let source = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();
    let layout = OrlinLayout::new(source.graph());

    assert_eq!(target.graph().num_vertices(), 12);
    assert_eq!(target.graph().num_edges(), 41);

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
    assert_eq!(target.evaluate(&target_solution), Min(Some(6)));
    assert_eq!(reduction.extract_solution(&target_solution), vec![0, 0, 1]);
}

#[test]
fn test_partitionintocliques_to_minimumcoveringbycliques_unsat_extracts_invalid_source() {
    let source = PartitionIntoCliques::new(SimpleGraph::new(2, vec![]), 1);
    let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source);
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
    assert_eq!(target.evaluate(&target_solution), Min(Some(4)));

    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(source.evaluate(&extracted), Or(false));
}
