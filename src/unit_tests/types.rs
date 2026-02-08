use super::*;

#[test]
fn test_unweighted() {
    let uw = Unweighted;
    // Test get() method
    assert_eq!(uw.get(0), 1);
    assert_eq!(uw.get(100), 1);
    assert_eq!(uw.get(usize::MAX), 1);

    // Test Display
    assert_eq!(format!("{}", uw), "Unweighted");

    // Test Clone, Copy, Default
    let uw2 = uw;
    let _uw3 = uw2; // Copy works (no clone needed)
    let _uw4: Unweighted = Default::default();

    // Test PartialEq
    assert_eq!(Unweighted, Unweighted);
}

#[test]
fn test_energy_mode() {
    let max_mode = EnergyMode::LargerSizeIsBetter;
    let min_mode = EnergyMode::SmallerSizeIsBetter;

    assert!(max_mode.is_maximization());
    assert!(!max_mode.is_minimization());
    assert!(!min_mode.is_maximization());
    assert!(min_mode.is_minimization());

    assert!(max_mode.is_better(&10, &5));
    assert!(!max_mode.is_better(&5, &10));
    assert!(min_mode.is_better(&5, &10));
    assert!(!min_mode.is_better(&10, &5));

    assert!(max_mode.is_better_or_equal(&10, &10));
    assert!(min_mode.is_better_or_equal(&10, &10));
}

#[test]
fn test_solution_size() {
    let valid = SolutionSize::valid(42);
    assert_eq!(valid.size, 42);
    assert!(valid.is_valid);

    let invalid = SolutionSize::invalid(0);
    assert!(!invalid.is_valid);

    let custom = SolutionSize::new(100, false);
    assert_eq!(custom.size, 100);
    assert!(!custom.is_valid);
}

#[test]
fn test_solution_size_display() {
    let valid = SolutionSize::valid(42);
    assert_eq!(format!("{}", valid), "SolutionSize(42, valid)");

    let invalid = SolutionSize::invalid(0);
    assert_eq!(format!("{}", invalid), "SolutionSize(0, invalid)");
}

#[test]
fn test_problem_size() {
    let ps = ProblemSize::new(vec![("vertices", 10), ("edges", 20)]);
    assert_eq!(ps.get("vertices"), Some(10));
    assert_eq!(ps.get("edges"), Some(20));
    assert_eq!(ps.get("unknown"), None);
}

#[test]
fn test_problem_size_display() {
    let ps = ProblemSize::new(vec![("vertices", 10), ("edges", 20)]);
    assert_eq!(format!("{}", ps), "ProblemSize{vertices: 10, edges: 20}");

    let empty = ProblemSize::new(vec![]);
    assert_eq!(format!("{}", empty), "ProblemSize{}");

    let single = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(format!("{}", single), "ProblemSize{n: 5}");
}

#[test]
fn test_local_constraint() {
    // Binary constraint on 2 variables: only (0,0) and (1,1) are valid
    let constraint = LocalConstraint::new(2, vec![0, 1], vec![true, false, false, true]);

    assert!(constraint.is_satisfied(&[0, 0]));
    assert!(!constraint.is_satisfied(&[0, 1]));
    assert!(!constraint.is_satisfied(&[1, 0]));
    assert!(constraint.is_satisfied(&[1, 1]));
    assert_eq!(constraint.num_variables(), 2);
}

#[test]
fn test_local_constraint_out_of_bounds() {
    let constraint = LocalConstraint::new(2, vec![5, 6], vec![true, false, false, true]);
    // Test with config that doesn't have indices 5 and 6 - defaults to 0
    assert!(constraint.is_satisfied(&[0, 0, 0]));
}

#[test]
fn test_local_solution_size() {
    // Binary objective on 1 variable: weight 0 for 0, weight 5 for 1
    let objective = LocalSolutionSize::new(2, vec![0], vec![0, 5]);

    assert_eq!(objective.evaluate(&[0]), 0);
    assert_eq!(objective.evaluate(&[1]), 5);
    assert_eq!(objective.num_variables(), 1);
}

#[test]
fn test_local_solution_size_multi_variable() {
    // Binary objective on 2 variables
    let objective = LocalSolutionSize::new(2, vec![0, 1], vec![0, 1, 2, 3]);
    assert_eq!(objective.evaluate(&[0, 0]), 0);
    assert_eq!(objective.evaluate(&[0, 1]), 1);
    assert_eq!(objective.evaluate(&[1, 0]), 2);
    assert_eq!(objective.evaluate(&[1, 1]), 3);
}

#[test]
fn test_numeric_weight_impls() {
    fn assert_numeric_weight<T: NumericWeight>() {}

    assert_numeric_weight::<i32>();
    assert_numeric_weight::<f64>();
    assert_numeric_weight::<i64>();
    assert_numeric_weight::<f32>();
}
