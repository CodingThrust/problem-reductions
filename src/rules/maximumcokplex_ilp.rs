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
use crate::types::{i64_to_exact_f64, One, WeightElement};
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
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

fn build_constraints(graph: &SimpleGraph, bound_k: usize) -> Result<Vec<LinearConstraint>, ()> {
    (0..graph.num_vertices())
        .map(|v| {
            let degree = i64::try_from(graph.degree(v)).map_err(|_| ())?;
            let bound_k = i64::try_from(bound_k).map_err(|_| ())?;
            let mut terms: Vec<(usize, i64)> =
                graph.neighbors(v).into_iter().map(|u| (u, 1)).collect();
            if degree > 0 {
                terms.push((v, degree));
            }
            let rhs = degree
                .checked_add(bound_k)
                .and_then(|value| value.checked_sub(1))
                .ok_or(())?;
            Ok(LinearConstraint::le(terms, rhs))
        })
        .collect()
}

fn reduce_cokplex_to_ilp<W>(
    src: &MaximumCoKPlex<SimpleGraph, W, KN>,
    constraints: Vec<LinearConstraint>,
    objective: Vec<(usize, f64)>,
) -> Result<ReductionCoKPlexToILP<W>, crate::registry::ConstructionError>
where
    W: WeightElement + VariantParam,
{
    let target = ILP::new(
        src.num_vertices(),
        constraints,
        objective,
        ObjectiveSense::Maximize,
    )?;
    Ok(ReductionCoKPlexToILP {
        target,
        _marker: PhantomData,
    })
}

#[reduction(
    size = exact {
        num_vars = "num_vertices",
        num_constraints = "num_vertices",
    },
)]
impl ReduceTo<ILP<bool>> for MaximumCoKPlex<SimpleGraph, i64, KN> {
    type Result = ReductionCoKPlexToILP<i64>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(vertex, &weight)| Ok((vertex, i64_to_exact_f64(weight)?)))
            .collect::<Result<_, crate::types::ExactI64ToF64Error>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    MaximumCoKPlex<SimpleGraph, i64, KN>,
                    ILP<bool>,
                >(error)
            })?;
        let constraints = build_constraints(self.graph(), self.bound_k()).map_err(|_| {
            crate::rules::ReductionError::integer_overflow::<
                MaximumCoKPlex<SimpleGraph, i64, KN>,
                ILP<bool>,
            >("encoding a degree or co-k-plex bound")
        })?;
        reduce_cokplex_to_ilp(self, constraints, objective).map_err(Self::target_construction)
    }
}

#[reduction(
    size = exact {
        num_vars = "num_vertices",
        num_constraints = "num_vertices",
    },
)]
impl ReduceTo<ILP<bool>> for MaximumCoKPlex<SimpleGraph, One, KN> {
    type Result = ReductionCoKPlexToILP<One>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(v, _)| (v, 1.0))
            .collect();
        let constraints = build_constraints(self.graph(), self.bound_k()).map_err(|_| {
            crate::rules::ReductionError::integer_overflow::<
                MaximumCoKPlex<SimpleGraph, One, KN>,
                ILP<bool>,
            >("encoding a degree or co-k-plex bound")
        })?;
        reduce_cokplex_to_ilp(self, constraints, objective).map_err(Self::target_construction)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "weighted_maximumcokplex_to_ilp",
            build: || {
                let source = MaximumCoKPlex::<_, i64, KN>::with_k(
                    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
                    vec![5, 1, 4, 1, 3],
                    2,
                );
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "cardinality_maximumcokplex_to_ilp",
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
