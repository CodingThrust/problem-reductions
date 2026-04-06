//! Minimum Code Generation with Unlimited Registers.
//!
//! Given a directed acyclic graph G = (V, A) with maximum out-degree 2
//! (an expression DAG) and a partition of arcs into left (L) and right (R)
//! operand sets, find a program of minimum number of instructions for an
//! unlimited-register machine using 2-address instructions. The left operand's
//! register is destroyed (overwritten by the result); a LOAD (copy) instruction
//! is needed to preserve values before destruction. NP-complete
//! [Aho, Johnson, and Ullman, 1977].

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCodeGenerationUnlimitedRegisters",
        display_name: "Minimum Code Generation (Unlimited Registers)",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum-length instruction sequence for an unlimited-register machine with 2-address instructions to evaluate an expression DAG",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices n = |V|" },
            FieldInfo { name: "left_arcs", type_name: "Vec<(usize, usize)>", description: "Left operand arcs L: (parent, child) — child's register is destroyed" },
            FieldInfo { name: "right_arcs", type_name: "Vec<(usize, usize)>", description: "Right operand arcs R: (parent, child) — child's register is preserved" },
        ],
    }
}

/// Minimum Code Generation with Unlimited Registers.
///
/// Given a directed acyclic graph G = (V, A) with maximum out-degree 2,
/// where arcs are partitioned into left (L) and right (R) operand sets,
/// leaves (out-degree 0) are input values each in its own register,
/// internal vertices are 2-address operations (the left operand's register
/// is overwritten by the result), and roots (in-degree 0) are values to
/// compute, find a program of minimum instructions using OP and LOAD (copy).
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
/// use problemreductions::models::misc::MinimumCodeGenerationUnlimitedRegisters;
/// use problemreductions::{Problem, Solver, BruteForce, Min};
///
/// // 5 vertices: leaves {3,4}, internal {0,1,2}
/// // v1 = op(v3, v4), v2 = op(v3, v4), v0 = op(v1, v2)
/// let problem = MinimumCodeGenerationUnlimitedRegisters::new(
///     5,
///     vec![(1,3),(2,3),(0,1)],  // left arcs (child destroyed)
///     vec![(1,4),(2,4),(0,2)],  // right arcs (child preserved)
/// );
/// let result = BruteForce::new().solve(&problem);
/// assert_eq!(result, Min(Some(4)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCodeGenerationUnlimitedRegisters {
    /// Number of vertices |V|.
    num_vertices: usize,
    /// Left operand arcs (parent, child) — child's register is destroyed.
    left_arcs: Vec<(usize, usize)>,
    /// Right operand arcs (parent, child) — child's register is preserved.
    right_arcs: Vec<(usize, usize)>,
}

impl MinimumCodeGenerationUnlimitedRegisters {
    /// Create a new instance.
    ///
    /// # Arguments
    ///
    /// * `num_vertices` - Total number of vertices
    /// * `left_arcs` - Left operand arcs (parent, child); child register is destroyed by OP
    /// * `right_arcs` - Right operand arcs (parent, child); child register is preserved
    ///
    /// # Panics
    ///
    /// Panics if any arc index is out of bounds, if any vertex has out-degree > 2,
    /// if left and right arcs for binary vertices are inconsistent, or if a vertex
    /// has a self-loop.
    pub fn new(
        num_vertices: usize,
        left_arcs: Vec<(usize, usize)>,
        right_arcs: Vec<(usize, usize)>,
    ) -> Self {
        let mut left_count = vec![0usize; num_vertices];
        let mut right_count = vec![0usize; num_vertices];

        for &(parent, child) in &left_arcs {
            assert!(
                parent < num_vertices && child < num_vertices,
                "Left arc ({parent}, {child}) out of bounds for {num_vertices} vertices"
            );
            assert!(
                parent != child,
                "Self-loop ({parent}, {parent}) not allowed"
            );
            left_count[parent] += 1;
        }
        for &(parent, child) in &right_arcs {
            assert!(
                parent < num_vertices && child < num_vertices,
                "Right arc ({parent}, {child}) out of bounds for {num_vertices} vertices"
            );
            assert!(
                parent != child,
                "Self-loop ({parent}, {parent}) not allowed"
            );
            right_count[parent] += 1;
        }

        for v in 0..num_vertices {
            let out = left_count[v] + right_count[v];
            assert!(out <= 2, "Vertex {v} has out-degree {out} > 2");
            // Binary vertex: exactly one left and one right
            if out == 2 {
                assert!(
                    left_count[v] == 1 && right_count[v] == 1,
                    "Binary vertex {v} must have exactly 1 left and 1 right arc"
                );
            }
            // Unary vertex: one left arc (result overwrites operand register)
            if out == 1 {
                assert!(
                    left_count[v] == 1 && right_count[v] == 0,
                    "Unary vertex {v} must have exactly 1 left arc and 0 right arcs"
                );
            }
        }

        Self {
            num_vertices,
            left_arcs,
            right_arcs,
        }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the left operand arcs.
    pub fn left_arcs(&self) -> &[(usize, usize)] {
        &self.left_arcs
    }

    /// Get the right operand arcs.
    pub fn right_arcs(&self) -> &[(usize, usize)] {
        &self.right_arcs
    }

    /// Get the number of leaf vertices (out-degree 0).
    pub fn num_leaves(&self) -> usize {
        self.num_vertices - self.num_internal()
    }

    /// Get the number of internal (non-leaf) vertices.
    pub fn num_internal(&self) -> usize {
        let mut has_children = vec![false; self.num_vertices];
        for &(parent, _) in &self.left_arcs {
            has_children[parent] = true;
        }
        for &(parent, _) in &self.right_arcs {
            has_children[parent] = true;
        }
        has_children.iter().filter(|&&b| b).count()
    }

    /// Determine which vertices are internal (non-leaf, i.e. out-degree > 0).
    fn internal_vertices(&self) -> Vec<usize> {
        let mut has_children = vec![false; self.num_vertices];
        for &(parent, _) in &self.left_arcs {
            has_children[parent] = true;
        }
        for &(parent, _) in &self.right_arcs {
            has_children[parent] = true;
        }
        (0..self.num_vertices)
            .filter(|&v| has_children[v])
            .collect()
    }

    /// Get the left child of a vertex, if any.
    fn left_child(&self, v: usize) -> Option<usize> {
        self.left_arcs
            .iter()
            .find(|&&(parent, _)| parent == v)
            .map(|&(_, child)| child)
    }

    /// Get the right child of a vertex, if any.
    fn right_child(&self, v: usize) -> Option<usize> {
        self.right_arcs
            .iter()
            .find(|&&(parent, _)| parent == v)
            .map(|&(_, child)| child)
    }

    /// Simulate the unlimited-register machine for a given evaluation order
    /// of internal vertices and return the instruction count, or `None` if
    /// the ordering is invalid (not a permutation or violates dependencies).
    ///
    /// With unlimited registers:
    /// - Each leaf starts in its own register
    /// - OP v: computes v, result overwrites the left operand's register
    /// - LOAD: copies a register value (needed when a left operand is still
    ///   needed later and would be destroyed)
    /// - Cost = num_OPs + num_LOADs
    pub fn simulate(&self, config: &[usize]) -> Option<usize> {
        let internal = self.internal_vertices();
        let n_internal = internal.len();
        if config.len() != n_internal {
            return None;
        }

        // config[i] = evaluation position for internal vertex index i
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

        // Track which vertices have been computed
        let mut computed = vec![false; self.num_vertices];
        // All leaves are "computed" (available in registers from the start)
        let has_children: Vec<bool> = {
            let mut hc = vec![false; self.num_vertices];
            for &(parent, _) in &self.left_arcs {
                hc[parent] = true;
            }
            for &(parent, _) in &self.right_arcs {
                hc[parent] = true;
            }
            hc
        };
        for v in 0..self.num_vertices {
            if !has_children[v] {
                computed[v] = true;
            }
        }

        // For each value, count how many future operations still need it
        // as a LEFT operand. Only left operands get destroyed.
        // But we also need to know total future uses (left + right) to know
        // if a value is still needed at all.
        let mut future_left_uses = vec![0usize; self.num_vertices];
        let mut future_right_uses = vec![0usize; self.num_vertices];
        for &idx in &order {
            let v = internal[idx];
            if let Some(lc) = self.left_child(v) {
                future_left_uses[lc] += 1;
            }
            if let Some(rc) = self.right_child(v) {
                future_right_uses[rc] += 1;
            }
        }

        let mut instructions = 0usize;

        // With unlimited registers, each value has its own register.
        // When OP v executes: result goes into left_child's register.
        // If left_child's value is still needed by a future operation,
        // we must LOAD (copy) it first.

        for step in 0..n_internal {
            let v = internal[order[step]];
            let lc = self.left_child(v);
            let rc = self.right_child(v);

            // Check dependencies
            if let Some(l) = lc {
                if !computed[l] {
                    return None;
                }
            }
            if let Some(r) = rc {
                if !computed[r] {
                    return None;
                }
            }

            // Decrement future use counts
            if let Some(l) = lc {
                future_left_uses[l] -= 1;
            }
            if let Some(r) = rc {
                future_right_uses[r] -= 1;
            }

            // Check if left operand needs to be copied before destruction
            if let Some(l) = lc {
                let still_needed = future_left_uses[l] + future_right_uses[l] > 0;
                if still_needed {
                    instructions += 1; // LOAD (copy)
                }
            }

            // OP v
            instructions += 1;

            // Mark v as computed
            computed[v] = true;
        }

        Some(instructions)
    }
}

impl Problem for MinimumCodeGenerationUnlimitedRegisters {
    const NAME: &'static str = "MinimumCodeGenerationUnlimitedRegisters";
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
    default MinimumCodeGenerationUnlimitedRegisters => "2 ^ num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_code_generation_unlimited_registers",
        // Issue #902 example: 5 vertices, leaves {3,4}, internal {0,1,2}
        // left_arcs: (1,3),(2,3),(0,1)
        // right_arcs: (1,4),(2,4),(0,2)
        // Optimal order: v1,v2,v0 with 1 copy of v3 = 4 instructions
        // Internal vertices sorted: [0, 1, 2]
        // Order v1(pos 0), v2(pos 1), v0(pos 2)
        // config[0]=2 (v0 at pos 2), config[1]=0 (v1 at pos 0), config[2]=1 (v2 at pos 1)
        instance: Box::new(MinimumCodeGenerationUnlimitedRegisters::new(
            5,
            vec![(1, 3), (2, 3), (0, 1)],
            vec![(1, 4), (2, 4), (0, 2)],
        )),
        optimal_config: vec![2, 0, 1],
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_code_generation_unlimited_registers.rs"]
mod tests;
