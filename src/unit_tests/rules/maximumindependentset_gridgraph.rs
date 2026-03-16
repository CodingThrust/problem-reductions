use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::rules::unitdiskmapping::ksg;
use crate::solvers::BruteForce;
use crate::topology::{Graph, KingsSubgraph, SimpleGraph};
use crate::types::One;

#[test]
fn test_map_unweighted_produces_uniform_weights() {
    // Triangle graph
    let result = ksg::map_unweighted(3, &[(0, 1), (1, 2), (0, 2)]);
    assert!(
        result.node_weights.iter().all(|&w| w == 1),
        "map_unweighted triangle should produce uniform weights, got: {:?}",
        result.node_weights
    );

    // Path graph
    let result2 = ksg::map_unweighted(3, &[(0, 1), (1, 2)]);
    assert!(
        result2.node_weights.iter().all(|&w| w == 1),
        "map_unweighted path should produce uniform weights, got: {:?}",
        result2.node_weights
    );

    // Cycle-5
    let result3 = ksg::map_unweighted(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)]);
    assert!(
        result3.node_weights.iter().all(|&w| w == 1),
        "map_unweighted cycle5 should produce uniform weights, got: {:?}",
        result3.node_weights
    );
}

#[test]
fn test_mis_simple_one_to_kings_one_closed_loop() {
    // Path graph: 0-1-2-3-4 (MIS = 3: select vertices 0, 2, 4)
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![One; 5],
    );
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, One>>::reduce_to(&problem);
    let target = result.target_problem();
    assert!(target.graph().num_vertices() > 5);

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]);
    assert_eq!(original_solution.len(), 5);
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 3, "Max IS in path of 5 should be 3");
}
