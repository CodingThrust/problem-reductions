use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Helper to build the issue example instance.
fn issue_example() -> ConjunctiveBooleanQuery {
    let relations = vec![
        Relation {
            arity: 2,
            tuples: vec![vec![0, 3], vec![1, 3], vec![2, 4], vec![3, 4], vec![4, 5]],
        },
        Relation {
            arity: 3,
            tuples: vec![vec![0, 1, 5], vec![1, 2, 5], vec![2, 3, 4], vec![0, 4, 3]],
        },
    ];
    let conjuncts = vec![
        (0, vec![QueryArg::Variable(0), QueryArg::Constant(3)]),
        (0, vec![QueryArg::Variable(1), QueryArg::Constant(3)]),
        (
            1,
            vec![
                QueryArg::Variable(0),
                QueryArg::Variable(1),
                QueryArg::Constant(5),
            ],
        ),
    ];
    ConjunctiveBooleanQuery::new(6, relations, 2, conjuncts)
}

#[test]
fn test_conjunctivebooleanquery_basic() {
    let problem = issue_example();
    assert_eq!(problem.domain_size(), 6);
    assert_eq!(problem.num_relations(), 2);
    assert_eq!(problem.num_variables(), 2);
    assert_eq!(problem.num_conjuncts(), 3);
    assert_eq!(problem.dims(), vec![6, 6]);
    assert_eq!(
        <ConjunctiveBooleanQuery as Problem>::NAME,
        "ConjunctiveBooleanQuery"
    );
    assert_eq!(<ConjunctiveBooleanQuery as Problem>::variant(), vec![]);
    assert_eq!(problem.relations().len(), 2);
    assert_eq!(problem.conjuncts().len(), 3);
}

#[test]
fn test_conjunctivebooleanquery_evaluate_yes() {
    let problem = issue_example();
    // y_0=0, y_1=1:
    //   conjunct 0: R_0(0, 3) = (0,3) in R_0 -> true
    //   conjunct 1: R_0(1, 3) = (1,3) in R_0 -> true
    //   conjunct 2: R_1(0, 1, 5) = (0,1,5) in R_1 -> true
    assert!(problem.evaluate(&[0, 1]));
}

#[test]
fn test_conjunctivebooleanquery_evaluate_no() {
    let problem = issue_example();
    // y_0=2, y_1=1:
    //   conjunct 0: R_0(2, 3) = (2,3) NOT in R_0 (R_0 has (2,4) not (2,3))
    assert!(!problem.evaluate(&[2, 1]));
}

#[test]
fn test_conjunctivebooleanquery_out_of_range() {
    let problem = issue_example();
    // value 6 is out of range for domain_size=6
    assert!(!problem.evaluate(&[6, 0]));
}

#[test]
fn test_conjunctivebooleanquery_wrong_length() {
    let problem = issue_example();
    // too short
    assert!(!problem.evaluate(&[0]));
    // too long
    assert!(!problem.evaluate(&[0, 1, 2]));
}

#[test]
fn test_conjunctivebooleanquery_brute_force() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_conjunctivebooleanquery_unsatisfiable() {
    // domain {0,1}, R_0 = {(0,0)}, query: (exists y_0)(R_0(y_0, y_0) /\ R_0(y_0, c1))
    // First conjunct: (y_0, y_0) in R_0 => y_0=0
    // Second conjunct: (0, 1) in R_0 => false
    let relations = vec![Relation {
        arity: 2,
        tuples: vec![vec![0, 0]],
    }];
    let conjuncts = vec![
        (0, vec![QueryArg::Variable(0), QueryArg::Variable(0)]),
        (0, vec![QueryArg::Variable(0), QueryArg::Constant(1)]),
    ];
    let problem = ConjunctiveBooleanQuery::new(2, relations, 1, conjuncts);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_conjunctivebooleanquery_serialization() {
    let problem = issue_example();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ConjunctiveBooleanQuery = serde_json::from_value(json).unwrap();
    assert_eq!(restored, problem);
}

#[test]
fn test_conjunctivebooleanquery_paper_example() {
    // Same instance as the issue example — count all satisfying assignments
    let problem = issue_example();
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    // (0,1) satisfies; verify count manually:
    // For each (y0, y1) in {0..5}x{0..5}:
    //   need R_0(y0, 3) and R_0(y1, 3) and R_1(y0, y1, 5)
    //   R_0(y0, 3): y0 in {0,1} (tuples (0,3) and (1,3))
    //   R_0(y1, 3): y1 in {0,1}
    //   R_1(y0, y1, 5): need (y0, y1, 5) in R_1
    //     (0,1,5) yes, (0,0,5) no, (1,0,5) no, (1,1,5) no
    //     Wait — R_1 = {(0,1,5),(1,2,5),(2,3,4),(0,4,3)}
    //     (0,1,5): yes. (1,2,5): y0=1,y1=2 but need R_0(2,3)=(2,3) not in R_0.
    //     So only (0,1) works given the R_0 constraint.
    assert_eq!(all.len(), 1);
    assert_eq!(all[0], vec![0, 1]);
}
