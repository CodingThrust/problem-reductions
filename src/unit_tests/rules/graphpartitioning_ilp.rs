use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense};
use crate::models::graph::GraphPartitioning;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::SolutionSize;

fn canonical_instance() -> GraphPartitioning<SimpleGraph> {
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
            (3, 5),
            (4, 5),
        ],
    );
    GraphPartitioning::new(graph)
}

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = canonical_instance();
    let reduction: ReductionGraphPartitioningToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 15);
    assert_eq!(ilp.constraints.len(), 19);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert_eq!(
        ilp.objective,
        vec![
            (6, 1.0),
            (7, 1.0),
            (8, 1.0),
            (9, 1.0),
            (10, 1.0),
            (11, 1.0),
            (12, 1.0),
            (13, 1.0),
            (14, 1.0),
        ]
    );
}

#[test]
fn test_reduction_constraint_shape() {
    let problem = GraphPartitioning::new(SimpleGraph::new(2, vec![(0, 1)]));
    let reduction: ReductionGraphPartitioningToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 3);
    assert_eq!(ilp.constraints.len(), 3);

    let balance = &ilp.constraints[0];
    assert_eq!(balance.cmp, Comparison::Eq);
    assert_eq!(balance.terms, vec![(0, 1.0), (1, 1.0)]);
    assert_eq!(balance.rhs, 1.0);

    let first_link = &ilp.constraints[1];
    assert_eq!(first_link.cmp, Comparison::Ge);
    assert_eq!(first_link.terms, vec![(2, 1.0), (0, -1.0), (1, 1.0)]);
    assert_eq!(first_link.rhs, 0.0);

    let second_link = &ilp.constraints[2];
    assert_eq!(second_link.cmp, Comparison::Ge);
    assert_eq!(second_link.terms, vec![(2, 1.0), (0, 1.0), (1, -1.0)]);
    assert_eq!(second_link.rhs, 0.0);
}

#[test]
fn test_graphpartitioning_to_ilp_closed_loop() {
    let problem = canonical_instance();
    let reduction: ReductionGraphPartitioningToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_best(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    assert_eq!(bf_obj, SolutionSize::Valid(3));
    assert_eq!(ilp_obj, SolutionSize::Valid(3));
}

#[test]
fn test_odd_vertices_reduce_to_infeasible_ilp() {
    let problem = GraphPartitioning::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionGraphPartitioningToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints[0].cmp, Comparison::Eq);
    assert_eq!(ilp.constraints[0].rhs, 1.5);

    let solver = ILPSolver::new();
    assert_eq!(solver.solve(ilp), None);
}

#[test]
fn test_solution_extraction() {
    let problem = canonical_instance();
    let reduction: ReductionGraphPartitioningToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = vec![0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0];
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![0, 0, 0, 1, 1, 1]);
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(3));
}

#[test]
fn test_solve_reduced() {
    let problem = canonical_instance();

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(3));
}
