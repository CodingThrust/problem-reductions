//! Reduction from KColoring to ILP (Integer Linear Programming).
//!
//! The Graph K-Coloring problem can be formulated as a binary ILP:
//! - Variables: x_{v,c} for each vertex v and color c (binary, 1 if vertex v has color c)
//! - Constraints:
//!   1. Each vertex has exactly one color: sum_c x_{v,c} = 1 for each vertex v
//!   2. Adjacent vertices have different colors: x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and color c
//! - Objective: None (feasibility problem, minimize 0)

use crate::models::graph::KColoring;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::poly;
use crate::rules::registry::{ReductionEntry, ReductionOverhead};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

// Register reduction in the inventory for automatic discovery
inventory::submit! {
    ReductionEntry {
        source_name: "KColoring",
        target_name: "ILP",
        source_variant: &[("k", "N"), ("graph", "SimpleGraph"), ("weight", "i32")],
        target_variant: &[("graph", ""), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_vars", poly!(num_vertices * num_colors)),
            ("num_constraints", poly!(num_vertices) + poly!(num_edges * num_colors)),
        ]),
        module_path: module_path!(),
    }
}

/// Result of reducing KColoring to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each (vertex, color) pair corresponds to a binary variable
/// - Constraints ensure each vertex has exactly one color
/// - Constraints ensure adjacent vertices have different colors
#[derive(Debug, Clone)]
pub struct ReductionKColoringToILP<const K: usize, G, W> {
    target: ILP,
    num_vertices: usize,
    _phantom: std::marker::PhantomData<(G, W)>,
}

impl<const K: usize, G, W> ReductionKColoringToILP<K, G, W> {
    /// Get the variable index for vertex v with color c.
    fn var_index(&self, vertex: usize, color: usize) -> usize {
        vertex * K + color
    }
}

impl<const K: usize, G, W> ReductionResult for ReductionKColoringToILP<K, G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + 'static,
{
    type Source = KColoring<K, G, W>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to KColoring.
    ///
    /// The ILP solution has num_vertices * K binary variables.
    /// For each vertex, we find which color has value 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_vertices)
            .map(|v| {
                (0..K)
                    .find(|&c| {
                        let var_idx = self.var_index(v, c);
                        var_idx < target_solution.len() && target_solution[var_idx] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }
}

impl<const K: usize, G, W> ReduceTo<ILP> for KColoring<K, G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + 'static,
{
    type Result = ReductionKColoringToILP<K, G, W>;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.num_vertices();
        let num_vars = num_vertices * K;

        // Helper function to get variable index
        let var_index = |v: usize, c: usize| -> usize { v * K + c };

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        let mut constraints = Vec::new();

        // Constraint 1: Each vertex has exactly one color
        // sum_c x_{v,c} = 1 for each vertex v
        for v in 0..num_vertices {
            let terms: Vec<(usize, f64)> = (0..K).map(|c| (var_index(v, c), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Constraint 2: Adjacent vertices have different colors
        // x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and each color c
        for (u, v) in self.edges() {
            for c in 0..K {
                constraints.push(LinearConstraint::le(
                    vec![(var_index(u, c), 1.0), (var_index(v, c), 1.0)],
                    1.0,
                ));
            }
        }

        // Objective: minimize 0 (feasibility problem)
        // We use an empty objective
        let objective: Vec<(usize, f64)> = vec![];

        let target = ILP::new(
            num_vars,
            bounds,
            constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionKColoringToILP {
            target,
            num_vertices,
            _phantom: std::marker::PhantomData,
        }
    }
}

// Keep the old type alias for backwards compatibility
pub type ReductionColoringToILP = ReductionKColoringToILP<3, SimpleGraph, i32>;

#[cfg(test)]
#[path = "../unit_tests/rules/coloring_ilp.rs"]
mod tests;
