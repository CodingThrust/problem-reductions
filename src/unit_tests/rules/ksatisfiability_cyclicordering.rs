use super::*;
use crate::models::formula::CNFClause;
use crate::models::misc::CyclicOrdering;
use crate::traits::Problem;
use crate::variant::K3;
use std::cmp::Reverse;

fn solve_cyclic_ordering(problem: &CyclicOrdering) -> Option<Vec<usize>> {
    let n = problem.num_elements();
    if n == 0 {
        return None;
    }
    if n == 1 {
        return if problem.triples().is_empty() {
            Some(vec![0])
        } else {
            None
        };
    }

    let mut triples_by_element = vec![Vec::new(); n];
    for (triple_idx, &(a, b, c)) in problem.triples().iter().enumerate() {
        triples_by_element[a].push(triple_idx);
        triples_by_element[b].push(triple_idx);
        triples_by_element[c].push(triple_idx);
    }

    let mut element_order: Vec<usize> = (1..n).collect();
    element_order.sort_by_key(|&element| Reverse(triples_by_element[element].len()));

    let mut positions = vec![None; n];
    let mut taken = vec![false; n];
    positions[0] = Some(0);
    taken[0] = true;

    #[allow(clippy::nonminimal_bool)]
    fn is_cyclic_order(a: usize, b: usize, c: usize) -> bool {
        (a < b && b < c) || (b < c && c < a) || (c < a && a < b)
    }

    fn triple_ok(
        problem: &CyclicOrdering,
        positions: &[Option<usize>],
        element: usize,
        triples_by_element: &[Vec<usize>],
    ) -> bool {
        for &triple_idx in &triples_by_element[element] {
            let (a, b, c) = problem.triples()[triple_idx];
            if let (Some(pa), Some(pb), Some(pc)) = (positions[a], positions[b], positions[c]) {
                if !is_cyclic_order(pa, pb, pc) {
                    return false;
                }
            }
        }
        true
    }

    fn recurse(
        problem: &CyclicOrdering,
        triples_by_element: &[Vec<usize>],
        element_order: &[usize],
        idx: usize,
        positions: &mut [Option<usize>],
        taken: &mut [bool],
    ) -> bool {
        if idx == element_order.len() {
            return true;
        }

        let element = element_order[idx];
        for pos in 0..problem.num_elements() {
            if taken[pos] {
                continue;
            }

            positions[element] = Some(pos);
            taken[pos] = true;

            if triple_ok(problem, positions, element, triples_by_element)
                && recurse(
                    problem,
                    triples_by_element,
                    element_order,
                    idx + 1,
                    positions,
                    taken,
                )
            {
                return true;
            }

            positions[element] = None;
            taken[pos] = false;
        }

        false
    }

    if recurse(
        problem,
        &triples_by_element,
        &element_order,
        0,
        &mut positions,
        &mut taken,
    ) {
        Some(
            positions
                .into_iter()
                .map(|position| position.expect("solver assigns every element"))
                .collect(),
        )
    } else {
        None
    }
}

#[test]
fn test_ksatisfiability_to_cyclicordering_single_clause_reference_vector() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source);
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

    let target_solution = solve_cyclic_ordering(target).expect("single-clause gadget should be solvable");
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1, 1, 1]);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_ksatisfiability_to_cyclicordering_all_negated_clause_matches_reference_vector() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);
    let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source);
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
    let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source);
    let target_solution = vec![0, 11, 1, 9, 12, 10, 6, 13, 7, 2, 3, 4, 8, 5];

    assert!(reduction.target_problem().evaluate(&target_solution).0);
    assert_eq!(reduction.extract_solution(&target_solution), vec![1, 1, 1]);
}

#[test]
fn test_ksatisfiability_to_cyclicordering_clause_gadget_truth_patterns() {
    let gadget = vec![
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
    ];

    for x_true in [false, true] {
        for y_true in [false, true] {
            for z_true in [false, true] {
                let mut triples = gadget.clone();
                triples.push(if x_true { (0, 2, 1) } else { (0, 1, 2) });
                triples.push(if y_true { (3, 5, 4) } else { (3, 4, 5) });
                triples.push(if z_true { (6, 8, 7) } else { (6, 7, 8) });

                let problem = CyclicOrdering::new(14, triples);
                let has_witness = solve_cyclic_ordering(&problem).is_some();
                assert_eq!(
                    has_witness,
                    x_true || y_true || z_true,
                    "truth pattern ({x_true}, {y_true}, {z_true})"
                );
            }
        }
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
    let reduction = ReduceTo::<CyclicOrdering>::reduce_to(&source);

    assert!(
        solve_cyclic_ordering(reduction.target_problem()).is_none(),
        "opposite repeated-literal clauses should be unsatisfiable after reduction"
    );
}
