use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::EulerianPath;
use crate::solvers::ILPSolver;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Or;

/// Canonical issue #1025 instance: V = {0,1,2}, A = [(0,1),(0,1),(1,2),(2,0)].
/// A witness exists: ordering (a_0, a_2, a_3, a_1) traces 0->1->2->0->1.
fn issue_instance() -> EulerianPath {
    EulerianPath::new(DirectedGraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]))
}

#[test]
fn test_eulerianpath_to_ilp_issue_structure() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // m = 4 arcs. Compatible pairs: head(a) = tail(b), a != b:
    //   a_0 = (0,1) -> head=1; arcs starting at 1: only a_2 -> (a_0, a_2)
    //   a_1 = (0,1) -> head=1; arcs starting at 1: only a_2 -> (a_1, a_2)
    //   a_2 = (1,2) -> head=2; arcs starting at 2: only a_3 -> (a_2, a_3)
    //   a_3 = (2,0) -> head=0; arcs starting at 0: a_0, a_1 -> (a_3, a_0), (a_3, a_1)
    // So p = 5. num_vars = 5 + 3*4 = 17.
    assert_eq!(ilp.num_vars, 17);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(
        ilp.objective.is_empty(),
        "Pure feasibility ILP should have no objective"
    );

    // Constraints count:
    //   2*m = 8 (predecessor + successor equalities)
    //   3*m = 12 (s_a <= 1, e_a <= 1, u_a <= m-1)
    //   2*p = 10 (y_{a,b} <= 1, order consistency)
    //   2 (unique start + unique end)
    // Total = 8 + 12 + 10 + 2 = 32.
    assert_eq!(ilp.constraints.len(), 32);
}

#[test]
fn test_eulerianpath_to_ilp_empty_instance() {
    // m = 0: empty ILP, vacuously feasible.
    let source = EulerianPath::new(DirectedGraph::empty(3));
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);

    let solution = ILPSolver::new()
        .solve(ilp)
        .expect("Empty ILP should be feasible");
    let extracted = reduction.extract_solution(&solution).unwrap();
    assert_eq!(extracted.len(), 0);
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_eulerianpath_to_ilp_closed_loop() {
    // Solve the ILP on the canonical instance and verify the extracted ordering
    // is a valid directed Eulerian trail in the source.
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible for a YES instance");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(extracted.len(), source.num_arcs());
    assert!(
        source.is_valid_solution(&extracted),
        "Extracted ordering must be a valid Eulerian trail, got {:?}",
        extracted
    );
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_eulerianpath_to_ilp_infeasible_no_instance() {
    // Two arcs sharing the same tail but disconnected heads. This breaks the
    // degree-balance criterion: vertex 0 has out-degree 2 / in-degree 0, so
    // no Eulerian trail exists.
    let source = EulerianPath::new(DirectedGraph::new(3, vec![(0, 1), (0, 2)]));
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    // The ILP must report infeasibility for a NO instance.
    let solution = ILPSolver::new().solve(reduction.target_problem());
    assert!(
        solution.is_err(),
        "ILP must be infeasible for a degree-unbalanced NO instance, got {:?}",
        solution
    );
}

#[test]
fn test_eulerianpath_to_ilp_closed_circuit_with_loop() {
    // Loop + closed trail: arcs (0,0), (0,1), (1,0).
    // Trail (0,0) -> (0,1) -> (1,0) is a valid closed Eulerian trail.
    let source = EulerianPath::new(DirectedGraph::new(2, vec![(0, 0), (0, 1), (1, 0)]));
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible for a closed Eulerian circuit");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(
        source.is_valid_solution(&extracted),
        "Extracted ordering must be a valid Eulerian trail, got {:?}",
        extracted
    );
}
