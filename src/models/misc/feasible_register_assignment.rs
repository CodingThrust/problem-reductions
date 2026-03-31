//! Feasible Register Assignment problem implementation.
//!
//! Given a directed acyclic graph G = (V, A), K registers, and a fixed
//! register assignment f: V → {0, ..., K-1}, determine whether there
//! exists a topological ordering of the vertices such that no register
//! conflict arises during execution. NP-complete [Bouchez et al., 2006].

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "FeasibleRegisterAssignment",
        display_name: "Feasible Register Assignment",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a DAG computation can be scheduled without register conflicts under a fixed assignment",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices n = |V|" },
            FieldInfo { name: "arcs", type_name: "Vec<(usize, usize)>", description: "Directed arcs (v, u) meaning v depends on u" },
            FieldInfo { name: "num_registers", type_name: "usize", description: "Number of registers K" },
            FieldInfo { name: "assignment", type_name: "Vec<usize>", description: "Register assignment f(v) for each vertex" },
        ],
    }
}

/// The Feasible Register Assignment problem.
///
/// Given a directed acyclic graph G = (V, A) where arcs represent data
/// dependencies, K registers, and an assignment f: V → {0, ..., K-1},
/// determine whether there exists a topological evaluation ordering such
/// that no two simultaneously live values share the same register.
///
/// # Representation
///
/// An arc `(v, u)` means vertex `v` depends on vertex `u` (i.e., `u` must
/// be computed before `v`). Each variable represents a vertex, with domain
/// `{0, ..., n-1}` giving its evaluation position (the config must be a
/// valid permutation).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::FeasibleRegisterAssignment;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 vertices: v0 depends on v1 and v2, v1 depends on v3
/// let problem = FeasibleRegisterAssignment::new(
///     4,
///     vec![(0, 1), (0, 2), (1, 3)],
///     2,
///     vec![0, 1, 0, 0],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibleRegisterAssignment {
    /// Number of vertices.
    num_vertices: usize,
    /// Directed arcs (v, u) meaning v depends on u.
    arcs: Vec<(usize, usize)>,
    /// Number of registers K.
    num_registers: usize,
    /// Register assignment f(v) for each vertex.
    assignment: Vec<usize>,
}

impl FeasibleRegisterAssignment {
    /// Create a new Feasible Register Assignment instance.
    ///
    /// # Panics
    ///
    /// Panics if any arc index is out of bounds (>= num_vertices),
    /// if any arc is a self-loop, if the assignment length does not
    /// match num_vertices, or if any assignment value >= num_registers.
    pub fn new(
        num_vertices: usize,
        arcs: Vec<(usize, usize)>,
        num_registers: usize,
        assignment: Vec<usize>,
    ) -> Self {
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
        assert_eq!(
            assignment.len(),
            num_vertices,
            "Assignment length {} does not match num_vertices {}",
            assignment.len(),
            num_vertices
        );
        if num_vertices > 0 {
            assert!(
                num_registers > 0,
                "num_registers must be positive when there are vertices"
            );
        }
        for (v, &r) in assignment.iter().enumerate() {
            assert!(
                r < num_registers,
                "Assignment[{}] = {} is out of bounds for {} registers",
                v,
                r,
                num_registers
            );
        }
        Self {
            num_vertices,
            arcs,
            num_registers,
            assignment,
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

    /// Get the number of registers.
    pub fn num_registers(&self) -> usize {
        self.num_registers
    }

    /// Get the arcs.
    pub fn arcs(&self) -> &[(usize, usize)] {
        &self.arcs
    }

    /// Get the register assignment.
    pub fn assignment(&self) -> &[usize] {
        &self.assignment
    }

    /// Check whether the given config (position assignment) is feasible.
    ///
    /// Returns `true` if the config is a valid permutation, respects
    /// topological ordering, and has no register conflicts.
    pub fn is_feasible(&self, config: &[usize]) -> bool {
        let n = self.num_vertices;
        if config.len() != n {
            return false;
        }

        // Check valid permutation: each position 0..n-1 used exactly once
        let mut order = vec![0usize; n]; // order[position] = vertex
        let mut used = vec![false; n];
        for (vertex, &position) in config.iter().enumerate() {
            if position >= n {
                return false;
            }
            if used[position] {
                return false;
            }
            used[position] = true;
            order[position] = vertex;
        }

        // Build dependency info
        let mut dependencies: Vec<Vec<usize>> = vec![vec![]; n];
        let mut dependents: Vec<Vec<usize>> = vec![vec![]; n];
        for &(v, u) in &self.arcs {
            dependencies[v].push(u);
            dependents[u].push(v);
        }

        // Check topological ordering and register conflicts
        let mut computed = vec![false; n];

        for step in 0..n {
            let vertex = order[step];

            // Check dependencies: all dependencies must have been computed
            for &dep in &dependencies[vertex] {
                if !computed[dep] {
                    return false;
                }
            }

            // Check register conflict: the register assigned to this vertex
            // must not be currently occupied by a live value.
            // A previously computed vertex w is "live" if:
            //   - assignment[w] == assignment[vertex] (same register)
            //   - w has at least one dependent (other than vertex) that hasn't
            //     been computed yet. The current vertex is consuming w's value
            //     at this step, so we exclude it from the liveness check.
            let reg = self.assignment[vertex];
            for &w in &order[..step] {
                if self.assignment[w] == reg {
                    let still_live = dependents[w].iter().any(|&d| d != vertex && !computed[d]);
                    if still_live {
                        return false;
                    }
                }
            }

            computed[vertex] = true;
        }

        true
    }
}

impl Problem for FeasibleRegisterAssignment {
    const NAME: &'static str = "FeasibleRegisterAssignment";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_vertices; self.num_vertices]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(self.is_feasible(config))
    }
}

crate::declare_variants! {
    default FeasibleRegisterAssignment => "factorial(num_vertices)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "feasible_register_assignment",
        // 4 vertices, arcs: (0,1),(0,2),(1,3), K=2, assignment [0,1,0,0]
        // Valid order: v3, v1, v2, v0 -> config [3, 1, 2, 0]
        instance: Box::new(FeasibleRegisterAssignment::new(
            4,
            vec![(0, 1), (0, 2), (1, 3)],
            2,
            vec![0, 1, 0, 0],
        )),
        // config[v] = position: v0 at pos 3, v1 at pos 1, v2 at pos 2, v3 at pos 0
        // Order: v3(pos0), v1(pos1), v2(pos2), v0(pos3)
        optimal_config: vec![3, 1, 2, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/feasible_register_assignment.rs"]
mod tests;
