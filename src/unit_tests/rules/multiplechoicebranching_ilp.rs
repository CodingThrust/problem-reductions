use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_multiplechoicebranching_to_ilp_closed_loop() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0), (0, 2)]);
    for threshold in -2..=5 {
        let problem = MultipleChoiceBranching::new(
            graph.clone(),
            vec![2, -1, 3, 1],
            vec![vec![0, 2], vec![1, 3]],
            threshold,
        );
        let expected = BruteForce::new().solve(&problem).unwrap();
        let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
        match expected {
            Some(_) => {
                let target = ILPSolver::new().solve(reduction.target_problem()).unwrap();
                let actual = reduction.extract_solution(&target).unwrap();
                assert!(problem.evaluate(&actual).unwrap().0);
            }
            None => assert!(ILPSolver::new().solve(reduction.target_problem()).is_err()),
        }
    }
}

#[test]
fn test_multiplechoicebranching_to_ilp_rejects_forced_cycle() {
    let problem = MultipleChoiceBranching::new(
        DirectedGraph::new(2, vec![(0, 1), (1, 0)]),
        vec![1, 1],
        vec![vec![0], vec![1]],
        2,
    );
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_multiplechoicebranching_to_ilp_size() {
    let problem = MultipleChoiceBranching::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (0, 2), (2, 2)]),
        vec![1, 2, 3, 4],
        vec![vec![0, 1], vec![2, 3]],
        3,
    );
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 7);
    assert_eq!(reduction.target_problem().num_constraints(), 17);
}

#[test]
fn test_multiplechoicebranching_to_ilp_empty_graph() {
    let problem = MultipleChoiceBranching::new(DirectedGraph::new(0, vec![]), vec![], vec![], 0);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
    let target = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    assert_eq!(
        reduction.extract_solution(&target).unwrap(),
        Vec::<bool>::new()
    );
}
