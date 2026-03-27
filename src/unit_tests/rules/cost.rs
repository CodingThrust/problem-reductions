use super::*;
use crate::expr::Expr;

fn test_overhead() -> ReductionOverhead {
    ReductionOverhead::new(vec![
        ("n", Expr::Const(2.0) * Expr::Var("n")),
        ("m", Expr::Var("m")),
    ])
}

#[test]
fn test_minimize_single() {
    let cost_fn = Minimize("n");
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    assert_eq!(cost_fn.edge_cost(&overhead, &size), 20.0); // 2 * 10
}

#[test]
fn test_minimize_steps() {
    let cost_fn = MinimizeSteps;
    let size = ProblemSize::new(vec![("n", 100)]);
    let overhead = test_overhead();

    assert_eq!(cost_fn.edge_cost(&overhead, &size), 1.0);
}

#[test]
fn test_custom_cost() {
    let cost_fn = CustomCost(|overhead: &ReductionOverhead, size: &ProblemSize| {
        let output = overhead.evaluate_output_size(size);
        (output.get("n").unwrap_or(0) + output.get("m").unwrap_or(0)) as f64
    });
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    // output n = 20, output m = 5
    // custom = 20 + 5 = 25
    assert_eq!(cost_fn.edge_cost(&overhead, &size), 25.0);
}

#[test]
fn test_minimize_missing_field() {
    let cost_fn = Minimize("nonexistent");
    let size = ProblemSize::new(vec![("n", 10)]);
    let overhead = test_overhead();

    assert_eq!(cost_fn.edge_cost(&overhead, &size), 0.0);
}

#[test]
fn test_minimize_output_size() {
    let cost_fn = MinimizeOutputSize;
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    // output n = 20, output m = 5 → total = 25
    assert_eq!(cost_fn.edge_cost(&overhead, &size), 25.0);
}

#[test]
fn test_minimize_steps_then_overhead() {
    let cost_fn = MinimizeStepsThenOverhead;
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    let cost = cost_fn.edge_cost(&overhead, &size);
    // Should be dominated by the step weight (1e9) with small overhead tiebreaker
    assert!(cost > 1e8, "step weight should dominate");
    assert!(cost < 2e9, "should be roughly 1e9 + small tiebreaker");

    // Two edges with different overhead should have different costs
    let small_overhead =
        ReductionOverhead::new(vec![("n", Expr::Const(1.0)), ("m", Expr::Const(1.0))]);
    let cost_small = cost_fn.edge_cost(&small_overhead, &size);
    // Both have the same step weight but different tiebreakers
    assert!(cost > cost_small, "larger overhead should cost more");
}

#[test]
fn test_problem_size_total() {
    let size = ProblemSize::new(vec![("a", 3), ("b", 7), ("c", 10)]);
    assert_eq!(size.total(), 20);
    assert_eq!(ProblemSize::new(vec![]).total(), 0);
}
