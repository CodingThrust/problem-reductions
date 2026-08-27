use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MaximumCoKPlex;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::ILPSolver;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Max, One};
use crate::variant::KN;

fn c5() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)])
}

fn issue_instance() -> MaximumCoKPlex<SimpleGraph, i64, KN> {
    MaximumCoKPlex::<_, i64, KN>::with_k(c5(), vec![5, 1, 4, 1, 3], 2)
}

#[test]
fn test_maximumcokplex_to_ilp_closed_loop() {
    let source = issue_instance();
    let reduction: ReductionCoKPlexToILP<i64> =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_maximumcokplex_to_ilp_issue_structure() {
    let source = issue_instance();
    let reduction: ReductionCoKPlexToILP<i64> =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 5);
    assert_eq!(ilp.constraints().len(), 5);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
    assert_eq!(
        ilp.objective(),
        vec![(0, 5.0), (1, 1.0), (2, 4.0), (3, 1.0), (4, 3.0)]
    );

    let expected_constraints = vec![
        vec![(0, 2), (1, 1), (4, 1)],
        vec![(0, 1), (1, 2), (2, 1)],
        vec![(1, 1), (2, 2), (3, 1)],
        vec![(2, 1), (3, 2), (4, 1)],
        vec![(0, 1), (3, 1), (4, 2)],
    ];
    for (constraint, expected_terms) in ilp.constraints().iter().zip(expected_constraints) {
        let mut terms = constraint.terms().to_vec();
        terms.sort_by_key(|(var, _)| *var);
        assert_eq!(terms, expected_terms);
        assert_eq!(constraint.rhs(), 3);
    }
}

#[test]
fn test_maximumcokplex_to_ilp_bf_vs_ilp() {
    let source = issue_instance();
    let reduction: ReductionCoKPlexToILP<i64> =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_maximumcokplex_to_ilp_k_equals_1_regression() {
    let source = MaximumCoKPlex::<_, One, KN>::with_k(c5(), vec![One; 5], 1);
    let reduction: ReductionCoKPlexToILP<One> =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("k=1 instance should be ILP-solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(source.evaluate(&extracted).unwrap(), Max(Some(2)));
    assert_eq!(extracted.iter().filter(|&&selected| selected).count(), 2);
    assert!(source.is_valid_solution(&extracted));
}

#[test]
fn test_maximumcokplex_to_ilp_extract_solution_identity() {
    let source = issue_instance();
    let reduction: ReductionCoKPlexToILP<i64> =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let target_solution = vec![1, 0, 1, 0, 1];
    let extracted = reduction.extract_solution(&target_solution).unwrap();

    assert_eq!(extracted, vec![true, false, true, false, true]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Max(Some(12)));
}
