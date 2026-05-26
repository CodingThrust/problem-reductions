use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Canonical 4-vertex diamond instance from issue #1031.
///
/// Max-flow value = 3, min cost among value-3 flows = 7.
fn canonical_source() -> MinimumCostMaximumFlow {
    MinimumCostMaximumFlow::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![2, 1, 1, 1, 2],
        vec![1, 0, 0, 1, 2],
    )
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_structure() {
    let source = canonical_source();
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    let target = reduction.target_problem();

    // Vertices preserved, arcs gain exactly one return arc.
    assert_eq!(target.num_vertices(), source.num_vertices());
    assert_eq!(target.num_arcs(), source.num_arcs() + 1);

    // First m capacities/costs match the source.
    let m = source.num_arcs();
    assert_eq!(&target.capacities()[..m], source.capacities());
    assert_eq!(&target.costs()[..m], source.costs());

    // Return arc parameters.
    // U = capacities of arcs leaving the source (0).  Arcs (0,1) and (0,2)
    // have capacities 2 and 1, so U = 3.
    assert_eq!(target.capacities()[m], 3);
    // B = 1 + sum of original costs = 1 + (1 + 0 + 0 + 1 + 2) = 5.
    assert_eq!(target.costs()[m], -5);

    // Return arc endpoints: (sink, source) = (3, 0).
    let target_arcs = target.graph().arcs();
    assert_eq!(target_arcs[m], (source.sink(), source.source()));
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_closed_loop() {
    let source = canonical_source();
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "canonical diamond MCMF -> MCC",
    );
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_bottleneck() {
    // Single bottleneck at the source: arc (0,1) with cap=1 forces max
    // flow value = 1. Two paths 1->3 (cost 1) and 1->2->3 (cost 2+3=5)
    // ensure the cheaper path is selected.
    let source = MinimumCostMaximumFlow::new(
        DirectedGraph::new(4, vec![(0, 1), (1, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![1, 1, 1, 1],
        vec![0, 2, 1, 3],
    );
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "bottleneck MCMF -> MCC",
    );

    // Brute-force the target and confirm the extracted source flow has
    // value 1 and cost 1 (the cheaper 1->3 path).
    let solver = BruteForce::new();
    let target_witness = solver.find_witness(reduction.target_problem()).unwrap();
    let extracted = reduction.extract_solution(&target_witness);
    assert_eq!(source.flow_value(&extracted), 1);
    assert_eq!(source.total_cost(&extracted), 1);
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_parallel_arcs() {
    // Parallel arcs with different costs from 0 to 1, single sink arc.
    // The cheaper parallel arc must be preferred.
    let source = MinimumCostMaximumFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (0, 1), (1, 2)]),
        0,
        2,
        vec![1, 1, 1],
        vec![5, 1, 0],
    );
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    let target = reduction.target_problem();

    // U = cap[(0,1)#1] + cap[(0,1)#2] = 1 + 1 = 2.
    assert_eq!(target.capacities()[source.num_arcs()], 2);
    // B = 1 + 5 + 1 + 0 = 7.
    assert_eq!(target.costs()[source.num_arcs()], -7);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "parallel arcs MCMF -> MCC",
    );

    // Max flow value = 1 (limited by arc (1,2) capacity), cheaper
    // parallel arc has cost 1, so optimal source cost = 1.
    let solver = BruteForce::new();
    let target_witness = solver.find_witness(target).unwrap();
    let extracted = reduction.extract_solution(&target_witness);
    assert_eq!(source.flow_value(&extracted), 1);
    assert_eq!(source.total_cost(&extracted), 1);
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_unused_low_cost_arc() {
    // Arc (1,2) is cheap (cost 0) but cannot help reach the sink t=3
    // because vertex 2 has no out-arc to t. The reduction must still
    // produce the correct projection.
    let source = MinimumCostMaximumFlow::new(
        DirectedGraph::new(4, vec![(0, 1), (1, 2), (1, 3)]),
        0,
        3,
        vec![1, 1, 1],
        vec![1, 0, 1],
    );
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "unused low-cost arc MCMF -> MCC",
    );
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_zero_capacity_arc() {
    // A zero-capacity arc must remain feasible but contribute nothing.
    let source = MinimumCostMaximumFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        0,
        2,
        vec![1, 1, 0],
        vec![0, 0, 0],
    );
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    let target = reduction.target_problem();
    // Return arc capacity = cap leaving source = 1 + 0 = 1.
    assert_eq!(target.capacities()[source.num_arcs()], 1);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "zero-capacity arc MCMF -> MCC",
    );

    let solver = BruteForce::new();
    let target_witness = solver.find_witness(target).unwrap();
    let extracted = reduction.extract_solution(&target_witness);
    assert_eq!(source.flow_value(&extracted), 1);
    // Zero-capacity arc must be 0 in the extracted flow.
    assert_eq!(extracted[2], 0);
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_value_priority_over_cost() {
    // A cheaper sub-maximum flow exists but must be rejected because
    // value has lex priority.
    //
    // Network: 0 -> 1 (cap=2, cost=0), 1 -> 2 (cap=2, cost=10).
    // Max flow = 2 with cost 20. A flow of value 1 has cost 10
    // (cheaper in raw cost but lower value), so the lex-optimum is
    // value 2.
    let source = MinimumCostMaximumFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        0,
        2,
        vec![2, 2],
        vec![0, 10],
    );
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);

    let solver = BruteForce::new();
    let target_witness = solver.find_witness(reduction.target_problem()).unwrap();
    let extracted = reduction.extract_solution(&target_witness);
    assert_eq!(source.flow_value(&extracted), 2);
    assert_eq!(source.total_cost(&extracted), 20);

    // The target circulation value uses cost 20 + 2*(-B) where
    // B = 1 + 0 + 10 = 11, so optimum = 20 - 22 = -2.
    let target_value = reduction.target_problem().evaluate(&target_witness);
    assert_eq!(target_value, Min(Some(-2)));
}

#[test]
fn test_minimumcostmaximumflow_to_minimumcostcirculation_extract_solution_length() {
    let source = canonical_source();
    let reduction = ReduceTo::<MinimumCostCirculation>::reduce_to(&source);
    // Provide a dummy target config of the right length; extract_solution
    // must truncate to num_original_arcs.
    let m = source.num_arcs();
    let mut padded = vec![0_usize; m + 1];
    for (i, v) in padded.iter_mut().enumerate().take(m) {
        *v = i % 2;
    }
    let extracted = reduction.extract_solution(&padded);
    assert_eq!(extracted.len(), m);
    assert_eq!(extracted, padded[..m].to_vec());
}
