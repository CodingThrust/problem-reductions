//! Polynomial reduction from HighlyConnectedDeletion to binary ILP.
//!
//! One variable per unordered vertex pair records membership in the same cluster.
//! Triangle inequalities make membership transitive. A binary flag per vertex
//! distinguishes singleton clusters, and linear degree constraints enforce
//! minimum degree strictly greater than half the cluster size otherwise.
//!
//! This degree condition is equivalent to high edge connectivity: for minimum
//! degree d > k/2, a cut with smaller side a <= k/2 has at least
//! a*(d-a+1) >= d edges. Conversely, edge connectivity never exceeds minimum
//! degree. See <https://hueffner.de/falk/highly-connected-subgraph-sofsem15.pdf>.
//!
//! Maximize the number of non-loop edges kept inside clusters. Any feasible
//! deletion can restore all internal edges without losing connectivity, so an
//! optimal source solution is represented. Self-loops are always kept.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::HighlyConnectedDeletion;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HighlyConnectedDeletion to binary ILP.
///
/// Variables are unordered pairs in lexicographic order, followed by one
/// non-singleton flag per vertex.
#[derive(Debug, Clone)]
pub struct ReductionHighlyConnectedDeletionToILP {
    target: ILP<bool>,
    /// Pair variable for each source edge; self-loops have no variable.
    edge_variables: Vec<Option<usize>>,
}

impl ReductionResult for ReductionHighlyConnectedDeletionToILP {
    type Source = HighlyConnectedDeletion<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(self
            .edge_variables
            .iter()
            .map(|variable| variable.is_some_and(|index| target_solution[index] == 0))
            .collect())
    }
}

// Three rows per vertex triple and two per vertex. Triple rows have three
// nonzeros; each vertex row has at most n. All row magnitudes are <= max(n-1, 2).
#[reduction(transform = {
    exact {
        num_vars = "num_vertices * (num_vertices + 1) / 2",
    },
    upper_bound {
        num_constraints = "num_vertices^3 + 2 * num_vertices",
        num_nonzeros = "3 * num_vertices^3 + 2 * num_vertices^2",
        max_constraint_magnitude_bits = "num_vertices + 2",
    },
})]
impl ReduceTo<ILP<bool>> for HighlyConnectedDeletion<SimpleGraph> {
    type Result = ReductionHighlyConnectedDeletionToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let graph = self.graph();
        let n = graph.num_vertices();
        let num_pairs = n.checked_mul(n.saturating_sub(1)).map(|value| value / 2);
        let num_vars = num_pairs
            .and_then(|pairs| pairs.checked_add(n))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(
                    "counting pair and non-singleton variables",
                )
            })?;
        let flag_offset = num_vars - n;
        let max_companions =
            Self::exact_i64(n.saturating_sub(1), "encoding the cluster-size bound")?;
        // Lexicographic pair index: preceding row lengths, then the column offset.
        // The checked n*(n-1) above also bounds these products for u < v < n.
        let pair = |u: usize, v: usize| {
            let (u, v) = (u.min(v), u.max(v));
            u * n - u * (u + 1) / 2 + (v - u - 1)
        };
        let mut constraints = Vec::new();
        for u in 0..n {
            for v in u + 1..n {
                for w in v + 1..n {
                    let (a, b, c) = (pair(u, v), pair(u, w), pair(v, w));
                    for (first, second, third) in [(a, b, c), (a, c, b), (b, c, a)] {
                        constraints.push(LinearConstraint::le(
                            vec![(first, 1), (second, 1), (third, -1)],
                            1,
                        ));
                    }
                }
            }
        }
        for v in 0..n {
            // s_v is the number of other vertices in v's cluster. The flag
            // a_v is forced on for s_v > 0 by s_v <= (n-1)*a_v.
            let mut size_terms = Vec::new();
            let mut degree_terms = Vec::new();
            for u in 0..n {
                if u != v {
                    let variable = pair(u, v);
                    size_terms.push((variable, 1));
                    // 2*d_v - s_v: count each distinct, non-loop neighbor once.
                    degree_terms.push((variable, if graph.has_edge(u, v) { 1 } else { -1 }));
                }
            }
            size_terms.push((flag_offset + v, -max_companions));
            constraints.push(LinearConstraint::le(size_terms, 0));
            // 2*d_v >= s_v + 2*a_v: singleton clusters pass, while other
            // clusters require 2*d_v > s_v+1 and automatically exclude pairs.
            degree_terms.push((flag_offset + v, -2));
            constraints.push(LinearConstraint::ge(degree_terms, 0));
        }
        let edge_variables: Vec<_> = graph
            .edges()
            .iter()
            .map(|&(u, v)| (u != v).then(|| pair(u, v)))
            .collect();
        // Duplicate edges contribute their multiplicity to the objective;
        // self-loops are constant and need no objective term.
        let objective = edge_variables
            .iter()
            .flatten()
            .map(|&variable| (variable, 1))
            .collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize)
            .map_err(Self::target_construction)?;
        Ok(ReductionHighlyConnectedDeletionToILP {
            target,
            edge_variables,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "highlyconnecteddeletion_to_ilp",
        build: || {
            // Canonical example: triangle {0,1,2} + leaf vertex 3.
            // Optimum deletes only the leaf edge (2,3); ILP keeps the triangle
            // cluster and the {3} singleton.
            let source = HighlyConnectedDeletion::new(SimpleGraph::new(
                4,
                vec![(0, 1), (0, 2), (1, 2), (2, 3)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/highlyconnecteddeletion_ilp.rs"]
mod tests;
