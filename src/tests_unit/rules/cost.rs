use super::*;
use crate::polynomial::Polynomial;

fn test_overhead() -> ReductionOverhead {
    ReductionOverhead::new(vec![
        ("n", Polynomial::var("n").scale(2.0)),
        ("m", Polynomial::var("m")),
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
fn test_minimize_weighted() {
    let cost_fn = MinimizeWeighted(vec![("n", 1.0), ("m", 2.0)]);
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    // output n = 20, output m = 5
    // cost = 1.0 * 20 + 2.0 * 5 = 30
    assert_eq!(cost_fn.edge_cost(&overhead, &size), 30.0);
}

#[test]
fn test_minimize_steps() {
    let cost_fn = MinimizeSteps;
    let size = ProblemSize::new(vec![("n", 100)]);
    let overhead = test_overhead();

    assert_eq!(cost_fn.edge_cost(&overhead, &size), 1.0);
}

#[test]
fn test_minimize_max() {
    let cost_fn = MinimizeMax(vec!["n", "m"]);
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    // output n = 20, output m = 5
    // max(20, 5) = 20
    assert_eq!(cost_fn.edge_cost(&overhead, &size), 20.0);
}

#[test]
fn test_minimize_lexicographic() {
    let cost_fn = MinimizeLexicographic(vec!["n", "m"]);
    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    let overhead = test_overhead();

    // output n = 20, output m = 5
    // cost = 20 * 1.0 + 5 * 1e-10 = 20.0000000005
    let cost = cost_fn.edge_cost(&overhead, &size);
    assert!(cost > 20.0 && cost < 20.001);
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
fn test_minimize_max_empty() {
    let cost_fn = MinimizeMax(vec![]);
    let size = ProblemSize::new(vec![("n", 10)]);
    let overhead = test_overhead();

    assert_eq!(cost_fn.edge_cost(&overhead, &size), 0.0);
}
