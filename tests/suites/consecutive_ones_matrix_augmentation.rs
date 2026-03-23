use problemreductions::models::algebraic::ConsecutiveOnesMatrixAugmentation;
use problemreductions::Problem;

#[test]
fn test_consecutive_ones_matrix_augmentation_yes_instance() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![
            vec![true, false, false, true, true],
            vec![true, true, false, false, false],
            vec![false, true, true, false, true],
            vec![false, false, true, true, false],
        ],
        2,
    );

    assert!(problem.evaluate(&[0, 1, 4, 2, 3]));
}
