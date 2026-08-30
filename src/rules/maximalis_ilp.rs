//! Reduction from MaximalIS to ILP (Integer Linear Programming).
//!
//! Binary variable x_v per vertex. Independence: ∀ edge (u,v): x_u + x_v ≤ 1.
//! Maximality: ∀ v: x_v + Σ_{u∈N(v)} x_u ≥ 1. Maximize Σ w_v·x_v.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximalIS;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::i64_to_exact_f64;

#[derive(Debug, Clone)]
pub struct ReductionMxISToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMxISToILP {
    type Source = MaximalIS<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices",
        num_constraints = "num_edges + num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MaximalIS<SimpleGraph, i64> {
    type Result = ReductionMxISToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let mut constraints = Vec::new();

        // Independence: ∀ edge (u,v): x_u + x_v ≤ 1
        for u in 0..n {
            for v in (u + 1)..n {
                if self.graph().has_edge(u, v) {
                    constraints.push(LinearConstraint::le(vec![(u, 1), (v, 1)], 1));
                }
            }
        }

        // Maximality: ∀ v: x_v + Σ_{u∈N(v)} x_u ≥ 1
        for v in 0..n {
            let mut terms = vec![(v, 1)];
            for u in self.graph().neighbors(v) {
                terms.push((u, 1));
            }
            constraints.push(LinearConstraint::ge(terms, 1));
        }

        // Objective: Maximize Σ w_v·x_v
        let weights = self.weights();
        let objective: Vec<(usize, f64)> = weights
            .iter()
            .enumerate()
            .map(|(i, &w)| {
                i64_to_exact_f64(w)
                    .map(|weight| (i, weight))
                    .map_err(|error| {
                        crate::rules::ReductionError::inexact_float_conversion::<
                            MaximalIS<SimpleGraph, i64>,
                            ILP<bool>,
                        >(error)
                    })
            })
            .collect::<Result<_, _>>()?;

        let target = ILP::new(n, constraints, objective, ObjectiveSense::Maximize)
            .map_err(Self::target_construction)?;
        Ok(ReductionMxISToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximalis_to_ilp",
        build: || {
            // Path P3: 0-1-2
            let source = MaximalIS::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 1, 1]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximalis_ilp.rs"]
mod tests;
