//! Reduction from KColoring to ILP (Integer Linear Programming).
//!
//! The Graph K-Coloring problem can be formulated as a binary ILP:
//! - Variables: x_{v,c} for each vertex v and color c (binary, 1 if vertex v has color c)
//! - Constraints:
//!   1. Each vertex has exactly one color: sum_c x_{v,c} = 1 for each vertex v
//!   2. Adjacent vertices have different colors: x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and color c
//! - Objective: None (feasibility problem, minimize 0)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::KColoring;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::{KValue, K1, K2, K3, K4, KN};

/// Result of reducing KColoring to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each (vertex, color) pair corresponds to a binary variable
/// - Constraints ensure each vertex has exactly one color
/// - Constraints ensure adjacent vertices have different colors
#[derive(Debug, Clone)]
pub struct ReductionKColoringToILP<K: KValue, G> {
    target: ILP<bool>,
    num_vertices: usize,
    num_colors: usize,
    _phantom: std::marker::PhantomData<(K, G)>,
}

impl<K: KValue, G> ReductionKColoringToILP<K, G> {
    /// Get the variable index for vertex v with color c.
    fn var_index(&self, vertex: usize, color: usize) -> usize {
        vertex * self.num_colors + color
    }
}

impl<K: KValue, G> ReductionResult for ReductionKColoringToILP<K, G>
where
    G: Graph + crate::variant::VariantParam,
{
    type Source = KColoring<K, G>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to KColoring.
    ///
    /// The ILP solution has num_vertices * K binary variables.
    /// For each vertex, we find which color has value 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let k = self.num_colors;
        (0..self.num_vertices)
            .map(|v| {
                (0..k)
                    .find(|&c| {
                        let var_idx = self.var_index(v, c);
                        var_idx < target_solution.len() && target_solution[var_idx] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }
}

/// Helper function implementing the KColoring to ILP reduction logic.
fn reduce_kcoloring_to_ilp<K: KValue, G: Graph>(
    problem: &KColoring<K, G>,
) -> ReductionKColoringToILP<K, G> {
    let k = problem.num_colors();
    let num_vertices = problem.graph().num_vertices();
    let num_vars = num_vertices * k;

    // Helper function to get variable index
    let var_index = |v: usize, c: usize| -> usize { v * k + c };

    let mut constraints = Vec::new();

    // Constraint 1: Each vertex has exactly one color
    // sum_c x_{v,c} = 1 for each vertex v
    for v in 0..num_vertices {
        let terms: Vec<(usize, f64)> = (0..k).map(|c| (var_index(v, c), 1.0)).collect();
        constraints.push(LinearConstraint::eq(terms, 1.0));
    }

    // Constraint 2: Adjacent vertices have different colors
    // x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and each color c
    for (u, v) in problem.graph().edges() {
        for c in 0..k {
            constraints.push(LinearConstraint::le(
                vec![(var_index(u, c), 1.0), (var_index(v, c), 1.0)],
                1.0,
            ));
        }
    }

    // Objective: minimize 0 (feasibility problem)
    // We use an empty objective
    let objective: Vec<(usize, f64)> = vec![];

    let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

    ReductionKColoringToILP {
        target,
        num_vertices,
        num_colors: k,
        _phantom: std::marker::PhantomData,
    }
}

// Register only the KN variant in the reduction graph
#[reduction(
    overhead = {
        num_vars = "num_vertices^2",
        num_constraints = "num_vertices + num_vertices * num_edges",
    }
)]
impl ReduceTo<ILP<bool>> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToILP<KN, SimpleGraph>;

    fn reduce_to(&self) -> Self::Result {
        reduce_kcoloring_to_ilp(self)
    }
}

// Additional concrete impls for tests (not registered in reduction graph)
macro_rules! impl_kcoloring_to_ilp {
    ($($ktype:ty),+) => {$(
        impl ReduceTo<ILP<bool>> for KColoring<$ktype, SimpleGraph> {
            type Result = ReductionKColoringToILP<$ktype, SimpleGraph>;
            fn reduce_to(&self) -> Self::Result { reduce_kcoloring_to_ilp(self) }
        }
    )+};
}

impl_kcoloring_to_ilp!(K1, K2, K3, K4);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::SimpleGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_ilp",
        build: || {
            let (n, edges) = crate::topology::small_graphs::petersen();
            let source = KColoring::<KN, _>::with_k(SimpleGraph::new(n, edges), 3);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 2, 0, 1, 2, 1, 1, 2, 0, 0],
                    target_config: vec![
                        1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1,
                        0, 0, 1, 0, 0,
                    ],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/coloring_ilp.rs"]
mod tests;
