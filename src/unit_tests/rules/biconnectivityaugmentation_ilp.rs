use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::BiconnectivityAugmentation;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn small_instance() -> BiconnectivityAugmentation<SimpleGraph, i64> {
    // Path 0-1-2-3, candidates: (0,2,1),(0,3,2),(1,3,1), budget=3
    BiconnectivityAugmentation::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![(0, 2, 1), (0, 3, 2), (1, 3, 1)],
        3,
    )
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionBiconnAugToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // Solve source with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&source).unwrap();
    assert!(!bf_solutions.is_empty(), "source should be satisfiable");

    // Solve ILP
    let ilp_solver = ILPSolver::new();
    let ilp_sol = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol).unwrap();

    assert!(
        source.evaluate(&extracted).unwrap().0,
        "extracted solution must be valid"
    );
}

#[test]
fn test_extract_solution() {
    let source = small_instance();
    let reduction: ReductionBiconnAugToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_trivial_single_vertex() {
    let source = BiconnectivityAugmentation::new(SimpleGraph::new(1, vec![]), vec![], 0);
    let reduction: ReductionBiconnAugToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("trivial ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol).unwrap();
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_already_biconnected() {
    // Triangle is already biconnected
    let source = BiconnectivityAugmentation::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![],
        0,
    );
    let reduction: ReductionBiconnAugToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver
        .solve(ilp)
        .expect("already biconnected should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol).unwrap();
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_bf_vs_ilp() {
    let source = small_instance();
    let reduction: ReductionBiconnAugToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_all_two_vertex_instances() {
    for base_edge in [false, true] {
        for weight in [-2, 0, 2] {
            for budget in [-3, -1, 0, 1, 3] {
                let source = BiconnectivityAugmentation::new(
                    SimpleGraph::new(2, if base_edge { vec![(0, 1)] } else { vec![] }),
                    if base_edge {
                        vec![]
                    } else {
                        vec![(0, 1, weight)]
                    },
                    budget,
                );
                let reduction: ReductionBiconnAugToILP =
                    ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
                let expected = BruteForce::new().solve(&source).unwrap().is_some();
                match ILPSolver::new().solve(reduction.target_problem()) {
                    Ok(z) => {
                        assert!(expected);
                        assert!(
                            source
                                .evaluate(&reduction.extract_solution(&z).unwrap())
                                .unwrap()
                                .0
                        );
                    }
                    Err(crate::solvers::ILPSolveError::Infeasible) => assert!(!expected),
                    Err(e) => panic!("unexpected solver error: {e}"),
                }
            }
        }
    }
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_empty_negative_budget() {
    for n in 0..=1 {
        for budget in [-1, 0, 1] {
            let source =
                BiconnectivityAugmentation::<_, i64>::new(SimpleGraph::empty(n), vec![], budget);
            let reduction: ReductionBiconnAugToILP =
                ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
            assert_eq!(
                reduction
                    .target_problem()
                    .evaluate(&vec![])
                    .unwrap()
                    .value
                    .is_some(),
                budget >= 0
            );
            assert_eq!(reduction.extract_solution(&vec![]).is_ok(), budget >= 0);
        }
    }
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_signed_cost_and_certificate_bounds() {
    for candidates in [vec![(0, 2, 2), (0, 3, -2)], vec![(0, 3, -2), (0, 2, 2)]] {
        let source = BiconnectivityAugmentation::new(SimpleGraph::path(4), candidates, 0);
        let reduction: ReductionBiconnAugToILP = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
        let z = ILPSolver::new().solve(reduction.target_problem()).unwrap();
        assert!(
            source
                .evaluate(&reduction.extract_solution(&z).unwrap())
                .unwrap()
                .0
        );
        assert!(reduction.extract_solution(&vec![0; z.len()]).is_err());
        assert!(reduction.extract_solution(&vec![1; z.len() + 1]).is_err());
        for value in [-1, 2] {
            let mut bad = z.clone();
            bad[0] = value;
            assert!(reduction.extract_solution(&bad).is_err());
        }
    }
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_checked_dimensions() {
    assert_eq!(
        ReductionBiconnAugToILP::dimensions(0, 0, 0).unwrap(),
        (0, 0)
    );
    assert_eq!(
        ReductionBiconnAugToILP::dimensions(2, 0, 1).unwrap(),
        (1, 13)
    );
    for (n, m, p) in [
        (usize::MAX, 0, 0),
        (1, usize::MAX, 0),
        (1, usize::MAX / 4, 4),
        (1, 0, usize::MAX / 4 + 1),
    ] {
        assert!(ReductionBiconnAugToILP::dimensions(n, m, p).is_err());
    }
}
