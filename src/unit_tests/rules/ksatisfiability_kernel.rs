use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::graph::Kernel;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_kernel_structure() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -2, 1])]);
    let reduction = ReduceTo::<Kernel>::reduce_to(&source).expect("reduction should succeed");
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
    let reduction = ReduceTo::<Kernel>::reduce_to(&source).expect("reduction should succeed");

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
    let reduction = ReduceTo::<Kernel>::reduce_to(&source).expect("reduction should succeed");

    assert!(BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .is_none());
}

#[test]
fn test_ksatisfiability_to_kernel_extract_solution_reads_variable_gadgets() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -2, 1])]);
    let reduction = ReduceTo::<Kernel>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, false, true, false, false, false])
            .unwrap(),
        vec![true, false]
    );
}

#[test]
fn test_ksatisfiability_to_kernel_native_clause_domain() {
    use crate::traits::Problem;
    let mut clauses = vec![vec![]];
    for len in 1..=3 {
        for mask in 0..(1 << len) {
            clauses.push(
                (0..len)
                    .map(|j| if mask & (1 << j) == 0 { 1 } else { -1 })
                    .collect(),
            );
        }
    }
    let mut formulas = vec![vec![]];
    for first in &clauses {
        formulas.push(vec![first.clone()]);
        for second in &clauses {
            formulas.push(vec![first.clone(), second.clone()]);
        }
    }
    for formula in formulas {
        let source = KSatisfiability::<K3>::try_new_allow_less(
            1,
            formula.iter().cloned().map(CNFClause::new).collect(),
        )
        .unwrap();
        let reduction = ReduceTo::<Kernel>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let active = usize::from(formula.iter().any(|clause| !clause.is_empty()));
        let occurrences: usize = formula.iter().map(Vec::len).sum();
        assert_eq!(target.num_vertices(), 2 * active + 3 * formula.len());
        assert_eq!(
            target.num_arcs(),
            2 * active + 3 * formula.len() + occurrences
        );
        let mut witnessed = [false; 2];
        for mask in 0..(1usize << target.num_vertices()) {
            let config = (0..target.num_vertices())
                .map(|v| mask & (1 << v) != 0)
                .collect();
            if target.evaluate(&config).unwrap().0 {
                let decoded = reduction.extract_solution(&config).unwrap();
                assert!(source.evaluate(&decoded).unwrap().0);
                witnessed[usize::from(decoded[0])] = true;
            }
        }
        for value in [false, true] {
            let satisfiable = source.evaluate(&vec![value]).unwrap().0;
            if active == 1 {
                assert_eq!(witnessed[usize::from(value)], satisfiable);
            } else {
                assert_eq!(witnessed[0], satisfiable);
                assert!(!witnessed[1]);
            }
        }
    }
}

#[test]
fn test_ksatisfiability_to_kernel_sparse_inverse() {
    use crate::traits::Problem;
    for literals in [vec![5], vec![-5], vec![5, -2], vec![5, -5, 2]] {
        let source =
            KSatisfiability::<K3>::try_new_allow_less(6, vec![CNFClause::new(literals.clone())])
                .unwrap();
        let reduction = ReduceTo::<Kernel>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let mut found = false;
        for mask in 0..(1usize << target.num_vertices()) {
            let config = (0..target.num_vertices())
                .map(|v| mask & (1 << v) != 0)
                .collect();
            if target.evaluate(&config).unwrap().0 {
                found = true;
                let decoded = reduction.extract_solution(&config).unwrap();
                assert_eq!(decoded.len(), 6);
                assert!(source.evaluate(&decoded).unwrap().0);
                for (variable, value) in decoded.iter().enumerate() {
                    if !literals
                        .iter()
                        .any(|l| l.unsigned_abs() as usize == variable + 1)
                    {
                        assert!(!value);
                    }
                }
            }
        }
        assert!(found);
    }
}

#[test]
fn test_ksatisfiability_to_kernel_rejects_non_kernel() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<Kernel>::reduce_to(&source).unwrap();
    for config in [vec![], vec![false; 5], vec![true; 5], vec![false; 6]] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn test_ksatisfiability_to_kernel_huge_sparse_construction() {
    let largest = usize::try_from(i64::MAX).unwrap_or(usize::MAX);
    let literal = i64::try_from(largest).unwrap();
    let source = KSatisfiability::<K3>::new(largest, vec![CNFClause::new(vec![literal; 3])]);
    let compact = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1; 3])]);
    let reduction = ReduceTo::<Kernel>::reduce_to(&source).unwrap();
    let compact_reduction = ReduceTo::<Kernel>::reduce_to(&compact).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 5);
    assert_eq!(
        reduction.target_problem().graph().arcs(),
        compact_reduction.target_problem().graph().arcs()
    );
}
