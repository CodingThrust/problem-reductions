use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::{Max, Min};

/// Build the canonical issue example: path P4 represented as a bipartite graph
/// with bipartition A = {v0, v2}, B = {v1, v3} and edges (v0,v1), (v1,v2),
/// (v2,v3). See the canonical builder for the unified-index encoding.
fn p4_bipartite() -> BipartiteGraph {
    // left_size = 2 (A), right_size = 2 (B)
    // edges in (left_idx, right_idx) form:
    //   (v0, v1) -> (0, 0)
    //   (v1, v2) -> (1, 0)
    //   (v2, v3) -> (1, 1)
    BipartiteGraph::new(2, 2, vec![(0, 0), (1, 0), (1, 1)])
}

#[test]
fn test_minimummaximalmatching_to_maximumachromaticnumber_closed_loop() {
    let source = MinimumMaximalMatching::new(p4_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    // |V| = 4, |E(G)| = 3, so |E(H)| = C(4,2) - 3 = 6 - 3 = 3
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 3);

    let solver = BruteForce::new();

    // Source MMM(P4) = 1 (the single middle edge is the minimum maximal matching).
    assert_eq!(solver.solve(&source), Min(Some(1)));

    // Target achromatic number of complement(P4) = |V| - mm(G) = 4 - 1 = 3.
    assert_eq!(solver.solve(target), Max(Some(3)));

    // Closed-loop: every optimal target witness extracts to a valid maximal
    // matching with size mm(G).
    let target_witnesses = solver.find_all_witnesses(target);
    assert!(
        !target_witnesses.is_empty(),
        "complement(P4) must admit an achromatic 3-coloring"
    );
    for witness in &target_witnesses {
        let extracted = reduction.extract_solution(witness);
        assert_eq!(
            source.evaluate(&extracted),
            Min(Some(1)),
            "extracted matching must be maximal of size 1"
        );
    }
}

#[test]
fn test_target_complement_structure() {
    let source = MinimumMaximalMatching::new(p4_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Source unified edges: (0,2), (1,2), (1,3).
    // Complement edges in K_4 minus source: (0,1), (0,3), (2,3).
    let mut target_edges = target.graph().edges();
    target_edges.sort();
    assert_eq!(target_edges, vec![(0, 1), (0, 3), (2, 3)]);
}

#[test]
fn test_extract_solution_known_coloring() {
    let source = MinimumMaximalMatching::new(p4_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source);

    // Coloring v0=0, v2=1, v1=1, v3=2 (unified order: v0,v2,v1,v3).
    // Size-2 class {1, 2} = {v2, v1}, which is the G-edge (v1, v2) =
    // unified edge (1, 2), source-edge index 1 in our edges list.
    let coloring = vec![0, 1, 1, 2];
    let extracted = reduction.extract_solution(&coloring);
    assert_eq!(extracted, vec![0, 1, 0]);
    assert_eq!(source.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_no_instance_higher_k_unreachable() {
    // For source K = 0, the source decision is NO because mm(P4) = 1 > 0.
    // The reduction sets K' = |V| - K = 4. The target asks for achromatic >= 4.
    // complement(P4) on 4 vertices admits at most achromatic number 3, so the
    // target decision is also NO (no 4-coloring is both proper and complete).
    let source = MinimumMaximalMatching::new(p4_bipartite());
    let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_value = solver.solve(target);

    // Achromatic value is exactly 3 on this complement (verified above), so
    // the threshold 4 is unreachable.
    if let Max(Some(value)) = target_value {
        assert!(value < 4, "complement(P4) cannot reach achromatic = 4");
    } else {
        panic!("target must have some achromatic number");
    }
}

#[test]
fn test_identity_on_random_bipartite_instances() {
    // Verify the central identity ach(complement(G)) = |V| - mm(G) on small
    // bipartite instances by reducing and solving both sides.
    let solver = BruteForce::new();

    // Small library of bipartite graphs:
    //  - K_{2,2} (4-cycle viewed as bipartite, left=2, right=2)
    //  - K_{1,3} (claw / star), and
    //  - the canonical P4 above.
    let instances = vec![
        BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        BipartiteGraph::new(1, 3, vec![(0, 0), (0, 1), (0, 2)]),
        p4_bipartite(),
    ];

    for graph in instances {
        let n = graph.num_vertices();
        let source = MinimumMaximalMatching::new(graph);
        let reduction = ReduceTo::<MaximumAchromaticNumber<SimpleGraph>>::reduce_to(&source);
        let target = reduction.target_problem();

        let Min(Some(mm)) = solver.solve(&source) else {
            panic!("MinimumMaximalMatching always has a feasible optimum");
        };
        let Max(Some(ach)) = solver.solve(target) else {
            panic!("MaximumAchromaticNumber always has a feasible optimum");
        };

        assert_eq!(ach + mm, n, "ach(complement(G)) + mm(G) must equal |V|");
    }
}
