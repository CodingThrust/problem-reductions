use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Helper: build expression (1 ∪ 4) + (3 ∪ 6) + (2 ∪ 5)
fn example_expr() -> IntExpr {
    IntExpr::Sum(
        Box::new(IntExpr::Sum(
            Box::new(IntExpr::Union(
                Box::new(IntExpr::Atom(1)),
                Box::new(IntExpr::Atom(4)),
            )),
            Box::new(IntExpr::Union(
                Box::new(IntExpr::Atom(3)),
                Box::new(IntExpr::Atom(6)),
            )),
        )),
        Box::new(IntExpr::Union(
            Box::new(IntExpr::Atom(2)),
            Box::new(IntExpr::Atom(5)),
        )),
    )
}

#[test]
fn test_integer_expression_membership_creation() {
    let expr = example_expr();
    let problem = IntegerExpressionMembership::new(expr.clone(), 12);
    assert_eq!(problem.target(), 12);
    assert_eq!(problem.num_union_nodes(), 3);
    assert_eq!(problem.num_atoms(), 6);
    assert_eq!(problem.expression_size(), 11); // 6 atoms + 3 unions + 2 sums
    assert_eq!(problem.expression_depth(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2]);
    assert_eq!(
        <IntegerExpressionMembership as Problem>::NAME,
        "IntegerExpressionMembership"
    );
    assert_eq!(<IntegerExpressionMembership as Problem>::variant(), vec![]);
}

#[test]
fn test_integer_expression_membership_evaluate_satisfying() {
    let problem = IntegerExpressionMembership::new(example_expr(), 12);
    // config [1,1,0]: choose 4, 6, 2 → 4+6+2=12
    assert!(problem.evaluate(&[1, 1, 0]));
    // config [0,1,1]: choose 1, 6, 5 → 1+6+5=12
    assert!(problem.evaluate(&[0, 1, 1]));
}

#[test]
fn test_integer_expression_membership_evaluate_unsatisfying() {
    let problem = IntegerExpressionMembership::new(example_expr(), 12);
    // config [0,0,0]: choose 1, 3, 2 → 1+3+2=6 ≠ 12
    assert!(!problem.evaluate(&[0, 0, 0]));
    // config [1,0,0]: choose 4, 3, 2 → 4+3+2=9 ≠ 12
    assert!(!problem.evaluate(&[1, 0, 0]));
    // config [1,1,1]: choose 4, 6, 5 → 4+6+5=15 ≠ 12
    assert!(!problem.evaluate(&[1, 1, 1]));
}

#[test]
fn test_integer_expression_membership_evaluate_wrong_config() {
    let problem = IntegerExpressionMembership::new(example_expr(), 12);
    // Wrong length
    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
    // Invalid value
    assert!(!problem.evaluate(&[2, 0, 0]));
}

#[test]
fn test_integer_expression_membership_brute_force() {
    let problem = IntegerExpressionMembership::new(example_expr(), 12);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_integer_expression_membership_brute_force_all() {
    let problem = IntegerExpressionMembership::new(example_expr(), 12);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // K=12 can be reached by [0,1,1] (1+6+5), [1,0,1] (4+3+5), [1,1,0] (4+6+2)
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_integer_expression_membership_unsatisfiable() {
    // Target 100 is unreachable from {1,4}+{3,6}+{2,5} (max is 15)
    let problem = IntegerExpressionMembership::new(example_expr(), 100);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_integer_expression_membership_single_atom() {
    let expr = IntExpr::Atom(42);
    let problem = IntegerExpressionMembership::new(expr, 42);
    assert_eq!(problem.num_union_nodes(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[])); // empty config, atom == target
}

#[test]
fn test_integer_expression_membership_single_atom_miss() {
    let expr = IntExpr::Atom(42);
    let problem = IntegerExpressionMembership::new(expr, 7);
    assert!(!problem.evaluate(&[])); // 42 ≠ 7
}

#[test]
fn test_integer_expression_membership_simple_union() {
    // (3 ∪ 7), target = 7
    let expr = IntExpr::Union(Box::new(IntExpr::Atom(3)), Box::new(IntExpr::Atom(7)));
    let problem = IntegerExpressionMembership::new(expr, 7);
    assert_eq!(problem.num_union_nodes(), 1);
    assert_eq!(problem.dims(), vec![2]);
    assert!(!problem.evaluate(&[0])); // 3 ≠ 7
    assert!(problem.evaluate(&[1])); // 7 == 7
}

#[test]
fn test_integer_expression_membership_simple_sum() {
    // Atom(3) + Atom(5), target = 8
    let expr = IntExpr::Sum(Box::new(IntExpr::Atom(3)), Box::new(IntExpr::Atom(5)));
    let problem = IntegerExpressionMembership::new(expr, 8);
    assert_eq!(problem.num_union_nodes(), 0);
    assert!(problem.evaluate(&[])); // 3+5=8
}

#[test]
fn test_integer_expression_membership_serialization() {
    let expr = IntExpr::Union(Box::new(IntExpr::Atom(1)), Box::new(IntExpr::Atom(4)));
    let problem = IntegerExpressionMembership::new(expr, 4);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: IntegerExpressionMembership = serde_json::from_value(json).unwrap();
    assert_eq!(restored.target(), 4);
    assert_eq!(restored.num_union_nodes(), 1);
    assert!(restored.evaluate(&[1])); // choose 4
}

#[test]
fn test_integer_expression_membership_evaluate_config() {
    let problem = IntegerExpressionMembership::new(example_expr(), 12);
    assert_eq!(problem.evaluate_config(&[1, 1, 0]), Some(12)); // 4+6+2
    assert_eq!(problem.evaluate_config(&[0, 0, 0]), Some(6)); // 1+3+2
    assert_eq!(problem.evaluate_config(&[1, 1, 1]), Some(15)); // 4+6+5
    assert_eq!(problem.evaluate_config(&[0, 0, 1]), Some(9)); // 1+3+5
}

#[test]
fn test_integer_expression_membership_paper_example() {
    // e = (1 ∪ 4) + (3 ∪ 6) + (2 ∪ 5), K = 12
    // Set = {6, 9, 12, 15}
    // Witness: config [1, 1, 0] → 4+6+2 = 12
    let problem = IntegerExpressionMembership::new(example_expr(), 12);

    // Verify the claimed witness
    assert_eq!(problem.evaluate_config(&[1, 1, 0]), Some(12));
    assert!(problem.evaluate(&[1, 1, 0]));

    // Verify all 8 configs produce the set {6, 9, 12, 15}
    let mut values: Vec<u64> = Vec::new();
    for c0 in 0..2 {
        for c1 in 0..2 {
            for c2 in 0..2 {
                values.push(problem.evaluate_config(&[c0, c1, c2]).unwrap());
            }
        }
    }
    values.sort();
    values.dedup();
    assert_eq!(values, vec![6, 9, 12, 15]);

    // Brute force confirms 3 satisfying configs for K=12:
    // [0,1,1] (1+6+5), [1,0,1] (4+3+5), [1,1,0] (4+6+2)
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 3);
}

#[test]
fn test_integer_expression_membership_nested_unions() {
    // ((1 ∪ 2) ∪ 3), target = 2
    let expr = IntExpr::Union(
        Box::new(IntExpr::Union(
            Box::new(IntExpr::Atom(1)),
            Box::new(IntExpr::Atom(2)),
        )),
        Box::new(IntExpr::Atom(3)),
    );
    let problem = IntegerExpressionMembership::new(expr, 2);
    assert_eq!(problem.num_union_nodes(), 2);
    // DFS order: outer union (idx 0), inner union (idx 1)
    // [0, 0] → left of outer → left of inner → 1
    // [0, 1] → left of outer → right of inner → 2
    // [1, _] → right of outer → 3 (inner union not visited)
    assert!(!problem.evaluate(&[0, 0])); // 1 ≠ 2
    assert!(problem.evaluate(&[0, 1])); // 2 == 2
    assert!(!problem.evaluate(&[1, 0])); // 3 ≠ 2
    assert!(!problem.evaluate(&[1, 1])); // 3 ≠ 2
}
