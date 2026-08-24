use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;
use crate::Solver;

fn issue_instance() -> MaximumCommonEdgeSubgraph {
    // Labels a/b/c/d encoded as 0/1/2/3 (alphabetical).
    MaximumCommonEdgeSubgraph::new(
        LabelledDigraph::new(
            5,
            vec![
                LabelledArc::new(0, 0, 1),
                LabelledArc::new(1, 1, 2),
                LabelledArc::new(0, 2, 2),
                LabelledArc::new(2, 0, 3),
                LabelledArc::new(1, 3, 3),
                LabelledArc::new(3, 1, 4),
            ],
        ),
        LabelledDigraph::new(
            4,
            vec![
                LabelledArc::new(0, 0, 1),
                LabelledArc::new(1, 1, 2),
                LabelledArc::new(0, 2, 2),
                LabelledArc::new(2, 0, 3),
                LabelledArc::new(1, 3, 3),
                LabelledArc::new(0, 1, 3),
            ],
        ),
    )
}

#[test]
fn test_maximum_common_edge_subgraph_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vertices_1(), 5);
    assert_eq!(problem.num_vertices_2(), 4);
    assert_eq!(problem.num_arcs_1(), 6);
    assert_eq!(problem.num_arcs_2(), 6);
    assert_eq!(problem.bottom_index(), 4);
    // dims must be [|V2| + 1; |V1|] = [5; 5].
    assert_eq!(problem.dims(), vec![5; 5]);
    assert_eq!(problem.num_variables(), 5);
}

#[test]
fn test_maximum_common_edge_subgraph_evaluate_optimum() {
    let problem = issue_instance();

    // Issue optimum: x = (0, 1, 2, 3, 4) with 4 = bottom (|V2| = 4).
    // Preserved arcs: (0,a,1), (1,b,2), (0,c,2), (2,a,3), (1,d,3); the last
    // source arc (3,b,4) is skipped because vertex 4 is unmatched.
    assert!(problem.is_valid_solution(&[0, 1, 2, 3, 4]));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4]).unwrap(), Max(Some(5)));
    assert_eq!(
        problem.preserved_arc_count(&[0, 1, 2, 3, 4]).unwrap(),
        Some(5)
    );
}

#[test]
fn test_maximum_common_edge_subgraph_evaluate_injectivity_violated() {
    let problem = issue_instance();

    // Two source vertices map to graph_2 vertex 0 -> injectivity violated.
    assert!(!problem.is_valid_solution(&[0, 0, 2, 3, 4]));
    assert_eq!(problem.evaluate(&[0, 0, 2, 3, 4]).unwrap(), Max(None));
    assert_eq!(problem.preserved_arc_count(&[0, 0, 2, 3, 4]).unwrap(), None);
}

#[test]
fn test_maximum_common_edge_subgraph_evaluate_fewer_preserved() {
    let problem = issue_instance();

    // Swap 1 and 2 in graph_2: f = {0->0, 1->2, 2->1, 3->3, 4->bottom}.
    // Check each source arc against G2:
    // (0,a=0,1) -> (0,0,2): NOT in G2 (G2 has (0,0,1)).
    // (1,b=1,2) -> (2,1,1): NOT in G2.
    // (0,c=2,2) -> (0,2,1): NOT in G2 (G2 has (0,2,2)).
    // (2,a=0,3) -> (1,0,3): NOT in G2.
    // (1,d=3,3) -> (2,3,3): NOT in G2 (G2 has (1,3,3)).
    // (3,b=1,4) -> vertex 4 unmatched, skip.
    // So preserved = 0.
    let config = [0, 2, 1, 3, 4];
    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(0)));

    // Unmatch vertex 3 as well: lose the (2,a,3) and (1,d,3) preservations
    // but keep (0,a,1), (1,b,2), (0,c,2). Total = 3.
    let config = [0, 1, 2, 4, 4];
    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(3)));

    // All unmatched -> nothing preserved but still feasible.
    let config = [4, 4, 4, 4, 4];
    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config).unwrap(), Max(Some(0)));
}

#[test]
fn test_maximum_common_edge_subgraph_brute_force_finds_optimum() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let value = solver.solve(&problem).unwrap();
    assert_eq!(value, Max(Some(5)));

    let witness = solver
        .find_witness(&problem)
        .unwrap()
        .expect("witness exists");
    assert!(problem.is_valid_solution(&witness));
    assert_eq!(problem.evaluate(&witness).unwrap(), Max(Some(5)));
}

#[test]
fn test_maximum_common_edge_subgraph_serialization_roundtrip() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).expect("serialize");
    let restored: MaximumCommonEdgeSubgraph = serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored.num_vertices_1(), 5);
    assert_eq!(restored.num_vertices_2(), 4);
    assert_eq!(restored.num_arcs_1(), 6);
    assert_eq!(restored.num_arcs_2(), 6);
    assert_eq!(restored.evaluate(&[0, 1, 2, 3, 4]).unwrap(), Max(Some(5)));
    assert_eq!(restored, problem);
}

#[test]
fn test_maximum_common_edge_subgraph_problem_name_and_variant() {
    assert_eq!(
        <MaximumCommonEdgeSubgraph as Problem>::NAME,
        "MaximumCommonEdgeSubgraph"
    );
    let v = <MaximumCommonEdgeSubgraph as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_maximum_common_edge_subgraph_rejects_wrong_length_config() {
    let problem = issue_instance();
    // |V1| = 5, but the config has 4 entries -> infeasible.
    assert!(!problem.is_valid_solution(&[0, 1, 2, 3]));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]).unwrap(), Max(None));
    // Too long.
    assert!(!problem.is_valid_solution(&[0, 1, 2, 3, 4, 4]));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4, 4]).unwrap(), Max(None));
}

#[test]
fn test_maximum_common_edge_subgraph_rejects_out_of_range_target() {
    let problem = issue_instance();
    // 5 is out of range: the only legal "unmatched" sentinel is |V2| = 4.
    assert!(!problem.is_valid_solution(&[0, 1, 2, 3, 5]));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 5]).unwrap(), Max(None));
}

#[test]
#[should_panic(expected = "labelled arc source")]
fn test_labelled_digraph_rejects_out_of_range_source() {
    let _ = LabelledDigraph::new(2, vec![LabelledArc::new(2, 0, 0)]);
}

#[test]
#[should_panic(expected = "labelled arc destination")]
fn test_labelled_digraph_rejects_out_of_range_destination() {
    let _ = LabelledDigraph::new(2, vec![LabelledArc::new(0, 0, 2)]);
}

#[test]
fn test_labelled_digraph_deduplicates_arcs() {
    let g = LabelledDigraph::new(
        3,
        vec![
            LabelledArc::new(0, 1, 2),
            LabelledArc::new(0, 1, 2),
            LabelledArc::new(1, 0, 2),
        ],
    );
    assert_eq!(g.num_arcs(), 2);
}
