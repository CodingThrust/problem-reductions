use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Canonical instance from issue #1026: path 0 - 1 - 2 with
/// edge costs c(0,1)=1, c(1,2)=6, vertex prizes p = (5, 2, 5),
/// beta = 1, omega = 2.
fn canonical_problem() -> PrizeCollectingSteinerForest<SimpleGraph, i64> {
    PrizeCollectingSteinerForest::<SimpleGraph, i64>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![5, 2, 5],
        vec![1, 6],
        1,
        2,
    )
    .unwrap()
}

#[test]
fn test_prize_collecting_steiner_forest_creation() {
    let problem = canonical_problem();
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
    assert_eq!(problem.vertex_prizes(), &[5, 2, 5]);
    assert_eq!(problem.edge_costs(), &[1, 6]);
    assert_eq!(*problem.beta(), 1);
    assert_eq!(*problem.omega(), 2);
    // n + m = 3 + 2 = 5 binary variables.
    assert_eq!(problem.dims(), vec![2; 5]);
    assert_eq!(problem.num_variables(), 5);
    assert!(problem.graph().has_edge(0, 1));
}

#[test]
fn test_prize_collecting_steiner_forest_problem_name_and_variant() {
    assert_eq!(
        <PrizeCollectingSteinerForest<SimpleGraph, i64> as Problem>::NAME,
        "PrizeCollectingSteinerForest"
    );
    let v = <PrizeCollectingSteinerForest<SimpleGraph, i64> as Problem>::variant();
    assert!(v.contains(&("graph", "SimpleGraph")));
    assert!(v.contains(&("weight", "i64")));
}

#[test]
fn test_prize_collecting_steiner_forest_evaluate_optimum() {
    // V_F = {0,1,2}, E_F = {(0,1)}: components {0,1} and {2}.
    // Objective = 1*0 + 1 + 2*2 = 5.
    let problem = canonical_problem();
    let config = vec![1, 1, 1, 1, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(5)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_prize_collecting_steiner_forest_evaluate_full_path() {
    // V_F = {0,1,2}, E_F = {(0,1),(1,2)}: single tree path.
    // Objective = 1*0 + (1+6) + 2*1 = 9.
    let problem = canonical_problem();
    let config = vec![1, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(9)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_prize_collecting_steiner_forest_evaluate_three_singletons() {
    // V_F = {0,1,2}, E_F = empty: three singleton trees.
    // Objective = 1*0 + 0 + 2*3 = 6.
    let problem = canonical_problem();
    let config = vec![1, 1, 1, 0, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(6)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_prize_collecting_steiner_forest_evaluate_empty_forest() {
    // V_F = empty, E_F = empty: kappa = 0, every prize omitted.
    // Objective = 1*(5+2+5) + 0 + 2*0 = 12.
    let problem = canonical_problem();
    let config = vec![0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(12)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_prize_collecting_steiner_forest_evaluate_edge_without_endpoint_infeasible() {
    // Select edge (0,1) but not vertex 1 -> infeasible.
    let problem = canonical_problem();
    let config = vec![1, 0, 1, 1, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
    assert!(!problem.is_valid_solution(&config));
}

#[test]
fn test_prize_collecting_steiner_forest_evaluate_cycle_infeasible() {
    // Triangle 0-1, 1-2, 0-2 with all three vertices and all three edges
    // selected forms a cycle, which is not a forest -> infeasible.
    let problem = PrizeCollectingSteinerForest::<SimpleGraph, i64>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
        vec![1, 1, 1],
        1,
        1,
    )
    .unwrap();
    let config = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
    assert!(!problem.is_valid_solution(&config));
}

#[test]
fn test_prize_collecting_steiner_forest_brute_force_solver() {
    // Brute force over 2^(3+2) = 32 configurations finds the optimum 5.
    let problem = canonical_problem();
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&problem).unwrap(), Min(Some(5)));
    let witness = solver
        .find_witness(&problem)
        .unwrap()
        .expect("witness exists");
    assert_eq!(problem.evaluate(&witness).unwrap(), Min(Some(5)));
    assert!(problem.is_valid_solution(&witness));
}

#[test]
fn test_prize_collecting_steiner_forest_serialization_roundtrip() {
    let problem = canonical_problem();
    let json = serde_json::to_value(&problem).expect("serialize");
    let restored: PrizeCollectingSteinerForest<SimpleGraph, i64> =
        serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
    assert_eq!(restored.vertex_prizes(), &[5, 2, 5]);
    assert_eq!(restored.edge_costs(), &[1, 6]);
    assert_eq!(*restored.beta(), 1);
    assert_eq!(*restored.omega(), 2);
    assert_eq!(restored.evaluate(&[1, 1, 1, 1, 0]).unwrap(), Min(Some(5)));
}

#[test]
fn test_prize_collecting_steiner_forest_f64_variant() {
    // Same canonical instance with f64 weights exercises the second registered variant.
    let problem = PrizeCollectingSteinerForest::<SimpleGraph, f64>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![5.0, 2.0, 5.0],
        vec![1.0, 6.0],
        1.0,
        2.0,
    )
    .unwrap();
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 0]).unwrap(), Min(Some(5.0)));
    assert_eq!(BruteForce::new().solve(&problem).unwrap(), Min(Some(5.0)));
}

#[test]
fn test_prize_collecting_steiner_forest_rejects_vertex_prizes_length_mismatch() {
    let error = PrizeCollectingSteinerForest::<SimpleGraph, i64>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![5, 2], // length 2 != 3 vertices
        vec![1, 6],
        1,
        2,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::Conversion(message)
            if message == "vertex_prizes length must match graph num_vertices"
    ));
}

#[test]
fn test_prize_collecting_steiner_forest_rejects_edge_costs_length_mismatch() {
    let error = PrizeCollectingSteinerForest::<SimpleGraph, i64>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![5, 2, 5],
        vec![1, 6, 2], // length 3 != 2 edges
        1,
        2,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::Conversion(message)
            if message == "edge_costs length must match graph num_edges"
    ));
}

#[test]
fn test_prize_collecting_steiner_forest_rejects_non_finite_weight() {
    assert!(PrizeCollectingSteinerForest::<SimpleGraph, f64>::new(
        SimpleGraph::new(1, vec![]),
        vec![f64::NAN],
        vec![],
        1.0,
        1.0,
    )
    .is_err());
}
#[test]
fn create_specs_default_prizes_and_costs_to_one() {
    let weighted =
        PrizeCollectingSteinerForest::try_from(PrizeCollectingSteinerForestI64CreateSpec {
            graph: vec![(0, 1)],
            num_vertices: Some(3),
            vertex_prizes: None,
            edge_costs: None,
            beta: 2,
            omega: 3,
        })
        .unwrap();
    let floating =
        PrizeCollectingSteinerForest::try_from(PrizeCollectingSteinerForestF64CreateSpec {
            graph: vec![(0, 1)],
            num_vertices: None,
            vertex_prizes: None,
            edge_costs: None,
            beta: 2.0,
            omega: 3.0,
        })
        .unwrap();
    assert_eq!(weighted.vertex_prizes(), &[1, 1, 1]);
    assert_eq!(weighted.edge_costs(), &[1]);
    assert_eq!(floating.vertex_prizes(), &[1.0, 1.0]);
    assert_eq!(floating.edge_costs(), &[1.0]);
    assert!(!PrizeCollectingSteinerForestI64CreateSpec::INPUTS[2].required);
    assert!(!PrizeCollectingSteinerForestI64CreateSpec::INPUTS[3].required);
}
