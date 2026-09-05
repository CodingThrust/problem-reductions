use crate::models::algebraic::QUBO;
use crate::models::graph::MinimumVertexCover;
use crate::rules::{ReductionChain, ReductionGraph, ReductionPath};
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

fn reduce_vc_to_qubo(
    problem: &MinimumVertexCover<SimpleGraph, i64>,
) -> (ReductionPath, ReductionChain) {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph
        .find_all_paths("MinimumVertexCover", &src, "QUBO", &dst)
        .into_iter()
        .find(|path| {
            path.type_names()
                == [
                    "MinimumVertexCover",
                    "MaximumIndependentSet",
                    "MaximumSetPacking",
                    "QUBO",
                ]
        })
        .expect("expected explicit MaximumIndependentSet route");
    let chain = graph
        .reduce_along_path(&path, problem as &dyn std::any::Any)
        .expect("MinimumVertexCover -> QUBO reduction should not fail")
        .expect("Should reduce MinimumVertexCover to QUBO along path");
    (path, chain)
}

#[test]
fn test_minimumvertexcover_to_qubo_via_path_closed_loop() {
    let problem = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
        vec![1i64; 4],
    );
    let (path, chain) = reduce_vc_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    assert!(
        path.len() > 1,
        "Removed rule should be exercised through a multi-step path"
    );
    assert_eq!(
        path.type_names(),
        vec![
            "MinimumVertexCover",
            "MaximumIndependentSet",
            "MaximumSetPacking",
            "QUBO",
        ]
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
fn test_minimumvertexcover_to_qubo_via_path_weighted() {
    let problem =
        MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![100, 1, 100]);
    let (_, chain) = reduce_vc_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    let solver = BruteForce::new();
    let qubo_solution = solver
        .solve(qubo)
        .unwrap()
        .expect("QUBO should be solvable via path");
    let extracted = chain.extract_solution(&qubo_solution).unwrap();

    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(1)));
    assert_eq!(extracted, vec![false, true, false]);
}

#[test]
fn test_minimumvertexcover_to_qubo_via_path_star_graph() {
    let problem = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i64; 4],
    );
    let (_, chain) = reduce_vc_to_qubo(&problem);
    let qubo: &QUBO<f64> = chain.target_problem();

    assert_eq!(qubo.num_variables(), 4);

    let solver = BruteForce::new();
    let qubo_solution = solver
        .solve(qubo)
        .unwrap()
        .expect("QUBO should be solvable");
    let extracted = chain.extract_solution(&qubo_solution).unwrap();

    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(1)));
    assert_eq!(extracted.iter().filter(|&&x| x).count(), 1);
}
