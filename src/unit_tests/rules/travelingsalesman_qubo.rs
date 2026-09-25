use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

fn assert_tour_recovery(source: TravelingSalesman<SimpleGraph, i64>) {
    let solver = BruteForce::new();
    let expected = solver
        .solve(&source)
        .unwrap()
        .map(|solution| source.evaluate(&solution).unwrap())
        .unwrap_or(Min(None));
    let result = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    assert_eq!(
        result.target_problem().num_vars(),
        source.num_vertices().pow(2)
    );
    for witness in solver.find_all_witnesses(result.target_problem()).unwrap() {
        let energy = result.target_problem().evaluate(&witness).unwrap();
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&result, energy),
            expected
        );
        match expected.0 {
            Some(_) => assert_eq!(
                source
                    .evaluate(&result.extract_solution(&witness).unwrap())
                    .unwrap(),
                expected
            ),
            None => assert!(result.extract_solution(&witness).is_err()),
        }
    }
}

#[test]
fn test_signed_tour_costs_preserve_all_optima() {
    for encoding in 0..27 {
        let mut digits = encoding;
        let weights = (0..3)
            .map(|_| {
                let weight = [-3, 0, 2][digits % 3];
                digits /= 3;
                weight
            })
            .collect();
        assert_tour_recovery(TravelingSalesman::new(SimpleGraph::complete(3), weights));
    }
    assert_tour_recovery(TravelingSalesman::new(SimpleGraph::path(3), vec![-5, 1]));
    assert_tour_recovery(TravelingSalesman::new(
        SimpleGraph::complete(4),
        vec![-9, 1, 2, 3, -4, 8],
    ));
}

#[test]
fn test_tours_with_parallel_edges_and_loops() {
    for weights in [
        vec![1, 2, 3, 9, -100],
        vec![9, 2, 3, 1, -100],
        vec![1, 2, 3, 1, -100],
    ] {
        assert_tour_recovery(TravelingSalesman::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0), (1, 0), (0, 0)]),
            weights,
        ));
    }
}

#[test]
fn test_small_tours_follow_edge_set_definition() {
    for (n, edges, weights) in [
        (0, vec![], vec![]),
        (1, vec![], vec![]),
        (1, vec![(0, 0), (0, 0)], vec![8, -2]),
        (1, vec![(0, 0)], vec![i64::MIN]),
        (2, vec![(0, 1)], vec![1]),
        (
            2,
            vec![(0, 1), (1, 0), (0, 1), (0, 0)],
            vec![8, -2, 1, -100],
        ),
    ] {
        assert_tour_recovery(TravelingSalesman::new(SimpleGraph::new(n, edges), weights));
    }
}

#[test]
fn test_tour_numeric_limits_fail_during_construction() {
    for source in [
        TravelingSalesman::new(SimpleGraph::complete(3), vec![i64::MIN, 0, 0]),
        TravelingSalesman::new(SimpleGraph::complete(3), vec![i64::MAX, 1, 1]),
        TravelingSalesman::new(SimpleGraph::complete(3), vec![i64::MAX / 10; 3]),
        TravelingSalesman::new(SimpleGraph::new(2, vec![(0, 1), (1, 0)]), vec![i64::MAX; 2]),
    ] {
        assert!(matches!(
            ReduceTo::<QUBO<i64>>::reduce_to(&source),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn test_tour_value_mapping_and_invalid_configurations() {
    let source = TravelingSalesman::new(SimpleGraph::complete(3), vec![-3, 0, 2]);
    let result = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    for config in [
        vec![],
        vec![false; 9],
        vec![true; 9],
        vec![true, true, true, false, false, false, false, false, false],
    ] {
        assert!(result.extract_solution(&config).is_err());
    }
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&result, Min(None)),
        Min(None)
    );
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&result, Min(Some(i64::MAX))),
        Min(None)
    );
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&result, Min(Some(i64::MIN))),
        Min(Some(i64::MIN + result.objective_offset))
    );
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| entry.source_name == "TravelingSalesman" && entry.target_name == "QUBO")
        .unwrap();
    let dynamic = (entry.reduce_aggregate_fn.unwrap())(&source).unwrap();
    let optimum = BruteForce::new()
        .solve(result.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        dynamic.extract_value_from_solution_dyn(&optimum).unwrap(),
        serde_json::json!(-1)
    );
}

#[test]
fn test_travelingsalesman_to_qubo_closed_loop() {
    // K3 complete graph with weights [1, 2, 3]
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let tsp = TravelingSalesman::new(graph, vec![1i64, 2, 3]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&tsp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // All QUBO solutions should extract to valid TSP solutions
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        let metric = tsp.evaluate(&extracted).unwrap();
        assert!(metric.is_valid(), "Extracted solution should be valid");
        // K3 has only one Hamiltonian cycle (all 3 edges), cost = 1+2+3 = 6
        assert_eq!(metric, Min(Some(6)));
    }

    // There are multiple QUBO optima (different position assignments for the same tour),
    // but they should all extract to valid tours with cost 6.
    assert!(
        !qubo_solutions.is_empty(),
        "Should find at least one QUBO solution"
    );
}

#[test]
fn test_travelingsalesman_to_qubo_k4() {
    // K4 with unit weights
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tsp = TravelingSalesman::new(graph, vec![1i64; 6]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&tsp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // Every Hamiltonian cycle in K4 uses exactly 4 edges, so cost = 4
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        let metric = tsp.evaluate(&extracted).unwrap();
        assert!(metric.is_valid(), "Extracted solution should be valid");
        assert_eq!(metric, Min(Some(4)));
    }

    // K4 has 3 distinct Hamiltonian cycles, but each has multiple position encodings
    // (4 rotations x 2 directions = 8 QUBO solutions per cycle, total 24).
    // Just verify we get a non-trivial number of solutions.
    assert!(
        qubo_solutions.len() >= 3,
        "Should find at least 3 QUBO solutions for K4"
    );
}

#[test]
fn test_travelingsalesman_to_qubo_sizes() {
    // K3: n=3, QUBO should have n^2 = 9 variables
    let graph3 = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let tsp3 = TravelingSalesman::new(graph3, vec![1i64; 3]);
    let reduction3 = ReduceTo::<QUBO<i64>>::reduce_to(&tsp3).expect("reduction should succeed");
    assert_eq!(reduction3.target_problem().num_variables(), 9);

    // K4: n=4, QUBO should have n^2 = 16 variables
    let graph4 = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tsp4 = TravelingSalesman::new(graph4, vec![1i64; 6]);
    let reduction4 = ReduceTo::<QUBO<i64>>::reduce_to(&tsp4).expect("reduction should succeed");
    assert_eq!(reduction4.target_problem().num_variables(), 16);
}

#[test]
fn test_travelingsalesman_to_qubo_weighted_corpus_regression() {
    // Unequal tour costs expose a transposed vertex/position permutation.
    let tsp = TravelingSalesman::new(SimpleGraph::complete(4), vec![9i64, 1, 2, 3, 4, 8]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&tsp).unwrap();
    crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target(
        &tsp,
        &reduction,
        "weighted TSP position encoding",
    );
}

#[test]
fn test_tour_penalty_constant_must_fit_valid_tour_energy() {
    // n = 3 and all weights -w give A = 3w + 1, so valid tours have energy -6A.
    let too_negative =
        TravelingSalesman::new(SimpleGraph::complete(3), vec![-600_000_000_000_000_000; 3]);
    assert!(matches!(
        ReduceTo::<QUBO<i64>>::reduce_to(&too_negative),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));

    let source =
        TravelingSalesman::new(SimpleGraph::complete(3), vec![-500_000_000_000_000_000; 3]);
    let result = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    let identity_tour = vec![true, false, false, false, true, false, false, false, true];
    let energy = result.target_problem().evaluate(&identity_tour).unwrap();
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&result, energy),
        Min(Some(-1_500_000_000_000_000_000))
    );
}
