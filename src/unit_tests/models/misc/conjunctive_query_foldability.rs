use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Build the YES instance (foldable):
/// domain_size=0, 1 distinguished var (x=X(0)), 3 undistinguished vars (u=U(0), v=U(1), a=U(2))
/// Single binary relation R (arity 2), relation_arities = [2]
/// Q1: R(x,u) ∧ R(u,v) ∧ R(v,x) ∧ R(u,u)  (triangle + self-loop)
/// Q2: R(x,a) ∧ R(a,a) ∧ R(a,x)            (lollipop)
/// σ(U0→U2, U1→U2, U2→U2) = config [3, 3, 3]
fn yes_instance() -> ConjunctiveQueryFoldability {
    use Term::{Distinguished as X, Undistinguished as U};
    ConjunctiveQueryFoldability::new(
        0, // domain_size
        1, // num_distinguished
        3, // num_undistinguished
        vec![2],
        // Q1: R(x,u) ∧ R(u,v) ∧ R(v,x) ∧ R(u,u)
        vec![
            (0, vec![X(0), U(0)]),
            (0, vec![U(0), U(1)]),
            (0, vec![U(1), X(0)]),
            (0, vec![U(0), U(0)]),
        ],
        // Q2: R(x,a) ∧ R(a,a) ∧ R(a,x)
        vec![
            (0, vec![X(0), U(2)]),
            (0, vec![U(2), U(2)]),
            (0, vec![U(2), X(0)]),
        ],
    )
}

/// Build the NO instance (not foldable):
/// Same schema but Q1 is a triangle (no self-loop) and Q2 is a 2-cycle.
/// Q1: R(x,u) ∧ R(u,v) ∧ R(v,x)
/// Q2: R(x,a) ∧ R(a,x)
fn no_instance() -> ConjunctiveQueryFoldability {
    use Term::{Distinguished as X, Undistinguished as U};
    ConjunctiveQueryFoldability::new(
        0,
        1,
        3,
        vec![2],
        // Q1: R(x,u) ∧ R(u,v) ∧ R(v,x)
        vec![
            (0, vec![X(0), U(0)]),
            (0, vec![U(0), U(1)]),
            (0, vec![U(1), X(0)]),
        ],
        // Q2: R(x,a) ∧ R(a,x)
        vec![(0, vec![X(0), U(2)]), (0, vec![U(2), X(0)])],
    )
}

#[test]
fn test_conjunctive_query_foldability_creation() {
    let problem = yes_instance();
    // dims = [domain_size + num_distinguished + num_undistinguished; num_undistinguished]
    //       = [0 + 1 + 3; 3] = [4, 4, 4]
    assert_eq!(problem.dims(), vec![4, 4, 4]);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(
        <ConjunctiveQueryFoldability as Problem>::NAME,
        "ConjunctiveQueryFoldability"
    );
    assert_eq!(<ConjunctiveQueryFoldability as Problem>::variant(), vec![]);
}

#[test]
fn test_conjunctive_query_foldability_yes_instance() {
    let problem = yes_instance();

    // Index encoding (domain_size=0):
    //   0 → Distinguished(0) = x
    //   1 → Undistinguished(0) = u
    //   2 → Undistinguished(1) = v
    //   3 → Undistinguished(2) = a
    //
    // config [3, 3, 3]: σ(U0→U2, U1→U2, U2→U2) = σ maps everything to `a`
    // Substituted Q1: R(x,a) ∧ R(a,a) ∧ R(a,x) ∧ R(a,a)
    // As a set: {R(x,a), R(a,a), R(a,x)} == Q2 ✓
    assert!(problem.evaluate(&[3, 3, 3]));

    // config [0, 0, 0]: σ maps all undistinguished vars to Distinguished(0) = x
    // Substituted Q1: R(x,x) ∧ R(x,x) ∧ R(x,x) ∧ R(x,x) = {R(x,x)}
    // Q2 = {R(x,a), R(a,a), R(a,x)} ≠ {R(x,x)} ✗
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_conjunctive_query_foldability_no_instance() {
    let problem = no_instance();
    // No substitution σ on {U0, U1, U2} maps the triangle into a 2-cycle
    let result = BruteForce::new().find_witness(&problem);
    assert_eq!(result, None);
}

#[test]
fn test_conjunctive_query_foldability_solver() {
    let problem = yes_instance();
    let result = BruteForce::new().find_witness(&problem);
    assert!(
        result.is_some(),
        "YES instance must have a satisfying config"
    );
    let config = result.unwrap();
    assert!(
        problem.evaluate(&config),
        "returned config must evaluate to true"
    );
}

#[test]
fn test_conjunctive_query_foldability_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ConjunctiveQueryFoldability = serde_json::from_value(json).unwrap();
    assert_eq!(restored.dims(), problem.dims());
    assert_eq!(restored.domain_size(), problem.domain_size());
    assert_eq!(restored.num_distinguished(), problem.num_distinguished());
    assert_eq!(
        restored.num_undistinguished(),
        problem.num_undistinguished()
    );
    assert_eq!(restored.num_conjuncts_q1(), problem.num_conjuncts_q1());
    assert_eq!(restored.num_conjuncts_q2(), problem.num_conjuncts_q2());
    assert_eq!(restored.relation_arities(), problem.relation_arities());
    // Verify the restored instance produces the same evaluation results
    assert!(restored.evaluate(&[3, 3, 3]));
    assert!(!restored.evaluate(&[0, 0, 0]));
}

#[test]
fn test_conjunctive_query_foldability_paper_example() {
    let problem = yes_instance();

    // The known satisfying config σ(all→a) = [3, 3, 3]
    assert!(problem.evaluate(&[3, 3, 3]));

    // Enumerate all satisfying configs.
    // U(2) (= a) does not appear in Q1, so σ(U2) is a free choice (4 values).
    // Only σ(U0)=3 and σ(U1)=3 are required; σ(U2) can be anything in 0..4.
    let all = BruteForce::new().find_all_witnesses(&problem);
    assert_eq!(
        all.len(),
        4,
        "σ(U2) is unconstrained → 4 satisfying substitutions"
    );
    assert!(all.iter().all(|c| c[0] == 3 && c[1] == 3));
}

#[test]
fn test_conjunctive_query_foldability_with_constants() {
    use Term::{Constant as C, Distinguished as X, Undistinguished as U};
    // Instance with domain constants: R is binary (arity 2).
    // Q1: R(c0, u) ∧ R(u, x)
    // Q2: R(c0, x) ∧ R(x, x)
    // σ: u → x  (index = domain_size + 0 = 1)
    // Substituted Q1: R(c0, x) ∧ R(x, x) = Q2 ✓
    let problem = ConjunctiveQueryFoldability::new(
        1, // domain_size (c0 = Constant(0))
        1, // num_distinguished (x = X(0))
        1, // num_undistinguished (u = U(0))
        vec![2],
        vec![
            (0, vec![C(0), U(0)]), // R(c0, u)
            (0, vec![U(0), X(0)]), // R(u, x)
        ],
        vec![
            (0, vec![C(0), X(0)]), // R(c0, x)
            (0, vec![X(0), X(0)]), // R(x, x)
        ],
    );
    // dims = [1+1+1; 1] = [3]
    assert_eq!(problem.dims(), vec![3]);
    // σ(u→x): index for X(0) = domain_size + 0 = 1
    assert!(problem.evaluate(&[1]));
    // σ(u→c0): index for C(0) = 0 → R(c0, c0) ∧ R(c0, x) ≠ Q2
    assert!(!problem.evaluate(&[0]));
}

#[test]
fn test_conjunctive_query_foldability_getters() {
    let problem = yes_instance();
    assert_eq!(problem.domain_size(), 0);
    assert_eq!(problem.num_distinguished(), 1);
    assert_eq!(problem.num_undistinguished(), 3);
    assert_eq!(problem.num_conjuncts_q1(), 4);
    assert_eq!(problem.num_conjuncts_q2(), 3);
    assert_eq!(problem.num_relations(), 1);
    assert_eq!(problem.relation_arities(), &[2]);
    assert_eq!(problem.query1_conjuncts().len(), 4);
    assert_eq!(problem.query2_conjuncts().len(), 3);
}

#[test]
fn test_conjunctive_query_foldability_evaluate_wrong_length() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[3, 3])); // too short
    assert!(!problem.evaluate(&[3, 3, 3, 3])); // too long
}

#[test]
fn test_conjunctive_query_foldability_evaluate_out_of_range() {
    let problem = yes_instance();
    // range = 0 + 1 + 3 = 4, so value 4 is out of range
    assert!(!problem.evaluate(&[4, 3, 3]));
}

#[test]
#[should_panic(expected = "relation index")]
fn test_conjunctive_query_foldability_bad_relation_index() {
    use Term::Distinguished as X;
    ConjunctiveQueryFoldability::new(
        0,
        1,
        0,
        vec![2],
        vec![(5, vec![X(0), X(0)])], // relation 5 doesn't exist
        vec![],
    );
}

#[test]
#[should_panic(expected = "arity")]
fn test_conjunctive_query_foldability_bad_arity() {
    use Term::Distinguished as X;
    ConjunctiveQueryFoldability::new(
        0,
        1,
        0,
        vec![2],
        vec![(0, vec![X(0)])], // arity 2 but 1 arg
        vec![],
    );
}

#[test]
#[should_panic(expected = "Distinguished")]
fn test_conjunctive_query_foldability_bad_distinguished() {
    use Term::Distinguished as X;
    ConjunctiveQueryFoldability::new(
        0,
        1,
        0,
        vec![2],
        vec![(0, vec![X(0), X(1)])], // X(1) out of range
        vec![],
    );
}

#[test]
#[should_panic(expected = "Constant")]
fn test_conjunctive_query_foldability_bad_constant() {
    use Term::{Constant as C, Distinguished as X};
    ConjunctiveQueryFoldability::new(
        1,
        1,
        0,
        vec![2],
        vec![(0, vec![X(0), C(1)])], // C(1) out of range for domain_size=1
        vec![],
    );
}

#[test]
fn test_conjunctive_query_foldability_no_undistinguished() {
    use Term::Distinguished as X;
    // Q1 = Q2 = {R(x, x)} — no undistinguished vars, trivially foldable
    let problem = ConjunctiveQueryFoldability::new(
        0,
        1,
        0,
        vec![2],
        vec![(0, vec![X(0), X(0)])],
        vec![(0, vec![X(0), X(0)])],
    );
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_conjunctive_query_foldability_no_undistinguished_not_equal() {
    use Term::Distinguished as X;
    // Q1 = {R(x0, x1)}, Q2 = {R(x1, x0)} — different sets, not foldable
    let problem = ConjunctiveQueryFoldability::new(
        0,
        2,
        0,
        vec![2],
        vec![(0, vec![X(0), X(1)])],
        vec![(0, vec![X(1), X(0)])],
    );
    assert!(!problem.evaluate(&[]));
}

#[test]
#[should_panic(expected = "Undistinguished")]
fn test_conjunctive_query_foldability_bad_undistinguished() {
    use Term::{Distinguished as X, Undistinguished as U};
    ConjunctiveQueryFoldability::new(
        0,
        1,
        1,
        vec![2],
        vec![(0, vec![X(0), U(1)])], // U(1) out of range for num_undistinguished=1
        vec![],
    );
}
