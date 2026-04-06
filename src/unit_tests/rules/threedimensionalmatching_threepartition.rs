use super::*;
use crate::models::misc::ThreePartition;
use crate::models::set::ThreeDimensionalMatching;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce(
    universe_size: usize,
    triples: &[(usize, usize, usize)],
) -> (
    ThreeDimensionalMatching,
    ReductionThreeDimensionalMatchingToThreePartition,
) {
    let source = ThreeDimensionalMatching::new(universe_size, triples.to_vec());
    let reduction = ReduceTo::<ThreePartition>::reduce_to(&source);
    (source, reduction)
}

#[test]
fn test_threedimensionalmatching_to_threepartition_q1_overhead_and_bounds() {
    let (_source, reduction) = reduce(1, &[(0, 0, 0)]);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 21);
    assert_eq!(target.num_groups(), 7);
    assert_eq!(target.bound(), 42_949_673_924);

    let bound = u128::from(target.bound());
    let total_sum: u128 = target.sizes().iter().map(|&size| u128::from(size)).sum();
    assert_eq!(total_sum, bound * target.num_groups() as u128);
    assert!(target
        .sizes()
        .iter()
        .all(|&size| 4 * u128::from(size) > bound && 2 * u128::from(size) < bound));
}

#[test]
fn test_threedimensionalmatching_to_threepartition_q2_overhead_matches_vector() {
    let (_source, reduction) = reduce(2, &[(0, 0, 0), (1, 1, 1)]);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 90);
    assert_eq!(target.num_groups(), 30);
    assert_eq!(target.bound(), 687_194_768_324);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_extracts_manual_q1_witness() {
    let (source, reduction) = reduce(1, &[(0, 0, 0)]);

    // Step 3 witness for the unique 4-partition group {0,1,2,3} using pair (0,1):
    // regulars 0..3, pairings 4..15, fillers 16..20.
    let target_config = vec![
        0, 0, 1, 1, 0, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 2, 3, 4, 5, 6,
    ];

    assert!(reduction.target_problem().evaluate(&target_config).0);

    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted, vec![1]);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_closed_loop_from_known_matching() {
    let (source, reduction) = reduce(1, &[(0, 0, 0)]);
    let target_solution = reduction.build_target_witness(&[1]);

    assert!(reduction.target_problem().evaluate(&target_solution).0);
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1]);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_round_trip_q2_minimal_matching() {
    let (source, reduction) = reduce(2, &[(0, 0, 0), (1, 1, 1)]);
    let target_solution = reduction.build_target_witness(&[1, 1]);

    assert!(reduction.target_problem().evaluate(&target_solution).0);

    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1, 1]);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_uncovered_coordinate_maps_to_fixed_no_instance()
{
    let (source, reduction) = reduce(2, &[(0, 0, 0), (0, 1, 1)]);

    assert!(
        BruteForce::new().find_witness(&source).is_none(),
        "source instance should be infeasible"
    );
    assert_eq!(reduction.target_problem().sizes(), &[6, 6, 6, 6, 7, 9]);
    assert_eq!(reduction.target_problem().bound(), 20);
    assert!(
        BruteForce::new()
            .find_witness(reduction.target_problem())
            .is_none(),
        "target instance should be infeasible"
    );
}
