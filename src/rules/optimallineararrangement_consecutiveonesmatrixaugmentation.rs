//! Reduction from Decision Optimal Linear Arrangement to Consecutive Ones
//! Matrix Augmentation.
//!
//! Establishes NP-completeness of CONSECUTIVE ONES MATRIX AUGMENTATION
//! (Garey & Johnson SR16) via transformation from OPTIMAL LINEAR ARRANGEMENT
//! (GT42). Given `Decision<OptimalLinearArrangement>(G, k)`, the edge-vertex
//! incidence matrix `A` of `G` has rows = edges and columns = vertices. A
//! column permutation is exactly a vertex ordering `f`; making each edge row
//! consecutive costs `|f(u) - f(v)| - 1` flips, so the total augmentation cost
//! equals `(total edge length) - |E|`. Hence the source is YES iff
//! `ConsecutiveOnesMatrixAugmentation(A, k - |E|)` is YES.

use crate::models::algebraic::ConsecutiveOnesMatrixAugmentation;
use crate::models::decision::Decision;
use crate::models::graph::OptimalLinearArrangement;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Which construction branch produced the target instance.
#[derive(Debug, Clone)]
enum ConstructionKind {
    /// Edgeless source (`m = 0`): always-YES `[[false]]` sentinel.
    /// Carries the source vertex count to reconstruct an identity arrangement.
    EdgelessYes { num_vertices: usize },
    /// `k < m`: genuine-NO 3x3 cyclic-overlap sentinel.
    FixedNo { num_vertices: usize },
    /// Generic incidence-matrix construction (`m >= 1`, `k >= m`).
    Incidence { num_vertices: usize },
}

/// Result of reducing `Decision<OptimalLinearArrangement<SimpleGraph>>` to
/// `ConsecutiveOnesMatrixAugmentation`.
#[derive(Debug, Clone)]
pub struct ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
    target: ConsecutiveOnesMatrixAugmentation,
    construction: ConstructionKind,
}

impl ReductionResult for ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
    type Source = Decision<OptimalLinearArrangement<SimpleGraph>>;
    type Target = ConsecutiveOnesMatrixAugmentation;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        match &self.construction {
            // No edges: any arrangement has total length 0 <= k, so emit the
            // identity arrangement f(v) = v over all source vertices.
            ConstructionKind::EdgelessYes { num_vertices } => (0..*num_vertices).collect(),
            // Genuine NO: there is no valid arrangement; return a sentinel
            // (identity) so the source decision evaluates correctly (NO).
            ConstructionKind::FixedNo { num_vertices } => (0..*num_vertices).collect(),
            ConstructionKind::Incidence { num_vertices } => {
                // The C1MA witness is a column permutation: `config[position] = col`.
                // Columns correspond to vertices, so this places vertex `col` at
                // `position`. The OLA arrangement is `f(vertex) = position`, i.e.
                // the inverse permutation.
                let n = *num_vertices;
                if target_solution.len() != n {
                    return (0..n).collect();
                }
                let mut arrangement = vec![0usize; n];
                let mut seen = vec![false; n];
                for (position, &vertex) in target_solution.iter().enumerate() {
                    if vertex >= n || seen[vertex] {
                        // Not a valid permutation; fall back to identity.
                        return (0..n).collect();
                    }
                    seen[vertex] = true;
                    arrangement[vertex] = position;
                }
                arrangement
            }
        }
    }
}

/// The fixed 3x3 cyclic-overlap NO sentinel: under every column permutation at
/// least one row's two 1's straddle a 0, so the minimum augmentation cost is
/// `1 > 0`.
fn no_sentinel() -> ConsecutiveOnesMatrixAugmentation {
    ConsecutiveOnesMatrixAugmentation::new(
        vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, true],
        ],
        0,
    )
}

#[reduction(
    overhead = {
        num_rows = "num_edges",
        num_cols = "num_vertices",
        bound = "k - num_edges",
    }
)]
impl ReduceTo<ConsecutiveOnesMatrixAugmentation> for Decision<OptimalLinearArrangement<SimpleGraph>> {
    type Result = ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_edges();
        let k = self.k();

        // Edgeless graph: total edge length is 0 for every arrangement, so the
        // source decision is YES for any bound. Emit a 1x1 all-zero matrix
        // (already C1P at cost 0 <= bound) to keep num_cols >= 1.
        if m == 0 {
            return ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
                target: ConsecutiveOnesMatrixAugmentation::new(vec![vec![false]], k as i64),
                construction: ConstructionKind::EdgelessYes { num_vertices: n },
            };
        }

        // Negative target bound (k < m): every arrangement costs at least m
        // (each edge contributes >= 1), so the source decision is NO. Route to
        // the fixed genuine-NO sentinel.
        if k < m {
            return ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
                target: no_sentinel(),
                construction: ConstructionKind::FixedNo { num_vertices: n },
            };
        }

        // Generic case: edge-vertex incidence matrix, rows = edges, cols = vertices.
        let mut matrix = vec![vec![false; n]; m];
        for (edge_idx, (u, v)) in self.inner().graph().edges().into_iter().enumerate() {
            matrix[edge_idx][u] = true;
            matrix[edge_idx][v] = true;
        }
        let bound = (k - m) as i64;

        ReductionOptimalLinearArrangementToConsecutiveOnesMatrixAugmentation {
            target: ConsecutiveOnesMatrixAugmentation::new(matrix, bound),
            construction: ConstructionKind::Incidence { num_vertices: n },
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimallineararrangement_to_consecutiveonesmatrixaugmentation",
        build: || {
            use crate::example_db::specs::assemble_rule_example;

            // 6 vertices, 7 edges (path + two chords). Optimal arrangement
            // [0,1,2,3,4,5] has total edge length 11; with k = 11 the target
            // bound is 11 - 7 = 4 and the identity column permutation works.
            let source = Decision::new(
                OptimalLinearArrangement::new(SimpleGraph::new(
                    6,
                    vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
                )),
                11,
            );
            let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);
            // Source arrangement f(v) = v <=> target column permutation = identity.
            let source_config = vec![0, 1, 2, 3, 4, 5];
            let target_config = vec![0, 1, 2, 3, 4, 5];
            assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/optimallineararrangement_consecutiveonesmatrixaugmentation.rs"]
mod tests;
