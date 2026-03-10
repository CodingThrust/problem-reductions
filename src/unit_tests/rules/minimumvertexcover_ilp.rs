use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MinimumVertexCover;
use crate::rules::{MinimizeSteps, ReductionChain, ReductionGraph, ReductionPath};
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{ProblemSize, SolutionSize};

fn reduce_vc_to_ilp(
    problem: &MinimumVertexCover<SimpleGraph, i32>,
) -> (ReductionPath, ReductionChain) {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_cheapest_path(
            "MinimumVertexCover",
            &src,
            "ILP",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .expect("Should find path MinimumVertexCover -> ILP");
    let chain = graph
        .reduce_along_path(&path, problem as &dyn std::any::Any)
        .expect("Should reduce MinimumVertexCover to ILP along path");
    (path, chain)
}

#[test]
fn test_minimumvertexcover_to_ilp_via_path_structure() {
    let problem = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    let (path, chain) = reduce_vc_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();

    assert!(
        path.len() > 1,
        "Removed rule should be exercised through a multi-step path"
    );
    assert_eq!(
        path.type_names(),
        vec!["MinimumVertexCover", "MinimumSetCovering", "ILP"]
    );
    assert_eq!(ilp.num_vars, 3);
    assert_eq!(ilp.constraints.len(), 3);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumvertexcover_to_ilp_via_path_closed_loop() {
    let problem = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let (_, chain) = reduce_vc_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let bf_solutions = bf.find_all_best(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = chain.extract_solution(&ilp_solution);

    let bf_size: usize = bf_solutions[0].iter().sum();
    let ilp_size: usize = extracted.iter().sum();
    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimumvertexcover_to_ilp_via_path_weighted() {
    let problem =
        MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![100, 1, 100]);
    let (_, chain) = reduce_vc_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = chain.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(1));
    assert_eq!(extracted, vec![0, 1, 0]);
}
