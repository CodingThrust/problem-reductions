use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{ObjectiveProblem, Problem};
use crate::types::{ExtremumSense, Min};

fn k5_btsp() -> BottleneckTravelingSalesman {
    BottleneckTravelingSalesman::new(
        SimpleGraph::new(
            5,
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 4),
                (3, 4),
            ],
        ),
        vec![5, 4, 4, 5, 4, 1, 2, 1, 5, 4],
    )
}

#[test]
fn test_bottleneck_traveling_salesman_creation_and_size_getters() {
    let mut problem = k5_btsp();

    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 10);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.dims(), vec![2; 10]);
    assert_eq!(problem.num_variables(), 10);
    assert_eq!(problem.weights(), vec![5, 4, 4, 5, 4, 1, 2, 1, 5, 4]);
    assert_eq!(
        problem.edges(),
        vec![
            (0, 1, 5),
            (0, 2, 4),
            (0, 3, 4),
            (0, 4, 5),
            (1, 2, 4),
            (1, 3, 1),
            (1, 4, 2),
            (2, 3, 1),
            (2, 4, 5),
            (3, 4, 4),
        ]
    );
    assert!(problem.is_weighted());

    problem.set_weights(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(problem.weights(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_bottleneck_traveling_salesman_evaluate_valid_and_invalid() {
    let problem = k5_btsp();

    let valid_cycle = vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 1];
    assert!(problem.is_valid_solution(&valid_cycle));
    assert_eq!(problem.evaluate(&valid_cycle), Min(Some(4)));

    let degree_violation = vec![1, 1, 1, 0, 1, 0, 1, 0, 0, 1];
    assert!(!problem.is_valid_solution(&degree_violation));
    assert_eq!(problem.evaluate(&degree_violation), Min(None));
}

#[test]
fn test_bottleneck_traveling_salesman_evaluate_disconnected_subtour_invalid() {
    let problem = BottleneckTravelingSalesman::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
        vec![1, 1, 1, 2, 2, 2],
    );

    let disconnected_subtour = vec![1, 1, 1, 1, 1, 1];
    assert!(!problem.is_valid_solution(&disconnected_subtour));
    assert_eq!(problem.evaluate(&disconnected_subtour), Min(None));
}

#[test]
fn test_bottleneck_traveling_salesman_direction() {
    let problem = BottleneckTravelingSalesman::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![7, 4, 6],
    );

    assert_eq!(problem.direction(), ExtremumSense::Minimize);
}

#[test]
fn test_bottleneck_traveling_salesman_bruteforce_unique_optimum() {
    let problem = k5_btsp();
    let solver = BruteForce::new();
    let best = solver.find_all_witnesses(&problem);

    assert_eq!(best, vec![vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 1]]);
    assert_eq!(problem.evaluate(&best[0]), Min(Some(4)));
}

#[test]
fn test_bottleneck_traveling_salesman_serialization() {
    let problem = k5_btsp();

    let json = serde_json::to_string(&problem).unwrap();
    let restored: BottleneckTravelingSalesman = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.graph(), problem.graph());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(
        restored.evaluate(&[0, 1, 1, 0, 1, 0, 1, 0, 0, 1]),
        Min(Some(4))
    );
}

#[test]
fn test_bottleneck_traveling_salesman_paper_example() {
    let problem = k5_btsp();
    let config = vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 1];

    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config), Min(Some(4)));

    let solver = BruteForce::new();
    let best = solver.find_all_witnesses(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], config);
}
