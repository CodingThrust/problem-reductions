use super::*;
use crate::models::formula::{CNFClause, OneInThreeSatisfiability};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        4,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![2, -3, 4]),
        ],
    );

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "3SAT->1in3SAT closed loop",
    );
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_structure_single_clause() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 11);
    assert_eq!(target.num_clauses(), 6);
    assert_eq!(
        target.clauses(),
        [
            CNFClause::new(vec![4, 4, 5]),
            CNFClause::new(vec![1, 6, 9]),
            CNFClause::new(vec![2, 7, 9]),
            CNFClause::new(vec![6, 7, 10]),
            CNFClause::new(vec![8, 9, 11]),
            CNFClause::new(vec![3, 8, 4]),
        ]
        .as_slice()
    );
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_structure_negated_clause() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 11);
    assert_eq!(target.num_clauses(), 6);
    assert_eq!(
        target.clauses(),
        [
            CNFClause::new(vec![4, 4, 5]),
            CNFClause::new(vec![-1, 6, 9]),
            CNFClause::new(vec![-2, 7, 9]),
            CNFClause::new(vec![6, 7, 10]),
            CNFClause::new(vec![8, 9, 11]),
            CNFClause::new(vec![-3, 8, 4]),
        ]
        .as_slice()
    );
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_unsatisfiable() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    assert!(solver.solve(&source).unwrap().is_none());
    assert!(solver.solve(target).unwrap().is_none());
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_extract_solution() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let target_solution = vec![
        false, false, true, false, true, false, false, false, true, true, false,
    ];
    assert!(target.evaluate(&target_solution).unwrap().0);

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![false, false, true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_oneinthree_native_empty_short_and_sparse_clauses() {
    for (num_vars, formula) in [
        (0, vec![]),
        (3, vec![]),
        (0, vec![vec![]]),
        (3, vec![vec![]]),
        (5, vec![vec![5]]),
        (5, vec![vec![-5]]),
        (5, vec![vec![5, -2]]),
        (5, vec![vec![5, -5, 2]]),
    ] {
        let source = KSatisfiability::<K3>::try_new_allow_less(
            num_vars,
            formula.iter().cloned().map(CNFClause::new).collect(),
        )
        .unwrap();
        let reduction = ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let appearing: std::collections::BTreeSet<_> = formula
            .iter()
            .flatten()
            .map(|l| l.unsigned_abs() as usize - 1)
            .collect();
        assert_eq!(target.num_vars(), appearing.len() + 2 + 6 * formula.len());
        assert_eq!(target.num_clauses(), 1 + 5 * formula.len());
        let source_exists = BruteForce::new().solve(&source).unwrap().is_some();
        let mut target_exists = false;
        for mask in 0..(1usize << target.num_vars()) {
            let config = (0..target.num_vars())
                .map(|v| mask & (1 << v) != 0)
                .collect();
            if target.evaluate(&config).unwrap().0 {
                target_exists = true;
                let extracted = reduction.extract_solution(&config).unwrap();
                assert_eq!(extracted.len(), num_vars);
                assert!(source.evaluate(&extracted).unwrap().0);
                for (v, &value) in extracted.iter().enumerate() {
                    if !appearing.contains(&v) {
                        assert!(!value);
                    }
                }
            }
        }
        assert_eq!(source_exists, target_exists);
    }
}

#[test]
fn test_oneinthree_all_literal_truth_patterns_and_auxiliary_assignments() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let mut extensions = [0usize; 8];
    for mask in 0..(1usize << target.num_vars()) {
        let config = (0..target.num_vars())
            .map(|v| mask & (1 << v) != 0)
            .collect();
        if target.evaluate(&config).unwrap().0 {
            extensions[mask & 7] += 1;
            let extracted = reduction.extract_solution(&config).unwrap();
            assert!(source.evaluate(&extracted).unwrap().0);
        }
    }
    assert_eq!(extensions, [0, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_oneinthree_rejects_infeasible_target_assignments() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1; 3])]);
    let reduction = ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).unwrap();
    for config in [vec![], vec![false; 9], vec![true; 9], vec![false; 10]] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn test_oneinthree_huge_sparse_source_uses_compact_allocator() {
    let largest = usize::try_from(i64::MAX).unwrap_or(usize::MAX);
    let literal = i64::try_from(largest).unwrap();
    let source = KSatisfiability::<K3>::new(largest, vec![CNFClause::new(vec![literal; 3])]);
    let compact = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1; 3])]);
    let reduction = ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).unwrap();
    let other = ReduceTo::<OneInThreeSatisfiability>::reduce_to(&compact).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 9);
    assert_eq!(
        reduction.target_problem().clauses(),
        other.target_problem().clauses()
    );
}
