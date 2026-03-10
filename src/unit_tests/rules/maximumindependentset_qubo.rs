use crate::models::algebraic::QUBO;
use crate::models::graph::MaximumIndependentSet;
use crate::rules::{Minimize, ReductionChain, ReductionGraph, ReductionPath};
use crate::solvers::{BruteForce, Solver};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{ProblemSize, SolutionSize};

fn reduce_mis_to_qubo(
    problem: &MaximumIndependentSet<SimpleGraph, i32>,
) -> (ReductionPath, ReductionChain) {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "QUBO",
            &dst,
            &ProblemSize::new(vec![
                ("num_vertices", problem.graph().num_vertices()),
                ("num_edges", problem.graph().num_edges()),
            ]),
            &Minimize("num_vars"),
        )
        .expect("Should find path MaximumIndependentSet -> QUBO");
    let chain = graph
        .reduce_along_path(&path, problem as &dyn std::any::Any)
        .expect("Should reduce MaximumIndependentSet to QUBO along path");
    (path, chain)
}

#[test]
fn test_maximumindependentset_to_qubo_via_path_closed_loop() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let (path, chain) = reduce_mis_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    assert!(
        path.len() > 1,
        "Removed rule should be exercised through a multi-step path"
    );
    assert_eq!(
        path.type_names(),
        vec!["MaximumIndependentSet", "MaximumSetPacking", "QUBO"]
    );
    assert_eq!(qubo.num_variables(), 4);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);
    for sol in &qubo_solutions {
        let extracted = chain.extract_solution(sol);
        assert!(problem.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_maximumindependentset_to_qubo_via_path_weighted() {
    let problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 100, 1]);
    let (_, chain) = reduce_mis_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    let solver = BruteForce::new();
    let qubo_solution = solver
        .find_best(qubo)
        .expect("QUBO should be solvable via path");
    let extracted = chain.extract_solution(&qubo_solution);

    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(100));
    assert_eq!(extracted, vec![0, 1, 0]);
}

#[test]
fn test_maximumindependentset_to_qubo_via_path_empty_graph() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
    let (_, chain) = reduce_mis_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    assert_eq!(qubo.num_variables(), 3);

    let solver = BruteForce::new();
    let qubo_solution = solver.find_best(qubo).expect("QUBO should be solvable");
    let extracted = chain.extract_solution(&qubo_solution);

    assert_eq!(extracted, vec![1, 1, 1]);
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(3));
}
