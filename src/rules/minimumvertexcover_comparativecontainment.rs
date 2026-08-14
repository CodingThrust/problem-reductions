//! Reduction from Decision Minimum Vertex Cover to Comparative Containment.
//!
//! Implements the Plaisted (1976) construction (Garey & Johnson SP10): given a
//! graph `G = (V, E)` and a bound `K`, build a Comparative Containment instance
//! over universe `X = V` with two weighted set families `R` and `S` such that a
//! subset `Y ⊆ X` satisfies the containment inequality iff `Y` is a vertex
//! cover of `G` of size at most `K`.
//!
//! - For each vertex `v`, add `R_v = V \ {v}` with weight `1`. The total
//!   R-weight equals `n - |Y|`.
//! - For each edge `e = {u, v}`, add `S_e = V \ {u, v}` with weight `n + 1`.
//!   Each uncovered edge contributes a penalty larger than the maximum possible
//!   R-weight, so any feasible `Y` must cover every edge.
//! - One budget set `S_0 = V` with weight `n - K`. The containment inequality
//!   becomes `K - |Y| ≥ (n + 1) · (# uncovered edges)`.
//!
//! Source: `Decision<MinimumVertexCover<SimpleGraph, i32>>` with unit weights.
//! See `decisionminimumvertexcover_hamiltoniancircuit.rs` for the analogous
//! unit-weight assertion pattern.

use crate::models::decision::Decision;
use crate::models::graph::MinimumVertexCover;
use crate::models::set::ComparativeContainment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing `Decision<MinimumVertexCover<SimpleGraph, i32>>` to
/// `ComparativeContainment<i32>`.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMVCToComparativeContainment {
    target: ComparativeContainment<i32>,
    num_source_vertices: usize,
    /// If `Some`, the bound makes every vertex subset trivially feasible
    /// (`K ≥ n`); the reduction emits an empty target instance and any
    /// extracted source configuration is forced to be a YES instance.
    trivial_yes: Option<Vec<usize>>,
}

impl ReductionResult for ReductionDecisionMVCToComparativeContainment {
    type Source = Decision<MinimumVertexCover<SimpleGraph, i32>>;
    type Target = ComparativeContainment<i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            if let Some(witness) = &self.trivial_yes {
                return Ok(witness.clone());
            }
            let mut cover = vec![0; self.num_source_vertices];
            for (vertex, &selected) in target_solution[..self.num_source_vertices]
                .iter()
                .enumerate()
            {
                cover[vertex] = selected;
            }
            cover
        })
    }
}

#[reduction(
    size = exact {
        universe_size = "num_vertices",
        num_r_sets = "num_vertices",
        num_s_sets = "num_edges + 1",
    }
)]
impl ReduceTo<ComparativeContainment<i32>> for Decision<MinimumVertexCover<SimpleGraph, i32>> {
    type Result = ReductionDecisionMVCToComparativeContainment;

    fn reduce_to(&self) -> Self::Result {
        let weights = self.inner().weights();
        assert!(
            weights.iter().all(|&weight| weight == 1),
            "Plaisted 1976 reduction requires unit vertex weights"
        );

        let num_vertices = self.inner().graph().num_vertices();
        let raw_bound = *self.bound();

        // Trivially NO corner case: a negative bound cannot be met by any
        // (nonnegative) cover size. Emit an unsatisfiable target: universe of
        // size 1 with a single S-set {0} and no R-sets, so the R-weight is
        // always 0 < 1 = S-weight regardless of Y.
        if raw_bound < 0 {
            let target = ComparativeContainment::with_weights(
                1,
                Vec::new(),
                vec![vec![0]],
                Vec::<i32>::new(),
                vec![1i32],
            );
            return ReductionDecisionMVCToComparativeContainment {
                target,
                num_source_vertices: num_vertices,
                trivial_yes: None,
            };
        }

        // Trivial YES corner case: when K >= n, every vertex subset of size at
        // most n is a feasible cover (in particular the all-ones configuration
        // covers every edge), so the answer is YES regardless of the graph.
        // Emit an empty target instance (universe size 0, no R/S sets); its
        // unique configuration is trivially satisfying.
        if i128::from(raw_bound) >= i128::try_from(num_vertices).expect("usize always fits in i128")
        {
            let target = ComparativeContainment::with_weights(
                0,
                Vec::new(),
                Vec::new(),
                Vec::<i32>::new(),
                Vec::<i32>::new(),
            );
            // The all-ones configuration is always a vertex cover with size
            // n <= K.
            let witness = vec![1; num_vertices];
            return ReductionDecisionMVCToComparativeContainment {
                target,
                num_source_vertices: num_vertices,
                trivial_yes: Some(witness),
            };
        }

        let k = self.k();
        let edges = self.inner().graph().edges();
        let n = num_vertices;

        // R sets: R_v = V \ {v} for each vertex v, weight 1.
        let r_sets: Vec<Vec<usize>> = (0..n).map(|v| complement_singleton(n, v)).collect();
        let r_weights: Vec<i32> = vec![1; n];

        // S sets: one per edge plus a single budget set.
        let mut s_sets: Vec<Vec<usize>> = Vec::with_capacity(edges.len() + 1);
        let mut s_weights: Vec<i32> = Vec::with_capacity(edges.len() + 1);

        let edge_weight =
            i32::try_from(n + 1).expect("Plaisted edge-penalty weight (n + 1) must fit in i32");
        for &(u, v) in &edges {
            s_sets.push(complement_pair(n, u, v));
            s_weights.push(edge_weight);
        }
        // Budget set S_0 = V with weight n - K. Since 0 <= K < n here, this is
        // a positive integer.
        let budget_weight =
            i32::try_from(n - k).expect("Plaisted budget weight (n - K) must fit in i32");
        s_sets.push((0..n).collect());
        s_weights.push(budget_weight);

        let target = ComparativeContainment::with_weights(n, r_sets, s_sets, r_weights, s_weights);

        ReductionDecisionMVCToComparativeContainment {
            target,
            num_source_vertices: n,
            trivial_yes: None,
        }
    }
}

fn complement_singleton(n: usize, v: usize) -> Vec<usize> {
    (0..n).filter(|&x| x != v).collect()
}

fn complement_pair(n: usize, u: usize, v: usize) -> Vec<usize> {
    (0..n).filter(|&x| x != u && x != v).collect()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumvertexcover_to_comparativecontainment",
        build: || {
            // Path P_4: 0-1-2-3, bound K=2. Minimum cover {1,2} has size 2.
            let inner = MinimumVertexCover::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1i32; 4],
            );
            let source = Decision::new(inner, 2);
            crate::example_db::specs::rule_example_with_witness::<_, ComparativeContainment<i32>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 1, 0],
                    target_config: vec![0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_comparativecontainment.rs"]
mod tests;
