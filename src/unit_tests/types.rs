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
fn test_unweighted() {
    let uw = Unweighted(0);
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
    assert_eq!(Unweighted(0), Unweighted(0));
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
fn test_numeric_weight_impls() {
    fn assert_numeric_weight<T: NumericWeight>() {}

    assert_numeric_weight::<i32>();
    assert_numeric_weight::<f64>();
    assert_numeric_weight::<i64>();
    assert_numeric_weight::<f32>();
}

#[test]
fn test_numeric_size_blanket_impl() {
    fn assert_numeric_size<T: NumericSize>() {}
    assert_numeric_size::<i32>();
    assert_numeric_size::<i64>();
    assert_numeric_size::<f64>();
}

#[test]
fn test_unweighted_weights_trait() {
    let w = Unweighted(5);
    assert_eq!(w.len(), 5);
    assert_eq!(w.weight(0), 1);
    assert_eq!(w.weight(4), 1);
    assert_eq!(Unweighted::NAME, "Unweighted");
}

#[test]
fn test_vec_i32_weights_trait() {
    let w = vec![3, 1, 4];
    assert_eq!(w.len(), 3);
    assert_eq!(w.weight(0), 3);
    assert_eq!(w.weight(2), 4);
    assert_eq!(<Vec<i32> as Weights>::NAME, "Weighted<i32>");
}

#[test]
fn test_vec_f64_weights_trait() {
    let w = vec![1.5, 2.5];
    assert_eq!(w.len(), 2);
    assert_eq!(w.weight(1), 2.5);
    assert_eq!(<Vec<f64> as Weights>::NAME, "Weighted<f64>");
}
