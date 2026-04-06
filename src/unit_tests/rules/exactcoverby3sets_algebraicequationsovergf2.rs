use crate::models::algebraic::AlgebraicEquationsOverGF2;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};

#[test]
fn test_exactcoverby3sets_to_algebraicequationsovergf2_closed_loop() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<AlgebraicEquationsOverGF2>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> AlgebraicEquationsOverGF2 closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_algebraicequationsovergf2_structure() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<AlgebraicEquationsOverGF2>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_variables(), 3);
    assert_eq!(target.num_equations(), 9);
    assert_eq!(
        target.equations(),
        &[
            vec![vec![0], vec![2], vec![]],
            vec![vec![0, 2]],
            vec![vec![0], vec![]],
            vec![vec![0], vec![]],
            vec![vec![1], vec![2], vec![]],
            vec![vec![1, 2]],
            vec![vec![1], vec![2], vec![]],
            vec![vec![1, 2]],
            vec![vec![1], vec![]],
        ]
    );
}

#[test]
fn test_exactcoverby3sets_to_algebraicequationsovergf2_extract_solution_is_identity() {
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction = ReduceTo::<AlgebraicEquationsOverGF2>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[1, 0, 1]), vec![1, 0, 1]);
}
