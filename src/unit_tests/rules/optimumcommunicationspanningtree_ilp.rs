use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

fn k3_problem() -> OptimumCommunicationSpanningTree {
    let edge_weights = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
    let requirements = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    OptimumCommunicationSpanningTree::new(edge_weights, requirements)
}

fn k4_problem() -> OptimumCommunicationSpanningTree {
    let edge_weights = vec![
        vec![0, 1, 3, 2],
        vec![1, 0, 2, 4],
        vec![3, 2, 0, 1],
        vec![2, 4, 1, 0],
    ];
    let requirements = vec![
        vec![0, 2, 1, 3],
        vec![2, 0, 1, 1],
        vec![1, 1, 0, 2],
        vec![3, 1, 2, 0],
    ];
    OptimumCommunicationSpanningTree::new(edge_weights, requirements)
}

#[test]
fn test_ocst_to_ilp_closed_loop_k3() {
    let problem = k3_problem();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "OptimumCommunicationSpanningTree->ILP K3 closed loop",
    );
}

#[test]
fn test_ocst_to_ilp_structure_k4() {
    let problem = k4_problem();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // K4: n=4, m=6, 6 commodities (all pairs have r>0)
    // num_vars = 6 + 2*6*6 = 78
    assert_eq!(ilp.num_vars(), 78);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // Constraints: 1 (tree size) + 4*6 (flow conservation) + 2*6*6 (capacity) = 1+24+72 = 97
    assert_eq!(ilp.num_constraints(), 97);
}

#[test]
fn test_ocst_to_ilp_structure_k3() {
    let problem = k3_problem();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // K3: n=3, m=3, 3 commodities (all pairs have r>0)
    // num_vars = 3 + 2*3*3 = 21
    assert_eq!(ilp.num_vars(), 21);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // Constraints: 1 (tree size) + 3*3 (flow conservation) + 2*3*3 (capacity) = 1+9+18 = 28
    assert_eq!(ilp.num_constraints(), 28);
}

#[test]
fn test_ocst_to_ilp_bf_vs_ilp_k3() {
    let problem = k3_problem();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_solutions = BruteForce::new().find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert!(ilp_value.is_valid());
    assert_eq!(ilp_value, Min(Some(6)));
}

#[test]
fn test_ocst_to_ilp_bf_vs_ilp_k4() {
    let problem = k4_problem();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_solutions = BruteForce::new().find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert!(ilp_value.is_valid());
    assert_eq!(ilp_value, Min(Some(20)));
}

#[test]
fn test_ocst_to_ilp_extraction() {
    let problem = k3_problem();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Should be a valid config with m=3 entries
    assert_eq!(extracted.len(), 3);
    // Should form a valid spanning tree with value 6
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());
    assert_eq!(value, Min(Some(6)));
}

#[cfg(feature = "example-db")]
#[test]
fn test_ocst_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "optimum_communication_spanning_tree_to_ilp")
        .expect("missing canonical OCST -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "OptimumCommunicationSpanningTree");
    assert_eq!(example.target.problem, "ILP");
    assert!(!example.solutions.is_empty());
}
