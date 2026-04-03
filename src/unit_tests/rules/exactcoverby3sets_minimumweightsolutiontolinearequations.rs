use crate::models::algebraic::MinimumWeightSolutionToLinearEquations;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};

#[test]
fn test_exactcoverby3sets_to_minimumweightsolutiontolinearequations_closed_loop() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<MinimumWeightSolutionToLinearEquations>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> MinimumWeightSolutionToLinearEquations closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_minimumweightsolutiontolinearequations_structure() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<MinimumWeightSolutionToLinearEquations>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(
        target.coefficients(),
        &[
            vec![1, 0, 1],
            vec![1, 0, 0],
            vec![1, 0, 0],
            vec![0, 1, 1],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ]
    );
    assert_eq!(target.rhs(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(target.bound(), 2);
    assert_eq!(target.num_variables(), 3);
    assert_eq!(target.num_equations(), 6);
}

#[test]
fn test_exactcoverby3sets_to_minimumweightsolutiontolinearequations_extract_solution_is_identity() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<MinimumWeightSolutionToLinearEquations>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[1, 0, 1]), vec![1, 0, 1]);
}
