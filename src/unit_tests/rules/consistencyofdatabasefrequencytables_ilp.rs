use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::{ConsistencyOfDatabaseFrequencyTables, FrequencyTable, KnownValue};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn binary_attributes_reduce_without_materializing_the_domain_product() {
    let source = ConsistencyOfDatabaseFrequencyTables::new(1, vec![2; 64], vec![], vec![]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 128);
    assert_eq!(reduction.target_problem().num_constraints(), 64);
    let witness = vec![0; 64];
    let encoded = reduction.encode_source_solution(&witness);
    assert!(reduction
        .target_problem()
        .evaluate(&encoded)
        .unwrap()
        .value
        .is_some());
    assert_eq!(reduction.extract_solution(&encoded).unwrap(), witness);
}

#[test]
fn ilp_encoding_overflow_is_a_reduction_error() {
    let auxiliary_objects = usize::MAX / 6 + 1;
    for (objects, domains, tables) in [
        (usize::MAX / 2 + 1, vec![2], vec![]),
        (
            auxiliary_objects,
            vec![3, 1, 1],
            vec![
                FrequencyTable::new(0, 1, vec![vec![auxiliary_objects as i64], vec![0], vec![0]]),
                FrequencyTable::new(0, 2, vec![vec![auxiliary_objects as i64], vec![0], vec![0]]),
            ],
        ),
        (
            usize::MAX / 3 + 1,
            vec![1, 1],
            vec![FrequencyTable::new(
                0,
                1,
                vec![vec![(usize::MAX / 3 + 1) as i64]],
            )],
        ),
        (
            usize::MAX / 4,
            vec![1, 1],
            vec![FrequencyTable::new(
                0,
                1,
                vec![vec![(usize::MAX / 4) as i64]],
            )],
        ),
    ] {
        let source = ConsistencyOfDatabaseFrequencyTables::new(objects, domains, tables, vec![]);
        let restored: ConsistencyOfDatabaseFrequencyTables =
            serde_json::from_value(serde_json::to_value(&source).unwrap()).unwrap();
        assert_eq!(restored.parameters(), source.parameters());
        assert!(matches!(
            ReduceTo::<ILP<bool>>::reduce_to(&restored),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn unrepresentable_ilp_row_storage_does_not_restrict_source_evaluation() {
    let source = ConsistencyOfDatabaseFrequencyTables::new(1, vec![usize::MAX], vec![], vec![]);
    assert_eq!(source.evaluate(&vec![0]).unwrap(), crate::types::Or(true));
    assert!(matches!(
        ReduceTo::<ILP<bool>>::reduce_to(&source),
        Err(crate::rules::ReductionError::InvalidTarget { .. })
    ));
}

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
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 16);
    assert_eq!(ilp.constraints().len(), 33);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
    assert!(ilp.objective().is_empty());
}

#[test]
fn test_cdft_to_ilp_closed_loop() {
    let problem = small_yes_instance();
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_cdft_to_ilp_solution_encoding_round_trip() {
    let problem = small_yes_instance();
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = reduction.encode_source_solution(&small_yes_witness());
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, small_yes_witness());
}

#[test]
fn test_cdft_to_ilp_unsat_instance_is_infeasible() {
    let problem = small_no_instance();
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let solver = ILPSolver::new();
    assert!(solver.solve(reduction.target_problem()).is_err());
}

#[test]
fn test_cdft_solve_via_ilp_pipeline() {
    let problem = small_yes_instance();
    let solver = ILPSolver::new();
    let solution = solver
        .solve(&problem)
        .expect("ILP pipeline should find a satisfying assignment");
    assert!(problem.evaluate(&solution).unwrap());
}

#[test]
fn test_consistency_to_ilp_bf_vs_ilp() {
    let problem = small_yes_instance();
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let bf_witness = BruteForce::new()
        .solve(&problem)
        .unwrap()
        .expect("should be satisfiable");
    assert!(problem.evaluate(&bf_witness).unwrap());

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert!(problem.evaluate(&extracted).unwrap());
}

fn issue_instance() -> ConsistencyOfDatabaseFrequencyTables {
    ConsistencyOfDatabaseFrequencyTables::new(
        6,
        vec![2, 3, 2],
        vec![
            FrequencyTable::new(0, 1, vec![vec![1, 1, 1], vec![1, 1, 1]]),
            FrequencyTable::new(1, 2, vec![vec![1, 1], vec![0, 2], vec![1, 1]]),
        ],
        vec![
            KnownValue::new(0, 0, 0),
            KnownValue::new(3, 0, 1),
            KnownValue::new(1, 2, 1),
        ],
    )
}

fn issue_witness() -> Vec<usize> {
    vec![0, 0, 0, 0, 1, 1, 0, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2, 0]
}

#[test]
fn test_cdft_to_ilp_issue_instance_closed_loop() {
    let problem = issue_instance();
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let solver = ILPSolver::new();
    let target_solution = solver
        .solve(reduction.target_problem())
        .expect("ILP solver should find a feasible solution for the issue instance");
    let source_solution = reduction.extract_solution(&target_solution).unwrap();
    assert!(
        problem.evaluate(&source_solution).unwrap(),
        "extracted source solution must satisfy the original CDFT instance"
    );
}

#[test]
fn test_cdft_to_ilp_issue_instance_encoding_round_trip() {
    let problem = issue_instance();
    let reduction: ReductionCDFTToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = reduction.encode_source_solution(&issue_witness());
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, issue_witness());
}
