use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_negative_cycle_cancellation_minimum_cost_circulation_matches_brute_force() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0), (1, 0)]);
    for capacity_mask in 0usize..16 {
        let capacities = (0..4)
            .map(|arc| ((capacity_mask >> arc) & 1) as i64)
            .collect::<Vec<_>>();
        for encoded_costs in 0usize..81 {
            let mut encoded = encoded_costs;
            let costs = (0..4)
                .map(|_| {
                    let cost = (encoded % 3) as i64 - 1;
                    encoded /= 3;
                    cost
                })
                .collect::<Vec<_>>();
            let problem = MinimumCostCirculation::new(graph.clone(), capacities.clone(), costs);
            let expected = BruteForce::new().solve(&problem).unwrap().unwrap();
            let actual = solve(&problem).unwrap();
            assert_eq!(
                problem.evaluate(&actual).unwrap(),
                problem.evaluate(&expected).unwrap()
            );
        }
    }
}

#[test]
fn test_negative_cycle_cancellation_minimum_cost_circulation_handles_multigraph() {
    let problem = MinimumCostCirculation::new(
        DirectedGraph::new(2, vec![(0, 0), (0, 1), (0, 1), (1, 0)]),
        vec![3, 2, 4, 3],
        vec![-2, 4, -3, 1],
    );
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(-12));
}
