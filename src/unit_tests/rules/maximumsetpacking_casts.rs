use super::*;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_maximumsetpacking_one_to_i64_cast_closed_loop() {
    let sp_one =
        MaximumSetPacking::with_weights(vec![vec![0, 1], vec![1, 2], vec![3, 4]], vec![One; 3])
            .unwrap();

    let reduction =
        ReduceTo::<MaximumSetPacking<i64>>::reduce_to(&sp_one).expect("reduction should succeed");
    let sp_i64 = reduction.target_problem();
    assert_eq!(sp_i64.weights_ref(), &vec![1i64, 1, 1]);

    let solver = BruteForce::new();
    let target_solution = solver.solve(sp_i64).unwrap().unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    let metric = sp_one.evaluate(&source_solution).unwrap();
    assert!(metric.is_valid());
}

#[test]
fn test_maximumsetpacking_i64_to_f64_cast_closed_loop() {
    let sp_i64 =
        MaximumSetPacking::with_weights(vec![vec![0, 1], vec![1, 2], vec![3, 4]], vec![2i64, 3, 5])
            .unwrap();

    let reduction =
        ReduceTo::<MaximumSetPacking<f64>>::reduce_to(&sp_i64).expect("reduction should succeed");
    let sp_f64 = reduction.target_problem();
    assert_eq!(sp_f64.weights_ref(), &vec![2.0f64, 3.0, 5.0]);

    let solver = BruteForce::new();
    let target_solution = solver.solve(sp_f64).unwrap().unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    let metric = sp_i64.evaluate(&source_solution).unwrap();
    assert!(metric.is_valid());
}
