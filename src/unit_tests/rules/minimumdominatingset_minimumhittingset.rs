use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::traits::Problem;

fn source(
    num_vertices: usize,
    edges: Vec<(usize, usize)>,
) -> MinimumDominatingSet<SimpleGraph, One> {
    MinimumDominatingSet::new(
        SimpleGraph::new(num_vertices, edges),
        vec![One; num_vertices],
    )
}

fn assert_config_equivalent(
    problem: &MinimumDominatingSet<SimpleGraph, One>,
    target: &MinimumHittingSet,
    config: &[usize],
) {
    let selected = config.iter().sum::<usize>();
    let source_value = problem.evaluate(config).0.map(|value| value as usize);
    let target_value = target.evaluate(config).0;
    let expected = problem.is_valid_solution(config).then_some(selected);

    assert_eq!(source_value, expected);
    assert_eq!(target_value, expected);
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_closed_loop() {
    let problem = source(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "MinimumDominatingSet(One)->MinimumHittingSet closed loop",
    );
}

#[test]
fn test_closed_neighborhood_structure() {
    let problem = source(5, vec![(0, 2), (0, 1), (1, 3), (3, 4)]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&problem);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 5);
    assert_eq!(target.num_sets(), 5);
    assert_eq!(
        target.sets(),
        &[
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 2],
            vec![1, 3, 4],
            vec![3, 4],
        ]
    );
}

#[test]
fn test_identity_extraction() {
    let problem = source(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&problem);

    assert_eq!(reduction.extract_solution(&[0, 1, 0]), vec![0, 1, 0]);
}

#[test]
fn test_empty_graph() {
    let problem = source(0, vec![]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&problem);

    assert_eq!(reduction.target_problem().universe_size(), 0);
    assert!(reduction.target_problem().sets().is_empty());
    assert_config_equivalent(&problem, reduction.target_problem(), &[]);
}

#[test]
fn test_isolated_and_disconnected_vertices() {
    let problem = source(5, vec![(0, 1), (2, 3)]);
    let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&problem);

    assert_eq!(
        reduction.target_problem().sets(),
        &[vec![0, 1], vec![0, 1], vec![2, 3], vec![2, 3], vec![4],]
    );
    assert_config_equivalent(&problem, reduction.target_problem(), &[1, 0, 1, 0, 1]);
}

#[test]
fn test_star_and_complete_graphs() {
    let star = source(4, vec![(0, 1), (0, 2), (0, 3)]);
    let star_reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&star);
    assert_config_equivalent(&star, star_reduction.target_problem(), &[1, 0, 0, 0]);

    let complete = source(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let complete_reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&complete);
    assert!(complete_reduction
        .target_problem()
        .sets()
        .iter()
        .all(|set| set == &vec![0, 1, 2, 3]));
    assert_config_equivalent(
        &complete,
        complete_reduction.target_problem(),
        &[0, 0, 1, 0],
    );
}

#[test]
fn test_exhaustive_equivalence_through_four_vertices() {
    for num_vertices in 0..=4 {
        let possible_edges: Vec<_> = (0..num_vertices)
            .flat_map(|u| ((u + 1)..num_vertices).map(move |v| (u, v)))
            .collect();

        for graph_mask in 0..(1_usize << possible_edges.len()) {
            let edges = possible_edges
                .iter()
                .enumerate()
                .filter_map(|(index, &edge)| ((graph_mask >> index) & 1 == 1).then_some(edge))
                .collect();
            let problem = source(num_vertices, edges);
            let reduction = ReduceTo::<MinimumHittingSet>::reduce_to(&problem);

            assert_eq!(reduction.target_problem().universe_size(), num_vertices);
            assert_eq!(reduction.target_problem().num_sets(), num_vertices);

            for config_mask in 0..(1_usize << num_vertices) {
                let config: Vec<_> = (0..num_vertices)
                    .map(|vertex| (config_mask >> vertex) & 1)
                    .collect();
                assert_config_equivalent(&problem, reduction.target_problem(), &config);
            }
        }
    }
}
