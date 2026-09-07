use crate::models::misc::CosineProductIntegration;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;

#[test]
fn test_cosine_product_integration_creation() {
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    assert_eq!(p.coefficients(), &[2, 3, 5]);
    assert_eq!(p.num_coefficients(), 3);
}

#[test]
fn test_cosine_product_integration_dims() {
    let p = CosineProductIntegration::new(vec![1, 2, 3]);
    assert_eq!(p.dimensions(), vec![2, 2, 2]);
}

#[test]
fn test_cosine_product_integration_evaluate_satisfying() {
    // [2, 3, 5]: (+2, +3, -5) = 0 → satisfying
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    assert!(p.evaluate(&vec![false, false, true]).unwrap().0);
}

#[test]
fn test_cosine_product_integration_evaluate_not_satisfying() {
    // [2, 3, 5]: (+2, +3, +5) = 10 → not satisfying
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    assert!(!p.evaluate(&vec![false, false, false]).unwrap().0);
}

#[test]
fn test_cosine_product_integration_unsatisfiable() {
    // [1, 2, 6]: total=9 (odd), no balanced sign assignment
    let p = CosineProductIntegration::new(vec![1, 2, 6]);
    let solver = BruteForce::new();
    assert!(solver.solve(&p).unwrap().is_none());
}

#[test]
fn test_cosine_product_integration_solver() {
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    let solver = BruteForce::new();
    let witness = solver.solve(&p).unwrap().unwrap();
    assert!(p.evaluate(&witness).unwrap().0);
}

#[test]
fn test_cosine_product_integration_aggregate() {
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    let solver = BruteForce::new();
    let value_solution = solver.solve(&p).unwrap().unwrap();
    let value = p.evaluate(&value_solution).unwrap();
    assert!(value.0);

    let p2 = CosineProductIntegration::new(vec![1, 2, 6]);
    let value2 = solver.solve(&p2).unwrap();
    assert!(value2.is_none());
}

#[test]
fn test_cosine_product_integration_negative_coefficients() {
    // [-3, 2, 1]: (-(-3), +2, -1) = (3, 2, -1) = 4, not zero
    // but (-3, +2, +1) = 0 → config [0, 0, 0] → -3+2+1=0
    let p = CosineProductIntegration::new(vec![-3, 2, 1]);
    assert!(p.evaluate(&vec![false, false, false]).unwrap().0); // -3 + 2 + 1 = 0
}

#[test]
fn test_cosine_product_integration_invalid_config() {
    let p = CosineProductIntegration::new(vec![1, 2, 3]);
    // Wrong length
    assert!(matches!(
        p.evaluate(&vec![false, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    // Out of range
    assert!(
        crate::registry::DynProblem::evaluate_dyn(&p, &serde_json::json!([false, 2, false]))
            .is_err()
    );
}

#[test]
fn test_cosine_product_integration_serialization() {
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    let json = serde_json::to_string(&p).unwrap();
    let p2: CosineProductIntegration = serde_json::from_str(&json).unwrap();
    assert_eq!(p2.coefficients(), p.coefficients());
}

#[test]
fn test_cosine_product_integration_all_witnesses() {
    // [2, 3, 5]: two balanced assignments: (+2,+3,-5)=0 and (-2,-3,+5)=0
    let p = CosineProductIntegration::new(vec![2, 3, 5]);
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&p).unwrap();
    assert_eq!(witnesses.len(), 2);
    for w in &witnesses {
        assert!(p.evaluate(w).unwrap().0);
    }
}

#[test]
#[should_panic]
fn test_cosine_product_integration_empty() {
    CosineProductIntegration::new(vec![]);
}
