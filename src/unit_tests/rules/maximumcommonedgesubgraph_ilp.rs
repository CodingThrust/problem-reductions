use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::{LabelledArc, LabelledDigraph, MaximumCommonEdgeSubgraph};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Max;

/// Small instance with two identical 3-vertex, 2-arc paths.
/// Distinct labels on each arc force a unique optimal partial map.
fn matched_paths() -> MaximumCommonEdgeSubgraph {
    MaximumCommonEdgeSubgraph::new(
        LabelledDigraph::new(
            3,
            vec![LabelledArc::new(0, 0, 1), LabelledArc::new(1, 1, 2)],
        ),
        LabelledDigraph::new(
            3,
            vec![LabelledArc::new(0, 0, 1), LabelledArc::new(1, 1, 2)],
        ),
    )
}

/// |V1| = 3 mapped into |V2| = 2, with one source arc that has no
/// label-compatible target arc: optimum is at most |E2| = 1.
fn truncated_instance() -> MaximumCommonEdgeSubgraph {
    MaximumCommonEdgeSubgraph::new(
        LabelledDigraph::new(
            3,
            vec![
                LabelledArc::new(0, 0, 1),
                LabelledArc::new(1, 7, 2), // label 7 absent in G2
            ],
        ),
        LabelledDigraph::new(2, vec![LabelledArc::new(0, 0, 1)]),
    )
}

#[test]
fn test_maximumcommonedgesubgraph_to_ilp_structure() {
    let source = matched_paths();
    let reduction: ReductionMCESToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // n1 = 3, n2 = 3 → 9 x-variables. Label-compatible arc pairs: (a,0)<->(b,0)
    // matches label 0; (a,1)<->(b,1) matches label 1. Cross-label pairs are
    // incompatible, so y_pairs = 2.
    let num_x = 9;
    let num_y = 2;
    assert_eq!(ilp.num_vars, num_x + num_y);
    // 3 row + 3 column + 3 McCormick constraints per y pair.
    assert_eq!(ilp.constraints.len(), 3 + 3 + 3 * num_y);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    // Objective is sum of y variables only.
    assert_eq!(ilp.objective, vec![(num_x, 1.0), (num_x + 1, 1.0)]);
}

#[test]
fn test_maximumcommonedgesubgraph_to_ilp_closed_loop() {
    let source = matched_paths();
    let reduction: ReductionMCESToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("matched paths ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(2)));
    // Optimal mapping preserves both arcs by aligning 0->0, 1->1, 2->2.
    assert_eq!(extracted, vec![0, 1, 2]);
}

#[test]
fn test_maximumcommonedgesubgraph_to_ilp_bf_vs_ilp() {
    let source = matched_paths();
    let reduction: ReductionMCESToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_maximumcommonedgesubgraph_to_ilp_truncated_target() {
    let source = truncated_instance();
    let reduction: ReductionMCESToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // n1=3, n2=2 → 6 x-vars; only the label-0 arc has a match, so 1 y-var.
    assert_eq!(ilp.num_vars, 6 + 1);
    // 3 row + 2 column + 3 McCormick.
    assert_eq!(ilp.constraints.len(), 3 + 2 + 3);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("truncated ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(1)));
}

#[test]
fn test_maximumcommonedgesubgraph_to_ilp_empty_graphs() {
    // Edge corner case: both graphs have no arcs. Optimum is 0 and the
    // resulting ILP has no y-variables and no McCormick constraints.
    let source = MaximumCommonEdgeSubgraph::new(
        LabelledDigraph::new(2, vec![]),
        LabelledDigraph::new(2, vec![]),
    );
    let reduction: ReductionMCESToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 2 * 2);
    assert_eq!(ilp.constraints.len(), 2 + 2);
    assert!(ilp.objective.is_empty());

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("empty-arc ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(0)));
}

#[test]
fn test_maximumcommonedgesubgraph_to_ilp_self_loop() {
    // Corner case: self-loops with matching labels. A single self-loop
    // mapped to a matching target self-loop preserves the arc.
    let source = MaximumCommonEdgeSubgraph::new(
        LabelledDigraph::new(1, vec![LabelledArc::new(0, 3, 0)]),
        LabelledDigraph::new(2, vec![LabelledArc::new(1, 3, 1)]),
    );
    let reduction: ReductionMCESToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("self-loop ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(1)));
    assert_eq!(extracted, vec![1]);
}
