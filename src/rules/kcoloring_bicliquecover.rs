//! Reduction from KColoring to BicliqueCover via a guard-gadget construction.
//!
//! Self-contained gadget: given a KColoring instance `(G, q)` with `n = |V|`
//! and `m = |E|`, build a bipartite graph `H = (L, R, F)` with `2n` left
//! vertices and `2n` right vertices, and ask for a biclique cover of `H`
//! using `n + q` sub-bicliques. The construction is designed so that
//! exactly `n` of the bicliques are forced to cover guard-anchor edges,
//! leaving at most `q` bicliques to cover the `n` diagonal edges
//! `(a_v, b_v)`. These remaining bicliques behave as color classes: two
//! source vertices may share one only when they are nonadjacent in `G`.
//!
//! See issue #1058 for the full proof sketch.
//!
//! ## Vertex layout
//!
//! For `v in 0..n`, the gadget produces four target vertices:
//!
//! - Left partition (size `2n`):
//!   - `a_v` at local index `v`
//!   - `g_v` at local index `n + v`
//! - Right partition (size `2n`):
//!   - `b_v` at local index `v`
//!   - `h_v` at local index `n + v`
//!
//! In unified vertex space (used by `BicliqueCover::dims()`):
//!
//! - `a_v` -> `v`
//! - `g_v` -> `n + v`
//! - `b_v` -> `2n + v` (i.e. `left_size + v`)
//! - `h_v` -> `3n + v` (i.e. `left_size + n + v`)

use crate::models::graph::{BicliqueCover, KColoring};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{BipartiteGraph, Graph, SimpleGraph};
use crate::variant::KN;
use std::collections::BTreeSet;

/// Result of reducing KColoring to BicliqueCover.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToBicliqueCover {
    target: BicliqueCover,
    /// Number of source vertices `n`. Stored so `extract_solution` can locate
    /// the diagonal indices of each source vertex without re-reading the
    /// reduction parameters.
    num_vertices: usize,
    /// Number of source colors `q`. Used as the upper bound on the number of
    /// color bicliques recovered during extraction.
    num_colors: usize,
}

impl ReductionResult for ReductionKColoringToBicliqueCover {
    type Source = KColoring<KN, SimpleGraph>;
    type Target = BicliqueCover;

    fn target_problem(&self) -> &BicliqueCover {
        &self.target
    }

    /// Recover a source coloring from a BicliqueCover witness.
    ///
    /// For each source vertex `v`, find any biclique `r` that contains both
    /// the left vertex `a_v` and the right vertex `b_v`. The diagonal edge
    /// `(a_v, b_v)` must lie in some biclique of any valid cover. Compact
    /// the distinct diagonal-covering biclique indices into colors
    /// `0..q-1` in first-seen order; vertices whose biclique is one of
    /// these get the compacted color. By the correctness proof, a valid
    /// cover yields at most `q` such distinct bicliques, so the result is a
    /// proper `q`-coloring of the source.
    ///
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_vertices;
            let k = self.target.k();
            let left_size = 2 * n;

            // For each source vertex v, find the first biclique r that contains
            // both a_v (unified index v) and b_v (unified index left_size + v).
            let mut diagonal_biclique = Vec::with_capacity(n);
            for v in 0..n {
                let a_v = v;
                let b_v = left_size + v;
                let biclique = (0..k)
                    .find(|&r| target_solution[r][a_v] && target_solution[r][b_v])
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(format!(
                            "target cover leaves diagonal gadget edge {v} uncovered"
                        ))
                    })?;
                diagonal_biclique.push(biclique);
            }

            // Compact distinct biclique indices into colors 0..q-1 in first-seen order.
            let mut color_of_biclique: std::collections::HashMap<usize, usize> =
                std::collections::HashMap::new();
            let mut coloring = Vec::with_capacity(n);
            for biclique in diagonal_biclique {
                let next_color = color_of_biclique.len();
                let color = *color_of_biclique.entry(biclique).or_insert(next_color);
                if color >= self.num_colors {
                    return Err(crate::rules::ExtractionError::invalid(format!(
                        "target cover uses more than {} diagonal bicliques",
                        self.num_colors
                    )));
                }
                coloring.push(color);
            }
            coloring
        })
    }
}

#[reduction(
    size = exact {
        num_vertices = "4 * num_vertices",
        num_edges = "2 * num_vertices * (num_vertices - 1) - 4 * num_edges + 3 * num_vertices",
    },
    unavailable = {
        rank = "the target rank depends on the number of colors, which is a problem parameter rather than a source size parameter",
    }
)]
impl ReduceTo<BicliqueCover> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToBicliqueCover;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let q = self.num_colors();

        // Build the set of source edges as an undirected lookup so the
        // construction can skip endpoints {u,v} in E in O(1).
        let mut source_edges: BTreeSet<(usize, usize)> = BTreeSet::new();
        for (u, v) in self.graph().edges() {
            let (a, b) = if u <= v { (u, v) } else { (v, u) };
            source_edges.insert((a, b));
        }
        let has_source_edge = |u: usize, v: usize| -> bool {
            if u == v {
                return false;
            }
            let (a, b) = if u <= v { (u, v) } else { (v, u) };
            source_edges.contains(&(a, b))
        };

        // Target vertex layout (bipartite-local indices):
        //   left:  a_v at v,         g_v at n + v
        //   right: b_v at v,         h_v at n + v
        let a_left = |v: usize| -> usize { v };
        let g_left = |v: usize| -> usize { n + v };
        let b_right = |v: usize| -> usize { v };
        let h_right = |v: usize| -> usize { n + v };

        let mut edges: Vec<(usize, usize)> = Vec::new();

        // 1. Diagonal edges (a_v, b_v).
        for v in 0..n {
            edges.push((a_left(v), b_right(v)));
        }

        // 2. Compatibility edges (a_u, b_v) for ordered u != v with {u,v} not in E.
        for u in 0..n {
            for v in 0..n {
                if u == v {
                    continue;
                }
                if !has_source_edge(u, v) {
                    edges.push((a_left(u), b_right(v)));
                }
            }
        }

        // 3. Guard-anchor edges (a_v, h_v) and (g_v, h_v).
        for v in 0..n {
            edges.push((a_left(v), h_right(v)));
            edges.push((g_left(v), h_right(v)));
        }

        // 4. Guard compatibility edges (g_v, b_w) for v != w with {v,w} not in E.
        for v in 0..n {
            for w in 0..n {
                if v == w {
                    continue;
                }
                if !has_source_edge(v, w) {
                    edges.push((g_left(v), b_right(w)));
                }
            }
        }

        let left_size = 2 * n;
        let right_size = 2 * n;
        let target = BicliqueCover::new(BipartiteGraph::new(left_size, right_size, edges), n + q);

        Ok(ReductionKColoringToBicliqueCover {
            target,
            num_vertices: n,
            num_colors: q,
        })
    }
}

/// Build the canonical forward witness described in the issue.
///
/// For each source vertex `v` (with `v in 0..n`), create one guard biclique
///
/// ```text
/// G_v = ({a_v, g_v}, {h_v} ∪ {b_w : w != v, {v,w} ∉ E})
/// ```
///
/// and for each color class `C ⊆ V`, create one color biclique
///
/// ```text
/// C_color = ({a_v : v in C}, {b_v : v in C}).
/// ```
///
/// Returns one membership row per biclique. Each row has one Boolean entry
/// per target vertex.
///
/// `coloring[v]` must be in `0..q`. The order of color bicliques is the
/// order of first appearance of each color along `0..n`, so unused colors
/// at the tail produce empty bicliques.
#[cfg(any(test, feature = "example-db"))]
pub(crate) fn forward_witness(
    source: &KColoring<KN, SimpleGraph>,
    coloring: &[usize],
) -> Vec<Vec<bool>> {
    let n = source.graph().num_vertices();
    let q = source.num_colors();
    let k = n + q;
    let left_size = 2 * n;
    let num_vertices = 4 * n;
    let mut config = vec![vec![false; num_vertices]; k];

    let set_member = |config: &mut Vec<Vec<bool>>, vertex: usize, biclique: usize| {
        config[biclique][vertex] = true;
    };

    // Edge-membership lookup for source edges (undirected).
    let mut source_edges: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (u, v) in source.graph().edges() {
        let (a, b) = if u <= v { (u, v) } else { (v, u) };
        source_edges.insert((a, b));
    }
    let has_source_edge = |u: usize, v: usize| -> bool {
        if u == v {
            return false;
        }
        let (a, b) = if u <= v { (u, v) } else { (v, u) };
        source_edges.contains(&(a, b))
    };

    // Guard bicliques: biclique index v (for v in 0..n).
    for v in 0..n {
        let biclique = v;
        // Left: a_v (unified index v), g_v (unified index n + v).
        set_member(&mut config, v, biclique);
        set_member(&mut config, n + v, biclique);
        // Right: h_v (unified index left_size + n + v) and b_w for nonadjacent w != v.
        set_member(&mut config, left_size + n + v, biclique); // h_v
        for w in 0..n {
            if w != v && !has_source_edge(v, w) {
                set_member(&mut config, left_size + w, biclique); // b_w
            }
        }
    }

    // Color bicliques: biclique index n + c for color c (in first-seen order).
    let mut color_to_biclique: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
    for (v, &c) in coloring.iter().enumerate().take(n) {
        let next_slot = color_to_biclique.len();
        let slot = *color_to_biclique.entry(c).or_insert(next_slot);
        let biclique = n + slot;
        // Left: a_v (unified index v).
        set_member(&mut config, v, biclique);
        // Right: b_v (unified index left_size + v).
        set_member(&mut config, left_size + v, biclique);
    }

    config
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_bicliquecover",
        build: || {
            // P_2 with q = 2: vertices {0, 1}, one edge (0, 1).
            // A valid 2-coloring is (0, 1). Target has 8 vertices and rank 4,
            // small enough to keep the canonical bundle compact.
            let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 2);
            let coloring = vec![0usize, 1usize];
            let target_config = forward_witness(&source, &coloring);
            crate::example_db::specs::rule_example_with_witness::<_, BicliqueCover>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(coloring),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_bicliquecover.rs"]
mod tests;
