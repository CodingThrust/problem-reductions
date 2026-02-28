use super::*;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_maximumsetpacking_one_to_i32_cast_closed_loop() {
    let sp_one =
        MaximumSetPacking::with_weights(vec![vec![0, 1], vec![1, 2], vec![3, 4]], vec![One; 3]);

    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&sp_one);
    let sp_i32 = reduction.target_problem();
    assert_eq!(sp_i32.weights_ref(), &vec![1i32, 1, 1]);

    let solver = BruteForce::new();
    let target_solution = solver.find_best(sp_i32).unwrap();
    let source_solution = reduction.extract_solution(&target_solution);

    let metric = sp_one.evaluate(&source_solution);
    assert!(metric.is_valid());
}

#[test]
fn test_maximumsetpacking_i32_to_f64_cast_closed_loop() {
    let sp_i32 =
        MaximumSetPacking::with_weights(vec![vec![0, 1], vec![1, 2], vec![3, 4]], vec![2i32, 3, 5]);

    let reduction = ReduceTo::<MaximumSetPacking<f64>>::reduce_to(&sp_i32);
    let sp_f64 = reduction.target_problem();
    assert_eq!(sp_f64.weights_ref(), &vec![2.0f64, 3.0, 5.0]);

    let solver = BruteForce::new();
    let target_solution = solver.find_best(sp_f64).unwrap();
    let source_solution = reduction.extract_solution(&target_solution);

    let metric = sp_i32.evaluate(&source_solution);
    assert!(metric.is_valid());
}
