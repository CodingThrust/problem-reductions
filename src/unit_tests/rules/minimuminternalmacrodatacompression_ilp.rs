use crate::models::algebraic::ILP;
use crate::models::misc::MinimumInternalMacroDataCompression;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_imdc_to_ilp_closed_loop_simple() {
    // s = "ab", alphabet {a,b}, h=2
    // Optimal: uncompressed, cost=2
    let source = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witness = solver.find_witness(target).expect("ILP should be feasible");
    let source_config = reduction.extract_solution(&target_witness);
    let val = source.evaluate(&source_config);
    assert!(val.0.is_some());
    assert_eq!(val.0.unwrap(), 2);
}

#[test]
fn test_imdc_to_ilp_closed_loop_repeated() {
    // s = "abab", alphabet {a,b}, h=2
    // Optimal: cost=4 (uncompressed or pointer, both cost 4)
    let source = MinimumInternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witness = solver.find_witness(target).expect("ILP should be feasible");
    let source_config = reduction.extract_solution(&target_witness);
    let val = source.evaluate(&source_config);
    assert!(val.0.is_some());
    assert_eq!(val.0.unwrap(), 4);
}

#[test]
fn test_imdc_to_ilp_closed_loop_low_pointer_cost() {
    // s = "abab", alphabet {a,b}, h=1
    // With h=1, pointers cost 0 extra: cost = |C|
    // Optimal with pointer: C=[a,b,ptr(0)], active=3, ptrs=1, cost=3+0=3
    let source = MinimumInternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 1);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witness = solver.find_witness(target).expect("ILP should be feasible");
    let source_config = reduction.extract_solution(&target_witness);
    let val = source.evaluate(&source_config);
    assert!(val.0.is_some());
    // Verify against brute force
    let bf_val = BruteForce::new().solve(&source);
    assert_eq!(val, bf_val);
}

#[test]
fn test_imdc_to_ilp_empty_string() {
    let source = MinimumInternalMacroDataCompression::new(2, vec![], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_variables(), 0);
    let source_config = reduction.extract_solution(&[]);
    assert_eq!(source.evaluate(&source_config), Min(Some(0)));
}

#[test]
fn test_imdc_to_ilp_single_char() {
    // s = "a", alphabet {a}, h=2
    // Only valid: literal, cost=1
    let source = MinimumInternalMacroDataCompression::new(1, vec![0], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witness = solver.find_witness(target).expect("ILP should be feasible");
    let source_config = reduction.extract_solution(&target_witness);
    assert_eq!(source.evaluate(&source_config), Min(Some(1)));
}

#[test]
fn test_imdc_to_ilp_structure() {
    // Verify the ILP has the right number of variables
    let source = MinimumInternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();
    // n=4 literals + valid ptr triples
    assert!(target.num_variables() >= 4);
    // Must be a minimization problem
    assert_eq!(target.dims(), vec![2; target.num_variables()]);
}

#[test]
fn test_imdc_to_ilp_vs_brute_force() {
    // Compare ILP result against brute-force for small instances
    for (k, s, h) in [
        (1, vec![0, 0, 0], 2),
        (2, vec![0, 1, 0], 2),
        (2, vec![0, 0, 1, 1], 1),
    ] {
        let source = MinimumInternalMacroDataCompression::new(k, s.clone(), h);
        let bf_val = BruteForce::new().solve(&source);

        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
        let target = reduction.target_problem();
        let target_witness = BruteForce::new()
            .find_witness(target)
            .expect("ILP should be feasible");
        let source_config = reduction.extract_solution(&target_witness);
        let ilp_val = source.evaluate(&source_config);

        assert_eq!(
            ilp_val, bf_val,
            "ILP and brute-force disagree for k={}, s={:?}, h={}",
            k, s, h
        );
    }
}
