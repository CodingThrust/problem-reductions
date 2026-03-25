use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 2 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2);
    let reduction: ReductionERCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_records=2, num_sectors=2: n=4 x-vars, n^2=16 z-vars -> 20 total
    let n = 2 * 2; // 4
    assert_eq!(ilp.num_vars, n + n * n, "Should have n + n^2 variables");

    // num_constraints = 2 assignment + 3*n^2 McCormick = 2 + 48 = 50
    assert_eq!(
        ilp.constraints.len(),
        2 + 3 * n * n,
        "Should have 2 + 3*n^2 constraints"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize cost");
    // Objective should have non-empty coefficients
    assert!(
        !ilp.objective.is_empty(),
        "Objective should have cost coefficients"
    );
}

#[test]
fn test_expectedretrievalcost_to_ilp_bf_vs_ilp() {
    // 3 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.3, 0.4, 0.3], 2);
    let reduction: ReductionERCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).unwrap();
    let bf_cost = problem.expected_cost(&bf_witness).unwrap();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_cost = problem.expected_cost(&extracted).unwrap();

    // ILP cost should match BF optimal cost
    assert!(
        (ilp_cost - bf_cost).abs() < 1e-6,
        "ILP cost {ilp_cost} should match BF cost {bf_cost}"
    );
}

#[test]
fn test_solution_extraction() {
    // 2 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2);
    let reduction: ReductionERCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // record 0 -> sector 0, record 1 -> sector 1
    // x_{0,0}=1, x_{0,1}=0, x_{1,0}=0, x_{1,1}=1
    let mut ilp_solution = vec![0usize; 4 + 16]; // n + n^2
                                                 // x vars
    ilp_solution[0] = 1; // x_{0,0}
    ilp_solution[3] = 1; // x_{1,1}
                         // z vars: z_{r,s,r',s'} at offset 4 + (r*2+s)*4 + (r'*2+s')
                         // z_{0,0,0,0} = x_{0,0}*x_{0,0} = 1: offset 4 + 0*4 + 0 = 4
    ilp_solution[4] = 1;
    // z_{1,1,1,1} = x_{1,1}*x_{1,1} = 1: offset 4 + 3*4 + 3 = 4+15=19
    ilp_solution[19] = 1;

    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1]);
}

#[test]
fn test_expectedretrievalcost_to_ilp_closed_loop() {
    // 2 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2);
    let reduction: ReductionERCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(
        matches!(value, Min(Some(_))),
        "Should produce a valid assignment"
    );
}
