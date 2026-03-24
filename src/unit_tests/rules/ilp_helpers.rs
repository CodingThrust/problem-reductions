use super::*;
use crate::models::algebraic::{Comparison, LinearConstraint};

#[test]
fn test_mccormick_product_constraints() {
    let constraints = mccormick_product(2, 0, 1);
    assert_eq!(constraints.len(), 3);

    // y <= x_a: y - x_a <= 0
    assert_eq!(constraints[0].cmp, Comparison::Le);
    assert_eq!(constraints[0].rhs, 0.0);
    assert_eq!(constraints[0].terms, vec![(2, 1.0), (0, -1.0)]);

    // y <= x_b: y - x_b <= 0
    assert_eq!(constraints[1].cmp, Comparison::Le);
    assert_eq!(constraints[1].rhs, 0.0);
    assert_eq!(constraints[1].terms, vec![(2, 1.0), (1, -1.0)]);

    // y >= x_a + x_b - 1: x_a + x_b - y <= 1
    assert_eq!(constraints[2].cmp, Comparison::Le);
    assert_eq!(constraints[2].rhs, 1.0);
    assert_eq!(constraints[2].terms, vec![(0, 1.0), (1, 1.0), (2, -1.0)]);
}

#[test]
fn test_mccormick_product_satisfies_truth_table() {
    let constraints = mccormick_product(2, 0, 1);
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
        let i64_vals: Vec<i64> = vals.iter().map(|&v| v as i64).collect();
        let all_satisfied = constraints.iter().all(|c| c.is_satisfied(&i64_vals));
        assert_eq!(all_satisfied, expected, "case {:?}", vals);
    }
}

#[test]
fn test_mtz_ordering_creates_arc_and_bound_constraints() {
    let arcs = vec![(0, 1), (1, 2)];
    let n = 3;
    let constraints = mtz_ordering(&arcs, n, 0, 3);
    // 2 arc constraints + 2*3 bound constraints = 8
    assert_eq!(constraints.len(), 8);
}

#[test]
fn test_flow_conservation_simple_path() {
    // Simple path: 0 -> 1 -> 2, demand: +1 at source(0), -1 at sink(2), 0 at transit(1)
    let arcs = vec![(0, 1), (1, 2)];
    let demand = vec![1.0, 0.0, -1.0];
    let constraints = flow_conservation(&arcs, 3, &|i| i, &demand);
    assert_eq!(constraints.len(), 3);

    // Node 0: f_01 = 1
    assert_eq!(constraints[0].cmp, Comparison::Eq);
    assert_eq!(constraints[0].rhs, 1.0);

    // Node 1: f_12 - f_01 = 0
    assert_eq!(constraints[1].cmp, Comparison::Eq);
    assert_eq!(constraints[1].rhs, 0.0);

    // Node 2: -f_12 = -1
    assert_eq!(constraints[2].cmp, Comparison::Eq);
    assert_eq!(constraints[2].rhs, -1.0);

    // Solution: f_01 = 1, f_12 = 1
    let values = vec![1i64, 1];
    assert!(constraints.iter().all(|c| c.is_satisfied(&values)));
}

#[test]
fn test_big_m_activation() {
    let c = big_m_activation(0, 1, 10.0);
    assert_eq!(c.cmp, Comparison::Le);
    // f - 10*y <= 0
    assert_eq!(c.terms, vec![(0, 1.0), (1, -10.0)]);
    assert_eq!(c.rhs, 0.0);

    // y=1, f=5: 5 - 10 = -5 <= 0 ✓
    assert!(c.is_satisfied(&[5, 1]));
    // y=0, f=5: 5 - 0 = 5 > 0 ✗
    assert!(!c.is_satisfied(&[5, 0]));
    // y=1, f=10: 10 - 10 = 0 <= 0 ✓
    assert!(c.is_satisfied(&[10, 1]));
}

#[test]
fn test_abs_diff_le() {
    let constraints = abs_diff_le(0, 1, 2);
    assert_eq!(constraints.len(), 2);

    // |a - b| <= z
    // a=3, b=1, z=2: |3-1|=2 <= 2 ✓
    assert!(constraints.iter().all(|c| c.is_satisfied(&[3, 1, 2])));
    // a=3, b=1, z=1: |3-1|=2 > 1 ✗
    assert!(!constraints.iter().all(|c| c.is_satisfied(&[3, 1, 1])));
    // a=1, b=3, z=2: |1-3|=2 <= 2 ✓
    assert!(constraints.iter().all(|c| c.is_satisfied(&[1, 3, 2])));
}

#[test]
fn test_minimax_constraints() {
    // z >= x_0, z >= x_1
    let exprs = vec![vec![(0, 1.0)], vec![(1, 1.0)]];
    let constraints = minimax_constraints(2, &exprs);
    assert_eq!(constraints.len(), 2);

    // z=5, x_0=3, x_1=4: z >= max(3,4) ✓
    assert!(constraints.iter().all(|c| c.is_satisfied(&[3, 4, 5])));
    // z=3, x_0=3, x_1=4: z < max(3,4) ✗
    assert!(!constraints.iter().all(|c| c.is_satisfied(&[3, 4, 3])));
}

#[test]
fn test_one_hot_decode_permutation() {
    // 3x3 assignment: item 0 at slot 2, item 1 at slot 0, item 2 at slot 1
    // Layout: x_{v*3+p}
    let mut solution = vec![0usize; 9];
    solution[0 * 3 + 2] = 1; // item 0 -> slot 2
    solution[1 * 3 + 0] = 1; // item 1 -> slot 0
    solution[2 * 3 + 1] = 1; // item 2 -> slot 1
    let decoded = one_hot_decode(&solution, 3, 3, 0);
    assert_eq!(decoded, vec![1, 2, 0]); // slot 0 gets item 1, slot 1 gets item 2, slot 2 gets item 0
}

#[test]
fn test_one_hot_decode_with_offset() {
    // Same as above but with offset=5
    let mut solution = vec![0usize; 14];
    solution[5 + 0 * 3 + 2] = 1;
    solution[5 + 1 * 3 + 0] = 1;
    solution[5 + 2 * 3 + 1] = 1;
    let decoded = one_hot_decode(&solution, 3, 3, 5);
    assert_eq!(decoded, vec![1, 2, 0]);
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
        assert_eq!(c.cmp, Comparison::Eq);
        assert_eq!(c.rhs, 1.0);
    }
    // Last 3 are le (slot capacity)
    for c in &constraints[3..] {
        assert_eq!(c.cmp, Comparison::Le);
        assert_eq!(c.rhs, 1.0);
    }

    // Valid permutation: item 0->slot 0, item 1->slot 1, item 2->slot 2
    let mut solution = vec![0i64; 9];
    solution[0] = 1; // item 0 -> slot 0
    solution[4] = 1; // item 1 -> slot 1
    solution[8] = 1; // item 2 -> slot 2
    assert!(constraints.iter().all(|c| c.is_satisfied(&solution)));
}
