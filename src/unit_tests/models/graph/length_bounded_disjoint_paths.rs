use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_spec_derives_path_slot_bound() {
    let problem = LengthBoundedDisjointPaths::try_from(LengthBoundedDisjointPathsCreateSpec {
        graph: vec![(0, 1), (1, 3), (0, 2), (2, 3)],
        num_vertices: None,
        source: 0,
        sink: 3,
        max_length: 2,
    })
    .unwrap();
    assert_eq!(problem.max_paths(), 2);
}
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn sample_graph() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (1, 4), (0, 2), (2, 4), (0, 3), (3, 4)]).unwrap()
}

fn sample_problem() -> LengthBoundedDisjointPaths<SimpleGraph> {
    // max_paths = min(deg(0), deg(4)) = min(3, 3) = 3
    LengthBoundedDisjointPaths::new(sample_graph(), 0, 4, 3).unwrap()
}

#[test]
fn test_length_bounded_disjoint_paths_creation() {
    let problem = sample_problem();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.max_paths(), 3);
    assert_eq!(problem.max_length(), 3);
    // 3 slots * 6 edges = 18 binary variables
    assert_eq!(
        crate::solvers::cartesian_dimensions(&problem).unwrap(),
        vec![2; 18]
    );
}

#[test]
fn test_length_bounded_disjoint_paths_allows_large_bounds() {
    let problem = LengthBoundedDisjointPaths::new(sample_graph(), 0, 4, 10).unwrap();
    let config = encode_paths(6, 3, &[&[0, 1], &[2, 3]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(2)));
}

#[test]
fn test_length_bounded_disjoint_paths_creation_rejects_invalid_source() {
    assert!(LengthBoundedDisjointPaths::new(sample_graph(), 5, 4, 3).is_err());
}

#[test]
fn test_length_bounded_disjoint_paths_creation_rejects_invalid_sink() {
    assert!(LengthBoundedDisjointPaths::new(sample_graph(), 0, 5, 3).is_err());
}

#[test]
fn test_length_bounded_disjoint_paths_creation_rejects_equal_terminals() {
    assert!(LengthBoundedDisjointPaths::new(sample_graph(), 0, 0, 3).is_err());
}

#[test]
fn test_length_bounded_disjoint_paths_creation_rejects_zero_bound() {
    assert!(LengthBoundedDisjointPaths::new(sample_graph(), 0, 4, 0).is_err());
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_optimal() {
    let problem = sample_problem();
    // All 3 paths used
    let config = encode_paths(6, 3, &[&[0, 1], &[2, 3], &[4, 5]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(3)));
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_partial() {
    let problem = sample_problem();
    // Only 2 of 3 slots used, third slot empty
    let config = encode_paths(6, 3, &[&[0, 1], &[2, 3]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(2)));
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_single_path() {
    let problem = sample_problem();
    // Only 1 slot used
    let config = encode_paths(6, 3, &[&[0, 1]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(1)));
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_empty_config() {
    let problem = sample_problem();
    // All slots empty → 0 paths
    let config = vec![vec![false; 6]; 3];
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(0)));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_missing_terminal() {
    let problem = sample_problem();
    // Slot 1 is non-empty but missing sink
    let config = encode_paths(6, 3, &[&[0], &[2, 3]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_disconnected_slot() {
    let problem = sample_problem();
    // The selected edges 0-1 and 3-4 do not connect the terminals.
    let config = encode_paths(6, 3, &[&[0, 5]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_overlong_slot() {
    // Use a graph where a path has 3 edges but max_length=1
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]).unwrap();
    // max_paths = min(deg(0), deg(3)) = min(2, 2) = 2
    let problem = LengthBoundedDisjointPaths::new(graph, 0, 3, 1).unwrap();
    // Path [0,1,2,3] has 3 edges but max_length=1
    let config = encode_paths(4, 2, &[&[0, 1, 2]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_shared_internal_vertices() {
    let problem = sample_problem();
    // Two slots share internal vertex 1
    let config = encode_paths(6, 3, &[&[0, 1], &[0, 1]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_reused_direct_edge() {
    let problem =
        LengthBoundedDisjointPaths::new(SimpleGraph::new(2, vec![(0, 1)]).unwrap(), 0, 1, 1)
            .unwrap();
    // max_paths = min(deg(0), deg(1)) = 1, so only 1 slot
    let config = encode_paths(1, 1, &[&[0]]);
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(1)));
    let triangle = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]).unwrap(),
        0,
        2,
        2,
    )
    .unwrap();
    assert_eq!(
        triangle
            .evaluate(&encode_paths(3, 2, &[&[2], &[2]]))
            .unwrap(),
        Max(None)
    );
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_non_binary_entries() {
    let problem = sample_problem();
    assert!(crate::registry::DynProblem::evaluate_dyn(
        &problem,
        &serde_json::json!([
            [true, true, false, 2, false, false],
            vec![false; 6],
            vec![false; 6]
        ])
    )
    .is_err());
}

#[test]
fn test_length_bounded_disjoint_paths_solver() {
    let problem = sample_problem();
    let solver = BruteForce::new();
    let witness = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&witness).unwrap(), Max(Some(3)));
}

#[test]
fn test_length_bounded_disjoint_paths_serialization() {
    let problem = sample_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let round_trip: LengthBoundedDisjointPaths<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(round_trip.num_vertices(), 5);
    assert_eq!(round_trip.source(), 0);
    assert_eq!(round_trip.sink(), 4);
    assert_eq!(round_trip.max_paths(), 3);
    assert_eq!(round_trip.max_length(), 3);
}

#[test]
fn test_length_bounded_disjoint_paths_graph_getter() {
    let problem = sample_problem();
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 6);
}

#[test]
fn test_length_bounded_disjoint_paths_num_variables() {
    let problem = sample_problem();
    assert_eq!(problem.num_variables().unwrap(), 18);
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_wrong_length_config() {
    let problem = sample_problem();
    assert!(problem.evaluate(&vec![vec![false, true, false]]).is_err());
    assert!(problem.evaluate(&vec![vec![false; 5]; 3]).is_err());
}

#[test]
fn test_length_bounded_disjoint_paths_chorded_path() {
    let problem = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]).unwrap(),
        0,
        2,
        2,
    )
    .unwrap();
    let solution = encode_paths(3, 2, &[&[0, 1], &[2]]);
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(2)));
    let best = BruteForce::new().solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&best).unwrap(), Max(Some(2)));
    assert_eq!(
        problem
            .evaluate(&encode_paths(3, 2, &[&[0, 1, 2]]))
            .unwrap(),
        Max(None)
    );
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_disconnected_cycle() {
    let problem = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(5, vec![(0, 1), (2, 3), (3, 4), (4, 2)]).unwrap(),
        0,
        1,
        4,
    )
    .unwrap();
    assert_eq!(problem.evaluate(&vec![vec![true; 4]]).unwrap(), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_edgeless_graph() {
    let problem =
        LengthBoundedDisjointPaths::new(SimpleGraph::new(2, vec![]).unwrap(), 0, 1, 1).unwrap();
    let solution = BruteForce::new().solve(&problem).unwrap().unwrap();
    assert!(solution.is_empty());
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(0)));
}
