use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::graph::Kernel;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_kernel_structure() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -2, 1])]);
    let reduction = ReduceTo::<Kernel>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 7);
    assert_eq!(target.num_arcs(), 10);

    for arc in [
        (0, 1),
        (1, 0),
        (2, 3),
        (3, 2),
        (4, 5),
        (5, 6),
        (6, 4),
        (4, 0),
        (5, 3),
        (6, 0),
    ] {
        assert!(target.graph().has_arc(arc.0, arc.1));
    }
}

#[test]
fn test_ksatisfiability_to_kernel_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<Kernel>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "3SAT -> Kernel closed loop",
    );
}

#[test]
fn test_ksatisfiability_to_kernel_unsatisfiable_instance_has_no_kernel() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction = ReduceTo::<Kernel>::reduce_to(&source);

    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[test]
fn test_ksatisfiability_to_kernel_extract_solution_reads_variable_gadgets() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -2, 1])]);
    let reduction = ReduceTo::<Kernel>::reduce_to(&source);

    assert_eq!(
        reduction.extract_solution(&[1, 0, 0, 1, 0, 0, 0]),
        vec![1, 0]
    );
}
