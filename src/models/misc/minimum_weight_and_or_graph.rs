//! Minimum Weight AND/OR Graph problem implementation.
//!
//! Given a directed acyclic graph with AND/OR gates, find the minimum-weight
//! solution subgraph from a designated source vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumWeightAndOrGraph",
        display_name: "Minimum Weight AND/OR Graph",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find the minimum-weight solution subgraph from a source in a DAG with AND/OR gates",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices in the DAG" },
            FieldInfo { name: "arcs", type_name: "Vec<(usize, usize)>", description: "Directed arcs (u, v)" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex index" },
            FieldInfo { name: "gate_types", type_name: "Vec<Option<bool>>", description: "Gate type per vertex: Some(true)=AND, Some(false)=OR, None=leaf" },
            FieldInfo { name: "arc_weights", type_name: "Vec<i32>", description: "Weight of each arc" },
        ],
    }
}

/// The Minimum Weight AND/OR Graph problem.
///
/// Given a directed acyclic graph G = (V, A) where each non-leaf vertex is
/// either an AND gate or an OR gate, a source vertex s, and arc weights
/// w: A -> Z, find a solution subgraph of minimum total arc weight.
///
/// A solution subgraph is a subset of arcs S such that:
/// - The source vertex is "solved"
/// - For each solved AND-gate vertex v: all outgoing arcs from v are in S
/// - For each solved OR-gate vertex v: at least one outgoing arc from v is in S
/// - For each arc (u,v) in S: the target vertex v is also solved (recursively)
/// - Leaf vertices are trivially solved (no outgoing arcs needed)
///
/// The configuration space is binary over arcs: each arc is either selected (1)
/// or not (0).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumWeightAndOrGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 7 vertices: AND at 0, OR at 1 and 2, leaves 3-6
/// let problem = MinimumWeightAndOrGraph::new(
///     7,
///     vec![(0,1), (0,2), (1,3), (1,4), (2,5), (2,6)],
///     0,
///     vec![Some(true), Some(false), Some(false), None, None, None, None],
///     vec![1, 2, 3, 1, 4, 2],
/// );
/// let solver = BruteForce::new();
/// use problemreductions::solvers::Solver as _;
/// let optimal = solver.solve(&problem);
/// assert_eq!(optimal, problemreductions::types::Min(Some(6)));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MinimumWeightAndOrGraph {
    /// Number of vertices.
    num_vertices: usize,
    /// Directed arcs (u, v).
    arcs: Vec<(usize, usize)>,
    /// Source vertex index.
    source: usize,
    /// Gate type per vertex: Some(true)=AND, Some(false)=OR, None=leaf.
    gate_types: Vec<Option<bool>>,
    /// Weight of each arc.
    arc_weights: Vec<i32>,
    /// Precomputed: outgoing arcs for each vertex (arc indices).
    #[serde(skip)]
    outgoing: Vec<Vec<usize>>,
}

#[derive(Deserialize)]
struct MinimumWeightAndOrGraphData {
    num_vertices: usize,
    arcs: Vec<(usize, usize)>,
    source: usize,
    gate_types: Vec<Option<bool>>,
    arc_weights: Vec<i32>,
}

impl<'de> Deserialize<'de> for MinimumWeightAndOrGraph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = MinimumWeightAndOrGraphData::deserialize(deserializer)?;
        let outgoing = Self::build_outgoing(data.num_vertices, &data.arcs);
        Ok(Self {
            num_vertices: data.num_vertices,
            arcs: data.arcs,
            source: data.source,
            gate_types: data.gate_types,
            arc_weights: data.arc_weights,
            outgoing,
        })
    }
}

impl MinimumWeightAndOrGraph {
    /// Create a new Minimum Weight AND/OR Graph instance.
    ///
    /// # Panics
    ///
    /// Panics if any arc index is out of bounds, if the source is out of bounds,
    /// if gate_types length does not match num_vertices, if arc_weights length
    /// does not match the number of arcs, or if the source is a leaf.
    pub fn new(
        num_vertices: usize,
        arcs: Vec<(usize, usize)>,
        source: usize,
        gate_types: Vec<Option<bool>>,
        arc_weights: Vec<i32>,
    ) -> Self {
        assert!(
            source < num_vertices,
            "Source vertex {} out of bounds for {} vertices",
            source,
            num_vertices
        );
        assert_eq!(
            gate_types.len(),
            num_vertices,
            "gate_types length {} does not match num_vertices {}",
            gate_types.len(),
            num_vertices
        );
        assert_eq!(
            arc_weights.len(),
            arcs.len(),
            "arc_weights length {} does not match number of arcs {}",
            arc_weights.len(),
            arcs.len()
        );
        for (i, &(u, v)) in arcs.iter().enumerate() {
            assert!(
                u < num_vertices && v < num_vertices,
                "Arc {} ({}, {}) out of bounds for {} vertices",
                i,
                u,
                v,
                num_vertices
            );
        }
        assert!(
            gate_types[source].is_some(),
            "Source vertex must be an AND or OR gate, not a leaf"
        );
        let outgoing = Self::build_outgoing(num_vertices, &arcs);
        Self {
            num_vertices,
            arcs,
            source,
            gate_types,
            arc_weights,
            outgoing,
        }
    }

    /// Build outgoing arc index lists for each vertex.
    fn build_outgoing(num_vertices: usize, arcs: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut outgoing = vec![vec![]; num_vertices];
        for (i, &(u, _v)) in arcs.iter().enumerate() {
            outgoing[u].push(i);
        }
        outgoing
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.arcs.len()
    }

    /// Get the arcs.
    pub fn arcs(&self) -> &[(usize, usize)] {
        &self.arcs
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the gate types.
    pub fn gate_types(&self) -> &[Option<bool>] {
        &self.gate_types
    }

    /// Get the arc weights.
    pub fn arc_weights(&self) -> &[i32] {
        &self.arc_weights
    }
}

impl Problem for MinimumWeightAndOrGraph {
    const NAME: &'static str = "MinimumWeightAndOrGraph";
    type Value = Min<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.arcs.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i32> {
        if config.len() != self.arcs.len() {
            return Min(None);
        }

        // Check all config values are 0 or 1
        if config.iter().any(|&c| c > 1) {
            return Min(None);
        }

        // Determine which arcs are selected
        let selected: Vec<bool> = config.iter().map(|&c| c == 1).collect();

        // Propagate "solved" status top-down from source
        let mut solved = vec![false; self.num_vertices];
        let mut stack = vec![self.source];
        solved[self.source] = true;

        while let Some(v) = stack.pop() {
            match self.gate_types[v] {
                None => {
                    // Leaf vertex: trivially solved, no outgoing arcs needed
                }
                Some(is_and) => {
                    let out_arcs = &self.outgoing[v];
                    let selected_out: Vec<usize> = out_arcs
                        .iter()
                        .copied()
                        .filter(|&ai| selected[ai])
                        .collect();

                    if is_and {
                        // AND gate: all outgoing arcs must be selected
                        if selected_out.len() != out_arcs.len() {
                            return Min(None);
                        }
                    } else {
                        // OR gate: at least one outgoing arc must be selected
                        if selected_out.is_empty() {
                            return Min(None);
                        }
                    }

                    // Mark children of selected arcs as solved
                    for &ai in &selected_out {
                        let (_u, child) = self.arcs[ai];
                        if !solved[child] {
                            solved[child] = true;
                            stack.push(child);
                        }
                    }
                }
            }
        }

        // Check no selected arcs come from non-solved vertices (no dangling arcs)
        for (ai, &sel) in selected.iter().enumerate() {
            if sel {
                let (u, _v) = self.arcs[ai];
                if !solved[u] {
                    return Min(None);
                }
            }
        }

        // Compute total weight of selected arcs
        let total_weight: i32 = selected
            .iter()
            .enumerate()
            .filter(|(_, &sel)| sel)
            .map(|(i, _)| self.arc_weights[i])
            .sum();

        Min(Some(total_weight))
    }
}

crate::declare_variants! {
    default MinimumWeightAndOrGraph => "2^num_arcs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 7 vertices: source=0 (AND), v1 (OR), v2 (OR), v3-v6 (leaves)
    // Arcs: (0,1,1), (0,2,2), (1,3,3), (1,4,1), (2,5,4), (2,6,2)
    // Optimal: AND at 0 requires both arcs to 1 and 2 (cost 1+2=3).
    // OR at 1: pick arc to 4 (cost 1). OR at 2: pick arc to 6 (cost 2).
    // Total = 1+2+1+2 = 6... but actually we should check: is there a cheaper?
    // arc0(0->1,w=1), arc1(0->2,w=2), arc3(1->4,w=1), arc5(2->6,w=2) => 1+2+1+2=6
    // arc0(0->1,w=1), arc1(0->2,w=2), arc2(1->3,w=3), arc5(2->6,w=2) => 1+2+3+2=8
    // arc0(0->1,w=1), arc1(0->2,w=2), arc3(1->4,w=1), arc4(2->5,w=4) => 1+2+1+4=8
    // So optimal is config [1,1,0,1,0,1] with value 6... but wait, let me also check
    // if val=5 is achievable: 1+2+1+1=5 impossible because OR at 2 must pick at least one.
    // Actually optimal = 1(arc0) + 2(arc1) + 1(arc3) + 2(arc5) = 6
    // Hmm, let me reconsider: is there a solution with value 5?
    // Source is AND, so both arcs 0 and 1 must be selected (cost 1+2=3).
    // Then OR at 1: cheapest outgoing arc is arc3 (w=1), OR at 2: cheapest is arc5 (w=2).
    // Total = 3+1+2 = 6. Can't do better since source AND forces both.
    // Wait — check: what if we change arc weights. The issue says value 5 might be optimal.
    // Let me re-read: issue example says Config [1,1,0,1,0,1] -> weight 1+2+1+2 = 6 -> Min(6).
    // So 6 is the correct optimal. But let me verify: is there any config with value < 6?
    // No — source is AND so arcs 0,1 are forced (cost 3), then OR nodes each need at least one.
    // Min at OR-1 is 1 (arc3), min at OR-2 is 2 (arc5). Total = 3+1+2 = 6.
    // Optimal config: [1,1,0,1,0,1]
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_weight_and_or_graph",
        instance: Box::new(MinimumWeightAndOrGraph::new(
            7,
            vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)],
            0,
            vec![Some(true), Some(false), Some(false), None, None, None, None],
            vec![1, 2, 3, 1, 4, 2],
        )),
        optimal_config: vec![1, 1, 0, 1, 0, 1],
        optimal_value: serde_json::json!(6),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_weight_and_or_graph.rs"]
mod tests;
