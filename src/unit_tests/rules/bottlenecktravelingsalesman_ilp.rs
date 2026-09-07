use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn k4_btsp() -> BottleneckTravelingSalesman {
    BottleneckTravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1, 3, 2, 4, 2, 1],
    )
}

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = k4_btsp();
    let reduction: ReductionBTSPToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // n=4, m=6: 16 position bits, 48 edge-use bits, 6 maximum selectors.
    assert_eq!(ilp.num_vars(), 70);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_bottlenecktravelingsalesman_to_ilp_closed_loop() {
    let problem = k4_btsp();
    let bf = BruteForce::new();
    let bf_solution = bf.solve(&problem).unwrap().expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution).unwrap();

    let reduction: ReductionBTSPToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = problem.evaluate(&extracted).unwrap();

    assert!(
        ilp_value.is_valid(),
        "Extracted solution should be a valid Hamiltonian cycle"
    );
    assert_eq!(
        ilp_value, bf_value,
        "ILP and brute-force should agree on optimal value"
    );
}

#[test]
fn test_bottlenecktravelingsalesman_to_ilp_c4() {
    // C4 with varying weights: bottleneck = max weight in the only cycle
    let problem = BottleneckTravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
        vec![1, 2, 3, 4],
    );
    let bf = BruteForce::new();
    let bf_solution = bf.solve(&problem).unwrap().expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution).unwrap();

    let reduction: ReductionBTSPToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = problem.evaluate(&extracted).unwrap();

    assert!(ilp_value.is_valid());
    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_solution_extraction() {
    let problem = k4_btsp();
    let reduction: ReductionBTSPToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let metric = problem.evaluate(&extracted).unwrap();
    assert!(metric.is_valid());
}

#[test]
fn test_no_hamiltonian_cycle_infeasible() {
    // Path graph: no Hamiltonian cycle
    let problem = BottleneckTravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionBTSPToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(reduction.target_problem());
    assert!(
        result.is_err(),
        "Path graph should have no Hamiltonian cycle"
    );
}

#[test]
fn test_bottlenecktravelingsalesman_to_ilp_bf_vs_ilp() {
    let problem = k4_btsp();
    let reduction: ReductionBTSPToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

fn tour_witness(
    source: &BottleneckTravelingSalesman,
    tour: &[usize],
    edge_order: &[usize],
) -> Vec<i64> {
    let n = source.num_vertices();
    let m = source.num_edges();
    let edges = source.graph().edges();
    let weights = source.weights();
    let mut config = vec![0; n * n + 2 * m * n + m];
    for (p, &v) in tour.iter().enumerate() {
        config[v * n + p] = 1;
        let edge = edge_order[p];
        let direction = usize::from(edges[edge] != (v, tour[(p + 1) % n]));
        config[n * n + 2 * (edge * n + p) + direction] = 1;
    }
    let maximum = *edge_order
        .iter()
        .max_by_key(|&&edge| weights[edge])
        .unwrap();
    config[n * n + 2 * m * n + maximum] = 1;
    config
}

#[test]
fn test_bottleneck_ilp_signed_full_range_and_native_cycles() {
    for (n, edges, weights, tour, edge_order) in [
        (
            3,
            vec![(0, 1), (1, 2), (2, 0)],
            vec![-5, -3, -4],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ),
        (
            3,
            vec![(0, 1), (1, 2), (2, 0)],
            vec![i64::MIN; 3],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ),
        (
            3,
            vec![(0, 1), (1, 2), (2, 0)],
            vec![i64::MIN, i64::MAX, 0],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ),
        (1, vec![(0, 0)], vec![-7], vec![0], vec![0]),
        (2, vec![(0, 1), (1, 0)], vec![2, 3], vec![0, 1], vec![0, 1]),
        (
            3,
            vec![(0, 1), (0, 1), (1, 2), (2, 0)],
            vec![9, -5, -3, -4],
            vec![0, 1, 2],
            vec![1, 2, 3],
        ),
    ] {
        let source = BottleneckTravelingSalesman::new(SimpleGraph::new(n, edges), weights);
        let result = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
        let witness = tour_witness(&source, &tour, &edge_order);
        let extracted = result.extract_solution(&witness).unwrap();
        let expected = source.evaluate(&extracted).unwrap().unwrap();
        assert_eq!(
            result.target_problem().evaluate(&witness).unwrap().value,
            Some(expected)
        );
        assert_eq!(extracted.iter().filter(|&&x| x).count(), n);
        for variable in 0..witness.len() {
            let mut invalid = witness.clone();
            invalid[variable] = 2;
            assert!(result.extract_solution(&invalid).is_err());
        }
        assert!(result
            .extract_solution(&witness[..witness.len() - 1].to_vec())
            .is_err());
    }
}

#[test]
fn test_bottleneck_ilp_maximum_must_be_used_and_dominate() {
    let source = k4_btsp();
    let result = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
    let mut config = tour_witness(&source, &[0, 1, 2, 3], &[0, 3, 5, 2]);
    let selector = 4 * 4 + 2 * 6 * 4;
    config[selector..].fill(0);
    assert!(result.extract_solution(&config).is_err());
    config[selector] = 1; // used, but lower than the maximum edge
    assert!(result.extract_solution(&config).is_err());
    config[selector] = 0;
    config[selector + 1] = 1; // unused
    assert!(result.extract_solution(&config).is_err());
}

#[test]
fn test_bottleneck_ilp_empty_and_single_edge_are_infeasible() {
    for (n, edges, weights) in [(0, vec![], vec![]), (2, vec![(0, 1)], vec![1])] {
        let source = BottleneckTravelingSalesman::new(SimpleGraph::new(n, edges), weights);
        let result = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
        assert!(matches!(
            ILPSolver::new().solve(result.target_problem()),
            Err(crate::solvers::ILPSolveError::Infeasible)
        ));
    }
}

#[test]
fn test_bottleneck_ilp_dimensions_and_malformed_weights() {
    assert_eq!(
        ReductionBTSPToILP::dimensions(4, 6).unwrap(),
        (16, 48, 70, 197)
    );
    assert_eq!(ReductionBTSPToILP::dimensions(0, 0).unwrap(), (0, 0, 0, 1));
    for (n, m) in [(usize::MAX, 0), (1, usize::MAX), (0, usize::MAX)] {
        assert!(ReductionBTSPToILP::dimensions(n, m).is_err());
    }
    let source: BottleneckTravelingSalesman = serde_json::from_value(serde_json::json!({
        "graph": {"num_vertices": 0, "edges": []}, "edge_weights": [1]
    }))
    .unwrap();
    assert!(ReduceTo::<ILP<i64>>::reduce_to(&source).is_err());
}
