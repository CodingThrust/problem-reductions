use super::*;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_partitionintocliques_to_ilp_size() {
    let problem = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();

    assert_eq!(reduction.target_problem().num_vars(), 6);
    assert_eq!(reduction.target_problem().constraints().len(), 7);
}

#[test]
fn test_partitionintocliques_to_ilp_closed_loop() {
    let problem = PartitionIntoCliques::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();
    let target_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("two disjoint edges form two cliques");
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert_eq!(problem.evaluate(&source_solution).unwrap(), Or(true));
}

#[test]
fn test_partitionintocliques_to_ilp_preserves_infeasibility() {
    let problem = PartitionIntoCliques::new(SimpleGraph::new(3, vec![]), 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();

    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}
