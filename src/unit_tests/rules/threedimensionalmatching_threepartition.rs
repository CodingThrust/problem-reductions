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
    let reduction =
        ReduceTo::<ThreePartition>::reduce_to(&source).expect("reduction should succeed");
    (source, reduction)
}

#[test]
fn test_threedimensionalmatching_to_threepartition_q1_overhead_and_bounds() {
    let (_source, reduction) = reduce(1, &[(0, 0, 0)]);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 21);
    assert_eq!(target.num_groups(), 7);
    assert_eq!(target.bound(), 42_949_673_924);

    let bound = i128::from(target.bound());
    let total_sum: i128 = target.sizes().iter().map(|&size| i128::from(size)).sum();
    assert_eq!(total_sum, bound * target.num_groups() as i128);
    assert!(target
        .sizes()
        .iter()
        .all(|&size| 4 * i128::from(size) > bound && 2 * i128::from(size) < bound));
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

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_config)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_config).unwrap();
    assert_eq!(extracted, vec![true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_closed_loop_from_known_matching() {
    let (source, reduction) = reduce(1, &[(0, 0, 0)]);
    let target_solution = reduction.build_target_witness(&[1]);

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap()
            .0
    );
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_round_trip_q2_minimal_matching() {
    let (source, reduction) = reduce(2, &[(0, 0, 0), (1, 1, 1)]);
    let target_solution = reduction.build_target_witness(&[1, 1]);

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![true, true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_uncovered_coordinate_maps_to_fixed_no_instance()
{
    let (source, reduction) = reduce(2, &[(0, 0, 0), (0, 1, 1)]);

    assert!(
        BruteForce::new().solve(&source).unwrap().is_none(),
        "source instance should be infeasible"
    );
    assert_eq!(reduction.target_problem().sizes(), &[6, 6, 6, 6, 7, 9]);
    assert_eq!(reduction.target_problem().bound(), 20);
    assert!(
        BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .is_none(),
        "target instance should be infeasible"
    );
}

#[test]
fn test_threedimensionalmatching_to_threepartition_extracts_noncanonical_partition() {
    let (source, reduction) = reduce(1, &[(0, 0, 0)]);
    // A mathematically valid witness found independently by HiGHS. Both regular
    // triples initially contain UPrime elements; filler triples mix pair IDs.
    // Keep the witness fixed so this regression does not depend on the backend.
    let witness = vec![
        4, 0, 0, 4, 2, 1, 3, 3, 6, 0, 6, 4, 5, 5, 2, 1, 3, 1, 6, 2, 5,
    ];
    assert!(reduction.target_problem().evaluate(&witness).unwrap().0);
    let extracted = reduction.extract_solution(&witness).unwrap();
    assert_eq!(extracted, vec![true]);
    assert!(source.evaluate(&extracted).unwrap().0);
    // Group labels have no mathematical significance.
    let relabeled = witness.iter().map(|group| 6 - group).collect();
    assert_eq!(reduction.extract_solution(&relabeled).unwrap(), extracted);
}

#[test]
fn test_threedimensionalmatching_to_threepartition_equal_size_permutations() {
    let (source, reduction) = reduce(2, &[(0, 0, 0), (0, 1, 1), (1, 0, 0), (1, 1, 1)]);
    let target = reduction.target_problem();
    for matching in [[1, 0, 0, 1], [0, 1, 1, 0]] {
        let mut witness = reduction.build_target_witness(&matching);
        let mut exchanges = 0;
        // Cumulative equal-size exchanges preserve a valid partition while
        // exercising regular-item identities, mixed fillers, and dummy groups.
        for left in 0..target.num_elements() {
            for right in left + 1..target.num_elements() {
                if target.sizes()[left] == target.sizes()[right] && witness[left] != witness[right]
                {
                    witness.swap(left, right);
                    assert!(target.evaluate(&witness).unwrap().0);
                    let extracted = reduction.extract_solution(&witness).unwrap();
                    assert!(source.evaluate(&extracted).unwrap().0);
                    exchanges += 1;
                }
            }
        }
        assert!(exchanges > 0);
    }
}

#[test]
fn test_threedimensionalmatching_to_threepartition_rejects_invalid_partitions() {
    let (_, reduction) = reduce(1, &[(0, 0, 0)]);
    let valid = reduction.build_target_witness(&[1]);
    assert!(reduction.extract_solution(&vec![]).is_err());
    let mut invalid = valid.clone();
    invalid[0] = reduction.target_problem().num_groups();
    assert!(reduction.extract_solution(&invalid).is_err());
    assert!(reduction.extract_solution(&vec![0; valid.len()]).is_err());
    let mut wrong_sum = valid;
    wrong_sum.swap(0, 2);
    assert!(reduction.extract_solution(&wrong_sum).is_err());
}
