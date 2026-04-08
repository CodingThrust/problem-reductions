use crate::models::formula::{CNFClause, KSatisfiability};
use crate::models::graph::MonochromaticTriangle;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::K3;
use std::collections::BTreeSet;

#[cfg(feature = "ilp-solver")]
use crate::models::algebraic::ILP;
#[cfg(feature = "ilp-solver")]
use crate::solvers::ILPSolver;

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_structure() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 9);
    assert_eq!(target.num_edges(), 12);
    assert_eq!(target.triangles().len(), 4);

    let expected_edges = BTreeSet::from([
        (0, 3),
        (0, 6),
        (0, 7),
        (1, 4),
        (1, 6),
        (1, 8),
        (2, 5),
        (2, 7),
        (2, 8),
        (6, 7),
        (6, 8),
        (7, 8),
    ]);
    let actual_edges: BTreeSet<_> = target
        .graph()
        .edges()
        .into_iter()
        .map(|(u, v)| if u < v { (u, v) } else { (v, u) })
        .collect();
    assert_eq!(actual_edges, expected_edges);
}

#[test]
fn test_ksatisfiability_to_monochromatic_triangle_complement_extraction() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source);

    // Negation edges all use color 1, so direct extraction gives (0,0,0),
    // which does not satisfy (x1 v x2 v x3). The complement (1,1,1) does.
    let target_coloring = vec![1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0];

    assert!(
        reduction.target_problem().evaluate(&target_coloring),
        "the supplied target coloring must avoid monochromatic triangles"
    );

    let extracted = reduction.extract_solution(&target_coloring);
    assert_eq!(extracted, vec![1, 1, 1]);
    assert!(source.evaluate(&extracted));
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_monochromatic_triangle_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
        ],
    );
    let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source);
    let mono_to_ilp = ReduceTo::<ILP<bool>>::reduce_to(reduction.target_problem());

    let ilp_solution = ILPSolver::new()
        .solve(mono_to_ilp.target_problem())
        .expect("reduced MonochromaticTriangle instance should be feasible");
    let mono_solution = mono_to_ilp.extract_solution(&ilp_solution);

    assert!(reduction.target_problem().evaluate(&mono_solution));

    let extracted = reduction.extract_solution(&mono_solution);
    assert!(source.evaluate(&extracted));
}
