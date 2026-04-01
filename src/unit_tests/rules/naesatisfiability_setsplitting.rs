use super::*;
use crate::models::formula::CNFClause;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_naesatisfiability_to_setsplitting_closed_loop() {
    // YES instance from test vectors: n=4
    // clauses: (x1,x2,x3), (-x1,x3,x4), (x2,-x3,-x4), (x1,-x2,x4)
    let naesat = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
            CNFClause::new(vec![1, -2, 4]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &naesat,
        &reduction,
        "NAESAT->SetSplitting closed loop",
    );
}

#[test]
fn test_naesatisfiability_to_setsplitting_infeasible() {
    // NO instance from test vectors: n=3
    // clauses: (x1,x2,x3), (x1,x2,-x3), (x1,-x2,x3), (x1,-x2,-x3)
    // This is infeasible because x1 must be both true and false.
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let naesat_solutions = solver.find_all_witnesses(&naesat);
    let splitting_solutions = solver.find_all_witnesses(target);

    assert!(naesat_solutions.is_empty(), "NAESAT should be infeasible");
    assert!(
        splitting_solutions.is_empty(),
        "SetSplitting should also be infeasible"
    );
}

#[test]
fn test_reduction_structure() {
    let naesat = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
            CNFClause::new(vec![1, -2, 4]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);
    let target = reduction.target_problem();

    // universe_size = 2 * num_vars = 8
    assert_eq!(target.universe_size(), 8);
    // num_subsets = num_vars + num_clauses = 4 + 4 = 8
    assert_eq!(target.num_subsets(), 8);

    // First 4 subsets are complementarity: {0,1}, {2,3}, {4,5}, {6,7}
    let subsets = target.subsets();
    for i in 0..4 {
        assert_eq!(subsets[i], vec![2 * i, 2 * i + 1]);
    }

    // Clause subsets follow the literal mapping
    // Clause [1,2,3] -> [0,2,4]
    assert_eq!(subsets[4], vec![0, 2, 4]);
    // Clause [-1,3,4] -> [1,4,6]
    assert_eq!(subsets[5], vec![1, 4, 6]);
    // Clause [2,-3,-4] -> [2,5,7]
    assert_eq!(subsets[6], vec![2, 5, 7]);
    // Clause [1,-2,4] -> [0,3,6]
    assert_eq!(subsets[7], vec![0, 3, 6]);
}

#[test]
fn test_solution_extraction() {
    let naesat = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);

    // target_solution: [0, 1, 1, 0, 0, 1, 1, 0]
    // assignment[i] = 1 - target_solution[2*i]
    // x1 = 1-0 = 1 (true), x2 = 1-1 = 0 (false), x3 = 1-0 = 1 (true), x4 = 1-1 = 0 (false)
    let extracted = reduction.extract_solution(&[0, 1, 1, 0, 0, 1, 1, 0]);
    assert_eq!(extracted, vec![1, 0, 1, 0]);
}

#[test]
fn test_all_satisfying_assignments_map_back() {
    // Small instance: verify every SetSplitting solution maps to a valid NAESAT solution
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let splitting_solutions = solver.find_all_witnesses(target);

    assert!(
        !splitting_solutions.is_empty(),
        "SetSplitting should have solutions"
    );

    for sol in &splitting_solutions {
        let naesat_sol = reduction.extract_solution(sol);
        assert_eq!(naesat_sol.len(), 3);
        assert!(
            naesat.evaluate(&naesat_sol).0,
            "Extracted solution {:?} from splitting solution {:?} does not satisfy NAESAT",
            naesat_sol,
            sol
        );
    }
}

#[test]
fn test_larger_instance_closed_loop() {
    let naesat = NAESatisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -4, 5]),
            CNFClause::new(vec![-2, 3, -5]),
            CNFClause::new(vec![1, -3, 5]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &naesat,
        &reduction,
        "NAESAT->SetSplitting larger instance",
    );
}

#[test]
fn test_two_variable_instance() {
    // Minimal instance with 2 variables
    let naesat = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&naesat);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    // 2 complementarity + 1 clause = 3 subsets
    assert_eq!(target.num_subsets(), 3);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &naesat,
        &reduction,
        "NAESAT->SetSplitting two-variable",
    );
}
