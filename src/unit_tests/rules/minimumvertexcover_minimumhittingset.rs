use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;

#[test]
fn test_minimumvertexcover_to_minimumhittingset_closed_loop() {
    // 6-vertex graph from the issue example
    // Edges: {0,1}, {0,2}, {1,3}, {2,3}, {2,4}, {3,5}, {4,5}, {1,4}
    // Minimum vertex cover size = 3 (e.g., {0, 3, 4})
    let vc_problem = MinimumVertexCover::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 5),
                (4, 5),
                (1, 4),
            ],
        ),
        vec![One; 6],
    );
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&vc_problem);

    assert_optimization_round_trip_from_optimization_target(
        &vc_problem,
        &reduction,
        "VC(One)->HittingSet closed loop",
    );
}

#[test]
fn test_vc_to_hs_structure() {
    // Path graph 0-1-2 with edges (0,1) and (1,2)
    let vc_problem =
        MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![One; 3]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&vc_problem);
    let hs_problem = reduction.target_problem();

    // Universe size = num_vertices = 3
    assert_eq!(hs_problem.universe_size(), 3);
    // Number of sets = num_edges = 2
    assert_eq!(hs_problem.num_sets(), 2);

    // Each edge becomes a 2-element subset
    assert_eq!(hs_problem.get_set(0), Some(&vec![0, 1]));
    assert_eq!(hs_problem.get_set(1), Some(&vec![1, 2]));
}

#[test]
fn test_vc_to_hs_triangle() {
    // Triangle graph: 3 vertices, 3 edges
    let vc_problem = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![One; 3],
    );
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&vc_problem);
    let hs_problem = reduction.target_problem();

    assert_eq!(hs_problem.universe_size(), 3);
    assert_eq!(hs_problem.num_sets(), 3);

    // All sets have exactly 2 elements
    for i in 0..3 {
        assert_eq!(hs_problem.get_set(i).unwrap().len(), 2);
    }

    // Solve both and verify they match
    let solver = BruteForce::new();
    let vc_solutions = solver.find_all_witnesses(&vc_problem);
    let hs_solutions = solver.find_all_witnesses(hs_problem);

    // Minimum vertex cover of triangle = 2, same for hitting set
    assert_eq!(vc_solutions[0].iter().filter(|&&x| x == 1).count(), 2);
    assert_eq!(hs_solutions[0].iter().filter(|&&x| x == 1).count(), 2);
}

#[test]
fn test_vc_to_hs_empty_graph() {
    // Graph with no edges: no sets to hit
    let vc_problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![]), vec![One; 3]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&vc_problem);
    let hs_problem = reduction.target_problem();

    assert_eq!(hs_problem.universe_size(), 3);
    assert_eq!(hs_problem.num_sets(), 0);
}

#[test]
fn test_vc_to_hs_star_graph() {
    // Star graph: center vertex 0 connected to 1, 2, 3
    let vc_problem = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![One; 4],
    );
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&vc_problem);
    let hs_problem = reduction.target_problem();

    assert_eq!(hs_problem.universe_size(), 4);
    assert_eq!(hs_problem.num_sets(), 3);

    // Each set is a 2-element subset containing vertex 0
    for i in 0..3 {
        let set = hs_problem.get_set(i).unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains(&0));
    }

    // Minimum cover = just vertex 0
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&vc_problem);
    assert_eq!(solutions[0], vec![1, 0, 0, 0]);
}

#[test]
fn test_vc_to_hs_solution_extraction() {
    // Verify that extract_solution is identity (1:1 correspondence)
    let vc_problem =
        MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![One; 3]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&vc_problem);

    let target_solution = vec![0, 1, 0];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
}
