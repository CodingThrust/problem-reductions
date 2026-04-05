use super::*;
use crate::models::formula::CNFClause;
use crate::models::misc::TimetableDesign;
#[cfg(feature = "ilp-solver")]
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::variant::K3;

fn satisfiable_instance() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    )
}

fn unsatisfiable_instance() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new_allow_less(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])])
}

#[test]
fn test_ksatisfiability_to_timetabledesign_structure() {
    let source = satisfiable_instance();
    let reduction = ReduceTo::<TimetableDesign>::reduce_to(&source);
    let target = reduction.target_problem();

    assert!(target.num_periods() >= 4);
    assert!(target.num_craftsmen() > 0);
    assert!(target.num_tasks() > 0);
    assert!(
        target
            .requirements()
            .iter()
            .flatten()
            .all(|&requirement| requirement <= 1),
        "the construction should stay in the binary-requirement regime"
    );
}

#[test]
fn test_ksatisfiability_to_timetabledesign_extract_solution_from_constructed_timetable() {
    let source = satisfiable_instance();
    let reduction = ReduceTo::<TimetableDesign>::reduce_to(&source);
    let target_solution =
        construct_timetable_from_assignment(reduction.target_problem(), &[1, 1, 0], &source)
            .expect("a satisfying 3SAT assignment should lift to a timetable witness");

    assert!(reduction.target_problem().evaluate(&target_solution).0);

    let extracted = reduction.extract_solution(&target_solution);
    assert!(source.evaluate(&extracted).0);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_timetabledesign_closed_loop() {
    let source = satisfiable_instance();
    let reduction = ReduceTo::<TimetableDesign>::reduce_to(&source);

    let target_solution = ILPSolver::new()
        .solve_reduced(reduction.target_problem())
        .expect("satisfiable source instance should produce a feasible timetable");

    assert!(reduction.target_problem().evaluate(&target_solution).0);

    let extracted = reduction.extract_solution(&target_solution);
    assert!(source.evaluate(&extracted).0);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_timetabledesign_unsatisfiable() {
    let source = unsatisfiable_instance();
    let reduction = ReduceTo::<TimetableDesign>::reduce_to(&source);

    assert!(
        ILPSolver::new()
            .solve_reduced(reduction.target_problem())
            .is_none(),
        "unsatisfiable 3SAT instance should produce an infeasible timetable"
    );
}
