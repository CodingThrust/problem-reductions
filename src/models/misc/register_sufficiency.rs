//! Register Sufficiency problem implementation.
//!
//! Given a directed acyclic graph G = (V, A) representing a computation and a
//! bound K, determine whether the computation can be performed using at most K
//! registers. NP-complete even for out-degree <= 2 [Sethi, 1975].

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "RegisterSufficiency",
        display_name: "Register Sufficiency",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a DAG computation can be performed using K or fewer registers",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices n = |V|" },
            FieldInfo { name: "arcs", type_name: "Vec<(usize, usize)>", description: "Directed arcs (v, u) meaning v depends on u" },
            FieldInfo { name: "bound", type_name: "usize", description: "Register bound K" },
        ],
    }
}

/// The Register Sufficiency problem.
///
/// Given a directed acyclic graph G = (V, A) where arcs represent data
/// dependencies, and a positive integer K, determine whether there is an
/// evaluation ordering of all vertices such that at most K registers are
/// needed at any point during the computation.
///
/// # Representation
///
/// An arc `(v, u)` means vertex `v` depends on vertex `u` (i.e., `u` must be
/// in a register when `v` is evaluated). Each variable represents a vertex,
/// with domain `{0, ..., n-1}` giving its evaluation position (the config
/// must be a valid permutation).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::RegisterSufficiency;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 vertices: v2 depends on v0, v3 depends on v0 and v1
/// let problem = RegisterSufficiency::new(
///     4,
///     vec![(2, 0), (3, 0), (3, 1)],
///     2,
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterSufficiency {
    /// Number of vertices.
    num_vertices: usize,
    /// Directed arcs (v, u) meaning v depends on u.
    arcs: Vec<(usize, usize)>,
    /// Register bound K.
    bound: usize,
}

impl RegisterSufficiency {
    /// Create a new Register Sufficiency instance.
    ///
    /// # Panics
    ///
    /// Panics if any arc index is out of bounds (>= num_vertices),
    /// or if any arc is a self-loop.
    pub fn new(num_vertices: usize, arcs: Vec<(usize, usize)>, bound: usize) -> Self {
        for &(v, u) in &arcs {
            assert!(
                v < num_vertices && u < num_vertices,
                "Arc ({}, {}) out of bounds for {} vertices",
                v,
                u,
                num_vertices
            );
            assert!(v != u, "Self-loop ({}, {}) not allowed in a DAG", v, u);
        }
        Self {
            num_vertices,
            arcs,
            bound,
        }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.arcs.len()
    }

    /// Get the register bound K.
    pub fn bound(&self) -> usize {
        self.bound
    }

    /// Get the arcs.
    pub fn arcs(&self) -> &[(usize, usize)] {
        &self.arcs
    }

    /// Simulate register usage for a given evaluation ordering and return the
    /// maximum number of registers used, or `None` if the ordering is invalid
    /// (not a permutation or violates dependencies).
    pub fn simulate_registers(&self, config: &[usize]) -> Option<usize> {
        let n = self.num_vertices;
        if config.len() != n {
            return None;
        }

        // Check valid permutation: each position 0..n-1 used exactly once
        let mut order = vec![0usize; n]; // order[position] = vertex
        let mut used = vec![false; n];
        for (vertex, &position) in config.iter().enumerate() {
            if position >= n {
                return None;
            }
            if used[position] {
                return None;
            }
            used[position] = true;
            order[position] = vertex;
        }

        // Build dependency info:
        // dependents[u] = list of vertices that depend on u (i.e., arcs (v, u))
        // dependencies[v] = list of vertices that v depends on (i.e., arcs (v, u))
        let mut dependencies: Vec<Vec<usize>> = vec![vec![]; n];
        let mut dependents: Vec<Vec<usize>> = vec![vec![]; n];
        for &(v, u) in &self.arcs {
            dependencies[v].push(u);
            dependents[u].push(v);
        }

        // For each vertex u, compute the latest position among its dependents.
        // A vertex u must stay in registers until all its dependents have been evaluated.
        let mut last_use = vec![0usize; n];
        for u in 0..n {
            if dependents[u].is_empty() {
                // Vertex u has no dependents. It stays in registers from its
                // evaluation step until the end (final outputs must be in S_n).
                last_use[u] = n; // stays until the end
            } else {
                let mut latest = 0;
                for &v in &dependents[u] {
                    latest = latest.max(config[v]);
                }
                last_use[u] = latest;
            }
        }

        let mut max_registers = 0;

        // Simulate: process vertices in evaluation order
        for step in 0..n {
            let vertex = order[step];

            // Check dependencies: all dependencies of this vertex must have
            // been evaluated before this step
            for &dep in &dependencies[vertex] {
                if config[dep] >= step {
                    // Dependency not yet evaluated
                    return None;
                }
            }

            // Count registers at this step:
            // A vertex v is in registers if:
            // - v has been evaluated (config[v] <= step)
            // - v is still needed (last_use[v] > step, or v is the current vertex)
            // Actually, more precisely: after evaluating vertex at position `step`,
            // the register set contains all vertices evaluated so far whose last
            // use is > step (they're still needed later), plus the current vertex.
            let reg_count = order[..=step]
                .iter()
                .filter(|&&v| last_use[v] > step)
                .count();

            max_registers = max_registers.max(reg_count);
        }

        Some(max_registers)
    }
}

impl Problem for RegisterSufficiency {
    const NAME: &'static str = "RegisterSufficiency";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_vertices; self.num_vertices]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(
            self.simulate_registers(config)
                .is_some_and(|max_reg| max_reg <= self.bound),
        )
    }
}

crate::declare_variants! {
    default RegisterSufficiency => "num_vertices ^ 2 * 2 ^ num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "register_sufficiency",
        // Issue #515 example: 7 vertices, 8 arcs, K=3
        // Arcs (0-indexed): (2,0), (2,1), (3,1), (4,2), (4,3), (5,0), (6,4), (6,5)
        // Order: v0,v1,v2,v3,v5,v4,v6 -> positions [0,1,2,3,5,4,6]
        instance: Box::new(RegisterSufficiency::new(
            7,
            vec![
                (2, 0),
                (2, 1),
                (3, 1),
                (4, 2),
                (4, 3),
                (5, 0),
                (6, 4),
                (6, 5),
            ],
            3,
        )),
        // Order: v1,v2,v3,v4,v6,v5,v7 (1-indexed) = v0,v1,v2,v3,v5,v4,v6 (0-indexed)
        // Positions: v0->0, v1->1, v2->2, v3->3, v4->5, v5->4, v6->6
        optimal_config: vec![0, 1, 2, 3, 5, 4, 6],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/register_sufficiency.rs"]
mod tests;
