use super::*;
use crate::registry::find_problem_type_by_alias;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;
use crate::Solver;

fn issue_instance() -> MaximumContactMapOverlap {
    // Canonical example from the issue:
    //   G_1: n_1 = 4, E_1 = {{0,2}, {1,3}}
    //   G_2: n_2 = 5, E_2 = {{0,3}, {1,4}, {0,2}}
    MaximumContactMapOverlap::new(4, vec![(0, 2), (1, 3)], 5, vec![(0, 3), (1, 4), (0, 2)])
}

#[test]
fn test_maximum_contact_map_overlap_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vertices_1(), 4);
    assert_eq!(problem.num_vertices_2(), 5);
    assert_eq!(problem.num_contacts_1(), 2);
    assert_eq!(problem.num_contacts_2(), 3);
    // dims must be [|V_2| + 1; |V_1|] = [6; 4].
    assert_eq!(problem.dims(), vec![6; 4]);
    assert_eq!(problem.num_variables(), 4);
    // Contacts get normalized so the smaller endpoint comes first.
    let contacts_2 = problem.contacts_2();
    assert!(contacts_2.contains(&(0, 2)));
    assert!(contacts_2.contains(&(0, 3)));
    assert!(contacts_2.contains(&(1, 4)));
}

#[test]
fn test_maximum_contact_map_overlap_evaluate_optimum() {
    let problem = issue_instance();
    // Optimal alignment: 0->0, 1->1, 2->3, 3->4 (encoded as [1, 2, 4, 5]).
    //   - order-preserving: 1 < 2 < 4 < 5
    //   - injectivity: all values distinct
    //   - contact {0,2}: mapped (0, 3); sorted (0, 3) in E_2
    //   - contact {1,3}: mapped (1, 4); sorted (1, 4) in E_2
    //   - value = 2 contacts preserved.
    assert!(problem.is_valid_solution(&[1, 2, 4, 5]));
    assert_eq!(problem.evaluate(&[1, 2, 4, 5]).unwrap(), Max(Some(2)));
    assert_eq!(
        problem.preserved_contact_count(&[1, 2, 4, 5]).unwrap(),
        Some(2)
    );
}

#[test]
fn test_maximum_contact_map_overlap_evaluate_all_unmatched() {
    let problem = issue_instance();
    // No vertex matched -> no contacts preserved, but trivially feasible.
    assert!(problem.is_valid_solution(&[0, 0, 0, 0]));
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]).unwrap(), Max(Some(0)));
}

#[test]
fn test_maximum_contact_map_overlap_evaluate_single_match() {
    let problem = issue_instance();
    // Only one vertex matched: still feasible (a single nonzero value trivially
    // satisfies both injectivity and strict monotonicity), but no contact
    // has both endpoints matched, so the score is 0.
    assert!(problem.is_valid_solution(&[1, 0, 0, 0]));
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]).unwrap(), Max(Some(0)));
}

#[test]
fn test_maximum_contact_map_overlap_evaluate_not_injective() {
    let problem = issue_instance();
    // Two source vertices map to the same nonzero image -> infeasible.
    assert!(!problem.is_valid_solution(&[1, 1, 0, 0]));
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]).unwrap(), Max(None));
    assert_eq!(
        problem.preserved_contact_count(&[1, 1, 0, 0]).unwrap(),
        None
    );
}

#[test]
fn test_maximum_contact_map_overlap_evaluate_not_order_preserving() {
    let problem = issue_instance();
    // Vertex 0 maps to 2 (i.e. residue index 1) and vertex 1 maps to 1 (i.e.
    // residue index 0): both nonzero, but 2 > 1 in source order -> not
    // order-preserving.
    assert!(!problem.is_valid_solution(&[2, 1, 0, 0]));
    assert_eq!(problem.evaluate(&[2, 1, 0, 0]).unwrap(), Max(None));
}

#[test]
fn test_maximum_contact_map_overlap_evaluate_suboptimal_feasible() {
    let problem = issue_instance();
    // Alignment 0->0, 1->1, 2->2, 3->3 (encoded as [1, 2, 3, 4]).
    //   - order-preserving and injective.
    //   - contact {0,2}: mapped (0, 2) ∈ E_2 -> preserved
    //   - contact {1,3}: mapped (1, 3) ∉ E_2 -> not preserved
    //   - value = 1.
    assert!(problem.is_valid_solution(&[1, 2, 3, 4]));
    assert_eq!(problem.evaluate(&[1, 2, 3, 4]).unwrap(), Max(Some(1)));
}

#[test]
fn test_maximum_contact_map_overlap_brute_force_finds_optimum() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let value = solver.solve(&problem).unwrap();
    assert_eq!(value, Max(Some(2)));

    let witness = solver
        .find_witness(&problem)
        .unwrap()
        .expect("witness exists");
    assert!(problem.is_valid_solution(&witness));
    assert_eq!(problem.evaluate(&witness).unwrap(), Max(Some(2)));
}

#[test]
fn test_maximum_contact_map_overlap_rejects_wrong_length_config() {
    let problem = issue_instance();
    // |V_1| = 4, config has 3 entries -> infeasible.
    assert!(!problem.is_valid_solution(&[0, 0, 0]));
    assert_eq!(problem.evaluate(&[0, 0, 0]).unwrap(), Max(None));
    // Too long.
    assert!(!problem.is_valid_solution(&[0, 0, 0, 0, 0]));
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0]).unwrap(), Max(None));
}

#[test]
fn test_maximum_contact_map_overlap_rejects_out_of_range_value() {
    let problem = issue_instance();
    // Valid entries are 0..=|V_2| = 0..=5. Value 6 is out of range.
    assert!(!problem.is_valid_solution(&[0, 0, 0, 6]));
    assert_eq!(problem.evaluate(&[0, 0, 0, 6]).unwrap(), Max(None));
}

#[test]
fn test_maximum_contact_map_overlap_serialization_roundtrip() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).expect("serialize");
    let restored: MaximumContactMapOverlap = serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored, problem);
    assert_eq!(restored.evaluate(&[1, 2, 4, 5]).unwrap(), Max(Some(2)));
}

#[test]
fn test_maximum_contact_map_overlap_problem_name_and_variant() {
    assert_eq!(
        <MaximumContactMapOverlap as Problem>::NAME,
        "MaximumContactMapOverlap"
    );
    let v = <MaximumContactMapOverlap as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_maximum_contact_map_overlap_canonicalizes_unsorted_pairs() {
    // Pass contacts in (max, min) order -> constructor normalizes to (min, max).
    let problem = MaximumContactMapOverlap::new(4, vec![(2, 0), (3, 1)], 5, vec![(3, 0), (4, 1)]);
    assert!(problem.contacts_1().contains(&(0, 2)));
    assert!(problem.contacts_1().contains(&(1, 3)));
    assert!(problem.contacts_2().contains(&(0, 3)));
    assert!(problem.contacts_2().contains(&(1, 4)));
}

#[test]
fn test_maximum_contact_map_overlap_aliases() {
    // Both aliases should resolve to the canonical problem name.
    let pt = find_problem_type_by_alias("CMO").expect("CMO alias resolves");
    assert_eq!(pt.canonical_name, "MaximumContactMapOverlap");
    let pt = find_problem_type_by_alias("MaxCMO").expect("MaxCMO alias resolves");
    assert_eq!(pt.canonical_name, "MaximumContactMapOverlap");
}

#[test]
#[should_panic(expected = "self-loop")]
fn test_maximum_contact_map_overlap_panics_on_self_loop() {
    let _ = MaximumContactMapOverlap::new(3, vec![(1, 1)], 2, vec![]);
}

#[test]
#[should_panic(expected = "duplicate contact")]
fn test_maximum_contact_map_overlap_panics_on_duplicate_contact() {
    // (0,1) and (1,0) normalize to the same pair (0,1) -> duplicate.
    let _ = MaximumContactMapOverlap::new(3, vec![(0, 1), (1, 0)], 2, vec![]);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_maximum_contact_map_overlap_panics_on_endpoint_out_of_range() {
    let _ = MaximumContactMapOverlap::new(3, vec![(0, 3)], 2, vec![]);
}
