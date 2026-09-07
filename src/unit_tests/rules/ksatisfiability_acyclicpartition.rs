use super::incidence_parameters;
use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::graph::AcyclicPartition;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_acyclicpartition_closed_loop() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert_eq!((target.num_vertices(), target.num_arcs()), (9, 20));
    let mut count = 0;
    // Enumerate every two-anchor-block partition. Arbitrary additional blocks
    // are excluded by the cost lemma, independently checked in the proof audit.
    for mask in 0..128 {
        let mut labels: Vec<usize> = (0..7).map(|i| (mask >> i) & 1).collect();
        labels.extend([0, 1]);
        if target.evaluate(&labels).unwrap().0 {
            count += 1;
            assert!(
                source
                    .evaluate(&reduction.extract_solution(&labels).unwrap())
                    .unwrap()
                    .0
            );
            let renamed = labels.iter().map(|&x| if x == 0 { 8 } else { 3 }).collect();
            assert!(
                source
                    .evaluate(&reduction.extract_solution(&renamed).unwrap())
                    .unwrap()
                    .0
            );
        } else {
            assert!(reduction.extract_solution(&labels).is_err());
        }
    }
    assert_eq!(count, 3);
}

#[test]
fn test_acyclicpartition_extraction_rejects_invalid_targets() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).unwrap();
    for labels in [
        vec![],
        vec![0; 8],
        vec![0; 10],
        vec![9; 9],
        vec![0; 9],
        vec![2, 1, 1, 0, 0, 1, 1, 0, 1],
    ] {
        assert!(reduction.extract_solution(&labels).is_err());
    }
}

#[test]
fn test_acyclicpartition_native_empty_and_short_clauses() {
    for n in [0, 3] {
        let source = KSatisfiability::<K3>::new(n, vec![]);
        let reduction = ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).unwrap();
        let witnesses = BruteForce::new()
            .find_all_witnesses(reduction.target_problem())
            .unwrap();
        assert!(!witnesses.is_empty());
        for labels in witnesses {
            assert!(
                source
                    .evaluate(&reduction.extract_solution(&labels).unwrap())
                    .unwrap()
                    .0
            );
        }
    }
    for clauses in [vec![vec![]], vec![vec![1], vec![-1]]] {
        let source = KSatisfiability::<K3>::new_allow_less(
            1,
            clauses.into_iter().map(CNFClause::new).collect(),
        );
        let reduction = ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).unwrap();
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        let items = reduction.target_problem().num_vertices() - 2;
        for mask in 0..(1usize << items) {
            let mut labels: Vec<usize> = (0..items).map(|i| (mask >> i) & 1).collect();
            labels.extend([0, 1]);
            assert!(!reduction.target_problem().evaluate(&labels).unwrap().0);
        }
    }
}

#[test]
fn test_ksatisfiability_to_acyclicpartition_multi_variable_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).unwrap();
    let clique = reduction.sat_to_clique.target_problem();
    let edges = clique.graph().edges();
    let n = clique.num_vertices();
    assert_eq!(
        reduction.target_problem().num_vertices(),
        n + edges.len() + 2
    );
    let witnesses = BruteForce::new().find_all_witnesses(&source).unwrap();
    assert!(!witnesses.is_empty());
    for assignment in witnesses {
        let mut selected = vec![false; n];
        let mut offset = 0;
        for clause in source.clauses() {
            let index = clause
                .variables()
                .iter()
                .zip(&clause.literals)
                .position(|(&v, &lit)| assignment[v] == (lit > 0))
                .unwrap();
            selected[offset + index] = true;
            offset += clause.len();
        }
        selected[offset] = true;
        let mut labels: Vec<usize> = selected.iter().map(|&b| usize::from(!b)).collect();
        labels.extend(
            edges
                .iter()
                .map(|&(u, v)| usize::from(!(selected[u] && selected[v]))),
        );
        labels.extend([0, 1]);
        assert!(reduction.target_problem().evaluate(&labels).unwrap().0);
        assert!(
            source
                .evaluate(&reduction.extract_solution(&labels).unwrap())
                .unwrap()
                .0
        );
    }
}

#[test]
fn test_incidence_parameters_checked_arithmetic() {
    assert_eq!(incidence_parameters(1, 0, 1).unwrap(), (3, 2, 1, 3, 5, 2));
    assert_eq!(
        incidence_parameters(4, 3, 2).unwrap(),
        (9, 20, 3, 15, 21, 101)
    );
    for (n, e, k) in [
        (usize::MAX, 1, 1),
        (usize::MAX, 0, 1),
        (1, usize::MAX / 2, 1),
        (1, 0, usize::MAX),
        (1, 0, i64::MAX as usize),
        (1, 0, 5_000_000_000),
        (3_000_000_000, 0, 1),
    ] {
        assert!(incidence_parameters(n, e, k).is_err(), "{n}, {e}, {k}");
    }
}
