use super::*;
use crate::rules::test_helpers::{
    assert_optimization_round_trip_from_optimization_target, assert_parameter_predictions,
};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;

fn maxcut(n: usize, edges: Vec<(usize, usize)>) -> Source {
    Source::unweighted(SimpleGraph::new(n, edges))
}

fn reduce(source: &Source) -> ReductionMaxCutToMaximum2Satisfiability {
    ReduceTo::<Maximum2Satisfiability>::reduce_to(source).expect("reduction should succeed")
}

/// The 4-cycle 0-1-2-3-0 plus the diagonal {0, 2}.
fn issue_example() -> Source {
    maxcut(4, vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)])
}

fn assignments(n: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << n).map(move |mask| (0..n).map(|bit| (mask >> bit) & 1 == 1).collect())
}

fn value<P: Problem<Value = Max<i64>>>(problem: &P, config: &P::Solution) -> i64 {
    problem.evaluate(config).unwrap().0.unwrap()
}

fn optimum<P>(problem: &P) -> i64
where
    P: Problem<Value = Max<i64>> + crate::solvers::BruteForceProblem + 'static,
{
    let best = BruteForce::new().solve(problem).unwrap().unwrap();
    value(problem, &best)
}

/// `satisfied(x) = m + cut(x)` for every assignment, and extraction is the identity.
fn assert_pointwise_identity(source: &Source) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    let m = i64::try_from(source.num_edges()).unwrap();
    for x in assignments(source.num_vertices()) {
        assert_eq!(value(target, &x), m + value(source, &x), "assignment {x:?}");
        assert_eq!(reduction.extract_solution(&x).unwrap(), x);
    }
}

#[test]
fn test_maxcut_to_maximum2satisfiability_closed_loop() {
    let source = issue_example();
    let reduction = reduce(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut -> Maximum2Satisfiability closed loop",
    );
}

#[test]
fn test_maxcut_to_maximum2satisfiability_structure() {
    let source = issue_example();
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 10);
    let literals: Vec<Vec<i64>> = target
        .clauses()
        .iter()
        .map(|c| c.literals.clone())
        .collect();
    assert_eq!(
        literals,
        vec![
            vec![1, 2],
            vec![-1, -2],
            vec![2, 3],
            vec![-2, -3],
            vec![3, 4],
            vec![-3, -4],
            vec![4, 1],
            vec![-4, -1],
            vec![1, 3],
            vec![-1, -3],
        ]
    );
    assert_parameter_predictions(&source, &reduction);
}

#[test]
fn test_maxcut_to_maximum2satisfiability_issue_example_values() {
    let source = issue_example();
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    let witness = vec![false, true, false, true];
    assert_eq!(value(&source, &witness), 4);
    assert_eq!(value(target, &witness), 9);
    assert_eq!(value(target, &[false; 4].to_vec()), 5);
    assert_eq!(value(target, &[true; 4].to_vec()), 5);
    assert_eq!(optimum(&source), 4);
    assert_eq!(optimum(target), 9);
    assert_eq!(
        BruteForce::new().find_all_witnesses(target).unwrap().len(),
        2,
        "the witness and its complement"
    );
}

#[test]
fn test_maxcut_to_maximum2satisfiability_pointwise_identity_on_all_small_graphs() {
    for n in 0..=4usize {
        let pairs: Vec<(usize, usize)> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..1usize << pairs.len() {
            let edges = (0..pairs.len())
                .filter(|&i| (mask >> i) & 1 == 1)
                .map(|i| pairs[i])
                .collect();
            assert_pointwise_identity(&maxcut(n, edges));
        }
    }
}

#[test]
fn test_maxcut_to_maximum2satisfiability_preserves_edge_orientation() {
    let source = maxcut(3, vec![(2, 0), (1, 2)]);
    let reduction = reduce(&source);
    let literals: Vec<Vec<i64>> = reduction
        .target_problem()
        .clauses()
        .iter()
        .map(|c| c.literals.clone())
        .collect();
    assert_eq!(
        literals,
        vec![vec![3, 1], vec![-3, -1], vec![2, 3], vec![-2, -3]]
    );
    assert_pointwise_identity(&source);
}

#[test]
fn test_maxcut_to_maximum2satisfiability_empty_graph() {
    let source = maxcut(0, vec![]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_vars(), 0);
    assert_eq!(target.num_clauses(), 0);
    assert_eq!(value(target, &vec![]), 0);
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
    assert_optimization_round_trip_from_optimization_target(&source, &reduction, "empty graph");
}

#[test]
fn test_maxcut_to_maximum2satisfiability_edgeless_graph() {
    let source = maxcut(3, vec![]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_vars(), 3);
    assert_eq!(target.num_clauses(), 0);
    for x in assignments(3) {
        assert_eq!(value(target, &x), 0);
        assert_eq!(value(&source, &x), 0);
    }
}

#[test]
fn test_maxcut_to_maximum2satisfiability_isolated_vertex() {
    // Vertex 2 is isolated; the objectives need not be zero.
    let source = maxcut(3, vec![(0, 1)]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(value(&source, &vec![false, true, false]), 1);
    assert_eq!(value(target, &vec![false, true, false]), 2);
    for x in assignments(3) {
        let mut flipped = x.clone();
        flipped[2] = !flipped[2];
        assert_eq!(value(&source, &flipped), value(&source, &x));
        assert_eq!(value(target, &flipped), value(target, &x));
    }
    assert_pointwise_identity(&source);
}

#[test]
fn test_maxcut_to_maximum2satisfiability_disconnected_graph() {
    let source = maxcut(5, vec![(0, 1), (1, 2), (3, 4)]);
    let reduction = reduce(&source);
    assert_pointwise_identity(&source);
    assert_eq!(optimum(&source), 3);
    assert_eq!(optimum(reduction.target_problem()), 6);
    assert_optimization_round_trip_from_optimization_target(&source, &reduction, "disconnected");
}

#[test]
fn test_maxcut_to_maximum2satisfiability_thresholds_unattainable_on_both_sides() {
    // Single edge: no cut reaches 2, so no assignment satisfies m + 2 = 3 clauses.
    let edge = maxcut(2, vec![(0, 1)]);
    let edge_target = reduce(&edge).target_problem().clone();
    assert!(assignments(2).all(|x| value(&edge, &x) < 2));
    assert!(assignments(2).all(|x| value(&edge_target, &x) < 3));
    assert_eq!(optimum(&edge_target), 2);

    // Triangle: no cut reaches 3, so no assignment satisfies m + 3 = 6 clauses.
    let triangle = maxcut(3, vec![(0, 1), (1, 2), (0, 2)]);
    let triangle_target = reduce(&triangle).target_problem().clone();
    assert!(assignments(3).all(|x| value(&triangle, &x) < 3));
    assert!(assignments(3).all(|x| value(&triangle_target, &x) < 6));
    assert_eq!(optimum(&triangle), 2);
    assert_eq!(optimum(&triangle_target), 5);
}

#[test]
fn test_maxcut_to_maximum2satisfiability_source_rejects_loops_and_parallel_edges() {
    assert!(SimpleGraph::try_new(2, vec![(1, 1)]).is_err());
    assert!(SimpleGraph::try_new(2, vec![(0, 1), (1, 0)]).is_err());
    assert!(SimpleGraph::try_new(2, vec![(0, 1), (0, 1)]).is_err());
    let looped = serde_json::json!({
        "graph": {"num_vertices": 2, "edges": [[1, 1]]},
        "edge_weights": [1],
    });
    assert!(serde_json::from_value::<Source>(looped).is_err());
    let parallel = serde_json::json!({
        "graph": {"num_vertices": 2, "edges": [[0, 1], [1, 0]]},
        "edge_weights": [1, 1],
    });
    assert!(serde_json::from_value::<Source>(parallel).is_err());
}

#[test]
fn test_maxcut_to_maximum2satisfiability_extraction_rejects_wrong_lengths() {
    let reduction = reduce(&issue_example());
    assert!(reduction.extract_solution(&vec![false; 3]).is_err());
    assert!(reduction.extract_solution(&vec![false; 5]).is_err());
    assert!(reduction.extract_solution(&vec![]).is_err());
}

#[test]
fn test_maxcut_to_maximum2satisfiability_literal_boundary() {
    let largest = usize::try_from(i64::MAX).unwrap();
    assert_eq!(vertex_literal(0).unwrap(), 1);
    assert_eq!(vertex_literal(largest - 1).unwrap(), i64::MAX);
    assert!(matches!(
        vertex_literal(largest),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    assert!(matches!(
        vertex_literal(usize::MAX),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_maxcut_to_maximum2satisfiability_clause_count_boundary() {
    let half = usize::MAX / 2;
    assert_eq!(clause_count(0).unwrap(), 0);
    assert_eq!(clause_count(half).unwrap(), usize::MAX - 1);
    assert!(matches!(
        clause_count(half + 1),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}
