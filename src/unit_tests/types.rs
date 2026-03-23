use super::*;
use crate::types::Aggregate;

#[test]
fn test_max_identity_and_combine() {
    assert_eq!(Max::<i32>::identity(), Max(None));
    assert_eq!(Max(Some(7)).combine(Max(Some(3))), Max(Some(7)));
    assert_eq!(Max(Some(3)).combine(Max(Some(7))), Max(Some(7)));
    assert_eq!(Max::<i32>::identity().combine(Max(Some(5))), Max(Some(5)));
}

#[test]
fn test_min_identity_and_combine() {
    assert_eq!(Min::<i32>::identity(), Min(None));
    assert_eq!(Min(Some(3)).combine(Min(Some(7))), Min(Some(3)));
    assert_eq!(Min(Some(7)).combine(Min(Some(3))), Min(Some(3)));
    assert_eq!(Min::<i32>::identity().combine(Min(Some(5))), Min(Some(5)));
}

#[test]
fn test_sum_identity_and_combine() {
    assert_eq!(Sum::<u64>::identity(), Sum(0));
    assert_eq!(Sum(4_u64).combine(Sum(3_u64)), Sum(7));
}

#[test]
fn test_or_identity_and_combine() {
    assert_eq!(Or::identity(), Or(false));
    assert_eq!(Or(false).combine(Or(true)), Or(true));
    assert_eq!(Or(false).combine(Or(false)), Or(false));
}

#[test]
fn test_and_identity_and_combine() {
    assert_eq!(And::identity(), And(true));
    assert_eq!(And(true).combine(And(false)), And(false));
    assert_eq!(And(true).combine(And(true)), And(true));
}

#[test]
fn test_sum_witness_defaults() {
    assert!(!Sum::<u64>::supports_witnesses());
    assert!(!Sum::<u64>::contributes_to_witnesses(&Sum(3), &Sum(7)));
}

#[test]
fn test_and_witness_defaults() {
    assert!(!And::supports_witnesses());
    assert!(!And::contributes_to_witnesses(&And(true), &And(true)));
}

#[test]
fn test_max_witness_hooks() {
    assert!(Max::<i32>::supports_witnesses());
    assert!(Max::contributes_to_witnesses(&Max(Some(7)), &Max(Some(7))));
    assert!(!Max::contributes_to_witnesses(&Max(Some(3)), &Max(Some(7))));
    assert!(!Max::contributes_to_witnesses(&Max(None), &Max(Some(7))));
}

#[test]
fn test_min_witness_hooks() {
    assert!(Min::<i32>::supports_witnesses());
    assert!(Min::contributes_to_witnesses(&Min(Some(3)), &Min(Some(3))));
    assert!(!Min::contributes_to_witnesses(&Min(Some(7)), &Min(Some(3))));
    assert!(!Min::contributes_to_witnesses(&Min(None), &Min(Some(3))));
}

#[test]
fn test_or_witness_hooks() {
    assert!(Or::supports_witnesses());
    assert!(Or::contributes_to_witnesses(&Or(true), &Or(true)));
    assert!(!Or::contributes_to_witnesses(&Or(false), &Or(true)));
    assert!(!Or::contributes_to_witnesses(&Or(true), &Or(false)));
}

#[test]
fn test_max_helpers() {
    let size = Max(Some(42));
    assert!(size.is_valid());
    assert_eq!(size.size(), Some(&42));
    assert_eq!(size.unwrap(), 42);
}

#[test]
fn test_max_invalid() {
    let size = Max::<i32>(None);
    assert!(!size.is_valid());
    assert_eq!(size.size(), None);
}

#[test]
#[should_panic(expected = "called unwrap on invalid Max value")]
fn test_max_unwrap_panics() {
    let invalid = Max::<i32>(None);
    invalid.unwrap();
}

#[test]
fn test_min_helpers() {
    let size = Min(Some(10));
    assert!(size.is_valid());
    assert_eq!(size.size(), Some(&10));
    assert_eq!(size.unwrap(), 10);
}

#[test]
#[should_panic(expected = "called unwrap on invalid Min value")]
fn test_min_unwrap_panics() {
    let invalid = Min::<i32>(None);
    invalid.unwrap();
}

#[test]
fn test_extremum_helpers() {
    let max = Extremum::maximize(Some(10));
    assert!(max.is_valid());
    assert_eq!(max.size(), Some(&10));
    assert_eq!(max.sense, ExtremumSense::Maximize);

    let min = Extremum::minimize(Some(5));
    assert!(min.is_valid());
    assert_eq!(min.size(), Some(&5));
    assert_eq!(min.sense, ExtremumSense::Minimize);

    let invalid = Extremum::<i32>::minimize(None);
    assert!(!invalid.is_valid());
    assert_eq!(invalid.size(), None);
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
fn test_one_json() {
    let json = serde_json::to_value(vec![One, One]).unwrap();
    assert_eq!(json, serde_json::json!([1, 1]));

    let parsed: Vec<One> = serde_json::from_value(json).unwrap();
    assert_eq!(parsed, vec![One, One]);
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
fn test_extremum_aggregate_identity_and_combine() {
    // identity is Maximize(None)
    let id = Extremum::<i32>::identity();
    assert_eq!(id.sense, ExtremumSense::Maximize);
    assert_eq!(id.value, None);

    // None + Some => Some (takes rhs sense)
    let combined = Extremum::<i32>::identity().combine(Extremum::maximize(Some(5)));
    assert_eq!(combined, Extremum::maximize(Some(5)));

    // Some + None => Some (keeps lhs sense)
    let combined = Extremum::minimize(Some(3)).combine(Extremum::<i32>::identity());
    assert_eq!(combined, Extremum::minimize(Some(3)));

    // Maximize: keeps the larger
    let combined = Extremum::maximize(Some(3)).combine(Extremum::maximize(Some(7)));
    assert_eq!(combined, Extremum::maximize(Some(7)));
    let combined = Extremum::maximize(Some(7)).combine(Extremum::maximize(Some(3)));
    assert_eq!(combined, Extremum::maximize(Some(7)));

    // Minimize: keeps the smaller
    let combined = Extremum::minimize(Some(3)).combine(Extremum::minimize(Some(7)));
    assert_eq!(combined, Extremum::minimize(Some(3)));
    let combined = Extremum::minimize(Some(7)).combine(Extremum::minimize(Some(3)));
    assert_eq!(combined, Extremum::minimize(Some(3)));
}

#[test]
fn test_extremum_witness_hooks() {
    assert!(Extremum::<i32>::supports_witnesses());

    // Matching value and sense -> contributes
    assert!(Extremum::contributes_to_witnesses(
        &Extremum::maximize(Some(10)),
        &Extremum::maximize(Some(10)),
    ));

    // Different value -> does not contribute
    assert!(!Extremum::contributes_to_witnesses(
        &Extremum::maximize(Some(5)),
        &Extremum::maximize(Some(10)),
    ));

    // None config -> does not contribute
    assert!(!Extremum::contributes_to_witnesses(
        &Extremum::<i32>::maximize(None),
        &Extremum::maximize(Some(10)),
    ));
}

#[test]
fn test_extremum_display() {
    assert_eq!(format!("{}", Extremum::maximize(Some(42))), "Max(42)");
    assert_eq!(format!("{}", Extremum::<i32>::maximize(None)), "Max(None)");
    assert_eq!(format!("{}", Extremum::minimize(Some(7))), "Min(7)");
    assert_eq!(format!("{}", Extremum::<i32>::minimize(None)), "Min(None)");
}

#[test]
#[should_panic(expected = "called unwrap on invalid Extremum value")]
fn test_extremum_unwrap_panics() {
    Extremum::<i32>::minimize(None).unwrap();
}

#[test]
fn test_max_display() {
    assert_eq!(format!("{}", Max(Some(42))), "Max(42)");
    assert_eq!(format!("{}", Max::<i32>(None)), "Max(None)");
}

#[test]
fn test_min_display() {
    assert_eq!(format!("{}", Min(Some(7))), "Min(7)");
    assert_eq!(format!("{}", Min::<i32>(None)), "Min(None)");
}

#[test]
fn test_sum_display() {
    assert_eq!(format!("{}", Sum(56_u64)), "Sum(56)");
}

#[test]
fn test_or_display() {
    assert_eq!(format!("{}", Or(true)), "Or(true)");
    assert_eq!(format!("{}", Or(false)), "Or(false)");
}

#[test]
fn test_and_display() {
    assert_eq!(format!("{}", And(true)), "And(true)");
    assert_eq!(format!("{}", And(false)), "And(false)");
}
