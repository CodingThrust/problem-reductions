use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_symbol_block_order_grouping_by_swapping_matches_brute_force() {
    for alphabet_size in 1usize..=3 {
        for len in 1..=5 {
            let count = alphabet_size.pow(len as u32);
            for encoded in 0..count {
                let mut value = encoded;
                let mut string = Vec::with_capacity(len);
                for _ in 0..len {
                    string.push(value % alphabet_size);
                    value /= alphabet_size;
                }
                for budget in 0..=4 {
                    let problem = GroupingBySwapping::new(alphabet_size, string.clone(), budget);
                    let expected = BruteForce::new().solve(&problem).unwrap().is_some();
                    let actual = solve(&problem);
                    assert_eq!(actual.is_some(), expected, "{string:?}, budget={budget}");
                    if let Some(solution) = actual {
                        assert_eq!(problem.evaluate(&solution).unwrap(), Or(true));
                    }
                }
            }
        }
    }
}

#[test]
fn test_symbol_block_order_grouping_by_swapping_handles_scale() {
    assert_eq!(
        solve(&GroupingBySwapping::new(0, Vec::new(), 0)),
        Some(Vec::new())
    );

    let problem = GroupingBySwapping::new(4, vec![2, 1, 1, 1, 0, 0, 3, 2], 24);
    let solution = solve(&problem).expect("the instance is groupable within its budget");
    assert_eq!(solution.len(), 24);
    assert_eq!(problem.evaluate(&solution).unwrap(), Or(true));
}
