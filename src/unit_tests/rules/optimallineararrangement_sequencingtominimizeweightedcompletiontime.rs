use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

fn reduce_path(
    num_vertices: usize,
) -> (
    OptimalLinearArrangement<SimpleGraph>,
    ReductionOLAToSequencingToMinimizeWeightedCompletionTime,
) {
    let source = OptimalLinearArrangement::new(SimpleGraph::path(num_vertices));
    let reduction = ReduceTo::<SequencingToMinimizeWeightedCompletionTime>::reduce_to(&source)
        .expect("reduction should succeed");
    (source, reduction)
}

fn solve_target_cost(
    reduction: &ReductionOLAToSequencingToMinimizeWeightedCompletionTime,
) -> Min<i64> {
    let target = reduction.target_problem();
    let solution = BruteForce::new().solve(target).unwrap().unwrap();
    target.evaluate(&solution).unwrap()
}

#[test]
fn test_optimallineararrangement_to_sequencingtominimizeweightedcompletiontime_closed_loop_p4() {
    let (source, reduction) = reduce_path(4);
    let target = reduction.target_problem();

    assert_eq!(target.num_tasks(), 7);
    assert_eq!(target.lengths(), &[1, 1, 1, 1, 0, 0, 0]);
    assert_eq!(target.weights(), &[1, 0, 0, 1, 2, 2, 2]);
    assert_eq!(
        target.precedences(),
        &[(0, 4), (1, 4), (1, 5), (2, 5), (2, 6), (3, 6)]
    );

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "OptimalLinearArrangement -> SequencingToMinimizeWeightedCompletionTime P4",
    );

    assert_eq!(
        source
            .evaluate(&BruteForce::new().solve(&source).unwrap().unwrap())
            .unwrap(),
        Min(Some(3))
    );
    assert_eq!(solve_target_cost(&reduction), Min(Some(23)));
}

#[test]
fn test_optimallineararrangement_to_sequencingtominimizeweightedcompletiontime_closed_loop_k3() {
    let source = OptimalLinearArrangement::new(SimpleGraph::complete(3));
    let reduction = ReduceTo::<SequencingToMinimizeWeightedCompletionTime>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "OptimalLinearArrangement -> SequencingToMinimizeWeightedCompletionTime K3",
    );

    assert_eq!(
        source
            .evaluate(&BruteForce::new().solve(&source).unwrap().unwrap())
            .unwrap(),
        Min(Some(4))
    );
    assert_eq!(solve_target_cost(&reduction), Min(Some(16)));
}

#[test]
fn test_optimallineararrangement_to_sequencingtominimizeweightedcompletiontime_trivial_p2() {
    let (source, reduction) = reduce_path(2);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "OptimalLinearArrangement -> SequencingToMinimizeWeightedCompletionTime P2",
    );

    assert_eq!(
        source
            .evaluate(&BruteForce::new().solve(&source).unwrap().unwrap())
            .unwrap(),
        Min(Some(1))
    );
    assert_eq!(solve_target_cost(&reduction), Min(Some(4)));
}

#[test]
fn test_optimallineararrangement_to_sequencingtominimizeweightedcompletiontime_extracts_vertex_order(
) {
    let (source, reduction) = reduce_path(4);
    let schedule = vec![3, 2, 6, 1, 5, 0, 4];
    let extracted = reduction.extract_solution(&schedule).unwrap();

    assert_eq!(extracted, vec![3, 2, 1, 0]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(3)));
}
