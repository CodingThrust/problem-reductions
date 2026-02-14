use super::*;

#[test]
fn test_solution_size_valid() {
    let size: SolutionSize<i32> = SolutionSize::Valid(42);
    assert!(size.is_valid());
    assert_eq!(size.size(), Some(&42));
}

#[test]
fn test_solution_size_invalid() {
    let size: SolutionSize<i32> = SolutionSize::Invalid;
    assert!(!size.is_valid());
    assert_eq!(size.size(), None);
}

#[test]
fn test_solution_size_unwrap() {
    let valid: SolutionSize<i32> = SolutionSize::Valid(10);
    assert_eq!(valid.unwrap(), 10);
}

#[test]
#[should_panic(expected = "called unwrap on Invalid")]
fn test_solution_size_unwrap_panics() {
    let invalid: SolutionSize<i32> = SolutionSize::Invalid;
    invalid.unwrap();
}

#[test]
fn test_solution_size_map() {
    let valid: SolutionSize<i32> = SolutionSize::Valid(10);
    let mapped = valid.map(|x| x * 2);
    assert_eq!(mapped, SolutionSize::Valid(20));

    let invalid: SolutionSize<i32> = SolutionSize::Invalid;
    let mapped_invalid = invalid.map(|x| x * 2);
    assert_eq!(mapped_invalid, SolutionSize::Invalid);
}

#[test]
fn test_one() {
    let one = One;

    // Test Display
    assert_eq!(format!("{}", one), "One");

    // Test Clone, Copy, Default
    let one2 = one;
    let _one3 = one2; // Copy works (no clone needed)
    let _one4: One = Default::default();

    // Test PartialEq
    assert_eq!(One, One);

    // Test From<i32>
    let from_int: One = One::from(42);
    assert_eq!(from_int, One);
}

#[test]
fn test_direction() {
    let max_dir = Direction::Maximize;
    let min_dir = Direction::Minimize;

    assert_eq!(max_dir, Direction::Maximize);
    assert_eq!(min_dir, Direction::Minimize);
    assert_ne!(max_dir, min_dir);
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
fn test_numeric_size_blanket_impl() {
    fn assert_numeric_size<T: NumericSize>() {}
    assert_numeric_size::<i32>();
    assert_numeric_size::<i64>();
    assert_numeric_size::<f64>();
}

#[test]
fn test_weight_element_one() {
    let one = One;
    assert_eq!(one.to_sum(), 1);

    // Verify associated type
    fn assert_weight_element<T: WeightElement>() {}
    assert_weight_element::<One>();
}

#[test]
fn test_weight_element_i32() {
    let w: i32 = 42;
    assert_eq!(w.to_sum(), 42);

    let zero: i32 = 0;
    assert_eq!(zero.to_sum(), 0);

    let neg: i32 = -5;
    assert_eq!(neg.to_sum(), -5);
}

#[test]
fn test_weight_element_f64() {
    let w: f64 = 3.15;
    assert_eq!(w.to_sum(), 3.15);

    let zero: f64 = 0.0;
    assert_eq!(zero.to_sum(), 0.0);

    let neg: f64 = -2.5;
    assert_eq!(neg.to_sum(), -2.5);
}

#[test]
fn test_is_better_maximize_valid_vs_valid() {
    // For maximization: larger is better
    let a = SolutionSize::Valid(10);
    let b = SolutionSize::Valid(5);
    assert!(a.is_better(&b, Direction::Maximize));
    assert!(!b.is_better(&a, Direction::Maximize));
}

#[test]
fn test_is_better_minimize_valid_vs_valid() {
    // For minimization: smaller is better
    let a = SolutionSize::Valid(5);
    let b = SolutionSize::Valid(10);
    assert!(a.is_better(&b, Direction::Minimize));
    assert!(!b.is_better(&a, Direction::Minimize));
}

#[test]
fn test_is_better_valid_vs_invalid() {
    // Valid is always better than invalid
    let valid = SolutionSize::Valid(0);
    let invalid: SolutionSize<i32> = SolutionSize::Invalid;
    assert!(valid.is_better(&invalid, Direction::Maximize));
    assert!(valid.is_better(&invalid, Direction::Minimize));
    assert!(!invalid.is_better(&valid, Direction::Maximize));
    assert!(!invalid.is_better(&valid, Direction::Minimize));
}

#[test]
fn test_is_better_invalid_vs_invalid() {
    // Neither invalid is better
    let a: SolutionSize<i32> = SolutionSize::Invalid;
    let b: SolutionSize<i32> = SolutionSize::Invalid;
    assert!(!a.is_better(&b, Direction::Maximize));
    assert!(!a.is_better(&b, Direction::Minimize));
}

#[test]
fn test_is_better_equal_valid() {
    // Equal values: neither is better
    let a = SolutionSize::Valid(5);
    let b = SolutionSize::Valid(5);
    assert!(!a.is_better(&b, Direction::Maximize));
    assert!(!a.is_better(&b, Direction::Minimize));
}
