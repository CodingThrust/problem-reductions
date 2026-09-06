use super::add_equality_sender;
use crate::models::algebraic::{LinearConstraint, ILP};
use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::graph::MonochromaticTriangle;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::{ILPSolveError, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_monochromatic_triangle_sender_all_colorings() {
    let mut edges = vec![(0, 1), (2, 3)];
    add_equality_sender(&mut edges, (0, 1), (2, 3), 4);
    let sender = MonochromaticTriangle::new(SimpleGraph::new(7, edges));
    assert_eq!(sender.num_edges(), 17);
    assert_eq!(sender.num_triangles(), 19);
    let mut extensions = [0, 0];
    for bits in 0..(1 << 17) {
        let coloring: Vec<bool> = (0..17).map(|i| bits & (1 << i) != 0).collect();
        if sender.evaluate(&coloring).unwrap().0 {
            assert_eq!(coloring[0], coloring[1]);
            extensions[usize::from(coloring[0])] += 1;
        }
    }
    // Both equal terminal assignments extend; no unequal one extends.
    assert_eq!(extensions, [12, 12]);
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_structure() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    // N=5 logical variables, T=7 NAE triples, three senders per triple.
    assert_eq!(target.num_vertices(), 104);
    assert_eq!(target.num_edges(), 346);
    assert_eq!(target.num_triangles(), 406);
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_all_source_projections() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source).unwrap();
    let to_ilp = ReduceTo::<ILP<bool>>::reduce_to(reduction.target_problem()).unwrap();
    let ilp = to_ilp.target_problem();
    for bits in 0..8 {
        let assignment: Vec<bool> = (0..3).map(|i| bits & (1 << i) != 0).collect();
        for sentinel in [false, true] {
            let mut constraints = ilp.constraints().to_vec();
            for (i, value) in assignment.iter().copied().chain([false]).enumerate() {
                constraints.push(LinearConstraint::eq(
                    vec![(2 * i, 1)],
                    i64::from(value ^ sentinel),
                ));
            }
            let fixed = ILP::<bool>::new(
                ilp.num_variables(),
                constraints,
                ilp.objective().to_vec(),
                ilp.sense(),
            )
            .unwrap();
            match ILPSolver::new().solve(&fixed) {
                Ok(solution) => {
                    assert!(source.evaluate(&assignment).unwrap().0);
                    let coloring = to_ilp.extract_solution(&solution).unwrap();
                    assert_eq!(reduction.extract_solution(&coloring).unwrap(), assignment);
                    let swapped = coloring.iter().map(|value| !value).collect();
                    assert_eq!(reduction.extract_solution(&swapped).unwrap(), assignment);
                }
                Err(ILPSolveError::Infeasible) => assert!(!source.evaluate(&assignment).unwrap().0),
                Err(error) => panic!("unexpected solver error: {error}"),
            }
        }
    }
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
        ],
    );
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source).unwrap();
    let coloring = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    assert!(
        source
            .evaluate(&reduction.extract_solution(&coloring).unwrap())
            .unwrap()
            .0
    );
    assert!(reduction.extract_solution(&vec![]).is_err());
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_unsatisfiable_all_signs() {
    // Every assignment falsifies exactly one of these eight distinct-variable clauses.
    let clauses = (0..8)
        .map(|bits| {
            CNFClause::new(
                (0..3)
                    .map(|i| {
                        if bits & (1 << i) == 0 {
                            -(i + 1)
                        } else {
                            i + 1
                        }
                    })
                    .collect(),
            )
        })
        .collect();
    let source = KSatisfiability::<K3>::new(3, clauses);
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source).unwrap();
    assert_eq!(
        ILPSolver::new().solve(reduction.target_problem()),
        Err(ILPSolveError::Infeasible)
    );
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_short_and_repeated_literals() {
    for clauses in [
        vec![],
        vec![vec![]],
        vec![vec![1]],
        vec![vec![1, -2]],
        vec![vec![1, 1, 1], vec![-1, -1, -1]],
        vec![vec![1, -1, 2]],
    ] {
        let source = KSatisfiability::<K3>::new_allow_less(
            3,
            clauses.into_iter().map(CNFClause::new).collect(),
        );
        let feasible = (0..8).any(|bits| {
            source
                .evaluate(&(0..3).map(|i| bits & (1 << i) != 0).collect())
                .unwrap()
                .0
        });
        let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source).unwrap();
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(coloring) => {
                assert!(feasible);
                assert!(
                    source
                        .evaluate(&reduction.extract_solution(&coloring).unwrap())
                        .unwrap()
                        .0
                );
            }
            Err(ILPSolveError::Infeasible) => assert!(!feasible),
            Err(error) => panic!("unexpected solver error: {error}"),
        }
    }
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_zero_variables() {
    let source = KSatisfiability::<K3>::new(0, vec![]);
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source).unwrap();
    let coloring = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    assert_eq!(
        reduction.extract_solution(&coloring).unwrap(),
        Vec::<bool>::new()
    );
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_size_overflow() {
    let source = KSatisfiability::<K3>::new(usize::MAX / 16, vec![]);
    assert!(matches!(
        ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source),
        Err(crate::rules::ReductionError::IntegerOverflow { .. }),
    ));
}
