//! Reduction from MaximumDomaticNumber to ILP (Integer Linear Programming).
//!
//! The Maximum Domatic Number problem can be formulated as a binary ILP:
//! - Variables: x_{v,i} for each vertex v and set index i (binary: vertex v in set i),
//!   plus y_i for each set index i (binary: set i is used).
//! - Partition constraints: for each v, Σ_i x_{v,i} = 1
//! - Domination constraints: for each v and i, x_{v,i} + Σ_{u ∈ N(v)} x_{u,i} ≥ y_i
//! - Linking constraints: y_i ≤ Σ_v x_{v,i} for each i
//! - Objective: maximize Σ y_i

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumDomaticNumber;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaximumDomaticNumber to ILP.
///
/// Variable layout:
/// - x_{v,i} at index v*n + i (vertex v assigned to set i)
/// - y_i at index n*n + i (set i is used)
#[derive(Debug, Clone)]
pub struct ReductionDomaticNumberToILP {
    target: ILP<bool>,
    n: usize,
}

impl ReductionResult for ReductionDomaticNumberToILP {
    type Source = MaximumDomaticNumber<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MaximumDomaticNumber.
    ///
    /// For each vertex v, find the set index i where x_{v,i} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        let mut config = vec![0; n];
        for v in 0..n {
            for i in 0..n {
                if target_solution[v * n + i] == 1 {
                    config[v] = i;
                    break;
                }
            }
        }
        config
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices * num_vertices + num_vertices",
        num_constraints = "num_vertices + num_vertices * num_vertices + num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumDomaticNumber<SimpleGraph> {
    type Result = ReductionDomaticNumberToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let num_vars = n * n + n;
        let mut constraints = Vec::new();

        // Partition constraints: for each vertex v, Σ_i x_{v,i} = 1
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|i| (v * n + i, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Domination constraints: for each v, i: x_{v,i} + Σ_{u ∈ N(v)} x_{u,i} >= y_i
        // Rewritten as: x_{v,i} + Σ_{u ∈ N(v)} x_{u,i} - y_i >= 0
        for v in 0..n {
            let neighbors = self.graph().neighbors(v);
            for i in 0..n {
                let mut terms: Vec<(usize, f64)> = vec![(v * n + i, 1.0)];
                for &u in &neighbors {
                    terms.push((u * n + i, 1.0));
                }
                // -y_i
                terms.push((n * n + i, -1.0));
                constraints.push(LinearConstraint::ge(terms, 0.0));
            }
        }

        // Linking constraints: y_i <= Σ_v x_{v,i} for each i
        // Rewritten as: Σ_v x_{v,i} - y_i >= 0
        for i in 0..n {
            let mut terms: Vec<(usize, f64)> = (0..n).map(|v| (v * n + i, 1.0)).collect();
            terms.push((n * n + i, -1.0));
            constraints.push(LinearConstraint::ge(terms, 0.0));
        }

        // Objective: maximize Σ y_i
        let objective: Vec<(usize, f64)> = (0..n).map(|i| (n * n + i, 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionDomaticNumberToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumdomaticnumber_to_ilp",
        build: || {
            // Use small P3 graph (3 vertices, domatic number = 2)
            let source = MaximumDomaticNumber::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumdomaticnumber_ilp.rs"]
mod tests;
