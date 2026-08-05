use crate::models::decision::Decision;
use crate::models::graph::MinimumVertexCover;
use crate::models::set::ComparativeContainment;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn decision_mvc(
    num_vertices: usize,
    edges: &[(usize, usize)],
    k: i32,
) -> Decision<MinimumVertexCover<SimpleGraph, i32>> {
    Decision::new(
        MinimumVertexCover::new(
            SimpleGraph::new(num_vertices, edges.to_vec()),
            vec![1i32; num_vertices],
        ),
        k,
    )
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_structure_counts() {
    // Path P_4: 4 vertices, 3 edges. K=2.
    let source = decision_mvc(4, &[(0, 1), (1, 2), (2, 3)], 2);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    assert_eq!(target.num_r_sets(), 4);
    // num_s_sets = num_edges + 1 (one budget set).
    assert_eq!(target.num_s_sets(), 4);

    // Each R set has n - 1 = 3 elements and weight 1.
    for r in target.r_sets() {
        assert_eq!(r.len(), 3);
    }
    assert!(target.r_weights().iter().all(|&w| w == 1));

    // Edge S sets have n - 2 = 2 elements and weight n + 1 = 5.
    for s in target.s_sets().iter().take(3) {
        assert_eq!(s.len(), 2);
    }
    for &w in target.s_weights().iter().take(3) {
        assert_eq!(w, 5);
    }
    // Budget S set is the full universe, weight n - K = 2.
    assert_eq!(target.s_sets().last().unwrap().len(), 4);
    assert_eq!(*target.s_weights().last().unwrap(), 2);
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_closed_loop_yes() {
    // Path P_4: minimum vertex cover {1, 2} has size 2. With K=2 this is YES.
    let source = decision_mvc(4, &[(0, 1), (1, 2), (2, 3)], 2);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Decision<MVC>->ComparativeContainment YES instance",
    );
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_closed_loop_no() {
    // Triangle: minimum vertex cover has size 2. K=1 is NO.
    let source = decision_mvc(3, &[(0, 1), (1, 2), (0, 2)], 1);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);

    // Source is unsatisfiable, target must be too.
    let target = reduction.target_problem();
    let witnesses = BruteForce::new().find_all_witnesses(target);
    assert!(
        witnesses.is_empty(),
        "Triangle with K=1 should produce an unsatisfiable target instance"
    );
    assert!(!source.evaluate(&[1, 1, 0]).0);
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_extracts_cover() {
    // Triangle with K=2 has cover {0, 1} (and others).
    let source = decision_mvc(3, &[(0, 1), (1, 2), (0, 2)], 2);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);

    let witness = BruteForce::new()
        .find_witness(reduction.target_problem())
        .expect("triangle with K=2 should be satisfiable");
    let extracted = reduction.extract_solution(&witness).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_trivial_yes_k_equals_n() {
    // K = n corner case: bound equals number of vertices. Every cover is feasible.
    let source = decision_mvc(3, &[(0, 1), (1, 2), (0, 2)], 3);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Trivial-YES target: empty universe with no sets.
    assert_eq!(target.universe_size(), 0);
    assert_eq!(target.num_r_sets(), 0);
    assert_eq!(target.num_s_sets(), 0);

    // The empty configuration is trivially satisfying.
    assert!(target.evaluate(&[]).0);

    // Extracted source configuration must be a valid cover with size <= K.
    let extracted = reduction.extract_solution(&[]).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_trivial_yes_k_greater_than_n() {
    // K > n: still trivially YES.
    let source = decision_mvc(2, &[(0, 1)], 5);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 0);
    let extracted = reduction.extract_solution(&[]).unwrap();
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_minimumvertexcover_to_comparativecontainment_negative_bound() {
    // Negative bound is trivially NO; target must be unsatisfiable.
    let source = decision_mvc(2, &[(0, 1)], -1);
    let reduction = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let witnesses = BruteForce::new().find_all_witnesses(target);
    assert!(
        witnesses.is_empty(),
        "Negative bound should produce an unsatisfiable target"
    );
}

#[test]
#[should_panic(expected = "unit vertex weights")]
fn test_minimumvertexcover_to_comparativecontainment_rejects_non_unit_weights() {
    let source = Decision::new(
        MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![2i32, 1i32]),
        1,
    );
    let _ = ReduceTo::<ComparativeContainment<i32>>::reduce_to(&source);
}
