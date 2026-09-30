use super::*;
use crate::models::formula::CNFClause;
use crate::rules::AggregateReductionResult;
use crate::traits::Problem;

#[test]
fn test_one_in_three_to_ilp_preserves_literal_occurrences() {
    let literals = [-3, -2, -1, 1, 2, 3];
    for a in literals {
        for b in literals {
            for c in literals {
                let source = OneInThreeSatisfiability::new(3, vec![CNFClause::new(vec![a, b, c])]);
                let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
                let target = ReductionResult::target_problem(&reduction);
                assert_eq!(target.num_vars(), 3);
                assert_eq!(target.num_constraints(), 1);
                assert!(target.num_nonzeros() <= 3);
                assert!(target.max_constraint_magnitude_bits() <= 2);
                for mask in 0..8 {
                    let bits: Vec<bool> = (0..3).map(|i| mask & (1 << i) != 0).collect();
                    let values = bits.iter().copied().map(i64::from).collect();
                    let expected = source.evaluate(&bits).unwrap();
                    let actual = target.evaluate(&values).unwrap();
                    assert_eq!(actual.value.is_some(), expected.0);
                    assert_eq!(reduction.extract_value(actual).unwrap(), expected);
                    if expected.0 {
                        assert_eq!(reduction.extract_solution(&values).unwrap(), bits);
                    } else {
                        assert!(reduction.extract_solution(&values).is_err());
                    }
                }
            }
        }
    }
}

#[test]
fn test_oneinthreesatisfiability_to_ilp_closed_loop() {
    let source = OneInThreeSatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
    for malformed in [vec![1, 0], vec![1, 0, 0, 0], vec![2, 0, 0]] {
        assert!(reduction.extract_solution(&malformed).is_err());
    }
}
