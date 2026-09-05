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
