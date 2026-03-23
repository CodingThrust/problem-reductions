use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn feasible_instance() -> UndirectedTwoCommodityIntegralFlow {
    // 4-vertex graph: edges (0,2),(1,2),(2,3); capacities [1,1,2]
    // s1=0, t1=3, s2=1, t2=3, R1=1, R2=1
    UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
        vec![1, 1, 2],
        0,
        3,
        1,
        3,
        1,
        1,
    )
}

fn infeasible_instance() -> UndirectedTwoCommodityIntegralFlow {
    // Same topology but requirements that can't be met simultaneously
    // path graph: 0-1-2; cap=1 everywhere; s1=0,t1=2 req=1; s2=0,t2=2 req=1
    // Total demand = 2 on edge (0,1) but cap = 1 → infeasible
    UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1],
        0,
        2,
        0,
        2,
        1,
        1,
    )
}

#[test]
fn test_undirectedtwocommodityintegralflow_to_ilp_structure() {
    let problem = feasible_instance();
    let reduction: ReductionU2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3 edges → 8*3 = 24 variables
    assert_eq!(ilp.num_vars, 24);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_undirectedtwocommodityintegralflow_to_ilp_closed_loop() {
    let problem = feasible_instance();
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("feasible instance has a witness");
    assert!(problem.evaluate(&bf_solution).0, "brute force solution is valid");

    let reduction: ReductionU2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(
        problem.evaluate(&extracted).0,
        "ILP extracted solution should be a valid flow"
    );
}

#[test]
fn test_undirectedtwocommodityintegralflow_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionU2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible flow instance should yield infeasible ILP"
    );
}

#[test]
fn test_undirectedtwocommodityintegralflow_to_ilp_extract_solution() {
    let problem = feasible_instance();
    let reduction: ReductionU2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Manual solution: edge 0 (0,2): f1_uv=1, f1_vu=0, f2_uv=0, f2_vu=0
    // edge 1 (1,2): f1_uv=0, f1_vu=0, f2_uv=1, f2_vu=0
    // edge 2 (2,3): f1_uv=1, f1_vu=0, f2_uv=1, f2_vu=0
    // directions: d1_0=1,d2_0=0, d1_1=0,d2_1=1, d1_2=1,d2_2=1
    let target_solution = vec![
        1, 0, 0, 0,  // edge 0 flows
        0, 0, 1, 0,  // edge 1 flows
        1, 0, 1, 0,  // edge 2 flows
        1, 0,        // d1_0=1, d2_0=0
        0, 1,        // d1_1=0, d2_1=1
        1, 1,        // d1_2=1, d2_2=1
    ];
    let extracted = reduction.extract_solution(&target_solution);
    // extract_solution returns first 4*3=12 flow variables
    assert_eq!(extracted.len(), 12);
    assert!(
        problem.evaluate(&extracted).0,
        "manually extracted solution should be valid"
    );
}
