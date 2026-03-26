use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Two P3 paths: 0-1-2 and 3-4-5
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);
    let reduction: ReductionPIPL2ToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=6, q=2, num_edges=4
    // num_vars = 6*2 + 4*2 = 12 + 8 = 20
    assert_eq!(ilp.num_vars, 20, "Should have 20 variables");
    assert_eq!(
        ilp.sense,
        ObjectiveSense::Minimize,
        "Should minimize (feasibility)"
    );
    // Constraints: 6 assignment + 2 group-size + 4*2*3 McCormick + 2 edge count = 6+2+24+2=34
    assert_eq!(ilp.constraints.len(), 34, "Should have 34 constraints");
}

#[test]
fn test_partitionintopathsoflength2_to_ilp_bf_vs_ilp() {
    // Two P3 paths: 0-1-2 and 3-4-5
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);
    let reduction: ReductionPIPL2ToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf
        .find_witness(&problem)
        .expect("BF should find a solution");
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
    // Two P3 paths: 0-1-2 and 3-4-5
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);
    let reduction: ReductionPIPL2ToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // x vars (12): group 0 gets 0,1,2; group 1 gets 3,4,5
    // x_{0,0}=1,x_{0,1}=0, x_{1,0}=1,x_{1,1}=0, x_{2,0}=1,x_{2,1}=0,
    // x_{3,0}=0,x_{3,1}=1, x_{4,0}=0,x_{4,1}=1, x_{5,0}=0,x_{5,1}=1
    // y vars (8): e0=(0,1): y_{0,0}=1,y_{0,1}=0; e1=(1,2): y_{1,0}=1,y_{1,1}=0;
    //              e2=(3,4): y_{2,0}=0,y_{2,1}=1; e3=(4,5): y_{3,0}=0,y_{3,1}=1
    let ilp_solution = vec![
        1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, // x vars
        1, 0, 1, 0, 0, 1, 0, 1, // y vars
    ];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0, 0, 1, 1, 1]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_partitionintopathsoflength2_to_ilp_trivial() {
    // Minimal feasible: one P3 path 0-1-2
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);
    let reduction: ReductionPIPL2ToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "Single P3 should be feasible"
    );
}
