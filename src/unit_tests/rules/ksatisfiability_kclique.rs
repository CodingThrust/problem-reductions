use super::*;
use crate::models::formula::CNFClause;
use crate::models::graph::KClique;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_kclique_closed_loop() {
    // (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3), n=3, m=2
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),   // x1 ∨ x2 ∨ x3
            CNFClause::new(vec![-1, -2, 3]), // ¬x1 ∨ ¬x2 ∨ x3
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // Verify structure: 3*2 = 6 vertices, k = 2
    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.k(), 2);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(target);
    assert!(!solutions.is_empty());

    // Every KClique solution must map back to a satisfying 3-SAT assignment
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), 3);
        assert!(ksat.evaluate(&extracted));
    }
}

#[test]
fn test_ksatisfiability_to_kclique_unsatisfiable() {
    // (x1 ∨ x1 ∨ x1) ∧ (¬x1 ∨ ¬x1 ∨ ¬x1)
    // x1=T satisfies C0 but not C1; x1=F satisfies C1 but not C0.
    let ksat = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // 6 vertices, k=2
    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.k(), 2);

    // All cross-clause pairs contradict (x1 vs ¬x1), so no edges → no 2-clique.
    assert_eq!(target.num_edges(), 0);

    let solver = BruteForce::new();
    let solution = solver.find_witness(target);
    assert!(solution.is_none());
}

#[test]
fn test_ksatisfiability_to_kclique_single_clause() {
    // Single clause: (x1 ∨ x2 ∨ x3) — always satisfiable (7/8 assignments)
    // With m=1, k=1, any single vertex is a 1-clique.
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // 3 vertices, k=1, no edges needed for 1-clique
    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.k(), 1);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(target);

    // Each solution maps to a satisfying assignment
    let mut sat_assignments = std::collections::HashSet::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.evaluate(&extracted));
        sat_assignments.insert(extracted);
    }
    // 3 clique witnesses but they may map to different or same assignments
    assert!(!sat_assignments.is_empty());
}

#[test]
fn test_ksatisfiability_to_kclique_structure() {
    // Verify edge construction for a concrete example.
    // (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3)
    // Clause 0 literals: [1, 2, 3], Clause 1 literals: [-1, -2, 3]
    // Cross-clause pairs:
    //   (0,0)-(1,0): 1 vs -1 → contradict → no edge
    //   (0,0)-(1,1): 1 vs -2 → ok → edge (0,4)
    //   (0,0)-(1,2): 1 vs 3  → ok → edge (0,5)
    //   (0,1)-(1,0): 2 vs -1 → ok → edge (1,3)
    //   (0,1)-(1,1): 2 vs -2 → contradict → no edge
    //   (0,1)-(1,2): 2 vs 3  → ok → edge (1,5)
    //   (0,2)-(1,0): 3 vs -1 → ok → edge (2,3)
    //   (0,2)-(1,1): 3 vs -2 → ok → edge (2,4)
    //   (0,2)-(1,2): 3 vs 3  → ok → edge (2,5)
    // Total: 7 edges
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.num_edges(), 7);
    assert_eq!(target.k(), 2);
}

#[test]
fn test_ksatisfiability_to_kclique_three_clauses() {
    // (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ x2 ∨ ¬x3) ∧ (x1 ∨ ¬x2 ∨ x3)
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
            CNFClause::new(vec![1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // 9 vertices, k=3
    assert_eq!(target.num_vertices(), 9);
    assert_eq!(target.k(), 3);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(target);
    assert!(!solutions.is_empty());

    // Verify all solutions map back correctly
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), 3);
        assert!(ksat.evaluate(&extracted));
    }
}

#[test]
fn test_ksatisfiability_to_kclique_extract_solution_example() {
    // Verify a specific known solution.
    // (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3)
    // Assignment x1=F, x2=F, x3=T:
    //   Clause 0: x3 (position 2) true → vertex 2
    //   Clause 1: ¬x1 (position 0) true → vertex 3
    // These vertices should be connected (3 vs -1: not contradictory).
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<KClique<SimpleGraph>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // Vertices 2 and 3 selected
    let specific_config = vec![0, 0, 1, 1, 0, 0];
    assert!(target.evaluate(&specific_config));

    let extracted = reduction.extract_solution(&specific_config);
    // Vertex 2 = clause 0, pos 2 → literal 3 (x3) → x3=T → assignment[2]=1
    // Vertex 3 = clause 1, pos 0 → literal -1 (¬x1) → x1=F → assignment[0]=0
    // Unset variables default to 0.
    assert_eq!(extracted, vec![0, 0, 1]);
    assert!(ksat.evaluate(&extracted));
}
