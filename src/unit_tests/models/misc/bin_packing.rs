use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_bin_packing_creation() {
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10).unwrap();
    assert_eq!(problem.num_items(), 6);
    assert_eq!(problem.sizes(), &[6, 6, 5, 5, 4, 4]);
    assert_eq!(*problem.capacity(), 10);
    assert_eq!(problem.dims().len(), 6);
    // Each variable has domain {0, ..., 5}
    assert!(problem.dims().iter().all(|&d| d == 6));
}

#[test]
fn test_bin_packing_evaluate_valid() {
    // 6 items, capacity 10, sizes [6, 6, 5, 5, 4, 4]
    // Assignment: (0, 1, 2, 2, 0, 1) -> 3 bins
    // Bin 0: items 0,4 -> 6+4=10 OK
    // Bin 1: items 1,5 -> 6+4=10 OK
    // Bin 2: items 2,3 -> 5+5=10 OK
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10).unwrap();
    let result = problem.evaluate(&[0, 1, 2, 2, 0, 1]).unwrap();
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_bin_packing_evaluate_invalid_overweight() {
    // Bin 0: items 0,1 -> 6+6=12 > 10
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10).unwrap();
    let result = problem.evaluate(&[0, 0, 1, 1, 2, 2]).unwrap();
    assert!(!result.is_valid());
}

#[test]
fn test_bin_packing_evaluate_single_bin() {
    // All items fit in one bin
    let problem = BinPacking::new(vec![1, 2, 3], 10).unwrap();
    let result = problem.evaluate(&[0, 0, 0]).unwrap();
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_bin_packing_evaluate_all_separate() {
    // Each item in its own bin
    let problem = BinPacking::new(vec![3, 3, 3], 5).unwrap();
    let result = problem.evaluate(&[0, 1, 2]).unwrap();
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_bin_packing_problem_name() {
    assert_eq!(<BinPacking<i64> as Problem>::NAME, "BinPacking");
}

#[test]
fn test_bin_packing_brute_force_solver() {
    // 6 items, capacity 10, sizes [6, 6, 5, 5, 4, 4]
    // Optimal: 3 bins (lower bound ceil(30/10) = 3)
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10).unwrap();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .unwrap()
        .expect("should find a solution");
    let metric = problem.evaluate(&solution).unwrap();
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_bin_packing_brute_force_small() {
    // 3 items [3, 3, 4], capacity 7
    // Optimal: 2 bins (e.g., {3,4} + {3})
    let problem = BinPacking::new(vec![3, 3, 4], 7).unwrap();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .unwrap()
        .expect("should find a solution");
    let metric = problem.evaluate(&solution).unwrap();
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 2);
}

#[test]
fn test_bin_packing_empty_items() {
    let problem = BinPacking::new(Vec::<i64>::new(), 10).unwrap();
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    let result = problem.evaluate(&[]).unwrap();
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_bin_packing_wrong_config_length() {
    let problem = BinPacking::new(vec![3, 3, 4], 7).unwrap();
    assert!(!problem.evaluate(&[0, 1]).unwrap().is_valid());
    assert!(!problem.evaluate(&[0, 1, 2, 3]).unwrap().is_valid());
}

#[test]
fn test_bin_packing_out_of_range_bin() {
    let problem = BinPacking::new(vec![3, 3, 4], 7).unwrap();
    // Bin index 3 is out of range for 3 items (valid range 0..3)
    assert!(!problem.evaluate(&[0, 1, 3]).unwrap().is_valid());
}

#[test]
fn test_bin_packing_f64() {
    let problem = BinPacking::new(vec![2.5, 3.5, 4.0], 7.0).unwrap();
    // All fit in one bin: 2.5 + 3.5 + 4.0 = 10.0 > 7.0
    assert!(!problem.evaluate(&[0, 0, 0]).unwrap().is_valid());
    // Two bins: {2.5, 3.5} = 6.0, {4.0} = 4.0
    let result = problem.evaluate(&[0, 0, 1]).unwrap();
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_bin_packing_variant() {
    let v = <BinPacking<i64> as Problem>::variant();
    assert_eq!(v, vec![("weight", "i64")]);
    let v64 = <BinPacking<f64> as Problem>::variant();
    assert_eq!(v64, vec![("weight", "f64")]);
}

#[test]
fn test_bin_packing_serialization() {
    let problem = BinPacking::new(vec![6, 6, 5, 5, 4, 4], 10).unwrap();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: BinPacking<i64> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.capacity(), problem.capacity());
}

#[test]
fn test_bin_packing_rejects_non_finite_values() {
    assert!(BinPacking::new(vec![f64::NAN], 1.0).is_err());
    assert!(BinPacking::new(vec![1.0], f64::INFINITY).is_err());
}
