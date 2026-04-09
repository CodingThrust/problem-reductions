//! Minimum Fault Detection Test Set problem implementation.
//!
//! Given a directed acyclic graph with designated input and output vertices,
//! find the minimum set of input-output pairs whose coverage sets cover all
//! internal vertices.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashSet, VecDeque};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFaultDetectionTestSet",
        display_name: "Minimum Fault Detection Test Set",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum set of input-output paths covering all internal DAG vertices",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices in the DAG" },
            FieldInfo { name: "arcs", type_name: "Vec<(usize, usize)>", description: "Directed arcs (u, v)" },
            FieldInfo { name: "inputs", type_name: "Vec<usize>", description: "Input vertex indices" },
            FieldInfo { name: "outputs", type_name: "Vec<usize>", description: "Output vertex indices" },
        ],
    }
}

/// The Minimum Fault Detection Test Set problem.
///
/// Given a directed acyclic graph G = (V, A) with designated input vertices
/// I ⊆ V and output vertices O ⊆ V, find the minimum number of input-output
/// pairs (i, o) ∈ I × O such that the union of their coverage sets covers
/// every internal vertex V \ (I ∪ O).
///
/// For a pair (i, o), the coverage set is the set of vertices reachable from i
/// that can also reach o (i.e., vertices on some i-to-o path). Inputs and
/// outputs themselves are not required to be covered.
///
/// The configuration space is binary over all |I| × |O| pairs.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumFaultDetectionTestSet;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = MinimumFaultDetectionTestSet::new(
///     7,
///     vec![(0,2),(0,3),(1,3),(1,4),(2,5),(3,5),(3,6),(4,6)],
///     vec![0, 1],
///     vec![5, 6],
/// );
/// let solver = BruteForce::new();
/// use problemreductions::solvers::Solver as _;
/// let optimal = solver.solve(&problem);
/// assert_eq!(optimal, problemreductions::types::Min(Some(2)));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MinimumFaultDetectionTestSet {
    /// Number of vertices.
    num_vertices: usize,
    /// Directed arcs (u, v).
    arcs: Vec<(usize, usize)>,
    /// Input vertex indices.
    inputs: Vec<usize>,
    /// Output vertex indices.
    outputs: Vec<usize>,
    /// Precomputed coverage sets for each (input_idx, output_idx) pair.
    /// Indexed as coverage[i_idx * num_outputs + o_idx].
    #[serde(skip)]
    coverage: Vec<HashSet<usize>>,
}

#[derive(Deserialize)]
struct MinimumFaultDetectionTestSetData {
    num_vertices: usize,
    arcs: Vec<(usize, usize)>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl<'de> Deserialize<'de> for MinimumFaultDetectionTestSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = MinimumFaultDetectionTestSetData::deserialize(deserializer)?;
        let coverage =
            Self::build_coverage(data.num_vertices, &data.arcs, &data.inputs, &data.outputs);
        Ok(Self {
            num_vertices: data.num_vertices,
            arcs: data.arcs,
            inputs: data.inputs,
            outputs: data.outputs,
            coverage,
        })
    }
}

impl MinimumFaultDetectionTestSet {
    /// Create a new Minimum Fault Detection Test Set instance.
    ///
    /// # Panics
    ///
    /// Panics if any arc index is out of bounds, if any input or output index
    /// is out of bounds, or if inputs or outputs are empty.
    pub fn new(
        num_vertices: usize,
        arcs: Vec<(usize, usize)>,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
    ) -> Self {
        assert!(!inputs.is_empty(), "Inputs must not be empty");
        assert!(!outputs.is_empty(), "Outputs must not be empty");
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
        for &inp in &inputs {
            assert!(
                inp < num_vertices,
                "Input vertex {} out of bounds for {} vertices",
                inp,
                num_vertices
            );
        }
        for &out in &outputs {
            assert!(
                out < num_vertices,
                "Output vertex {} out of bounds for {} vertices",
                out,
                num_vertices
            );
        }
        let coverage = Self::build_coverage(num_vertices, &arcs, &inputs, &outputs);
        Self {
            num_vertices,
            arcs,
            inputs,
            outputs,
            coverage,
        }
    }

    /// Compute forward reachability from a given vertex using BFS on the DAG.
    fn forward_reachable(num_vertices: usize, adj: &[Vec<usize>], start: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            if v < adj.len() {
                for &w in &adj[v] {
                    if visited.insert(w) {
                        queue.push_back(w);
                    }
                }
            }
        }
        let _ = num_vertices; // used only to clarify signature
        visited
    }

    /// Compute backward reachability from a given vertex using BFS on the reverse DAG.
    fn backward_reachable(
        num_vertices: usize,
        rev_adj: &[Vec<usize>],
        start: usize,
    ) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            if v < rev_adj.len() {
                for &w in &rev_adj[v] {
                    if visited.insert(w) {
                        queue.push_back(w);
                    }
                }
            }
        }
        let _ = num_vertices;
        visited
    }

    /// Build coverage sets for all input-output pairs.
    fn build_coverage(
        num_vertices: usize,
        arcs: &[(usize, usize)],
        inputs: &[usize],
        outputs: &[usize],
    ) -> Vec<HashSet<usize>> {
        // Build adjacency lists
        let mut adj = vec![vec![]; num_vertices];
        let mut rev_adj = vec![vec![]; num_vertices];
        for &(u, v) in arcs {
            adj[u].push(v);
            rev_adj[v].push(u);
        }

        // Precompute forward reachability from each input
        let fwd: Vec<HashSet<usize>> = inputs
            .iter()
            .map(|&inp| Self::forward_reachable(num_vertices, &adj, inp))
            .collect();

        // Precompute backward reachability from each output
        let bwd: Vec<HashSet<usize>> = outputs
            .iter()
            .map(|&out| Self::backward_reachable(num_vertices, &rev_adj, out))
            .collect();

        let num_outputs = outputs.len();
        let mut coverage = Vec::with_capacity(inputs.len() * num_outputs);
        for (i_idx, _) in inputs.iter().enumerate() {
            for (o_idx, _) in outputs.iter().enumerate() {
                // Coverage = vertices reachable from input i AND reachable backwards from output o
                let cov: HashSet<usize> = fwd[i_idx].intersection(&bwd[o_idx]).copied().collect();
                coverage.push(cov);
            }
        }
        coverage
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

    /// Get the input vertices.
    pub fn inputs(&self) -> &[usize] {
        &self.inputs
    }

    /// Get the output vertices.
    pub fn outputs(&self) -> &[usize] {
        &self.outputs
    }

    /// Get the number of input vertices.
    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Get the number of output vertices.
    pub fn num_outputs(&self) -> usize {
        self.outputs.len()
    }
}

impl Problem for MinimumFaultDetectionTestSet {
    const NAME: &'static str = "MinimumFaultDetectionTestSet";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.inputs.len() * self.outputs.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let num_pairs = self.inputs.len() * self.outputs.len();
        if config.len() != num_pairs {
            return Min(None);
        }
        if config.iter().any(|&c| c > 1) {
            return Min(None);
        }

        let mut boundary = vec![false; self.num_vertices];
        for &input in &self.inputs {
            boundary[input] = true;
        }
        for &output in &self.outputs {
            boundary[output] = true;
        }
        let required_internal_vertices =
            boundary.iter().filter(|&&is_boundary| !is_boundary).count();

        // Collect union of internal vertices covered by the selected pairs.
        let mut covered: HashSet<usize> = HashSet::new();
        let mut count = 0usize;
        for (idx, &sel) in config.iter().enumerate() {
            if sel == 1 {
                count += 1;
                covered.extend(
                    self.coverage[idx]
                        .iter()
                        .copied()
                        .filter(|&vertex| !boundary[vertex]),
                );
            }
        }

        // Check all internal vertices are covered.
        if covered.len() == required_internal_vertices {
            Min(Some(count))
        } else {
            Min(None)
        }
    }
}

crate::declare_variants! {
    default MinimumFaultDetectionTestSet => "2^(num_inputs * num_outputs)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 7 vertices, inputs={0,1}, outputs={5,6}
    // Internal vertices are {2,3,4}
    // Arcs: (0,2),(0,3),(1,3),(1,4),(2,5),(3,5),(3,6),(4,6)
    // Pairs: (0,5)->{0,2,3,5}, (0,6)->{0,3,6}, (1,5)->{1,3,5}, (1,6)->{1,3,4,6}
    // Config [1,0,0,1]: select pairs (0,5) and (1,6) -> covers all internal vertices -> Min(2)
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_fault_detection_test_set",
        instance: Box::new(MinimumFaultDetectionTestSet::new(
            7,
            vec![
                (0, 2),
                (0, 3),
                (1, 3),
                (1, 4),
                (2, 5),
                (3, 5),
                (3, 6),
                (4, 6),
            ],
            vec![0, 1],
            vec![5, 6],
        )),
        optimal_config: vec![1, 0, 0, 1],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_fault_detection_test_set.rs"]
mod tests;
