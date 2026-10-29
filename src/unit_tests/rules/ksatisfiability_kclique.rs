use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_ksatisfiability_to_kclique_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert_eq!(
        (target.num_vertices(), target.num_edges(), target.k()),
        (7, 13, 3)
    );
    for witness in BruteForce::new().find_all_witnesses(target).unwrap() {
        assert!(witness[6]);
        assert!(
            source
                .evaluate(&reduction.extract_solution(&witness).unwrap())
                .unwrap()
                .0
        );
    }
    let witness = vec![false, false, true, true, false, false, true];
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![false, false, true]
    );
    let no = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&no).unwrap();
    assert_eq!(reduction.target_problem().num_edges(), 6);
    assert!(
        BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .is_none()
    );
}

#[test]
fn test_kclique_empty_formulas_and_repeated_literals() {
    for n in [0, 3, 4] {
        let source = KSatisfiability::<K3>::new(n, vec![]);
        let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&source).unwrap();
        assert_eq!(
            (
                reduction.target_problem().num_vertices(),
                reduction.target_problem().k()
            ),
            (1, 1)
        );
        assert_eq!(
            reduction.extract_solution(&vec![true]).unwrap(),
            vec![false; n]
        );
    }
    for clauses in [
        vec![vec![1, 1, 1], vec![-1, -1, -1]],
        vec![vec![1, 1, 1], vec![2, -1, -1]],
    ] {
        let source =
            KSatisfiability::<K3>::new(2, clauses.into_iter().map(CNFClause::new).collect());
        let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&source).unwrap();
        let sat = BruteForce::new().solve(&source).unwrap().is_some();
        let solutions = BruteForce::new()
            .find_all_witnesses(reduction.target_problem())
            .unwrap();
        assert_eq!(!solutions.is_empty(), sat);
        for witness in solutions {
            assert!(
                source
                    .evaluate(&reduction.extract_solution(&witness).unwrap())
                    .unwrap()
                    .0
            );
        }
    }
}

#[test]
fn test_kclique_all_two_clause_formulas_and_target_selections() {
    // All ordered three-literal clauses on one variable, including repetitions
    // and tautologies; every ordered pair and every raw target bitvector.
    let clauses: Vec<Vec<i64>> = (0..8usize)
        .map(|mask| {
            (0..3)
                .map(|p| if mask & (1 << p) == 0 { 1 } else { -1 })
                .collect()
        })
        .collect();
    for a in &clauses {
        for b in &clauses {
            let source = KSatisfiability::<K3>::new(
                1,
                vec![CNFClause::new(a.clone()), CNFClause::new(b.clone())],
            );
            let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&source).unwrap();
            let target = reduction.target_problem();
            let t = a.len() + b.len();
            assert_eq!(target.num_vertices(), t + 1);
            assert_eq!(target.k(), 3);
            let mut source_yes = false;
            for value in [false, true] {
                let assignment = vec![value];
                if source.evaluate(&assignment).unwrap().0 {
                    source_yes = true;
                    let mut witness = vec![false; target.num_vertices()];
                    let mut offset = 0;
                    for clause in [a, b] {
                        let p = clause.iter().position(|&l| (l > 0) == value).unwrap();
                        witness[offset + p] = true;
                        offset += clause.len();
                    }
                    witness[t] = true;
                    assert!(target.evaluate(&witness).unwrap().0);
                }
            }
            let mut target_yes = false;
            for mask in 0..(1usize << target.num_vertices()) {
                let witness: Vec<_> = (0..target.num_vertices())
                    .map(|v| mask & (1 << v) != 0)
                    .collect();
                if target.evaluate(&witness).unwrap().0 {
                    target_yes = true;
                    assert!(
                        source
                            .evaluate(&reduction.extract_solution(&witness).unwrap())
                            .unwrap()
                            .0
                    );
                } else {
                    assert!(reduction.extract_solution(&witness).is_err());
                }
            }
            assert_eq!(source_yes, target_yes);
        }
    }
}

#[test]
fn test_kclique_rejects_malformed_or_non_clique_selections() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 2]),
            CNFClause::new(vec![-1, 3, 3]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&source).unwrap();
    for bad in [
        vec![],
        vec![true; 8],
        vec![false; 7],
        vec![true, true, false, false, false, false, true],
        vec![true, false, false, true, false, false, true],
    ] {
        assert!(reduction.extract_solution(&bad).is_err());
    }
}

#[test]
fn test_kclique_count_boundaries() {
    assert_eq!(clique_sizes(0).unwrap(), (1, 1));
    assert_eq!(clique_sizes(2).unwrap(), (7, 3));
    let largest = (usize::MAX - 1) / 3;
    assert_eq!(
        clique_sizes(largest).unwrap(),
        (3 * largest + 1, largest + 1)
    );
    for m in [largest + 1, usize::MAX] {
        assert!(matches!(
            clique_sizes(m),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
