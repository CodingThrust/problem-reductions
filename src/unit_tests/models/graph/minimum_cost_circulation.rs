use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Canonical 3-vertex two-cycle instance with signed costs.
///
/// Arcs (capacity, cost):
/// - 0: (0,1)  cap=2, cost= 2
/// - 1: (1,0)  cap=2, cost=-3     -> cycle A {0,1} per-unit cost = -1
/// - 2: (0,2)  cap=1, cost= 1
/// - 3: (2,0)  cap=1, cost=-4     -> cycle B {2,3} per-unit cost = -3
///
/// Cycle B is cheaper per unit but only has capacity 1; cycle A has more
/// capacity. The optimum pushes both cycles to capacity:
///   config = [2, 2, 1, 1]
///   cost   = 2*2 + 2*(-3) + 1*1 + 1*(-4) = 4 - 6 + 1 - 4 = -5
fn canonical_instance() -> MinimumCostCirculation {
    MinimumCostCirculation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 0), (0, 2), (2, 0)]),
        vec![2, 2, 1, 1],
        vec![2, -3, 1, -4],
    )
}

#[test]
fn test_minimum_cost_circulation_creation() {
    let problem = canonical_instance();
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_arcs(), 4);
    assert_eq!(problem.capacities(), &[2, 2, 1, 1]);
    assert_eq!(problem.costs(), &[2, -3, 1, -4]);
    assert_eq!(problem.dims(), vec![3, 3, 2, 2]);
    assert_eq!(
        <MinimumCostCirculation as Problem>::NAME,
        "MinimumCostCirculation"
    );
}

#[test]
fn test_minimum_cost_circulation_evaluate_optimal() {
    let problem = canonical_instance();
    let config = vec![2, 2, 1, 1];
    assert!(problem.is_feasible(&config).unwrap());
    assert_eq!(problem.total_cost(&config).unwrap(), -5);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(-5)));
}

#[test]
fn test_minimum_cost_circulation_evaluate_zero_circulation() {
    let problem = canonical_instance();
    let config = vec![0, 0, 0, 0];
    assert!(problem.is_feasible(&config).unwrap());
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(0)));
}

#[test]
fn test_minimum_cost_circulation_evaluate_cycle_a_only() {
    let problem = canonical_instance();
    // Push cycle A to capacity, leave cycle B empty: [2, 2, 0, 0]
    // cost = 2*2 + 2*(-3) + 0 + 0 = -2
    let config = vec![2, 2, 0, 0];
    assert!(problem.is_feasible(&config).unwrap());
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(-2)));
}

#[test]
fn test_minimum_cost_circulation_evaluate_cycle_b_only() {
    let problem = canonical_instance();
    // Push cycle B to capacity, leave cycle A empty: [0, 0, 1, 1]
    // cost = 0 + 0 + 1 + (-4) = -3
    let config = vec![0, 0, 1, 1];
    assert!(problem.is_feasible(&config).unwrap());
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(-3)));
}

#[test]
fn test_minimum_cost_circulation_evaluate_infeasible_capacity() {
    let problem = canonical_instance();
    // Arc 0 has capacity 2, but g(0) = 3 violates it.
    let config = vec![3, 0, 0, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_minimum_cost_circulation_evaluate_infeasible_conservation() {
    let problem = canonical_instance();
    // [2, 1, 0, 0]: at vertex 1, inflow = 2 (from arc 0), outflow = 1
    // (via arc 1); balance != 0, so infeasible.
    let config = vec![2, 1, 0, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_minimum_cost_circulation_evaluate_wrong_config_length() {
    let problem = canonical_instance();
    assert_eq!(problem.evaluate(&[0; 3]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&[0; 5]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&[]).unwrap(), Min(None));
}

#[test]
fn test_minimum_cost_circulation_solver_canonical() {
    let problem = canonical_instance();
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(&problem)
        .unwrap()
        .expect("canonical instance must be feasible");
    assert_eq!(problem.total_cost(&witness).unwrap(), -5);
    // The unique optimum pushes both cycles to capacity.
    assert_eq!(witness, vec![2, 2, 1, 1]);
}

#[test]
fn test_minimum_cost_circulation_negative_cycle_beats_zero() {
    // Smoke test: a single negative-cost cycle on its own must beat the
    // trivial zero circulation. Graph is one cycle 0 -> 1 -> 0 with
    // per-unit cost 1 + (-3) = -2, capacity 1.
    let problem = MinimumCostCirculation::new(
        DirectedGraph::new(2, vec![(0, 1), (1, 0)]),
        vec![1, 1],
        vec![1, -3],
    );
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(&problem)
        .unwrap()
        .expect("instance must be feasible");
    assert_eq!(problem.total_cost(&witness).unwrap(), -2);
    assert_eq!(witness, vec![1, 1]);
    // And zero is feasible but strictly worse.
    assert_eq!(problem.evaluate(&[0, 0]).unwrap(), Min(Some(0)));
}

#[test]
fn test_minimum_cost_circulation_issue_example_1030() {
    // Verbatim instance from issue #1030: vertices {0,1}, arcs
    //   0->1 cap=2 cost= 3
    //   1->0 cap=1 cost=-5
    // Bottleneck is the back-arc (cap=1), so the optimum sends one unit
    // around the cycle: cost = 1*3 + 1*(-5) = -2.
    let problem = MinimumCostCirculation::new(
        DirectedGraph::new(2, vec![(0, 1), (1, 0)]),
        vec![2, 1],
        vec![3, -5],
    );
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(&problem)
        .unwrap()
        .expect("instance must be feasible");
    assert_eq!(witness, vec![1, 1]);
    assert_eq!(problem.total_cost(&witness).unwrap(), -2);
    assert_eq!(problem.evaluate(&[0, 0]).unwrap(), Min(Some(0)));
}

#[test]
fn test_minimum_cost_circulation_serialization() {
    let problem = canonical_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumCostCirculation = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_arcs(), 4);
    assert_eq!(deserialized.capacities(), &[2, 2, 1, 1]);
    assert_eq!(deserialized.costs(), &[2, -3, 1, -4]);
    // Optimal config evaluates identically after roundtrip.
    assert_eq!(
        deserialized.evaluate(&[2, 2, 1, 1]).unwrap(),
        problem.evaluate(&[2, 2, 1, 1]).unwrap()
    );
}
