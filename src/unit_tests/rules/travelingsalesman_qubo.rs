use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::solvers::SolveOutcome;
use crate::traits::Problem;
use crate::types::Min;

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
        let extracted = reduction
            .recover_result(
                &tsp,
                SolveOutcome::optimal(reduction.target_problem(), (sol).clone()).unwrap(),
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution");
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
        let extracted = reduction
            .recover_result(
                &tsp,
                SolveOutcome::optimal(reduction.target_problem(), (sol).clone()).unwrap(),
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution");
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
    assert_eq!(reduction3.target_problem().num_variables().unwrap(), 9);

    // K4: n=4, QUBO should have n^2 = 16 variables
    let graph4 = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tsp4 = TravelingSalesman::new(graph4, vec![1i64; 6]);
    let reduction4 = ReduceTo::<QUBO<i64>>::reduce_to(&tsp4).expect("reduction should succeed");
    assert_eq!(reduction4.target_problem().num_variables().unwrap(), 16);
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
fn signed_and_small_tours_recover_all_optima_or_infeasibility() {
    let cases = [
        (0, vec![], vec![]),
        (1, vec![], vec![]),
        (1, vec![(0, 0), (0, 0)], vec![4, -2]),
        (2, vec![(0, 1)], vec![1]),
        (2, vec![(0, 1), (0, 1), (0, 1)], vec![4, -2, 1]),
        (3, vec![(0, 1), (1, 2)], vec![-5, 2]),
        (3, vec![(0, 1), (1, 2), (0, 2)], vec![-5, 2, 1]),
        (
            3,
            vec![(0, 1), (0, 1), (1, 2), (0, 2), (1, 1)],
            vec![4, -5, 2, 1, -100],
        ),
        (
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
            vec![-9, 1, 2, 3, 4, -8],
        ),
    ];
    for (n, edges, weights) in cases {
        let m = edges.len();
        let source = TravelingSalesman::new(SimpleGraph::new(n, edges), weights);
        let expected = (0..1usize << m)
            .filter_map(|bits| {
                source
                    .evaluate(&(0..m).map(|i| bits & (1 << i) != 0).collect())
                    .unwrap()
                    .0
            })
            .min();
        let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
        let entry = inventory::iter::<crate::rules::ReductionEntry>
            .into_iter()
            .find(|entry| entry.source_name == "TravelingSalesman" && entry.target_name == "QUBO")
            .unwrap();
        let chain =
            crate::rules::ReductionChain::execute(&source, &[entry.reduce_fn.unwrap()]).unwrap();

        let solutions = BruteForce::new()
            .find_all_witnesses(reduction.target_problem())
            .unwrap();
        assert!(!solutions.is_empty());
        for solution in solutions {
            let completed = chain
                .recover_result_json(
                    &source,
                    serde_json::json!({"status": "optimal", "solution": solution}),
                )
                .unwrap()
                .0;
            assert_eq!(
                matches!(completed, SolveOutcome::Optimal { .. }),
                expected.is_some()
            );
            assert_eq!(
                reduction.map_value(reduction.target_problem().evaluate(&solution).unwrap()),
                Min(expected)
            );
            if expected.is_some() {
                assert_eq!(
                    source
                        .evaluate(
                            &reduction
                                .recover_result(
                                    &source,
                                    SolveOutcome::optimal(
                                        reduction.target_problem(),
                                        solution.clone()
                                    )
                                    .unwrap()
                                )
                                .map(|result| result.into_solution().expect(
                                    "qualifying target result must recover a source solution"
                                ))
                                .unwrap()
                        )
                        .unwrap(),
                    Min(expected)
                );
            }
        }
        assert_eq!(reduction.map_value(Min(None)), Min(None));
    }
}

/// Decode a position-encoded assignment (`x[v * n + p]`: vertex `v` at position `p`)
/// into the source edge selection, or `None` when it is not a permutation matrix.
fn decode_tour(
    source: &TravelingSalesman<SimpleGraph, i64>,
    assignment: &[bool],
) -> Option<Vec<bool>> {
    let n = source.num_vertices();
    let order: Vec<usize> = (0..n)
        .map(|p| {
            let mut at_position = (0..n).filter(|&v| assignment[v * n + p]);
            at_position.next().filter(|_| at_position.next().is_none())
        })
        .collect::<Option<_>>()?;
    if assignment.iter().filter(|&&bit| bit).count() != n || (0..n).any(|v| !order.contains(&v)) {
        return None;
    }
    Some(
        source
            .edges()
            .into_iter()
            .map(|(u, v, _)| {
                (0..n).any(|i| {
                    let (a, b) = (order[i], order[(i + 1) % n]);
                    (a, b) == (u, v) || (a, b) == (v, u)
                })
            })
            .collect(),
    )
}

#[test]
fn test_travelingsalesman_to_qubo_feasible_target_incumbents() {
    // K4 with tour costs 22, 22 and 10, so most valid tours are suboptimal.
    let tsp = TravelingSalesman::new(SimpleGraph::complete(4), vec![9i64, 1, 2, 3, 4, 8]);
    let optimum = BruteForce::new().solve(&tsp).unwrap().unwrap();
    assert_eq!(tsp.evaluate(&optimum).unwrap(), Min(Some(10)));

    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&tsp).unwrap();
    let qubo = reduction.target_problem();
    let mut suboptimal_tours = 0;
    let mut rejected = 0;
    for bits in 0..1usize << 16 {
        let candidate: Vec<bool> = (0..16).map(|i| bits & (1 << i) != 0).collect();
        let expected = decode_tour(&tsp, &candidate);
        let recovered =
            reduction.recover_result(&tsp, SolveOutcome::feasible(qubo, candidate).unwrap());
        match expected {
            Some(tour) => {
                let evaluation = tsp.evaluate(&tour).unwrap();
                suboptimal_tours += usize::from(evaluation == Min(Some(22)));
                assert_eq!(
                    recovered,
                    Ok(SolveOutcome::Feasible {
                        solution: tour,
                        evaluation,
                    })
                );
            }
            None => {
                rejected += 1;
                assert_eq!(
                    recovered,
                    Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
                );
            }
        }
    }
    // 4! position assignments, two thirds of which encode a cost-22 tour.
    assert_eq!(suboptimal_tours, 16);
    assert_eq!(rejected, (1 << 16) - 24);
}

#[test]
fn test_travelingsalesman_to_qubo_infeasible_source_recovers_infeasible() {
    // A path on four vertices has no Hamiltonian cycle.
    let tsp = TravelingSalesman::new(SimpleGraph::path(4), vec![1i64, 2, 3]);
    assert_eq!(BruteForce::new().solve(&tsp).unwrap(), None);

    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&tsp).unwrap();
    let qubo = reduction.target_problem();
    let optimum = BruteForce::new().solve(qubo).unwrap().unwrap();
    assert_eq!(
        reduction.recover_result(&tsp, SolveOutcome::optimal(qubo, optimum.clone()).unwrap()),
        Ok(SolveOutcome::Infeasible)
    );
    // The same assignment without an optimality proof establishes nothing.
    assert_eq!(
        reduction.recover_result(&tsp, SolveOutcome::feasible(qubo, optimum).unwrap()),
        Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
    );
}
