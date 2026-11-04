use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::traits::Problem;
use crate::types::Max;
use std::collections::HashSet;

fn example() -> MaximumContactMapOverlap {
    MaximumContactMapOverlap::new(
        4,
        vec![(0, 2), (1, 3), (0, 3)],
        4,
        vec![(0, 1), (2, 3), (0, 3)],
    )
}

fn reduce(source: &MaximumContactMapOverlap) -> ReductionCMOToClique {
    ReduceTo::<MaximumClique<SimpleGraph, One>>::reduce_to(source).expect("reduction succeeds")
}

fn edge_set(reduction: &ReductionCMOToClique) -> Vec<(usize, usize)> {
    let mut edges: Vec<_> = reduction
        .target_problem()
        .graph()
        .edges()
        .into_iter()
        .map(|(u, v)| (u.min(v), u.max(v)))
        .collect();
    edges.sort_unstable();
    edges
}

fn selection(len: usize, chosen: &[usize]) -> Vec<bool> {
    (0..len).map(|i| chosen.contains(&i)).collect()
}

fn optimum(source: &MaximumContactMapOverlap) -> i64 {
    let witness = BruteForce::new().solve(source).unwrap().unwrap();
    source.evaluate(&witness).unwrap().0.unwrap()
}

/// Checks the optimum equality, extraction from every clique and from every maximum clique.
fn check_instance(source: &MaximumContactMapOverlap) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    let n = target.num_vertices();
    let opt = optimum(source);
    let mut best = 0;
    let mut cliques = Vec::new();
    for mask in 0u32..(1 << n) {
        let config: Vec<bool> = (0..n).map(|i| mask >> i & 1 == 1).collect();
        if let Max(Some(size)) = target.evaluate(&config).unwrap() {
            best = best.max(size);
            cliques.push((config, size));
        }
    }
    assert_eq!(best, opt, "{source:?}");
    for (config, size) in cliques {
        let alignment = reduction.extract_solution(&config).unwrap();
        let score = source
            .evaluate(&alignment)
            .unwrap()
            .0
            .expect("feasible alignment");
        assert!(score >= size, "{source:?}: score {score} < clique {size}");
        if size == best {
            assert_eq!(score, opt, "{source:?}");
        }
    }
}

fn contact_sets(n: usize, max_len: usize) -> Vec<Vec<(usize, usize)>> {
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
        .collect();
    (0u32..(1 << pairs.len()))
        .map(|mask| {
            pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| mask >> i & 1 == 1)
                .map(|(_, &pair)| pair)
                .collect::<Vec<_>>()
        })
        .filter(|set| set.len() <= max_len)
        .collect()
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_closed_loop() {
    let source = example();
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 9);
    assert_eq!(edge_set(&reduction), vec![(0, 8), (4, 8)]);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumContactMapOverlap->MaximumClique closed loop",
    );
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_canonical_example() {
    let source = example();
    let reduction = reduce(&source);
    let len = 9;
    assert_eq!(
        reduction
            .extract_solution(&selection(len, &[0, 8]))
            .unwrap(),
        vec![1, 0, 2, 4]
    );
    assert_eq!(
        reduction.extract_solution(&selection(len, &[0])).unwrap(),
        vec![1, 0, 2, 0]
    );
    assert_eq!(
        reduction.extract_solution(&selection(len, &[4])).unwrap(),
        vec![0, 3, 0, 4]
    );
    assert_eq!(
        reduction.extract_solution(&selection(len, &[])).unwrap(),
        vec![0; 4]
    );
    // Candidates 0 and 4 assign crossing images to source positions 1 and 2.
    assert!(!edge_set(&reduction).contains(&(0, 4)));
    // The optimum is two; the unordered contact graphs are isomorphic paths but the
    // order forbids preserving all three contacts.
    assert_eq!(optimum(&source), 2);
    let max_clique = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        reduction.target_problem().evaluate(&max_clique).unwrap(),
        Max(Some(2))
    );
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_all_small_pairs() {
    // Every pair of contact sets on up to three residues per side.
    for n1 in 1..=3 {
        for n2 in 1..=3 {
            for e1 in contact_sets(n1, usize::MAX) {
                for e2 in contact_sets(n2, usize::MAX) {
                    check_instance(&MaximumContactMapOverlap::new(n1, e1.clone(), n2, e2));
                }
            }
        }
    }
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_sparse_pairs_to_five_residues() {
    // Every pair of contact sets with at most two contacts per side, up to five residues
    // (source brute force is limited to alignment spaces of at most 1300 configurations).
    for n1 in 1..=5usize {
        for n2 in 1..=5usize {
            if (n2 + 1).pow(n1 as u32) > 1300 || (n1 <= 3 && n2 <= 3) {
                continue;
            }
            for e1 in contact_sets(n1, 2) {
                for e2 in contact_sets(n2, 2) {
                    check_instance(&MaximumContactMapOverlap::new(n1, e1.clone(), n2, e2));
                }
            }
        }
    }
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_empty_contacts() {
    for source in [
        MaximumContactMapOverlap::new(3, vec![], 3, vec![(0, 1)]),
        MaximumContactMapOverlap::new(3, vec![(0, 1)], 3, vec![]),
        MaximumContactMapOverlap::new(0, vec![], 0, vec![]),
    ] {
        let reduction = reduce(&source);
        assert_eq!(reduction.target_problem().num_vertices(), 0);
        assert_eq!(reduction.target_problem().num_edges(), 0);
        assert_eq!(
            reduction.extract_solution(&vec![]).unwrap(),
            vec![0; source.num_vertices_1()]
        );
    }
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_more_source_than_target_vertices() {
    let source = MaximumContactMapOverlap::new(5, vec![(0, 4), (1, 2), (3, 4)], 2, vec![(0, 1)]);
    check_instance(&source);
    assert_eq!(optimum(&source), 1);
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_compatibility_rules() {
    // Shared source endpoint: contacts (0,1),(0,2) onto (0,1),(0,2),(1,2).
    let source =
        MaximumContactMapOverlap::new(3, vec![(0, 1), (0, 2)], 3, vec![(0, 1), (0, 2), (1, 2)]);
    let reduction = reduce(&source);
    let edges = edge_set(&reduction);
    let vertex = |i: usize, j: usize| i * 3 + j;
    // Images of position 0 agree: 0->0 in both.
    assert!(edges.contains(&(vertex(0, 0), vertex(1, 1))));
    // Images of position 0 disagree: 0->0 versus 0->1.
    assert!(!edges.contains(&(vertex(0, 0), vertex(1, 2))));
    // Vertices of one source contact are never adjacent, nor those of one target contact.
    assert!(!edges.contains(&(vertex(0, 0), vertex(0, 1))));
    assert!(!edges.contains(&(vertex(0, 0), vertex(1, 0))));

    // Disjoint source contacts cannot reuse a target endpoint.
    let source = MaximumContactMapOverlap::new(4, vec![(0, 1), (2, 3)], 3, vec![(0, 1), (1, 2)]);
    assert!(!edge_set(&reduce(&source)).contains(&(0, 3)));

    // Crossing assignments are excluded.
    let source = MaximumContactMapOverlap::new(4, vec![(0, 2), (1, 3)], 4, vec![(0, 3), (1, 2)]);
    assert!(!edge_set(&reduce(&source)).contains(&(0, 3)));
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_vertices_follow_stored_contact_order() {
    // Contacts keep the order they were given in; vertex `i * m2 + j` pairs the `i`-th
    // source contact with the `j`-th target contact.
    let source = MaximumContactMapOverlap::new(4, vec![(2, 3), (0, 1)], 4, vec![(2, 3), (0, 1)]);
    let reduction = reduce(&source);
    let len = 4;
    assert_eq!(
        reduction.extract_solution(&selection(len, &[0])).unwrap(),
        vec![0, 0, 3, 4]
    );
    assert_eq!(
        reduction.extract_solution(&selection(len, &[1])).unwrap(),
        vec![0, 0, 1, 2]
    );
    assert_eq!(
        reduction.extract_solution(&selection(len, &[2])).unwrap(),
        vec![3, 4, 0, 0]
    );
    // (2,3)->(2,3) with (0,1)->(0,1) is the identity alignment.
    assert_eq!(
        reduction
            .extract_solution(&selection(len, &[0, 3]))
            .unwrap(),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_threshold_three_is_infeasible() {
    let source = example();
    // Threshold three is infeasible on both sides.
    assert!(optimum(&source) < 3);
    let reduction = reduce(&source);
    assert!(
        BruteForce::new()
            .find_all_witnesses(reduction.target_problem())
            .unwrap()
            .iter()
            .all(|w| w.iter().filter(|&&b| b).count() < 3)
    );
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_extraction_rejects_malformed_witnesses() {
    let reduction = reduce(&example());
    // Vertices 0 and 4 are not adjacent.
    assert!(reduction.extract_solution(&selection(9, &[0, 4])).is_err());
    // All vertices selected.
    assert!(reduction.extract_solution(&vec![true; 9]).is_err());
    // Wrong lengths.
    assert!(reduction.extract_solution(&vec![false; 8]).is_err());
    assert!(reduction.extract_solution(&vec![false; 10]).is_err());
    assert!(reduction.extract_solution(&vec![]).is_err());
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_nonmaximum_clique_scores_above_its_size() {
    // Both maps are the triangle on positions 0, 1, 2. The vertices pairing contact
    // (0,1) with (0,1) and (1,2) with (1,2) form a clique of size two whose alignment
    // also preserves the contact (0,2).
    let triangle = vec![(0, 1), (0, 2), (1, 2)];
    let source = MaximumContactMapOverlap::new(3, triangle.clone(), 3, triangle);
    let reduction = reduce(&source);
    let witness = selection(9, &[0, 2 * 3 + 2]);
    // contacts are stored as (0,1),(0,2),(1,2): index 2 is (1,2)
    let alignment = reduction.extract_solution(&witness).unwrap();
    assert_eq!(alignment, vec![1, 2, 3]);
    let size = reduction.target_problem().evaluate(&witness).unwrap();
    assert_eq!(size, Max(Some(2)));
    let score = source.evaluate(&alignment).unwrap();
    assert_eq!(score, Max(Some(3)));
    assert!(score.0 > size.0);
    // At the optimum the two sides agree.
    assert_eq!(optimum(&source), 3);
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_parameter_predictions() {
    for source in [
        example(),
        MaximumContactMapOverlap::new(3, vec![(0, 1)], 3, vec![]),
        MaximumContactMapOverlap::new(4, vec![(0, 2), (1, 3)], 5, vec![(0, 3), (1, 4), (0, 2)]),
    ] {
        let reduction = reduce(&source);
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
        let m = source.num_contacts_1() * source.num_contacts_2();
        assert_eq!(reduction.target_problem().num_vertices(), m);
    }
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_pair_count_boundaries() {
    assert_eq!(pair_count(0, usize::MAX).unwrap(), 0);
    assert_eq!(pair_count(usize::MAX, 1).unwrap(), usize::MAX);
    assert_eq!(pair_count(1 << 32, 1 << 31).unwrap(), 1 << 63);
    for (a, b) in [(usize::MAX, 2), (1 << 32, 1 << 32)] {
        assert!(matches!(
            pair_count(a, b),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn test_maximumcontactmapoverlap_to_maximumclique_edge_bound_holds() {
    let mut seen = HashSet::new();
    for e1 in contact_sets(4, 4) {
        for e2 in contact_sets(4, 4) {
            let source = MaximumContactMapOverlap::new(4, e1.clone(), 4, e2.clone());
            let reduction = reduce(&source);
            let (m1, m2) = (e1.len(), e2.len());
            let bound = m1 * m1.saturating_sub(1) * m2 * m2.saturating_sub(1) / 2;
            assert!(reduction.target_problem().num_edges() <= bound);
            seen.insert((m1, m2));
        }
    }
    assert!(seen.len() > 10);
}
