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
    let reduction =
        ReduceTo::<RegisterSufficiency>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let layout = SethiRegisterLayout::new(source.num_vars(), source.num_clauses()).unwrap();

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
fn test_ksatisfiability_to_register_sufficiency_rejects_invalid_snapshot_order() {
    let source = repeated_positive_literal();
    let reduction =
        ReduceTo::<RegisterSufficiency>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let layout = SethiRegisterLayout::new(source.num_vars(), source.num_clauses()).unwrap();

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

    let positions = positions_from_order(&order, target.num_vertices());
    assert_eq!(target.evaluate(&positions).unwrap(), Or(false));
    assert!(reduction.extract_solution(&positions).is_err());
}

#[test]
fn test_ksatisfiability_to_register_sufficiency_forward_schedule() {
    let source = repeated_positive_literal();
    let reduction =
        ReduceTo::<RegisterSufficiency>::reduce_to(&source).expect("reduction should succeed");

    let register_schedule = reduction
        .layout
        .as_ref()
        .unwrap()
        .schedule_for_assignment(&[true]);
    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&register_schedule)
            .unwrap(),
        Or(true)
    );

    let extracted = reduction.extract_solution(&register_schedule).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
    assert_eq!(extracted, vec![true]);
}

#[test]
fn test_ksatisfiability_to_register_sufficiency_unsatisfiable_instance() {
    use crate::solvers::BruteForce;

    let source = contradictory_single_variable();
    // Verify the source is indeed unsatisfiable via brute force
    assert!(BruteForce::new().solve(&source).unwrap().is_none());

    // Verify the reduction produces a valid RS instance — we check that
    // the structure is correct (vertex/arc counts match Sethi layout) rather
    // than enumerating all permutations of this 24-vertex RS instance.
    let reduction =
        ReduceTo::<RegisterSufficiency>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let layout = SethiRegisterLayout::new(source.num_vars(), source.num_clauses()).unwrap();
    assert_eq!(target.num_vertices(), layout.total_vertices());
}

#[cfg(feature = "example-db")]
#[test]
fn test_ksatisfiability_to_register_sufficiency_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "ksatisfiability_to_registersufficiency")
        .expect("missing canonical KSatisfiability -> RegisterSufficiency example spec");
    assert_eq!(spec.id, "ksatisfiability_to_registersufficiency");
}

#[test]
fn test_ksatisfiability_to_registersufficiency_closed_loop_boundaries() {
    use crate::solvers::BruteForce;
    for declared in [0, 5] {
        let yes = KSatisfiability::<K3>::new(declared, vec![]);
        let no = KSatisfiability::<K3>::try_new_allow_less(declared, vec![CNFClause::new(vec![])])
            .unwrap();
        for (source, feasible) in [(yes, true), (no, false)] {
            let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source).unwrap();
            let solution = BruteForce::new().solve(reduction.target_problem()).unwrap();
            assert_eq!(solution.is_some(), feasible);
            if let Some(solution) = solution {
                let extracted = reduction.extract_solution(&solution).unwrap();
                assert_eq!(extracted, vec![false; declared]);
                assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
            } else {
                assert!(reduction.extract_solution(&vec![0]).is_err());
            }
        }
    }
}

#[test]
fn test_short_repeated_and_tautological_clauses() {
    for num_vars in 1..=3 {
        let literals: Vec<i64> = (1..=num_vars)
            .flat_map(|i| [i as i64, -(i as i64)])
            .collect();
        for width in 1..=3 {
            for code in 0..literals.len().pow(width) {
                let mut remaining = code;
                let clause: Vec<_> = (0..width)
                    .map(|_| {
                        let lit = literals[remaining % literals.len()];
                        remaining /= literals.len();
                        lit
                    })
                    .collect();
                let source = KSatisfiability::<K3>::try_new_allow_less(
                    num_vars,
                    vec![CNFClause::new(clause)],
                )
                .unwrap();
                let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source).unwrap();
                let layout = reduction.layout.as_ref().unwrap();
                for mask in 0..(1usize << num_vars) {
                    let original: Vec<_> = (0..num_vars).map(|i| mask & (1 << i) != 0).collect();
                    let compact: Vec<_> = reduction
                        .source_variables
                        .iter()
                        .map(|&i| original[i])
                        .collect();
                    let positions = layout.schedule_for_assignment(&compact);
                    assert!(reduction
                        .target_problem()
                        .simulate_registers(&positions)
                        .unwrap()
                        .is_some());
                    let source_value = source.evaluate(&original).unwrap();
                    assert_eq!(
                        reduction.target_problem().evaluate(&positions).unwrap(),
                        source_value
                    );
                    if source_value.0 {
                        let decoded = reduction.extract_solution(&positions).unwrap();
                        assert_eq!(source.evaluate(&decoded).unwrap(), Or(true));
                        for &i in &reduction.source_variables {
                            assert_eq!(decoded[i], original[i]);
                        }
                    } else {
                        assert!(reduction.extract_solution(&positions).is_err());
                    }
                }
            }
        }
    }
}

#[test]
fn test_sparse_original_variables_and_padding_bound() {
    let source =
        KSatisfiability::<K3>::try_new_allow_less(17, vec![CNFClause::new(vec![-17, 2])]).unwrap();
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source).unwrap();
    assert_eq!(reduction.source_variables, vec![1, 16]);
    let layout = reduction.layout.as_ref().unwrap();
    assert_eq!(layout.num_vars, 2);
    let positions = layout.schedule_for_assignment(&[true, false]);
    let decoded = reduction.extract_solution(&positions).unwrap();
    assert_eq!(decoded.len(), 17);
    assert!(decoded[1]);
    assert!(!decoded[16]);
    assert_eq!(decoded.iter().filter(|&&b| b).count(), 1);
    let many = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1]); 5]);
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&many).unwrap();
    let layout = reduction.layout.as_ref().unwrap();
    assert_eq!(layout.b_padding, 0);
    let positions = layout.schedule_for_assignment(&[true]);
    assert_eq!(
        reduction.target_problem().evaluate(&positions).unwrap(),
        Or(true)
    );
}

#[test]
fn test_feasible_snapshot_may_leave_both_literals_uncomputed() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<RegisterSufficiency>::reduce_to(&source).unwrap();
    let layout = reduction.layout.as_ref().unwrap();
    let base = layout.schedule_for_assignment(&[true, false, false]);
    let mut order: Vec<_> = (0..layout.total_vertices()).collect();
    order.sort_by_key(|&v| base[v]);
    let mut delayed = Vec::new();
    for variable in 1..3 {
        delayed.extend((0..2 * layout.num_vars - 2 * variable - 1).map(|i| layout.t(variable, i)));
        delayed.push(layout.x_neg(variable));
    }
    order.retain(|v| !delayed.contains(v));
    let insert = order.iter().position(|&v| v == layout.d()).unwrap() + 1;
    order.splice(insert..insert, delayed);
    let positions = positions_from_order(&order, layout.total_vertices());
    assert_eq!(
        reduction.target_problem().evaluate(&positions).unwrap(),
        Or(true)
    );
    assert!(
        positions[layout.x_pos(1)] > positions[layout.w(2)]
            && positions[layout.x_neg(1)] > positions[layout.w(2)]
    );
    assert_eq!(
        reduction.extract_solution(&positions).unwrap(),
        vec![true, false, false]
    );
}
