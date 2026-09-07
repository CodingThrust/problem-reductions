use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_clique_cover_dp_minimum_intersection_graph_basis_matches_brute_force() {
    for edge_mask in 0usize..8 {
        let all_edges = [(0, 1), (0, 2), (1, 2)];
        let edges = all_edges
            .iter()
            .enumerate()
            .filter_map(|(edge, pair)| (edge_mask & (1 << edge) != 0).then_some(*pair))
            .collect();
        let problem = MinimumIntersectionGraphBasis::new(SimpleGraph::new(3, edges));
        let expected = BruteForce::new().solve(&problem).unwrap().unwrap();
        let actual = solve(&problem).unwrap();
        assert_eq!(
            problem.evaluate(&actual).unwrap(),
            problem.evaluate(&expected).unwrap()
        );
    }
}

#[test]
fn test_clique_cover_dp_minimum_intersection_graph_basis_handles_overlapping_cliques() {
    let problem = MinimumIntersectionGraphBasis::new(SimpleGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 2), (2, 3), (2, 4), (3, 4)],
    ));
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(2));
}

#[test]
fn test_intersection_basis_handles_dense_graphs_beyond_machine_word_edges() {
    for n in [8, 12] {
        let edges = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        let problem = MinimumIntersectionGraphBasis::new(SimpleGraph::new(n, edges));
        let solution = solve(&problem).unwrap();
        assert_eq!(problem.evaluate(&solution).unwrap().0, Some(1));
    }
}

#[test]
fn test_minimum_cover_improves_a_redundant_initial_cover() {
    let covers = vec![
        vec![true, true, false],
        vec![true, false, true],
        vec![false, true, true],
    ];
    let mut best = vec![0, 1, 2];
    minimum_cover(&covers, &[false; 3], &mut Vec::new(), &mut best);
    assert_eq!(best.len(), 2);
    assert!((0..3).all(|edge| best.iter().any(|&clique| covers[clique][edge])));
}
