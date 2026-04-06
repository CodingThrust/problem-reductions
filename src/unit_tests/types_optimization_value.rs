use crate::types::{Max, Min, OptimizationValue};

#[test]
fn test_min_meets_bound_feasible() {
    assert!(Min::<i32>::meets_bound(&Min(Some(3)), &5));
}

#[test]
fn test_min_meets_bound_exact() {
    assert!(Min::<i32>::meets_bound(&Min(Some(5)), &5));
}

#[test]
fn test_min_meets_bound_exceeds() {
    assert!(!Min::<i32>::meets_bound(&Min(Some(7)), &5));
}

#[test]
fn test_min_meets_bound_infeasible() {
    assert!(!Min::<i32>::meets_bound(&Min(None), &5));
}

#[test]
fn test_max_meets_bound_feasible() {
    assert!(Max::<i32>::meets_bound(&Max(Some(7)), &5));
}

#[test]
fn test_max_meets_bound_exact() {
    assert!(Max::<i32>::meets_bound(&Max(Some(5)), &5));
}

#[test]
fn test_max_meets_bound_below() {
    assert!(!Max::<i32>::meets_bound(&Max(Some(3)), &5));
}

#[test]
fn test_max_meets_bound_infeasible() {
    assert!(!Max::<i32>::meets_bound(&Max(None), &5));
}
