use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_subset_dp_shortest_common_superstring_matches_brute_force() {
    let candidates = [vec![], vec![0], vec![1], vec![0, 0], vec![0, 1], vec![1, 0]];
    for first in &candidates {
        for second in &candidates {
            for third in &candidates {
                let problem = ShortestCommonSuperstring::new(
                    2,
                    vec![first.clone(), second.clone(), third.clone()],
                );
                let expected = BruteForce::new().solve(&problem).unwrap().unwrap();
                let actual = solve(&problem).unwrap();
                assert_eq!(
                    problem.evaluate(&actual).unwrap(),
                    problem.evaluate(&expected).unwrap()
                );
            }
        }
    }
}

#[test]
fn test_subset_dp_shortest_common_superstring_handles_containment_and_scale() {
    let problem = ShortestCommonSuperstring::new(
        4,
        vec![
            vec![0, 1, 2, 3],
            vec![1, 2],
            vec![2, 3, 0],
            vec![3, 0, 1],
            vec![0, 1],
        ],
    );
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(6));
}
