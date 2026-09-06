use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::variant::{K2, K3};

#[test]
fn test_kcoloring_to_qubo_closed_loop() {
    // Triangle K3, 3 colors → exactly 6 valid colorings (3! permutations)
    let kc = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&kc).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // All solutions should extract to valid colorings
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(kc.evaluate(&extracted).unwrap());
    }

    // Exactly 6 valid 3-colorings of K3
    assert_eq!(qubo_solutions.len(), 6);
}

#[test]
fn test_kcoloring_to_qubo_path() {
    // Path graph: 0-1-2, 2 colors
    let kc = KColoring::<K2, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&kc).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(kc.evaluate(&extracted).unwrap());
    }

    // 2-coloring of path: 0,1,0 or 1,0,1 → 2 solutions
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_kcoloring_to_qubo_reversed_edges() {
    // Edge (2, 0) triggers the idx_v < idx_u swap branch (line 104).
    // Path: 2-0-1 with reversed edge ordering
    let kc = KColoring::<K2, _>::new(SimpleGraph::new(3, vec![(2, 0), (0, 1)]));
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&kc).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(kc.evaluate(&extracted).unwrap());
    }

    // Same as path graph: 2 valid 2-colorings
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_kcoloring_to_qubo_sizes() {
    let kc = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&kc).expect("reduction should succeed");

    // QUBO should have n*K = 3*3 = 9 variables
    assert_eq!(reduction.target_problem().num_variables(), 9);
}

#[test]
fn test_kcoloring_to_qubo_all_small_graphs_and_configurations() {
    use crate::rules::AggregateReductionResult;
    for n in 0..=3 {
        let possible: Vec<_> = (0..n)
            .flat_map(|u| ((u + 1)..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..(1usize << possible.len()) {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter_map(|(i, &edge)| ((mask >> i) & 1 == 1).then_some(edge))
                .collect();
            for k in 0..=3 {
                let source = KColoring::<KN, _>::with_k(SimpleGraph::new(n, edges.clone()), k);
                let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
                let target = AggregateReductionResult::target_problem(&reduction);
                assert_eq!(target.num_vars(), n * k);
                let mut minimum = i64::MAX;
                let mut any_coloring = false;
                for bits in 0..(1usize << (n * k)) {
                    let config: Vec<_> = (0..n * k).map(|i| bits & (1 << i) != 0).collect();
                    let counts: Vec<_> = (0..n)
                        .map(|v| (0..k).filter(|&c| config[v * k + c]).count() as i64)
                        .collect();
                    let conflicts = edges
                        .iter()
                        .map(|&(u, v)| {
                            (0..k)
                                .filter(|&c| config[u * k + c] && config[v * k + c])
                                .count() as i64
                        })
                        .sum::<i64>();
                    let penalty = (n + 1) as i64;
                    let residual =
                        2 * counts.iter().map(|&count| (1 - count).pow(2)).sum::<i64>() + conflicts;
                    let value = target.evaluate(&config).unwrap();
                    assert_eq!(value.0, Some(penalty * residual - 2 * penalty * n as i64));
                    minimum = minimum.min(value.0.unwrap());
                    let expected = residual == 0;
                    assert_eq!(
                        AggregateReductionResult::extract_value(&reduction, value).0,
                        expected
                    );
                    match reduction.extract_solution(&config) {
                        Ok(coloring) => {
                            assert!(expected);
                            assert!(source.evaluate(&coloring).unwrap().0);
                            any_coloring = true;
                        }
                        Err(_) => assert!(!expected),
                    }
                }
                assert_eq!(
                    AggregateReductionResult::extract_value(
                        &reduction,
                        crate::types::Min(Some(minimum))
                    )
                    .0,
                    any_coloring
                );
                assert!(
                    !AggregateReductionResult::extract_value(&reduction, crate::types::Min(None)).0
                );
                assert!(reduction.extract_solution(&vec![false; n * k + 1]).is_err());
            }
        }
    }
}

#[test]
fn test_kcoloring_to_qubo_checked_parameters() {
    assert_eq!(coloring_qubo_parameters::<KN>(3, 2).unwrap(), (6, 4, -24));
    assert_eq!(
        coloring_qubo_parameters::<KN>(0, usize::MAX).unwrap(),
        (0, 1, 0)
    );
    assert_eq!(
        coloring_qubo_parameters::<KN>(2_147_483_647, 0).unwrap(),
        (0, 2_147_483_648, -9_223_372_032_559_808_512)
    );
    for (n, k) in [
        (usize::MAX, 2),
        (1, usize::MAX),
        (usize::MAX, 0),
        (i64::MAX as usize, 0),
        (2_147_483_648, 0),
    ] {
        assert!(matches!(
            coloring_qubo_parameters::<KN>(n, k),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
