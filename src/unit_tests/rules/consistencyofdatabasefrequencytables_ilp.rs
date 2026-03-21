use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::{ConsistencyOfDatabaseFrequencyTables, FrequencyTable, KnownValue};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::traits::Problem;

fn small_yes_instance() -> ConsistencyOfDatabaseFrequencyTables {
    ConsistencyOfDatabaseFrequencyTables::new(
        2,
        vec![2, 2],
        vec![FrequencyTable::new(0, 1, vec![vec![1, 0], vec![0, 1]])],
        vec![KnownValue::new(0, 0, 0)],
    )
}

fn small_yes_witness() -> Vec<usize> {
    vec![0, 0, 1, 1]
}

fn small_no_instance() -> ConsistencyOfDatabaseFrequencyTables {
    ConsistencyOfDatabaseFrequencyTables::new(
        2,
        vec![2, 2],
        vec![FrequencyTable::new(0, 1, vec![vec![1, 0], vec![0, 1]])],
        vec![KnownValue::new(0, 0, 0), KnownValue::new(1, 1, 0)],
    )
}

#[test]
fn test_cdft_to_ilp_structure() {
    let problem = small_yes_instance();
    let reduction: ReductionCDFTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 16);
    assert_eq!(ilp.constraints.len(), 33);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_cdft_to_ilp_closed_loop() {
    let problem = small_yes_instance();
    let reduction: ReductionCDFTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "ConsistencyOfDatabaseFrequencyTables->ILP closed loop",
    );
}

#[test]
fn test_cdft_to_ilp_solution_encoding_round_trip() {
    let problem = small_yes_instance();
    let reduction: ReductionCDFTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = reduction.encode_source_solution(&small_yes_witness());
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, small_yes_witness());
}

#[test]
fn test_cdft_to_ilp_unsat_instance_is_infeasible() {
    let problem = small_no_instance();
    let reduction: ReductionCDFTToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let solver = ILPSolver::new();
    assert!(solver.solve(reduction.target_problem()).is_none());
}

#[test]
fn test_cdft_to_ilp_solve_reduced() {
    let problem = small_yes_instance();
    let solver = ILPSolver::new();
    let solution = solver
        .solve_reduced(&problem)
        .expect("solve_reduced should find a satisfying assignment");
    assert!(problem.evaluate(&solution));
}
