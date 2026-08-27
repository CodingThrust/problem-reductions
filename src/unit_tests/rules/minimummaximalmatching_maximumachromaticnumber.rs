use crate::models::graph::{MaximumAchromaticNumber, MinimumMaximalMatching};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{BipartiteGraph, Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, Min};

/// Build the canonical T-tree example: spider with three legs at the centre
/// v1, namely v0--v1--v2--v3 with an extra leaf v1--v4. The bipartition is
/// A = {v0, v2, v4}, B = {v1, v3}. See the canonical builder for the full
/// unified-index encoding.
fn t_tree_bipartite() -> BipartiteGraph {
    // left_size = 3 (A), right_size = 2 (B)
    // edges in (left_idx, right_idx) form:
    //   (v0, v1) -> (0, 0)
    //   (v1, v2) -> (1, 0)
    //   (v2, v3) -> (1, 1)
    //   (v1, v4) -> (2, 0)
    BipartiteGraph::new(3, 2, vec![(0, 0), (1, 0), (1, 1), (2, 0)])
}

#[test]
fn test_minimummaximalmatching_to_maximumachromaticnumber_closed_loop() {
    let source = MinimumMaximalMatching::new(t_tree_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // |V| = 5, |E(G)| = 4, so |E(H)| = C(5,2) - 4 = 10 - 4 = 6
    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);

    let solver = BruteForce::new();

    // Source MMM(T-tree) = 1 (the central edge (v1, v2) is a minimum maximal
    // matching on its own).
    assert_eq!(
        source
            .evaluate(&solver.solve(&source).unwrap().unwrap())
            .unwrap(),
        Min(Some(1))
    );

    // Target achromatic number of complement(G) = |V| - mm(G) = 5 - 1 = 4.
    assert_eq!(
        target
            .evaluate(&solver.solve(target).unwrap().unwrap())
            .unwrap(),
        Max(Some(4))
    );

    // Closed-loop: every optimal target witness extracts to a valid maximal
    // matching with size mm(G).
    let target_witnesses = solver.find_all_witnesses(target).unwrap();
    assert!(
        !target_witnesses.is_empty(),
        "complement(T-tree) must admit an achromatic 4-coloring"
    );
    for witness in &target_witnesses {
        let extracted = reduction.extract_solution(witness).unwrap();
        assert_eq!(
            source.evaluate(&extracted).unwrap(),
            Min(Some(1)),
            "extracted matching must be maximal of size 1"
        );
    }
}

#[test]
fn test_target_complement_structure() {
    let source = MinimumMaximalMatching::new(t_tree_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // Source unified edges: (0,3), (1,3), (1,4), (2,3).
    // Complement edges in K_5 minus source: (0,1), (0,2), (0,4), (1,2), (2,4), (3,4).
    let mut target_edges = target.graph().edges();
    target_edges.sort();
    assert_eq!(
        target_edges,
        vec![(0, 1), (0, 2), (0, 4), (1, 2), (2, 4), (3, 4)]
    );
}

#[test]
fn test_extract_solution_known_coloring() {
    let source = MinimumMaximalMatching::new(t_tree_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    // Canonical 4-coloring of complement(G) (unified order v0,v2,v4,v1,v3):
    //   v0 -> 1, v2 -> 0, v4 -> 3, v1 -> 0, v3 -> 2.
    // The single size-2 class {v2, v1} is the G-edge (v1, v2) =
    // unified edge (1, 3), source-edge index 1 in the edges list.
    let coloring = vec![1, 0, 3, 0, 2];
    let extracted = reduction.extract_solution(&coloring).unwrap();
    assert_eq!(extracted, vec![false, true, false, false]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(1)));
}

#[test]
fn test_extract_solution_recovers_suboptimal_matchings() {
    // The T-tree exposes >=2 suboptimal maximal matchings besides the
    // optimum, exactly the situation that motivated the richer canonical
    // example. We feed the achromatic colorings induced by these size-2
    // maximal matchings and check the extractor recovers each one.
    let source = MinimumMaximalMatching::new(t_tree_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    // Unified labels: v0=0, v2=1, v4=2, v1=3, v3=4.
    // Suboptimal matching {(v0,v1), (v2,v3)} -> color v0,v1 the same and
    // v2,v3 the same; v4 takes a third color.
    // Source edges in unified order: (0,3), (1,3), (1,4), (2,3).
    // Edge 0 = (v0, v1) selected; edge 2 = (v2, v3) selected.
    let coloring_a = vec![0, 1, 2, 0, 1];
    let extracted_a = reduction.extract_solution(&coloring_a).unwrap();
    assert_eq!(extracted_a, vec![true, false, true, false]);
    assert_eq!(source.evaluate(&extracted_a).unwrap(), Min(Some(2)));

    // Suboptimal matching {(v1, v4), (v2, v3)} -> pair v1 with v4 and v2
    // with v3; v0 takes a singleton color. Edge 2 = (v2, v3); edge 3 = (v1, v4).
    let coloring_b = vec![2, 0, 1, 1, 0];
    let extracted_b = reduction.extract_solution(&coloring_b).unwrap();
    assert_eq!(extracted_b, vec![false, false, true, true]);
    assert_eq!(source.evaluate(&extracted_b).unwrap(), Min(Some(2)));
}

#[test]
fn test_identity_on_random_bipartite_instances() {
    // Verify the central identity ach(complement(G)) = |V| - mm(G) on small
    // bipartite instances by reducing and solving both sides.
    let solver = BruteForce::new();

    // Small library of bipartite graphs:
    //  - K_{2,2} (4-cycle viewed as bipartite, left=2, right=2)
    //  - K_{1,3} (claw / star), and
    //  - the canonical T-tree above.
    let instances = vec![
        BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        BipartiteGraph::new(1, 3, vec![(0, 0), (0, 1), (0, 2)]),
        t_tree_bipartite(),
    ];

    for graph in instances {
        let n = graph.num_vertices();
        let source = MinimumMaximalMatching::new(graph);
        let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source)
            .expect("reduction should succeed");
        let target = reduction.target_problem();

        let source_solution = solver.solve(&source).unwrap().unwrap();
        let Min(Some(mm)) = source.evaluate(&source_solution).unwrap() else {
            panic!("MinimumMaximalMatching always has a feasible optimum");
        };
        let target_solution = solver.solve(target).unwrap().unwrap();
        let Max(Some(ach)) = target.evaluate(&target_solution).unwrap() else {
            panic!("MaximumAchromaticNumber always has a feasible optimum");
        };

        assert_eq!(
            ach + mm,
            i64::try_from(n).unwrap(),
            "ach(complement(G)) + mm(G) must equal |V|"
        );
    }
}
