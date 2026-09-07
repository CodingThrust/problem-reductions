use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::ReduceTo;

#[test]
fn test_stackercrane_to_ilp_closed_loop() {
    // 3 vertices, 2 required arcs, 1 connector edge
    let source = StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_stackercrane_to_ilp_bf_vs_ilp() {
    let source = StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_stackercrane_to_ilp_unreachable_return() {
    let source = StackerCrane::new(2, vec![(0, 1)], vec![], vec![1], vec![]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_eq!(
        crate::solvers::ILPSolver::new().solve(reduction.target_problem()),
        Err(crate::solvers::ILPSolveError::Infeasible),
    );
}

#[test]
fn test_stackercrane_to_ilp_all_binary_assignments() {
    use crate::traits::Problem;

    // Each undirected connector is absent, zero-cost, or positive-cost.
    // Enumerate every ILP assignment, including malformed one-hot matrices and
    // false product indicators, rather than just the solver's chosen optimum.
    for encoding in 0..27 {
        let mut code = encoding;
        let mut edges = Vec::new();
        let mut lengths = Vec::new();
        for pair in [(0, 1), (0, 2), (1, 2)] {
            match code % 3 {
                1 => {
                    edges.push(pair);
                    lengths.push(0);
                }
                2 => {
                    edges.push(pair);
                    lengths.push(2);
                }
                _ => {}
            }
            code /= 3;
        }
        let source = StackerCrane::new(3, vec![(0, 1), (1, 2)], edges, vec![0, 1], lengths);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let mut decoded = std::collections::HashSet::new();
        for bits in 0..(1 << 12) {
            let solution = (0..12).map(|i| i64::from(bits & (1 << i) != 0)).collect();
            if let Some(cost) = target.evaluate(&solution).unwrap().value {
                let permutation = reduction.extract_solution(&solution).unwrap();
                assert_eq!(
                    source.evaluate(&permutation).unwrap(),
                    crate::types::Min(Some(cost + 1))
                );
                decoded.insert(permutation);
            }
        }
        for permutation in [vec![0, 1], vec![1, 0]] {
            assert_eq!(
                decoded.contains(&permutation),
                source.closed_walk_length(&permutation).is_some()
            );
        }
    }
}

#[test]
fn test_stackercrane_to_ilp_empty_and_self_loop() {
    for source in [
        StackerCrane::new(0, vec![], vec![], vec![], vec![]),
        StackerCrane::new(1, vec![(0, 0)], vec![], vec![0], vec![]),
    ] {
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert_bf_vs_ilp(&source, &reduction);
    }
}
