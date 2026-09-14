use super::*;
use crate::solvers::SolveOutcome;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 2 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2).unwrap();
    let reduction: ReductionERCToILP =
        ReduceTo::<ILP<bool, f64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // num_records=2, num_sectors=2: n=4 x-vars, n^2=16 z-vars -> 20 total
    let n = 2 * 2; // 4
    assert_eq!(ilp.num_vars(), n + n * n, "Should have n + n^2 variables");

    // num_constraints = 2 assignment + 3*n^2 McCormick = 2 + 48 = 50
    assert_eq!(
        ilp.constraints().len(),
        2 + 3 * n * n,
        "Should have 2 + 3*n^2 constraints"
    );
    assert_eq!(
        ilp.sense(),
        ObjectiveSense::Minimize,
        "Should minimize cost"
    );
    // Objective should have non-empty coefficients
    assert!(
        !ilp.objective().is_empty(),
        "Objective should have cost coefficients"
    );
}

#[test]
fn test_expectedretrievalcost_to_ilp_bf_vs_ilp() {
    // 3 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.3, 0.4, 0.3], 2).unwrap();
    let reduction: ReductionERCToILP =
        ReduceTo::<ILP<bool, f64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.solve(&problem).unwrap().unwrap();
    let bf_cost = problem.expected_cost(&bf_witness).unwrap().unwrap();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction
        .recover_result(
            &problem,
            SolveOutcome::optimal(reduction.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
    let ilp_cost = problem.expected_cost(&extracted).unwrap().unwrap();

    // ILP cost should match BF optimal cost
    assert!(
        (ilp_cost - bf_cost).abs() < 1e-6,
        "ILP cost {ilp_cost} should match BF cost {bf_cost}"
    );
}

#[test]
fn test_solution_extraction() {
    // 2 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2).unwrap();
    let reduction: ReductionERCToILP =
        ReduceTo::<ILP<bool, f64>>::reduce_to(&problem).expect("reduction should succeed");

    for assignment in [vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]] {
        let mut target = vec![0; reduction.target_problem().num_vars()];
        for (r, &sector) in assignment.iter().enumerate() {
            target[reduction.x_var(r, sector)] = 1;
        }
        for (r, &sector) in assignment.iter().enumerate() {
            for (other, &other_sector) in assignment.iter().enumerate() {
                target[reduction.z_var(r, sector, other, other_sector)] = 1;
            }
        }
        assert_eq!(
            reduction
                .recover_result(
                    &problem,
                    SolveOutcome::optimal(reduction.target_problem(), target.clone()).unwrap()
                )
                .unwrap()
                .into_solution()
                .expect("qualifying target result must recover a source solution"),
            assignment
        );
        assert_eq!(
            reduction
                .target_problem()
                .evaluate_objective(&target)
                .unwrap(),
            problem.expected_cost(&assignment).unwrap().unwrap()
        );
    }
}

#[test]
fn test_expectedretrievalcost_to_ilp_closed_loop() {
    // 2 records, 2 sectors
    let problem = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2).unwrap();
    let reduction: ReductionERCToILP =
        ReduceTo::<ILP<bool, f64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction
        .recover_result(
            &problem,
            SolveOutcome::optimal(reduction.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
    let value = problem.evaluate(&extracted).unwrap();
    assert!(
        matches!(value, Min(Some(_))),
        "Should produce a valid assignment"
    );
}
