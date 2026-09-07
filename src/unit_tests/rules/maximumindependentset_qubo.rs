use crate::models::algebraic::QUBO;
use crate::models::graph::MaximumIndependentSet;
use crate::rules::{ReductionChain, ReductionGraph, ReductionPath};
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn reduce_mis_to_qubo(
    problem: &MaximumIndependentSet<SimpleGraph, i64>,
) -> (ReductionPath, ReductionChain) {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "QUBO", &dst)
        .into_iter()
        .find(|path| path.type_names() == ["MaximumIndependentSet", "MaximumSetPacking", "QUBO"])
        .expect("expected explicit MaximumSetPacking route");
    let chain = graph
        .reduce_along_path(&path, problem as &dyn std::any::Any)
        .expect("MaximumIndependentSet -> QUBO reduction should not fail")
        .expect("Should reduce MaximumIndependentSet to QUBO along path");
    (path, chain)
}

#[test]
fn test_maximumindependentset_to_qubo_via_path_closed_loop() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i64; 4],
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
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();
    for sol in &qubo_solutions {
        let extracted = chain.extract_solution(sol).unwrap();
        assert!(problem.evaluate(&extracted).unwrap().is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x).count(), 2);
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
        .solve(qubo)
        .unwrap()
        .expect("QUBO should be solvable via path");
    let extracted = chain.extract_solution(&qubo_solution).unwrap();

    assert_eq!(problem.evaluate(&extracted).unwrap(), Max(Some(100)));
    assert_eq!(extracted, vec![false, true, false]);
}

#[test]
fn test_maximumindependentset_to_qubo_via_path_empty_graph() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![1i64; 3]);
    let (_, chain) = reduce_mis_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    assert_eq!(qubo.num_variables(), 3);

    let solver = BruteForce::new();
    let qubo_solution = solver
        .solve(qubo)
        .unwrap()
        .expect("QUBO should be solvable");
    let extracted = chain.extract_solution(&qubo_solution).unwrap();

    assert_eq!(extracted, vec![true, true, true]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Max(Some(3)));
}
