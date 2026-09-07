use super::*;
use crate::models::formula::CNFClause;
use crate::models::misc::CyclicOrdering;
use crate::traits::Problem;
use crate::variant::K3;
// Construct the paper's seven local orders and merge their auxiliary
// sequences into a single common order of the variable blocks.
fn forward_witness(source: &KSatisfiability<K3>, assignment: &[bool]) -> Vec<usize> {
    assert!(source.evaluate(&assignment.to_vec()).unwrap().0);
    let normalized = normalize(source).unwrap();
    if normalized.clauses.is_empty() {
        return vec![0];
    }
    let mut values = vec![false; normalized.num_vars];
    for (compact, &original) in normalized.source_variables.iter().enumerate() {
        values[compact] = assignment[original];
    }
    let base_count = 3 * normalized.num_vars;
    let size = base_count + 5 * normalized.clauses.len();
    let mut after = vec![Vec::new(); base_count];
    let templates = [
        "",
        "ackmbdefjlnghi",
        "abcjkdmflneghi",
        "ackmbdfejlnghi",
        "abcdefjklngimh",
        "ackmbdefjlngih",
        "abcjkdmflnegih",
        "acbjkdmflnegih",
    ];
    for (clause_index, clause) in normalized.clauses.iter().enumerate() {
        let mut symbols = Vec::new();
        let mut mask = 0;
        for (slot, &literal) in clause.iter().enumerate() {
            let (a, b, c) = literal_triple(literal);
            symbols.extend([a, b, c]);
            if values[literal.unsigned_abs() as usize - 1] == (literal > 0) {
                mask |= 1 << slot;
            }
        }
        assert_ne!(
            mask, 0,
            "source assignment must satisfy every expanded clause"
        );
        symbols.extend(base_count + 5 * clause_index..base_count + 5 * clause_index + 5);
        let mut anchor = None;
        for ch in templates[mask].bytes() {
            let vertex = symbols[(ch - b'a') as usize];
            if vertex < base_count {
                anchor = Some(vertex);
            } else {
                after[anchor.expect("the local order starts with a variable element")].push(vertex);
            }
        }
    }
    let mut order = Vec::new();
    for (i, &value) in values.iter().enumerate() {
        let (a, b, c) = variable_triple(i);
        for vertex in if value { [a, c, b] } else { [a, b, c] } {
            order.push(vertex);
            order.extend(&after[vertex]);
        }
    }
    assert_eq!(order.len(), size);
    let mut positions = vec![0; size];
    for (position, vertex) in order.into_iter().enumerate() {
        positions[vertex] = position;
    }
    positions
}

#[test]
fn test_ksatisfiability_to_cyclicordering_single_clause_reference_vector() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction =
        ReduceTo::<CyclicOrdering>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 14);
    assert_eq!(target.num_triples(), 10);
    assert_eq!(
        target.triples(),
        &[
            (0, 2, 9),
            (1, 9, 10),
            (2, 10, 11),
            (3, 5, 9),
            (4, 9, 11),
            (5, 11, 12),
            (6, 8, 10),
            (7, 10, 12),
            (8, 12, 13),
            (13, 12, 11),
        ]
    );

    let target_solution = forward_witness(&source, &[true, true, true]);
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![true, true, true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_ksatisfiability_to_cyclicordering_all_negated_clause_matches_reference_vector() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);
    let reduction =
        ReduceTo::<CyclicOrdering>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 14);
    assert_eq!(target.num_triples(), 10);
    assert_eq!(
        target.triples(),
        &[
            (0, 1, 9),
            (2, 9, 10),
            (1, 10, 11),
            (3, 4, 9),
            (5, 9, 11),
            (4, 11, 12),
            (6, 7, 10),
            (8, 10, 12),
            (7, 12, 13),
            (13, 12, 11),
        ]
    );
}

#[test]
fn test_ksatisfiability_to_cyclicordering_extract_solution_from_reference_witness() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction =
        ReduceTo::<CyclicOrdering>::reduce_to(&source).expect("reduction should succeed");
    let target_solution = vec![0, 11, 1, 9, 12, 10, 6, 13, 7, 2, 3, 4, 8, 5];

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap()
            .0
    );
    assert_eq!(
        reduction.extract_solution(&target_solution).unwrap(),
        vec![true, true, true]
    );
}

#[test]
fn test_ksatisfiability_to_cyclicordering_clause_gadget_truth_patterns() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source).unwrap();
    for mask in 1..8 {
        let assignment: Vec<_> = (0..3).map(|bit| mask & (1 << bit) != 0).collect();
        let config = forward_witness(&source, &assignment);
        assert!(reduction.target_problem().evaluate(&config).unwrap().0);
        assert_eq!(reduction.extract_solution(&config).unwrap(), assignment);
    }
}

#[test]
fn test_ksatisfiability_to_cyclicordering_unsatisfiable_repeated_literal_pair() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction =
        ReduceTo::<CyclicOrdering>::reduce_to(&source).expect("reduction should succeed");

    let normalized = normalize(&source).unwrap();
    assert_eq!(normalized.num_vars, 5);
    assert_eq!(normalized.clauses.len(), 8);
    assert_eq!(reduction.target_problem().num_elements(), 55);
    for mask in 0..1usize << normalized.num_vars {
        assert!(!normalized
            .clauses
            .iter()
            .all(|clause| clause.iter().any(|&literal| {
                (mask & (1 << (literal.unsigned_abs() as usize - 1)) != 0) == (literal > 0)
            })));
    }
}

#[test]
fn test_ksatisfiability_to_cyclicordering_closed_loop() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, 2, 1])]);

    let reduction =
        ReduceTo::<CyclicOrdering>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let target_solution = forward_witness(&source, &[true, false]);

    assert!(
        target.evaluate(&target_solution).unwrap().0,
        "target solution must evaluate as satisfying"
    );

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert!(
        source.evaluate(&extracted).unwrap().0,
        "extracted source config must satisfy the source"
    );
}

#[test]
fn repeated_short_and_unsorted_clauses_preserve_all_source_assignments() {
    let formulas = [
        vec![vec![5, 5, 5]],
        vec![vec![-5, -5, -5]],
        vec![vec![5, -5, 5]],
        vec![vec![5, 2, 5]],
        vec![vec![5]],
        vec![vec![-5, 2]],
        vec![vec![5, 1, -3], vec![-1, 5, 2], vec![-3, 2, -5]],
    ];
    for clauses in formulas {
        let source = KSatisfiability::<K3>::try_new_allow_less(
            6,
            clauses.into_iter().map(CNFClause::new).collect(),
        )
        .unwrap();
        let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source).unwrap();
        let normalized = normalize(&source).unwrap();
        assert!(normalized
            .clauses
            .iter()
            .all(|clause| clause[0].unsigned_abs() < clause[1].unsigned_abs()
                && clause[1].unsigned_abs() < clause[2].unsigned_abs()));
        for mask in 0..64 {
            let assignment: Vec<_> = (0..6).map(|bit| mask & (1 << bit) != 0).collect();
            if source.evaluate(&assignment).unwrap().0 {
                let config = forward_witness(&source, &assignment);
                assert!(reduction.target_problem().evaluate(&config).unwrap().0);
                let extracted = reduction.extract_solution(&config).unwrap();
                assert!(source.evaluate(&extracted).unwrap().0);
                for (original, used) in
                    (0..6).map(|i| (i, normalized.source_variables.contains(&i)))
                {
                    assert_eq!(extracted[original], used && assignment[original]);
                }
            }
        }
    }
}

#[test]
fn empty_formula_and_empty_clause_have_opposite_fixed_targets() {
    for num_vars in [0, 3] {
        let source = KSatisfiability::<K3>::new(num_vars, vec![]);
        let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source).unwrap();
        assert_eq!(reduction.target_problem().num_elements(), 1);
        assert_eq!(
            reduction.extract_solution(&vec![0]).unwrap(),
            vec![false; num_vars]
        );
        let source =
            KSatisfiability::<K3>::try_new_allow_less(num_vars, vec![CNFClause::new(vec![])])
                .unwrap();
        let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source).unwrap();
        assert_eq!(reduction.target_problem().num_elements(), 3);
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
    }
}

#[test]
fn reject_invalid_orderings_and_accept_every_rotation() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source).unwrap();
    let config = forward_witness(&source, &[true, false, true]);
    let n = config.len();
    for shift in 0..n {
        let rotated = config
            .iter()
            .map(|position| (position + shift) % n)
            .collect();
        assert_eq!(
            reduction.extract_solution(&rotated).unwrap(),
            vec![true, false, true]
        );
    }
    for config in [vec![], vec![0; n], vec![n; n], (0..n).collect()] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn huge_sparse_variable_index_does_not_enlarge_the_target() {
    let largest = usize::try_from(i64::MAX).unwrap_or(usize::MAX);
    let literal = i64::try_from(largest).unwrap();
    let sparse = KSatisfiability::<K3>::new(largest, vec![CNFClause::new(vec![literal; 3])]);
    let compact = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1; 3])]);
    let a = ReduceTo::<CyclicOrdering>::reduce_to(&sparse).unwrap();
    let b = ReduceTo::<CyclicOrdering>::reduce_to(&compact).unwrap();
    assert_eq!(
        a.target_problem().num_elements(),
        b.target_problem().num_elements()
    );
    assert_eq!(a.target_problem().triples(), b.target_problem().triples());
    // This construction test does not allocate an enormous source witness.
}
