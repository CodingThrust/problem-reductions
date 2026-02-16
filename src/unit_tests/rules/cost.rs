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

