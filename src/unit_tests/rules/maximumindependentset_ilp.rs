use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MaximumIndependentSet;
use crate::rules::{ReductionChain, ReductionGraph, ReductionPath};
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn reduce_mis_to_ilp(
    problem: &MaximumIndependentSet<SimpleGraph, i64>,
) -> (ReductionPath, ReductionChain) {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "ILP", &dst)
        .into_iter()
        .find(|path| path.type_names() == ["MaximumIndependentSet", "MaximumSetPacking", "ILP"])
        .expect("expected explicit MaximumSetPacking route");
    let chain = graph
        .reduce_along_path(&path, problem as &dyn std::any::Any)
        .expect("MaximumIndependentSet -> ILP reduction should not fail")
        .expect("Should reduce MaximumIndependentSet to ILP along path");
    (path, chain)
}

#[test]
fn test_maximumindependentset_to_ilp_via_path_structure() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i64; 3],
    );
    let (path, chain) = reduce_mis_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();

    assert!(
        path.len() > 1,
        "Removed rule should be exercised through a multi-step path"
    );
    let names = path.type_names();
    assert!(
        names == vec!["MaximumIndependentSet", "MaximumClique", "ILP"]
            || names == vec!["MaximumIndependentSet", "MaximumSetPacking", "ILP"],
        "Expected 2-step path through MaxClique or MaxSetPacking, got {:?}",
        names
    );
    assert_eq!(ilp.num_vars(), 3);
    assert_eq!(ilp.constraints().len(), 3);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
}

#[test]
fn test_maximumindependentset_to_ilp_via_path_closed_loop() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i64; 4],
    );
    let (_, chain) = reduce_mis_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted: Vec<bool> = chain.extract_solution(&ilp_solution).unwrap();

    let ilp_size = extracted.iter().filter(|&&selected| selected).count();
    assert_eq!(ilp_size, 2);
    assert!(problem.evaluate(&extracted).unwrap().is_valid());
}

#[test]
fn test_maximumindependentset_to_ilp_via_path_weighted() {
    let problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 100, 1]);
    let (_, chain) = reduce_mis_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = chain.extract_solution(&ilp_solution).unwrap();

    assert_eq!(problem.evaluate(&extracted).unwrap(), Max(Some(100)));
    assert_eq!(extracted, vec![false, true, false]);
}

#[test]
fn test_maximumindependentset_to_ilp_bf_vs_ilp() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i64; 4],
    );
    let (_, chain) = reduce_mis_to_ilp(&problem);
    let ilp: &ILP<bool> = chain.target_problem();
    let bf_value_solution = BruteForce::new().solve(&problem).unwrap().unwrap();
    let bf_value = problem.evaluate(&bf_value_solution).unwrap();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = chain.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), bf_value);
}
