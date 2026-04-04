//! Reduction from MinimumMaximalMatching to ILP (Integer Linear Programming).
//!
//! The Minimum Maximal Matching problem can be formulated as a binary ILP:
//! - Variables: One binary variable e_i per edge (0 = not selected, 1 = selected)
//! - Matching constraints: For each vertex v, sum of e_i for edges incident to v <= 1
//! - Maximality constraints: For each edge j, e_j + sum_{i shares endpoint with j, i≠j} e_i >= 1
//!   (if edge j is not selected, at least one edge adjacent to it must be)
//! - Objective: Minimize sum e_i

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumMaximalMatching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumMaximalMatching to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each edge corresponds to a binary variable
/// - Vertex constraints ensure at most one incident edge is selected per vertex
/// - Edge constraints ensure that each edge is either selected or blocked by an adjacent
///   selected edge (maximality)
/// - The objective minimizes the total number of selected edges
#[derive(Debug, Clone)]
pub struct ReductionMMMToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMMMToILP {
    type Source = MinimumMaximalMatching<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumMaximalMatching.
    ///
    /// Since the mapping is 1:1 (each edge maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges",
        num_constraints = "num_vertices + num_edges",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumMaximalMatching<SimpleGraph> {
    type Result = ReductionMMMToILP;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.graph().edges();
        let num_vars = edges.len();
        let mut constraints = Vec::new();

        // Matching constraints: for each vertex v, sum of incident edge variables <= 1.
        // Build vertex -> incident edge index map.
        let n = self.graph().num_vertices();
        let mut v2e: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (idx, &(u, v)) in edges.iter().enumerate() {
            v2e[u].push(idx);
            v2e[v].push(idx);
        }
        for incident in &v2e {
            if !incident.is_empty() {
                let terms: Vec<(usize, f64)> = incident.iter().map(|&e| (e, 1.0)).collect();
                constraints.push(LinearConstraint::le(terms, 1.0));
            }
        }

        // Maximality constraints: for each edge j, the closed neighborhood (j itself plus all
        // edges sharing an endpoint with j) must contain at least one selected edge.
        // i.e. e_j + sum_{i: i shares endpoint with j, i≠j} e_i >= 1  for all j.
        for (j, &(uj, vj)) in edges.iter().enumerate() {
            // Collect all edges in the closed neighborhood of edge j.
            let mut neighbors: Vec<usize> = vec![j];
            for &i in v2e[uj].iter().chain(v2e[vj].iter()) {
                if i != j && !neighbors.contains(&i) {
                    neighbors.push(i);
                }
            }
            let terms: Vec<(usize, f64)> = neighbors.iter().map(|&i| (i, 1.0)).collect();
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Objective: minimize sum e_i
        let objective: Vec<(usize, f64)> = (0..num_vars).map(|i| (i, 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);
        ReductionMMMToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummaximalmatching_to_ilp",
        build: || {
            // Path graph P6
            let source = MinimumMaximalMatching::new(SimpleGraph::new(
                6,
                vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummaximalmatching_ilp.rs"]
mod tests;
