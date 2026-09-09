use super::*;
use crate::models::algebraic::ILP;
use crate::models::formula::CNFClause;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;
use std::collections::BTreeSet;

fn forward_witness(source: &KSatisfiability<K3>, values: &[bool]) -> Vec<usize> {
    assert!(source.evaluate(&values.to_vec()).unwrap().0);
    let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(source).unwrap();
    let target = reduction.target_problem();
    let a = reduction.source_variables.len();
    let mut order = Vec::new();
    let mut done = vec![false; target.num_vertices()];
    fn append(order: &mut Vec<usize>, done: &mut [bool], vertex: usize) {
        assert!(!done[vertex]);
        order.push(vertex);
        done[vertex] = true;
    }
    // Consume every use of the first leaf before its opposite overwrites S.
    for (compact, &original) in reduction.source_variables.iter().enumerate() {
        let (first, second) = if values[original] {
            (s_pos_idx(compact), s_neg_idx(a, compact))
        } else {
            (s_neg_idx(a, compact), s_pos_idx(compact))
        };
        append(&mut order, &mut done, first);
        for &(consumer, dependency) in target.arcs() {
            if dependency == first {
                append(&mut order, &mut done, consumer);
            }
        }
        append(&mut order, &mut done, second);
    }
    // Every true occurrence has r available; consume it before writing rbar.
    for clause in 0..source.num_clauses() {
        for slot in 0..3 {
            if done[r_idx(a, clause, slot)] {
                append(&mut order, &mut done, p_idx(a, clause, slot));
                append(&mut order, &mut done, rbar_idx(a, clause, slot));
            }
        }
    }
    // A true position breaks the clause's cyclic register dependency.
    for (clause, literals) in source.clauses().iter().enumerate() {
        let start = literals
            .literals
            .iter()
            .position(|&literal| values[literal.unsigned_abs() as usize - 1] == (literal > 0))
            .unwrap();
        for offset in 0..3 {
            let slot = (start + offset) % 3;
            if !done[r_idx(a, clause, slot)] {
                append(&mut order, &mut done, r_idx(a, clause, slot));
                append(&mut order, &mut done, p_idx(a, clause, slot));
            }
            append(&mut order, &mut done, q_idx(a, clause, slot));
        }
    }
    assert!(done.iter().all(|&value| value));
    let mut positions = vec![0; order.len()];
    for (position, vertex) in order.into_iter().enumerate() {
        positions[vertex] = position;
    }
    positions
}

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
    let realization = forward_witness(&source, &[true, false]);
    assert!(reduction.target_problem().evaluate(&realization).unwrap().0);

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

    assert_eq!(
        ILPSolver::new().solve(fra_to_ilp.target_problem()),
        Err(crate::solvers::ILPSolveError::Infeasible),
        "an unsatisfiable source formula should yield an infeasible FRA instance"
    );
}

#[test]
fn native_empty_clause_is_infeasible_and_empty_conjunction_is_feasible() {
    for num_vars in [0, 3] {
        let source =
            KSatisfiability::<K3>::try_new_allow_less(num_vars, vec![CNFClause::new(vec![])])
                .unwrap();
        let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source).unwrap();
        assert_eq!(reduction.target_problem().num_vertices(), 3);
        for config in [
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ] {
            assert!(!reduction.target_problem().evaluate(&config).unwrap().0);
            assert!(reduction.extract_solution(&config).is_err());
        }
        let source = KSatisfiability::<K3>::new(num_vars, vec![]);
        let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source).unwrap();
        assert_eq!(reduction.target_problem().num_vertices(), 0);
        assert_eq!(
            reduction.extract_solution(&vec![]).unwrap(),
            vec![false; num_vars]
        );
    }
}

#[test]
fn short_repeated_and_mixed_clauses_have_complete_forward_witnesses() {
    for formula in [
        vec![vec![5]],
        vec![vec![-5]],
        vec![vec![5, 2]],
        vec![vec![-5, 2]],
        vec![vec![5, 5, 5]],
        vec![vec![5, -5, 5]],
        vec![vec![5, 1, -3], vec![-1, 5, 2], vec![-3, 2, -5]],
    ] {
        let source = KSatisfiability::<K3>::try_new_allow_less(
            6,
            formula.into_iter().map(CNFClause::new).collect(),
        )
        .unwrap();
        let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source).unwrap();
        let a = reduction.source_variables.len();
        assert_eq!(
            reduction.target_problem().num_same_register_pairs(),
            a + 3 * source.num_clauses()
        );
        assert_eq!(
            reduction.target_problem().num_arcs(),
            15 * source.num_clauses()
        );
        for mask in 0..64 {
            let values: Vec<_> = (0..6).map(|bit| mask & (1 << bit) != 0).collect();
            if source.evaluate(&values).unwrap().0 {
                let config = forward_witness(&source, &values);
                assert!(reduction.target_problem().evaluate(&config).unwrap().0);
                let extracted = reduction.extract_solution(&config).unwrap();
                assert!(source.evaluate(&extracted).unwrap().0);
                for original in 0..6 {
                    assert_eq!(
                        extracted[original],
                        reduction.source_variables.contains(&original) && values[original]
                    );
                }
            }
        }
    }
}

#[test]
fn short_clause_padding_matches_its_equivalent_three_literal_formula() {
    let short = KSatisfiability::<K3>::try_new_allow_less(
        3,
        vec![CNFClause::new(vec![3]), CNFClause::new(vec![-3])],
    )
    .unwrap();
    let padded = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![3, 3, 3]),
            CNFClause::new(vec![-3, -3, -3]),
        ],
    );
    let a = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&short).unwrap();
    let b = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&padded).unwrap();
    assert_eq!(a.target_problem().arcs(), b.target_problem().arcs());
    assert_eq!(
        a.target_problem().assignment(),
        b.target_problem().assignment()
    );
    assert_eq!(
        a.target_problem().num_vertices(),
        b.target_problem().num_vertices()
    );
}

#[test]
fn invalid_realizations_are_rejected() {
    let source = issue_example();
    let reduction = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source).unwrap();
    let n = reduction.target_problem().num_vertices();
    for config in [vec![], vec![n; n], vec![0; n], (0..n).collect()] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn sparse_high_variable_indices_only_change_the_inverse_map() {
    let largest = usize::try_from(i64::MAX).unwrap_or(usize::MAX);
    let literal = i64::try_from(largest).unwrap();
    let source = KSatisfiability::<K3>::new(largest, vec![CNFClause::new(vec![literal; 3])]);
    let small = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1; 3])]);
    let a = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&source).unwrap();
    let b = ReduceTo::<FeasibleRegisterAssignment>::reduce_to(&small).unwrap();
    assert_eq!(a.target_problem().arcs(), b.target_problem().arcs());
    assert_eq!(
        a.target_problem().assignment(),
        b.target_problem().assignment()
    );
    assert_eq!(a.target_problem().num_vertices(), 14);
    // The enormous source witness is deliberately not materialized.
}
