use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Single triangle: 3 vertices, 3 edges, q=1 group
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = PartitionIntoTriangles::new(graph);
    let reduction: ReductionPITToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_vars = 3 vertices * 1 group = 3
    assert_eq!(ilp.num_vars, 3, "Should have 3 variables");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize (feasibility)");
    // Constraints: 3 assignment + 1 group-size = 4
    // Non-edges: none (complete triangle), so no triangle constraints
    assert_eq!(ilp.constraints.len(), 4, "Should have 4 constraints");
}

#[test]
fn test_partitionintotriangles_to_ilp_bf_vs_ilp() {
    // Two triangles: vertices {0,1,2} and {3,4,5}
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5)],
    );
    let problem = PartitionIntoTriangles::new(graph);
    let reduction: ReductionPITToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("BF should find a solution");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "Extracted ILP solution should be valid"
    );
}

#[test]
fn test_solution_extraction() {
    // Two triangles: 6 vertices, q=2 groups
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5)],
    );
    let problem = PartitionIntoTriangles::new(graph);
    let reduction: ReductionPITToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // x_{v,g}: v0g0=1,v0g1=0, v1g0=1,v1g1=0, v2g0=1,v2g1=0,
    //           v3g0=0,v3g1=1, v4g0=0,v4g1=1, v5g0=0,v5g1=1
    let ilp_solution = vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0, 0, 1, 1, 1]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_partitionintotriangles_to_ilp_trivial() {
    // Minimal: single triangle
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = PartitionIntoTriangles::new(graph);
    let reduction: ReductionPITToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
