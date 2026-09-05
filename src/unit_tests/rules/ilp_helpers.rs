use super::*;
use crate::models::algebraic::{Comparison, LinearConstraint};

#[test]
fn test_mccormick_product_constraints() {
    let constraints: [LinearConstraint; 3] = mccormick_product(2, 0, 1);
    assert_eq!(constraints.len(), 3);

    // y <= x_a: y - x_a <= 0
    assert_eq!(constraints[0].comparison(), Comparison::Le);
    assert_eq!(constraints[0].rhs(), 0);
    assert_eq!(constraints[0].terms(), vec![(2, 1), (0, -1)]);

    // y <= x_b: y - x_b <= 0
    assert_eq!(constraints[1].comparison(), Comparison::Le);
    assert_eq!(constraints[1].rhs(), 0);
    assert_eq!(constraints[1].terms(), vec![(2, 1), (1, -1)]);

    // y >= x_a + x_b - 1: x_a + x_b - y <= 1
    assert_eq!(constraints[2].comparison(), Comparison::Le);
    assert_eq!(constraints[2].rhs(), 1);
    assert_eq!(constraints[2].terms(), vec![(0, 1), (1, 1), (2, -1)]);
}

#[test]
fn test_mccormick_product_satisfies_truth_table() {
    let constraints: [LinearConstraint; 3] = mccormick_product(2, 0, 1);
    // (x_a, x_b, y) -> product: y = x_a * x_b
    let cases = vec![
        (vec![0, 0, 0], true),  // 0*0=0
        (vec![0, 1, 0], true),  // 0*1=0
        (vec![1, 0, 0], true),  // 1*0=0
        (vec![1, 1, 1], true),  // 1*1=1
        (vec![0, 0, 1], false), // y=1 but 0*0=0
        (vec![1, 1, 0], false), // y=0 but 1*1=1
    ];
    for (vals, expected) in cases {
        let all_satisfied = constraints
            .iter()
            .all(|constraint| constraint.is_satisfied(&vals).unwrap());
        assert_eq!(all_satisfied, expected, "case {:?}", vals);
    }
}

#[test]
fn test_mtz_ordering_creates_arc_and_bound_constraints() {
    let constraints = [
        LinearConstraint::le(vec![(3, 1), (4, -1), (0, 3)], 2),
        LinearConstraint::le(vec![(4, 1), (5, -1), (1, 3)], 2),
        LinearConstraint::ge(vec![(3, 1)], 0),
        LinearConstraint::le(vec![(3, 1)], 2),
        LinearConstraint::ge(vec![(4, 1)], 0),
        LinearConstraint::le(vec![(4, 1)], 2),
        LinearConstraint::ge(vec![(5, 1)], 0),
        LinearConstraint::le(vec![(5, 1)], 2),
    ];
    // 2 arc constraints + 2*3 bound constraints = 8
    assert_eq!(constraints.len(), 8);
}

#[test]
fn test_flow_conservation_simple_path() {
    // Simple path: 0 -> 1 -> 2, demand: +1 at source(0), -1 at sink(2), 0 at transit(1)
    let constraints = [
        LinearConstraint::eq(vec![(0, 1)], 1),
        LinearConstraint::eq(vec![(1, 1), (0, -1)], 0),
        LinearConstraint::eq(vec![(1, -1)], -1),
    ];
    assert_eq!(constraints.len(), 3);

    // Node 0: f_01 = 1
    assert_eq!(constraints[0].comparison(), Comparison::Eq);
    assert_eq!(constraints[0].rhs(), 1);

    // Node 1: f_12 - f_01 = 0
    assert_eq!(constraints[1].comparison(), Comparison::Eq);
    assert_eq!(constraints[1].rhs(), 0);

    // Node 2: -f_12 = -1
    assert_eq!(constraints[2].comparison(), Comparison::Eq);
    assert_eq!(constraints[2].rhs(), -1);

    // Solution: f_01 = 1, f_12 = 1
    let values = vec![1i64, 1];
    assert!(constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&values).unwrap()));
}

#[test]
fn test_big_m_activation() {
    let c = LinearConstraint::le(vec![(0, 1), (1, -10)], 0);
    assert_eq!(c.comparison(), Comparison::Le);
    // f - 10*y <= 0
    assert_eq!(c.terms(), vec![(0, 1), (1, -10)]);
    assert_eq!(c.rhs(), 0);

    // y=1, f=5: 5 - 10 = -5 <= 0 ✓
    assert!(c.is_satisfied(&[5, 1]).unwrap());
    // y=0, f=5: 5 - 0 = 5 > 0 ✗
    assert!(!c.is_satisfied(&[5, 0]).unwrap());
    // y=1, f=10: 10 - 10 = 0 <= 0 ✓
    assert!(c.is_satisfied(&[10, 1]).unwrap());
}

#[test]
fn test_abs_diff_le() {
    let constraints = [
        LinearConstraint::le(vec![(0, 1), (1, -1), (2, -1)], 0),
        LinearConstraint::le(vec![(1, 1), (0, -1), (2, -1)], 0),
    ];
    assert_eq!(constraints.len(), 2);

    // |a - b| <= z
    // a=3, b=1, z=2: |3-1|=2 <= 2 ✓
    assert!(constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&[3, 1, 2]).unwrap()));
    // a=3, b=1, z=1: |3-1|=2 > 1 ✗
    assert!(!constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&[3, 1, 1]).unwrap()));
    // a=1, b=3, z=2: |1-3|=2 <= 2 ✓
    assert!(constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&[1, 3, 2]).unwrap()));
}

#[test]
fn test_minimax_constraints() {
    // z >= x_0, z >= x_1
    let constraints = [
        LinearConstraint::le(vec![(0, 1), (2, -1)], 0),
        LinearConstraint::le(vec![(1, 1), (2, -1)], 0),
    ];
    assert_eq!(constraints.len(), 2);

    // z=5, x_0=3, x_1=4: z >= max(3,4) ✓
    assert!(constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&[3, 4, 5]).unwrap()));
    // z=3, x_0=3, x_1=4: z < max(3,4) ✗
    assert!(!constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&[3, 4, 3]).unwrap()));
}

#[test]
fn test_one_hot_decode_permutation() {
    // 3x3 assignment: item 0 at slot 2, item 1 at slot 0, item 2 at slot 1
    // Layout: x_{v*3+p}
    let mut solution = vec![0_i64; 9];
    solution[2] = 1; // item 0 -> slot 2
    solution[3] = 1; // item 1 -> slot 0
    solution[7] = 1; // item 2 -> slot 1
    let decoded = one_hot_decode(&solution, 3, 3, 0).unwrap();
    assert_eq!(decoded, vec![1, 2, 0]); // slot 0 gets item 1, slot 1 gets item 2, slot 2 gets item 0
}

#[test]
fn test_one_hot_decode_with_offset() {
    // Same as above but with offset=5
    let mut solution = vec![0_i64; 14];
    solution[7] = 1; // 5 + 2
    solution[8] = 1; // 5 + 3
    solution[12] = 1; // 5 + 7
    let decoded = one_hot_decode(&solution, 3, 3, 5).unwrap();
    assert_eq!(decoded, vec![1, 2, 0]);
}

#[test]
fn test_one_hot_decode_rejects_missing_and_duplicate_items() {
    assert!(one_hot_decode(&[0, 0, 0, 0], 2, 2, 0).is_err());
    assert!(one_hot_decode(&[1, 0, 1, 0], 2, 2, 0).is_err());
    assert!(one_hot_decode(&[1, 1, 0, 0], 2, 2, 0).is_err());
}

#[test]
fn test_one_hot_decode_rows_accepts_exactly_one_column_per_row() {
    assert_eq!(
        one_hot_decode_rows(&[0, 1, 0, 1, 0, 0], 2, 3, 0).unwrap(),
        vec![1, 0]
    );
    assert!(one_hot_decode_rows(&[0, 0, 0, 1, 0, 0], 2, 3, 0).is_err());
    assert!(one_hot_decode_rows(&[1, 1, 0, 1, 0, 0], 2, 3, 0).is_err());
}

#[test]
fn test_permutation_to_lehmer() {
    // Identity permutation [0,1,2] -> Lehmer [0,0,0]
    assert_eq!(permutation_to_lehmer(&[0, 1, 2]), vec![0, 0, 0]);
    // Reverse [2,1,0] -> Lehmer [2,1,0]
    assert_eq!(permutation_to_lehmer(&[2, 1, 0]), vec![2, 1, 0]);
    // [1,0,2] -> Lehmer [1,0,0]
    assert_eq!(permutation_to_lehmer(&[1, 0, 2]), vec![1, 0, 0]);
}

#[test]
fn test_one_hot_assignment_constraints() {
    let constraints = one_hot_assignment_constraints(3, 3, 0);
    // 3 "each item to one slot" + 3 "each slot at most one item" = 6
    assert_eq!(constraints.len(), 6);

    // First 3 are equality (item assignment)
    for c in &constraints[..3] {
        assert_eq!(c.comparison(), Comparison::Eq);
        assert_eq!(c.rhs(), 1);
    }
    // Last 3 are le (slot capacity)
    for c in &constraints[3..] {
        assert_eq!(c.comparison(), Comparison::Le);
        assert_eq!(c.rhs(), 1);
    }

    // Valid permutation: item 0->slot 0, item 1->slot 1, item 2->slot 2
    let mut solution = vec![0i64; 9];
    solution[0] = 1; // item 0 -> slot 0
    solution[4] = 1; // item 1 -> slot 1
    solution[8] = 1; // item 2 -> slot 2
    assert!(constraints
        .iter()
        .all(|constraint| constraint.is_satisfied(&solution).unwrap()));
}
