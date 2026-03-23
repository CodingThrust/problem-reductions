use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn feasible_instance() -> UndirectedFlowLowerBounds {
    // 3-vertex path: edges (0,1) cap=2 lower=1, (1,2) cap=2 lower=1
    // source=0, sink=2, requirement=1
    UndirectedFlowLowerBounds::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![2, 2],
        vec![1, 1],
        0,
        2,
        1,
    )
}

fn infeasible_instance() -> UndirectedFlowLowerBounds {
    // 3-vertex path: edges (0,1) cap=2 lower=2, (1,2) cap=1 lower=0
    // source=0, sink=2, requirement=2: need 2 units but edge (1,2) cap=1 limits to 1
    UndirectedFlowLowerBounds::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![2, 1],
        vec![0, 0],
        0,
        2,
        2,
    )
}

#[test]
fn test_undirectedflowlowerbounds_to_ilp_structure() {
    let problem = feasible_instance();
    let reduction: ReductionUFLBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 2 edges → 3*2 = 6 variables
    assert_eq!(ilp.num_vars, 6);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_undirectedflowlowerbounds_to_ilp_closed_loop() {
    let problem = feasible_instance();
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("feasible instance has a witness");
    assert!(
        problem.evaluate(&bf_solution).0,
        "brute force solution is valid"
    );

    let reduction: ReductionUFLBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    // extract_solution returns edge orientations z_e
    assert_eq!(extracted.len(), 2);
    assert!(
        problem.evaluate(&extracted).0,
        "ILP extracted orientation should be a valid flow"
    );
}

#[test]
fn test_undirectedflowlowerbounds_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionUFLBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible instance should produce infeasible ILP"
    );
}

#[test]
fn test_undirectedflowlowerbounds_to_ilp_extract_solution() {
    let problem = feasible_instance();
    let reduction: ReductionUFLBToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // f_{01}=1, f_{10}=0, f_{12}=1, f_{21}=0, z_0=1, z_1=1
    // z_e=1 means u→v direction; model expects config[e]=0 for u→v → extract returns 1-z_e
    let target_solution = vec![1, 0, 1, 0, 1, 1];
    let extracted = reduction.extract_solution(&target_solution);
    // z_0=1, z_1=1 → extracted = [1-1, 1-1] = [0, 0] (both u→v = 0→1 and 1→2)
    assert_eq!(extracted, vec![0, 0]);
    assert!(
        problem.evaluate(&extracted).0,
        "manually extracted orientation should be valid"
    );
}
