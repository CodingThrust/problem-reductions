use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Universe {0,1,2}, subset {0,1,2}
    let problem = SetSplitting::new(3, vec![vec![0, 1, 2]]);
    let reduction: ReductionSetSplittingToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 3, "one ILP var per universe element");
    assert_eq!(
        ilp.constraints.len(),
        2,
        "two constraints per subset (ge + le)"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty(), "feasibility: no objective terms");
}

#[test]
fn test_reduction_constraint_structure() {
    // Subset {0,1,2}: need sum >= 1 and sum <= 2
    let problem = SetSplitting::new(3, vec![vec![0, 1, 2]]);
    let reduction: ReductionSetSplittingToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // One ge constraint (rhs=1) and one le constraint (rhs=2)
    let ge_constraints: Vec<_> = ilp.constraints.iter().filter(|c| c.rhs == 1.0).collect();
    let le_constraints: Vec<_> = ilp.constraints.iter().filter(|c| c.rhs == 2.0).collect();
    assert_eq!(ge_constraints.len(), 1);
    assert_eq!(le_constraints.len(), 1);
}

#[test]
fn test_setsplitting_to_ilp_closed_loop() {
    // Canonical 4-subset instance, feasible
    let problem = SetSplitting::new(
        6,
        vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
    );
    let reduction: ReductionSetSplittingToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "extracted solution must split all subsets"
    );
}

#[test]
fn test_setsplitting_to_ilp_infeasible() {
    // Single-element universe, subset {0,0}: sum(x_0) >= 1 and sum(x_0) <= 0 — contradiction
    let problem = SetSplitting::new(1, vec![vec![0, 0]]);
    let reduction: ReductionSetSplittingToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    assert!(
        ilp_solver.solve(ilp).is_none(),
        "ILP should be infeasible for unsplittable instance"
    );
}

#[test]
fn test_setsplitting_bf_vs_ilp() {
    let problem = SetSplitting::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem);
    assert!(bf_witness.is_some());
    let bf_result = problem.evaluate(&bf_witness.unwrap());

    let reduction: ReductionSetSplittingToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_result = problem.evaluate(&extracted);

    assert_eq!(bf_result, ilp_result, "BruteForce and ILP must agree");
    assert_eq!(ilp_result, Or(true));
}

#[test]
fn test_overhead_dimensions() {
    // 5 elements, 3 subsets → 5 vars, 6 constraints
    let problem = SetSplitting::new(5, vec![vec![0, 1], vec![2, 3], vec![0, 4]]);
    let reduction: ReductionSetSplittingToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 5);
    assert_eq!(ilp.constraints.len(), 6); // 2 per subset
}
