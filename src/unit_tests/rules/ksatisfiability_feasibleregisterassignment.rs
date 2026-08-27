use super::*;
use crate::models::algebraic::ILP;
use crate::models::formula::CNFClause;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;
use std::collections::BTreeSet;

fn issue_example() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    )
}

#[test]
fn test_ksatisfiability_to_feasible_register_assignment_structure() {
    let source = issue_example();
    let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 30);
    assert_eq!(target.num_arcs(), 30);
    assert_eq!(target.num_registers(), 21);

    assert_eq!(
        target.assignment()[s_pos_idx(0)],
        target.assignment()[s_neg_idx(3, 0)]
    );
    assert_eq!(
        target.assignment()[s_pos_idx(1)],
        target.assignment()[s_neg_idx(3, 1)]
    );
    assert_eq!(
        target.assignment()[s_pos_idx(2)],
        target.assignment()[s_neg_idx(3, 2)]
    );
    assert_eq!(
        target.assignment()[r_idx(3, 0, 0)],
        target.assignment()[rbar_idx(3, 0, 0)]
    );
    assert_eq!(
        target.assignment()[r_idx(3, 1, 2)],
        target.assignment()[rbar_idx(3, 1, 2)]
    );

    let arc_set: BTreeSet<_> = target.arcs().iter().copied().collect();
    assert!(arc_set.contains(&(q_idx(3, 0, 0), p_idx(3, 0, 0))));
    assert!(arc_set.contains(&(p_idx(3, 0, 0), r_idx(3, 0, 0))));
    assert!(arc_set.contains(&(q_idx(3, 0, 0), rbar_idx(3, 0, 1))));
    assert!(arc_set.contains(&(q_idx(3, 0, 1), rbar_idx(3, 0, 2))));
    assert!(arc_set.contains(&(q_idx(3, 0, 2), rbar_idx(3, 0, 0))));
    assert!(arc_set.contains(&(r_idx(3, 0, 0), s_pos_idx(0))));
    assert!(arc_set.contains(&(rbar_idx(3, 0, 0), s_neg_idx(3, 0))));
    assert!(arc_set.contains(&(r_idx(3, 0, 1), s_neg_idx(3, 1))));
    assert!(arc_set.contains(&(rbar_idx(3, 0, 1), s_pos_idx(1))));
    assert!(arc_set.contains(&(r_idx(3, 1, 2), s_neg_idx(3, 2))));
    assert!(arc_set.contains(&(rbar_idx(3, 1, 2), s_pos_idx(2))));
}

#[test]
fn test_ksatisfiability_to_feasible_register_assignment_extract_solution() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -2, 1])]);
    let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source)
        .expect("reduction should succeed");
    let mut realization: Vec<usize> = (0..reduction.target_problem().num_vertices()).collect();
    realization.swap(s_pos_idx(1), s_neg_idx(2, 1));

    let extracted = reduction.extract_solution(&realization).unwrap();

    assert_eq!(extracted, vec![true, false]);
}

#[test]
fn test_ksatisfiability_to_feasible_register_assignment_closed_loop_via_ilp() {
    let source = issue_example();
    let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source)
        .expect("reduction should succeed");
    let fra_to_ilp = ReduceTo::<ILP<i64>>::reduce_to(reduction.target_problem())
        .expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(fra_to_ilp.target_problem())
        .expect("satisfiable FRA gadget should reduce to a feasible ILP");
    let fra_solution = fra_to_ilp.extract_solution(&ilp_solution).unwrap();
    assert_eq!(
        reduction.target_problem().evaluate(&fra_solution).unwrap(),
        Or(true)
    );

    let extracted = reduction.extract_solution(&fra_solution).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_ksatisfiability_to_feasible_register_assignment_unsatisfiable_instance() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source)
        .expect("reduction should succeed");
    let fra_to_ilp = ReduceTo::<ILP<i64>>::reduce_to(reduction.target_problem())
        .expect("reduction should succeed");

    assert!(
        ILPSolver::new().solve(fra_to_ilp.target_problem()).is_err(),
        "an unsatisfiable source formula should yield an infeasible FRA instance"
    );
}
