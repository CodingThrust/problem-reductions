use super::*;
use crate::models::formula::CNFClause;
use crate::models::graph::MinimumVertexCover;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_minimumvertexcover_closed_loop() {
    // (x1 v x2 v x3) ^ (~x1 v ~x2 v x3), n=3, m=2
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),   // x1 v x2 v x3
            CNFClause::new(vec![-1, -2, 3]), // ~x1 v ~x2 v x3
        ],
    );
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // Verify structure: 2*3 + 3*2 = 12 vertices
    assert_eq!(target.num_vertices(), 12);
    // Edges: 3 truth-setting + 6*2 = 15
    assert_eq!(target.num_edges(), 15);

    // Use the helper to verify full round-trip correctness
    assert_satisfaction_round_trip_from_optimization_target(
        &ksat,
        &reduction,
        "3SAT -> MVC closed loop",
    );
}

#[test]
fn test_ksatisfiability_to_minimumvertexcover_unsatisfiable() {
    // Unsatisfiable: (x1 v x1 v x1) ^ (~x1 v ~x1 v ~x1) ^ (x1 v x1 v x1)
    let ksat = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
            CNFClause::new(vec![1, 1, 1]),
        ],
    );
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // n=1, m=3 -> 2 + 9 = 11 vertices, minimum VC should be > n + 2m = 7
    // if unsatisfiable. Actually MVC always has a solution (empty set is not valid
    // for graphs with edges, but any superset works). The key property is:
    // SAT is satisfiable iff MVC has size <= n + 2m.
    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(witness.is_some());
    let vc_config = witness.unwrap();
    let vc_size: usize = vc_config.iter().sum();
    // Unsatisfiable -> minimum VC size > n + 2m = 1 + 6 = 7
    assert!(vc_size > 7);
}

#[test]
fn test_ksatisfiability_to_minimumvertexcover_single_clause() {
    // Single clause: (x1 v x2 v x3) — 7 out of 8 assignments satisfy it
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // 2*3 + 3*1 = 9 vertices, 3 + 6 = 9 edges
    assert_eq!(target.num_vertices(), 9);
    assert_eq!(target.num_edges(), 9);

    assert_satisfaction_round_trip_from_optimization_target(
        &ksat,
        &reduction,
        "3SAT single clause -> MVC",
    );
}

#[test]
fn test_ksatisfiability_to_minimumvertexcover_extract_solution() {
    // Verify specific extraction: x1=F, x2=F, x3=T
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&ksat);

    // Literal vertices: u1(0), ~u1(1), u2(2), ~u2(3), u3(4), ~u3(5)
    // Clause 0 triangle: v6, v7, v8
    // Clause 1 triangle: v9, v10, v11
    //
    // For x1=F, x2=F, x3=T:
    //   Truth-setting: pick ~u1(1), ~u2(3), u3(4) [the true literal]
    //   Clause 0 (1,2,3): communication edges (6,0), (7,2), (8,4).
    //     u1(0) not in cover -> must pick v6. u2(2) not in cover -> must pick v7.
    //     u3(4) in cover -> edge (8,4) covered. Triangle covered by v6 and v7.
    //   Clause 1 (-1,-2,3): communication edges (9,1), (10,3), (11,4).
    //     All three endpoints (~u1, ~u2, u3) in cover. Pick any 2 from triangle: v9, v10.
    let vc_config = vec![0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0];
    // Verify this is a valid vertex cover
    assert!(reduction.target_problem().is_valid_solution(&vc_config));

    let extracted = reduction.extract_solution(&vc_config);
    assert_eq!(extracted, vec![0, 0, 1]); // x1=F, x2=F, x3=T
    assert!(ksat.evaluate(&extracted));
}

#[test]
fn test_ksatisfiability_to_minimumvertexcover_all_negated() {
    // (~x1 v ~x2 v ~x3) — 7 satisfying assignments
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&ksat);

    assert_satisfaction_round_trip_from_optimization_target(
        &ksat,
        &reduction,
        "3SAT all negated -> MVC",
    );
}

#[test]
fn test_ksatisfiability_to_minimumvertexcover_structure() {
    // Verify edge structure for a simple case
    let ksat = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -1, 2])]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // n=2, m=1 -> 4 + 3 = 7 vertices
    assert_eq!(target.num_vertices(), 7);
    // 2 truth-setting + 6*1 = 8 edges
    assert_eq!(target.num_edges(), 8);

    // Minimum cover size for satisfiable formula = n + 2m = 2 + 2 = 4
    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(witness.is_some());
    let vc_size: usize = witness.unwrap().iter().sum();
    assert_eq!(vc_size, 4);
}
