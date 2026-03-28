use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_register_sufficiency_basic() {
    let problem = RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        3,
    );
    assert_eq!(problem.num_vertices(), 7);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.bound(), 3);
    assert_eq!(
        problem.arcs(),
        &[
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5)
        ]
    );
    assert_eq!(problem.dims(), vec![7; 7]);
    assert_eq!(
        <RegisterSufficiency as Problem>::NAME,
        "RegisterSufficiency"
    );
    assert_eq!(<RegisterSufficiency as Problem>::variant(), vec![]);
}

#[test]
fn test_register_sufficiency_evaluate_valid() {
    // Issue #515 example: 7 vertices, 8 arcs, K=3
    let problem = RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        3,
    );
    // Order: v0,v1,v2,v3,v5,v4,v6 (0-indexed)
    // Positions: v0->0, v1->1, v2->2, v3->3, v4->5, v5->4, v6->6
    let config = vec![0, 1, 2, 3, 5, 4, 6];
    assert!(problem.evaluate(&config));

    // Verify register count
    let max_reg = problem.simulate_registers(&config).unwrap();
    assert_eq!(max_reg, 3);
}

#[test]
fn test_register_sufficiency_evaluate_invalid_permutation() {
    let problem = RegisterSufficiency::new(4, vec![(2, 0), (3, 0), (3, 1)], 2);
    // Not a permutation: position 0 used twice
    assert!(!problem.evaluate(&[0, 0, 1, 2]));
    // Wrong length
    assert!(!problem.evaluate(&[0, 1, 2]));
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4]));
    // Position out of range
    assert!(!problem.evaluate(&[0, 1, 2, 4]));
}

#[test]
fn test_register_sufficiency_evaluate_invalid_dependency() {
    // v2 depends on v0, v3 depends on v0 and v1
    let problem = RegisterSufficiency::new(4, vec![(2, 0), (3, 0), (3, 1)], 4);
    // v2 at position 0, v0 at position 1 -> v2 evaluated before its dependency v0
    assert!(!problem.evaluate(&[1, 2, 0, 3]));
}

#[test]
fn test_register_sufficiency_evaluate_exceeds_bound() {
    // Issue example with K=2 (should fail - minimum is 3)
    let problem = RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        2,
    );
    // Same valid ordering but K=2 is too small
    let config = vec![0, 1, 2, 3, 5, 4, 6];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_register_sufficiency_brute_force() {
    // Small instance: 4 vertices, v2 depends on v0, v3 depends on v1
    let problem = RegisterSufficiency::new(4, vec![(2, 0), (3, 1)], 2);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_register_sufficiency_brute_force_all() {
    let problem = RegisterSufficiency::new(4, vec![(2, 0), (3, 1)], 2);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_register_sufficiency_unsatisfiable() {
    // Chain: v3 depends on v2, v2 depends on v1, v1 depends on v0
    // Plus: v3 also depends on v0
    // This requires 3 registers (v0 must stay alive until v3)
    // With K=1, impossible
    let problem = RegisterSufficiency::new(4, vec![(1, 0), (2, 1), (3, 2), (3, 0)], 1);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_register_sufficiency_serialization() {
    let problem = RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        3,
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: RegisterSufficiency = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_arcs(), problem.num_arcs());
    assert_eq!(restored.bound(), problem.bound());
    assert_eq!(restored.arcs(), problem.arcs());
}

#[test]
fn test_register_sufficiency_empty() {
    let problem = RegisterSufficiency::new(0, vec![], 0);
    assert_eq!(problem.num_vertices(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_register_sufficiency_single_vertex() {
    let problem = RegisterSufficiency::new(1, vec![], 1);
    assert!(problem.evaluate(&[0]));
    // K=0 should fail (vertex needs one register)
    let problem_k0 = RegisterSufficiency::new(1, vec![], 0);
    assert!(!problem_k0.evaluate(&[0]));
}

#[test]
fn test_register_sufficiency_paper_example() {
    // The issue example: 7 vertices, 8 arcs, K=3
    let problem = RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        3,
    );

    // The order from the issue: v1,v2,v3,v4,v6,v5,v7 (1-indexed)
    // = v0,v1,v2,v3,v5,v4,v6 (0-indexed)
    // Positions: v0->0, v1->1, v2->2, v3->3, v4->5, v5->4, v6->6
    let config = vec![0, 1, 2, 3, 5, 4, 6];
    assert!(problem.evaluate(&config));
    assert_eq!(problem.simulate_registers(&config).unwrap(), 3);

    // Verify K=2 is impossible using brute force
    let problem_k2 = RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        2,
    );
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem_k2).is_none());
}
