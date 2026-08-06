//! Reduction from MaximumCoKPlex to ILP (Integer Linear Programming).
//!
//! Binary variable `x_v` per vertex.
//! Objective: maximize `sum_v w_v x_v`.
//! For each vertex `v`, if `x_v = 1` then at most `k - 1` neighbours may also
//! be selected, encoded by `sum_{u in N(v)} x_u + d(v) x_v <= d(v) + k - 1`.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumCoKPlex;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{One, WeightElement};
use crate::variant::{VariantParam, KN};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReductionCoKPlexToILP<W> {
    target: ILP<bool>,
    _marker: PhantomData<W>,
}

impl<W> ReductionResult for ReductionCoKPlexToILP<W>
where
    W: WeightElement + VariantParam,
{
    type Source = MaximumCoKPlex<SimpleGraph, W, KN>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

fn build_constraints(graph: &SimpleGraph, bound_k: usize) -> Vec<LinearConstraint> {
    (0..graph.num_vertices())
        .map(|v| {
            let degree = graph.degree(v) as f64;
            let mut terms: Vec<(usize, f64)> =
                graph.neighbors(v).into_iter().map(|u| (u, 1.0)).collect();
            if degree > 0.0 {
                terms.push((v, degree));
            }
            LinearConstraint::le(terms, degree + (bound_k - 1) as f64)
        })
        .collect()
}

fn reduce_cokplex_to_ilp<W>(
    src: &MaximumCoKPlex<SimpleGraph, W, KN>,
    objective: Vec<(usize, f64)>,
) -> ReductionCoKPlexToILP<W>
where
    W: WeightElement + VariantParam,
{
    let target = ILP::new(
        src.num_vertices(),
        build_constraints(src.graph(), src.bound_k()),
        objective,
        ObjectiveSense::Maximize,
    );
    ReductionCoKPlexToILP {
        target,
        _marker: PhantomData,
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_constraints = "num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumCoKPlex<SimpleGraph, i32, KN> {
    type Result = ReductionCoKPlexToILP<i32>;

    fn reduce_to(&self) -> Self::Result {
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(v, &weight)| (v, weight as f64))
            .collect();
        reduce_cokplex_to_ilp(self, objective)
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_constraints = "num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumCoKPlex<SimpleGraph, One, KN> {
    type Result = ReductionCoKPlexToILP<One>;

    fn reduce_to(&self) -> Self::Result {
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(v, _)| (v, 1.0))
            .collect();
        reduce_cokplex_to_ilp(self, objective)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumcokplex_i32_to_ilp",
            build: || {
                let source = MaximumCoKPlex::<_, i32, KN>::with_k(
                    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
                    vec![5, 1, 4, 1, 3],
                    2,
                );
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumcokplex_one_to_ilp",
            build: || {
                let source = MaximumCoKPlex::<_, One, KN>::with_k(
                    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
                    vec![One; 5],
                    2,
                );
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumcokplex_ilp.rs"]
mod tests;
