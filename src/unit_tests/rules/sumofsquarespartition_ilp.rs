use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3 elements, 2 groups
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2, 20);
    let reduction: ReductionSSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3, K=2: num_vars = 3*2 + 3^2*2 = 6 + 18 = 24
    assert_eq!(ilp.num_vars, 24, "Should have 24 variables (3*2 + 9*2)");
    // num_constraints = 3 assignment + 3*9*2 McCormick + 1 bound = 3 + 54 + 1 = 58
    assert_eq!(ilp.constraints.len(), 58, "Should have 58 constraints");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize (feasibility)");
}

#[test]
fn test_sumofsquarespartition_to_ilp_bf_vs_ilp() {
    // 4 elements [1,2,3,4], 2 groups, bound=50
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3, 4], 2, 50);

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("BF should find a solution");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let reduction: ReductionSSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
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
    // 4 elements, 2 groups
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3, 4], 2, 50);
    let reduction: ReductionSSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // element 0→g0, element 1→g1, element 2→g1, element 3→g0
    // x_{0,0}=1,x_{0,1}=0, x_{1,0}=0,x_{1,1}=1, x_{2,0}=0,x_{2,1}=1, x_{3,0}=1,x_{3,1}=0
    // Set x vars, leave z vars as 0 for extraction test
    let mut ilp_solution = vec![0usize; 4 * 2 + 4 * 4 * 2];
    ilp_solution[0] = 1; // x_{0,0}
    ilp_solution[3] = 1; // x_{1,1}
    ilp_solution[5] = 1; // x_{2,1}
    ilp_solution[6] = 1; // x_{3,0}
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 1, 0]);
}

#[test]
fn test_sumofsquarespartition_to_ilp_trivial() {
    // 2 elements, 2 groups, generous bound
    let problem = SumOfSquaresPartition::new(vec![1, 2], 2, 10);
    let reduction: ReductionSSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=2, K=2: num_vars = 2*2 + 4*2 = 4+8 = 12
    assert_eq!(ilp.num_vars, 12);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "Should be feasible"
    );
}
