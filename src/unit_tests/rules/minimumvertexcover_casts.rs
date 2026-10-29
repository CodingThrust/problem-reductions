use crate::models::graph::MinimumVertexCover;
use crate::rules::ReductionGraph;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One};

#[test]
fn test_minimumvertexcover_one_to_i64_closed_loop() {
    type Source = MinimumVertexCover<SimpleGraph, One>;
    type Target = MinimumVertexCover<SimpleGraph, i64>;
    let graph = ReductionGraph::new();
    let paths = graph.find_paths_up_to(
        Source::NAME,
        &ReductionGraph::variant_to_map(&Source::variant()),
        Target::NAME,
        &ReductionGraph::variant_to_map(&Target::variant()),
        1,
    );
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].steps.len(), 2, "weight conversion must be direct");

    for (input, optimum) in [
        (SimpleGraph::new(0, vec![]), 0),
        (SimpleGraph::new(3, vec![]), 0),
        (SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2)]), 2),
    ] {
        let source = Source::new(input.clone(), vec![One; input.num_vertices()]);
        let chain = graph
            .reduce_along_path(&paths[0], &source)
            .unwrap()
            .unwrap();
        let target = chain.target_problem::<Target>();
        assert_eq!(target.graph(), source.graph());
        assert_eq!(target.weights(), &vec![1_i64; input.num_vertices()]);
        assert_eq!(target.parameters(), source.parameters());

        for mask in 0..(1 << input.num_vertices()) {
            let solution: Vec<bool> = (0..input.num_vertices())
                .map(|vertex| mask & (1 << vertex) != 0)
                .collect();
            assert_eq!(target.evaluate(&solution), source.evaluate(&solution));
            assert_eq!(
                chain.extract_solution::<Vec<bool>, _>(&solution).unwrap(),
                solution
            );
        }
        let solution = BruteForce::new().solve(target).unwrap().unwrap();
        let recovered = chain.extract_solution::<Vec<bool>, _>(&solution).unwrap();
        assert_eq!(source.evaluate(&recovered).unwrap(), Min(Some(optimum)));
        assert!(chain.has_value_mapping());
        assert_eq!(
            chain
                .extract_value(serde_json::to_value(target.evaluate(&solution).unwrap()).unwrap())
                .unwrap(),
            serde_json::to_value(Min(Some(optimum))).unwrap(),
        );
        assert!(
            chain
                .extract_solution::<Vec<bool>, _>(&vec![false; input.num_vertices() + 1])
                .is_err()
        );
    }
}
