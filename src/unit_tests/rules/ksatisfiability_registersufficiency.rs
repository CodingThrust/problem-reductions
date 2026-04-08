use super::*;
use crate::models::formula::CNFClause;
use crate::models::misc::RegisterSufficiency;
use crate::traits::Problem;
use crate::types::Or;
use crate::variant::K3;
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

fn repeated_positive_literal() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])])
}

fn contradictory_single_variable() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    )
}

fn positions_from_order(order: &[usize], total_vertices: usize) -> Vec<usize> {
    assert_eq!(order.len(), total_vertices);
    let mut positions = vec![usize::MAX; total_vertices];
    for (position, &vertex) in order.iter().enumerate() {
        positions[vertex] = position;
    }
    assert!(positions.iter().all(|&position| position != usize::MAX));
    positions
}

#[test]
fn test_ksatisfiability_to_register_sufficiency_structure_issue_example() {
    let source = issue_example();
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source);
    let target = reduction.target_problem();
    let layout = SethiRegisterLayout::new(source.num_vars(), source.num_clauses());

    assert_eq!(target.num_vertices(), 70);
    assert_eq!(target.num_arcs(), 152);
    assert_eq!(target.bound(), 23);

    let arc_set: BTreeSet<_> = target.arcs().iter().copied().collect();
    assert!(arc_set.contains(&(layout.initial(), layout.a(0))));
    assert!(arc_set.contains(&(layout.initial(), layout.bnode(3))));
    assert!(arc_set.contains(&(layout.c(0), layout.w(2))));
    assert!(arc_set.contains(&(layout.c(0), layout.z(2))));
    assert!(arc_set.contains(&(layout.x_pos(0), layout.f(0, 0))));
    assert!(arc_set.contains(&(layout.x_neg(1), layout.f(0, 1))));
    assert!(arc_set.contains(&(layout.x_pos(2), layout.f(0, 2))));
    assert!(arc_set.contains(&(layout.x_neg(0), layout.f(0, 1))));
    assert!(arc_set.contains(&(layout.x_neg(0), layout.f(0, 2))));
    assert!(arc_set.contains(&(layout.x_pos(1), layout.f(0, 2))));
}

#[test]
fn test_ksatisfiability_to_register_sufficiency_extract_solution_uses_w_snapshot_and_x_pos_sign() {
    let source = repeated_positive_literal();
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source);
    let target = reduction.target_problem();
    let layout = SethiRegisterLayout::new(source.num_vars(), source.num_clauses());

    let mut order = Vec::with_capacity(target.num_vertices());
    order.push(layout.z(0));
    order.push(layout.x_pos(0));
    order.push(layout.w(0));
    order.push(layout.x_neg(0));
    for vertex in 0..target.num_vertices() {
        if !matches!(
            vertex,
            v if v == layout.z(0)
                || v == layout.x_pos(0)
                || v == layout.w(0)
                || v == layout.x_neg(0)
        ) {
            order.push(vertex);
        }
    }

    let extracted =
        reduction.extract_solution(&positions_from_order(&order, target.num_vertices()));
    assert_eq!(extracted, vec![1]);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_register_sufficiency_closed_loop_via_ilp() {
    use crate::models::algebraic::ILP;
    use crate::solvers::ILPSolver;

    let source = repeated_positive_literal();
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source);
    let to_ilp = ReduceTo::<ILP<i32>>::reduce_to(reduction.target_problem());

    let ilp_solution = ILPSolver::new()
        .solve(to_ilp.target_problem())
        .expect("satisfiable source formula should yield a feasible register schedule");
    let register_schedule = to_ilp.extract_solution(&ilp_solution);
    assert_eq!(
        reduction.target_problem().evaluate(&register_schedule),
        Or(true)
    );

    let extracted = reduction.extract_solution(&register_schedule);
    assert_eq!(source.evaluate(&extracted), Or(true));
    assert_eq!(extracted, vec![1]);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_register_sufficiency_unsatisfiable_instance() {
    use crate::models::algebraic::ILP;
    use crate::solvers::ILPSolver;

    let source = contradictory_single_variable();
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source);
    let to_ilp = ReduceTo::<ILP<i32>>::reduce_to(reduction.target_problem());

    assert!(
        ILPSolver::new().solve(to_ilp.target_problem()).is_none(),
        "unsatisfiable source formula should yield an infeasible register-sufficiency instance"
    );
}

#[cfg(all(feature = "example-db", feature = "ilp-solver"))]
#[test]
fn test_ksatisfiability_to_register_sufficiency_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "ksatisfiability_to_registersufficiency")
        .expect("missing canonical KSatisfiability -> RegisterSufficiency example spec");
    assert_eq!(spec.id, "ksatisfiability_to_registersufficiency");
}
