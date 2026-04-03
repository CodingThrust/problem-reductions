use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;

#[test]
fn test_partitionintocliques_creation() {
    use crate::traits::Problem;

    let problem = PartitionIntoCliques::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2)]), 3);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_cliques(), 3);
    assert_eq!(problem.dims(), vec![3; 4]);
}

#[test]
fn test_partitionintocliques_evaluate_valid_and_invalid() {
    use crate::traits::Problem;

    let graph = SimpleGraph::new(5, vec![(0, 3), (1, 2)]);
    let problem = PartitionIntoCliques::new(graph, 3);

    assert!(problem.evaluate(&[0, 1, 1, 0, 2]));
    assert!(!problem.evaluate(&[0, 1, 0, 1, 2]));
    assert!(!problem.evaluate(&[0, 1, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 1, 0, 3]));
}

#[test]
fn test_partitionintocliques_solver() {
    use crate::traits::Problem;

    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = PartitionIntoCliques::new(graph, 2);
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    assert!(problem.evaluate(&witness.unwrap()));
}

#[test]
fn test_partitionintocliques_serialization() {
    let problem = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PartitionIntoCliques<SimpleGraph> = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_edges(), 1);
    assert_eq!(deserialized.num_cliques(), 2);
}

#[test]
fn test_partitionintocliques_paper_example() {
    use crate::traits::Problem;

    let graph = SimpleGraph::new(5, vec![(0, 3), (0, 4), (1, 2), (1, 4)]);
    let problem = PartitionIntoCliques::new(graph, 3);
    let config = vec![0, 1, 1, 0, 2];

    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
}
