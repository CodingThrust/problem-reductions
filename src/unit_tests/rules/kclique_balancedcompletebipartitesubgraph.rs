use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_kclique_to_balancedcompletebipartitesubgraph_closed_loop() {
    // 4-vertex graph with edges {0,1}, {0,2}, {1,2}, {2,3}, k=3
    // Known 3-clique: {0, 1, 2}
    let source = KClique::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]), 3);
    let reduction = ReduceTo::<BalancedCompleteBipartiteSubgraph>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify target sizes
    // left_size = n + C(k,2) = 4 + 3 = 7
    assert_eq!(target.left_size(), 7);
    // right_size = m + (n - k) = 4 + 1 = 5
    assert_eq!(target.right_size(), 5);
    // target k = left_size - k = 7 - 3 = 4
    assert_eq!(target.k(), 4);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "KClique->BalancedCompleteBipartiteSubgraph closed loop",
    );
}

#[test]
fn test_kclique_to_bcbs_complete_graph() {
    // K4 graph, k=3 -> should find a 3-clique
    let source = KClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        3,
    );
    let reduction = ReduceTo::<BalancedCompleteBipartiteSubgraph>::reduce_to(&source);
    let target = reduction.target_problem();

    // left_size = 4 + 3 = 7, right_size = 6 + 1 = 7, target_k = 7 - 3 = 4
    assert_eq!(target.left_size(), 7);
    assert_eq!(target.right_size(), 7);
    assert_eq!(target.k(), 4);

    let bf = BruteForce::new();
    let witness = bf.find_witness(target).expect("K4 should contain K3");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(source.evaluate(&extracted), Or(true));
    // Exactly 3 vertices should be selected
    assert_eq!(extracted.iter().sum::<usize>(), 3);
}

#[test]
fn test_kclique_to_bcbs_no_clique() {
    // Path graph: 0-1-2-3, k=3 -> no 3-clique exists
    let source = KClique::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), 3);
    let reduction = ReduceTo::<BalancedCompleteBipartiteSubgraph>::reduce_to(&source);
    let target = reduction.target_problem();

    // left_size = 4 + 3 = 7, right_size = 3 + 1 = 4, target_k = 7 - 3 = 4
    assert_eq!(target.left_size(), 7);
    assert_eq!(target.right_size(), 4);
    assert_eq!(target.k(), 4);

    // No balanced biclique should exist
    let bf = BruteForce::new();
    let witness = bf.find_witness(target);
    assert!(
        witness.is_none(),
        "path graph should not contain a 3-clique"
    );

    // Also verify brute force on source agrees
    let source_witness = bf.find_witness(&source);
    assert!(source_witness.is_none());
}

#[test]
fn test_kclique_to_bcbs_k_equals_2() {
    // k=2 means we need an edge
    let source = KClique::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), 2);
    let reduction = ReduceTo::<BalancedCompleteBipartiteSubgraph>::reduce_to(&source);
    let target = reduction.target_problem();

    // left_size = 4 + 1 = 5, right_size = 2 + 2 = 4, target_k = 5 - 2 = 3
    assert_eq!(target.left_size(), 5);
    assert_eq!(target.right_size(), 4);
    assert_eq!(target.k(), 3);

    let bf = BruteForce::new();
    let witness = bf
        .find_witness(target)
        .expect("graph has edges, so 2-clique exists");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(source.evaluate(&extracted), Or(true));
    assert_eq!(extracted.iter().sum::<usize>(), 2);
}

#[test]
fn test_kclique_to_bcbs_k_equals_1() {
    // k=1: any graph has a 1-clique (single vertex)
    let source = KClique::new(SimpleGraph::new(3, vec![(0, 1)]), 1);
    let reduction = ReduceTo::<BalancedCompleteBipartiteSubgraph>::reduce_to(&source);
    let target = reduction.target_problem();

    // left_size = 3 + 0 = 3, right_size = 1 + 2 = 3, target_k = 3 - 1 = 2
    assert_eq!(target.left_size(), 3);
    assert_eq!(target.right_size(), 3);
    assert_eq!(target.k(), 2);

    let bf = BruteForce::new();
    let witness = bf.find_witness(target).expect("should find a 1-clique");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(source.evaluate(&extracted), Or(true));
    assert_eq!(extracted.iter().sum::<usize>(), 1);
}

#[test]
fn test_kclique_to_bcbs_bipartite_counterexample() {
    // K_{3,3} bipartite graph: vertices {0,1,2} on left, {3,4,5} on right
    // All 9 cross-edges. Max clique = 2 (bipartite => no triangles).
    // k=3 should fail.
    let source = KClique::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 3),
                (0, 4),
                (0, 5),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 3),
                (2, 4),
                (2, 5),
            ],
        ),
        3,
    );
    let reduction = ReduceTo::<BalancedCompleteBipartiteSubgraph>::reduce_to(&source);
    let target = reduction.target_problem();

    let bf = BruteForce::new();
    let witness = bf.find_witness(target);
    assert!(
        witness.is_none(),
        "K_{{3,3}} has no 3-clique, so target should be unsatisfiable"
    );
}
