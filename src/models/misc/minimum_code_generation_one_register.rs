//! Minimum Code Generation on a One-Register Machine.
//!
//! Given a directed acyclic graph G = (V, A) with maximum out-degree 2
//! (an expression DAG), find a program of minimum number of instructions
//! for a one-register machine (LOAD, STORE, OP) that computes all root
//! vertices. NP-complete [Bruno and Sethi, 1976].

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCodeGenerationOneRegister",
        display_name: "Minimum Code Generation (One Register)",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum-length instruction sequence for a one-register machine to evaluate an expression DAG",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices n = |V|" },
            FieldInfo { name: "edges", type_name: "Vec<(usize, usize)>", description: "Directed arcs (parent, child) in the expression DAG" },
            FieldInfo { name: "num_leaves", type_name: "usize", description: "Number of leaf vertices (out-degree 0)" },
        ],
    }
}

/// Minimum Code Generation on a One-Register Machine.
///
/// Given a directed acyclic graph G = (V, A) with maximum out-degree 2,
/// where leaves (out-degree 0) are input values in memory, internal vertices
/// are operations, and roots (in-degree 0) are values to compute, find a
/// program of minimum instructions using LOAD, STORE, and OP.
///
/// # Representation
///
/// The configuration is a permutation of internal (non-leaf) vertices
/// giving their evaluation order. `config[i]` is the evaluation position
/// for internal vertex `i` (0-indexed among internal vertices).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumCodeGenerationOneRegister;
/// use problemreductions::{Problem, Solver, BruteForce, Min};
///
/// // 7 vertices: leaves {4,5,6}, internal {0,1,2,3}
/// // v3 = op(v5, v6), v1 = op(v3, v4), v2 = op(v3, v5), v0 = op(v1, v2)
/// let problem = MinimumCodeGenerationOneRegister::new(
///     7,
///     vec![(0,1),(0,2),(1,3),(1,4),(2,3),(2,5),(3,5),(3,6)],
///     3,
/// );
/// let result = BruteForce::new().solve(&problem);
/// assert_eq!(result, Min(Some(8)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCodeGenerationOneRegister {
    /// Number of vertices |V|.
    num_vertices: usize,
    /// Directed arcs (parent, child) in the expression DAG.
    edges: Vec<(usize, usize)>,
    /// Number of leaf vertices (out-degree 0).
    num_leaves: usize,
}

impl MinimumCodeGenerationOneRegister {
    /// Create a new instance.
    ///
    /// # Arguments
    ///
    /// * `num_vertices` - Total number of vertices
    /// * `edges` - Directed arcs (parent, child); parent depends on child
    /// * `num_leaves` - Number of leaf vertices (out-degree 0)
    ///
    /// # Panics
    ///
    /// Panics if any edge index is out of bounds, if any vertex has
    /// out-degree > 2, or if `num_leaves > num_vertices`.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>, num_leaves: usize) -> Self {
        assert!(
            num_leaves <= num_vertices,
            "num_leaves ({num_leaves}) exceeds num_vertices ({num_vertices})"
        );
        let mut out_degree = vec![0usize; num_vertices];
        for &(parent, child) in &edges {
            assert!(
                parent < num_vertices && child < num_vertices,
                "Edge ({parent}, {child}) out of bounds for {num_vertices} vertices"
            );
            assert!(
                parent != child,
                "Self-loop ({parent}, {parent}) not allowed"
            );
            out_degree[parent] += 1;
        }
        for (v, &deg) in out_degree.iter().enumerate() {
            assert!(deg <= 2, "Vertex {v} has out-degree {deg} > 2");
        }
        // Verify leaf count: leaves are vertices with out-degree 0
        let actual_leaves = out_degree.iter().filter(|&&d| d == 0).count();
        assert_eq!(
            actual_leaves, num_leaves,
            "Declared num_leaves ({num_leaves}) != actual leaf count ({actual_leaves})"
        );
        Self {
            num_vertices,
            edges,
            num_leaves,
        }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Get the number of leaf vertices.
    pub fn num_leaves(&self) -> usize {
        self.num_leaves
    }

    /// Get the number of internal (non-leaf) vertices.
    pub fn num_internal(&self) -> usize {
        self.num_vertices - self.num_leaves
    }

    /// Get the edges.
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// Compute the children (operands) of each vertex from the edge list.
    fn children(&self) -> Vec<Vec<usize>> {
        let mut ch = vec![vec![]; self.num_vertices];
        for &(parent, child) in &self.edges {
            ch[parent].push(child);
        }
        ch
    }

    /// Determine which vertices are internal (non-leaf, i.e. out-degree > 0).
    fn internal_vertices(&self) -> Vec<usize> {
        let children = self.children();
        (0..self.num_vertices)
            .filter(|&v| !children[v].is_empty())
            .collect()
    }

    /// Determine which vertices are leaves (out-degree 0).
    fn leaf_set(&self) -> Vec<bool> {
        let children = self.children();
        (0..self.num_vertices)
            .map(|v| children[v].is_empty())
            .collect()
    }

    /// Simulate the one-register machine for a given evaluation order of
    /// internal vertices and return the instruction count, or `None` if the
    /// ordering is invalid (not a permutation or violates dependencies).
    pub fn simulate(&self, config: &[usize]) -> Option<usize> {
        let internal = self.internal_vertices();
        let n_internal = internal.len();
        if config.len() != n_internal {
            return None;
        }

        // config[i] = evaluation position for internal vertex index i
        // (i indexes into the `internal` array)
        // Build order: order[pos] = index into `internal`
        let mut order = vec![0usize; n_internal];
        let mut used = vec![false; n_internal];
        for (i, &pos) in config.iter().enumerate() {
            if pos >= n_internal {
                return None;
            }
            if used[pos] {
                return None;
            }
            used[pos] = true;
            order[pos] = i;
        }

        let children = self.children();
        let is_leaf = self.leaf_set();

        // Track which internal vertices have been computed
        let mut computed = vec![false; self.num_vertices];
        // All leaves are "computed" (available in memory)
        for v in 0..self.num_vertices {
            if is_leaf[v] {
                computed[v] = true;
            }
        }

        // Build: for each vertex, which future internal vertices need it?
        // We'll track this dynamically.
        let mut future_uses = vec![0usize; self.num_vertices];
        for &idx in &order {
            let v = internal[idx];
            for &c in &children[v] {
                future_uses[c] += 1;
            }
        }

        let mut register: Option<usize> = None; // which vertex value is in register
        let mut in_memory = vec![false; self.num_vertices];
        // Leaves start in memory
        for v in 0..self.num_vertices {
            if is_leaf[v] {
                in_memory[v] = true;
            }
        }

        let mut instructions = 0usize;

        for step in 0..n_internal {
            let v = internal[order[step]];

            // Check dependencies: all children must be available
            for &c in &children[v] {
                let available = in_memory[c] || register == Some(c);
                if !available {
                    return None; // child was computed but lost (not stored, overwritten)
                }
            }

            // Decrement future uses for children of v
            for &c in &children[v] {
                future_uses[c] -= 1;
            }

            let operands = &children[v];

            // Before computing v, check if we need to STORE the current register value
            // We need to store if:
            // 1. Register holds a value
            // 2. That value is still needed in the future
            // 3. That value is not already in memory
            if let Some(r) = register {
                if !in_memory[r] && future_uses[r] > 0 {
                    instructions += 1; // STORE
                    in_memory[r] = true;
                }
            }

            // Now compute v
            if operands.len() == 2 {
                let c0 = operands[0];
                let c1 = operands[1];
                let one_in_register = (register == Some(c0) && in_memory[c1])
                    || (register == Some(c1) && in_memory[c0]);
                if one_in_register {
                    instructions += 1; // OP v (one operand in register, other in memory)
                } else {
                    // Need to LOAD one operand, OP with the other from memory
                    instructions += 1; // LOAD
                    instructions += 1; // OP
                }
            } else if operands.len() == 1 {
                let c0 = operands[0];
                if register == Some(c0) {
                    instructions += 1; // OP v (unary)
                } else {
                    instructions += 1; // LOAD
                    instructions += 1; // OP
                }
            }

            register = Some(v);
        }

        Some(instructions)
    }
}

impl Problem for MinimumCodeGenerationOneRegister {
    const NAME: &'static str = "MinimumCodeGenerationOneRegister";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n_internal = self.num_internal();
        vec![n_internal; n_internal]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        Min(self.simulate(config))
    }
}

crate::declare_variants! {
    default MinimumCodeGenerationOneRegister => "2 ^ num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_code_generation_one_register",
        // Issue #900 example: 7 vertices, leaves {4,5,6}, internal {0,1,2,3}
        // Edges: (0,1),(0,2),(1,3),(1,4),(2,3),(2,5),(3,5),(3,6)
        // Optimal order: v3, v2, v1, v0 with positions [3, 2, 1, 0]
        // Wait — config[i] = position for internal vertex i.
        // Internal vertices sorted: [0, 1, 2, 3]
        // Optimal evaluation order: v3, v2, v1, v0
        // v3 at position 0, v2 at position 1, v1 at position 2, v0 at position 3
        // So config = [3, 2, 1, 0] (internal idx 0=v0 -> pos 3, idx 1=v1 -> pos 2, ...)
        instance: Box::new(MinimumCodeGenerationOneRegister::new(
            7,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 5),
                (3, 5),
                (3, 6),
            ],
            3,
        )),
        optimal_config: vec![3, 2, 1, 0],
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_code_generation_one_register.rs"]
mod tests;
