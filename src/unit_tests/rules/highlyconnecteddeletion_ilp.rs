use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::HighlyConnectedDeletion;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Canonical issue #1023 instance: triangle {0,1,2} with leaf vertex 3
/// attached at vertex 2. Optimum deletes only the leaf edge (2,3).
fn issue_instance() -> HighlyConnectedDeletion<SimpleGraph> {
    HighlyConnectedDeletion::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]))
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_issue_structure() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // 4 singletons + the triangle cluster {0,1,2}: 5 variables in total.
    assert_eq!(ilp.num_vars(), 5);
    assert_eq!(ilp.constraints().len(), 4);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);

    // The induced-edge counts: singletons contribute 0, triangle contributes 3.
    let triangle_coeffs: Vec<f64> = ilp
        .objective()
        .iter()
        .filter(|(_, w)| *w > 0.0)
        .map(|(_, w)| *w)
        .collect();
    assert_eq!(triangle_coeffs, vec![3.0]);

    // Vertex 3 only appears in its own singleton, so its partition constraint
    // is `x_{3} = 1` -- a single-term equality with rhs 1.
    let v3_constraint = &ilp.constraints()[3];
    assert_eq!(v3_constraint.terms().len(), 1);
    assert_eq!(v3_constraint.rhs(), 1);

    // Vertex 0 appears in two clusters (its singleton and the triangle).
    let v0_constraint = &ilp.constraints()[0];
    assert_eq!(v0_constraint.terms().len(), 2);
    assert_eq!(v0_constraint.rhs(), 1);
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_closed_loop() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_bf_vs_ilp() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_extract_solution_decode() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");

    // ILP solution: pick triangle cluster {0,1,2} and singleton {3}.
    // The triangle cluster is the last variable (index 4); singleton {3} is
    // index 3. Build the assignment directly.
    let mut target_solution = vec![0; reduction.target_problem().num_vars()];
    target_solution[3] = 1; // singleton {3}
    target_solution[4] = 1; // triangle {0,1,2}

    let extracted = reduction.extract_solution(&target_solution).unwrap();

    // Edges in input order: (0,1), (0,2), (1,2) all inside the triangle (kept);
    // (2,3) crosses clusters and is deleted.
    assert_eq!(extracted, vec![false, false, false, true]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(1)));
    assert!(source.is_valid_solution(&extracted));
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_rejects_unassigned_vertex() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let target_solution = vec![0; reduction.target_problem().num_vars()];

    assert_eq!(
        reduction
            .extract_solution(&target_solution)
            .unwrap_err()
            .to_string(),
        "vertex 0 has no selected cluster"
    );
}

#[test]
fn test_highlyconnecteddeletion_to_ilp_disconnected_no_cluster() {
    // Two disjoint K3's stitched by a single bridge edge. The bridge is the
    // only "bad" edge: removing it leaves two K3's, both highly connected.
    let source = HighlyConnectedDeletion::new(SimpleGraph::new(
        6,
        vec![
            // Triangle on {0,1,2}.
            (0, 1),
            (0, 2),
            (1, 2),
            // Triangle on {3,4,5}.
            (3, 4),
            (3, 5),
            (4, 5),
            // Bridge edge.
            (2, 3),
        ],
    ));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // No cluster of size >= 3 may straddle the bridge (the only sets {2,3} or
    // any 4+ subsets crossing it fail edge-connectivity). The two triangles
    // are feasible; mixed 4-vertex sets are not.
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
    let large_cluster_count = ilp.objective().iter().filter(|(_, w)| *w > 0.0).count();
    assert_eq!(large_cluster_count, 2);

    assert_bf_vs_ilp(&source, &reduction);
}
