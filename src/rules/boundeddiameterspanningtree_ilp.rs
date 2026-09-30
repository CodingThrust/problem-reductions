//! A bounded-depth tree rooted at a vertex or the two ends of a center edge.
use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BoundedDiameterSpanningTree;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

#[derive(Debug, Clone)]
pub struct ReductionBoundedDiameterSpanningTreeToILP {
    target: ILP<i64, i64, Bounded>,
    num_edges: usize,
}
impl ReductionResult for ReductionBoundedDiameterSpanningTreeToILP {
    type Source = BoundedDiameterSpanningTree<SimpleGraph, i64>;
    type Target = ILP<i64, i64, Bounded>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |v| v.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(solution[..self.num_edges].iter().map(|&v| v == 1).collect())
    }
}
#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionBoundedDiameterSpanningTreeToILP {}
#[crate::reduction(transform = upper_bound {
    num_vars = "4 * num_edges + 2 * num_vertices",
    num_constraints = "3 * num_edges + 3 * num_vertices + 2",
    num_nonzeros = "16 * num_edges + 4 * num_vertices",
    max_constraint_magnitude_bits = "64",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for BoundedDiameterSpanningTree<SimpleGraph, i64> {
    type Result = ReductionBoundedDiameterSpanningTreeToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let m = self.num_edges();
        if n == 0 {
            return Ok(Self::Result {
                target: ILP::empty(),
                num_edges: 0,
            });
        }
        let diameter = self.diameter_bound().min(n - 1);
        let odd = diameter % 2 == 1;
        let q = Self::exact_i64(diameter / 2, "bounding tree depth")?;
        let big_m = q + 1;
        let count = m
            .checked_mul(if odd { 4 } else { 3 })
            .and_then(|v| n.checked_mul(2).and_then(|w| v.checked_add(w)))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting tree variables",
                )
            })?;
        let root = 3 * m;
        let depth = root + n;
        let center = depth + n;
        let mut variables = vec![IntegerVariable::binary(); count];
        variables[depth..center]
            .fill(IntegerVariable::new(Some(0), Some(q)).map_err(Self::target_construction)?);
        let mut incoming: Vec<_> = (0..n).map(|v| vec![(root + v, 1)]).collect();
        let mut centers: Vec<_> = (0..n).map(|v| vec![(root + v, 1)]).collect();
        let mut rows = Vec::new();
        for (e, &(u, v)) in self.edge_list().iter().enumerate() {
            let forward = m + 2 * e;
            let reverse = forward + 1;
            if u == v {
                for i in [e, forward, reverse]
                    .into_iter()
                    .chain(odd.then_some(center + e))
                {
                    variables[i] = IntegerVariable::new(Some(0), Some(0))
                        .map_err(Self::target_construction)?;
                }
            }
            let mut terms = vec![(e, 1), (forward, -1), (reverse, -1)];
            if odd {
                terms.push((center + e, -1));
            }
            rows.push(LinearConstraint::eq(terms, 0));
            incoming[v].push((forward, 1));
            incoming[u].push((reverse, 1));
            rows.push(LinearConstraint::ge(
                vec![(depth + v, 1), (depth + u, -1), (forward, -big_m)],
                1 - big_m,
            ));
            rows.push(LinearConstraint::ge(
                vec![(depth + u, 1), (depth + v, -1), (reverse, -big_m)],
                1 - big_m,
            ));
            if odd {
                centers[u].push((center + e, -1));
                if u != v {
                    centers[v].push((center + e, -1));
                }
            }
        }
        for (v, terms) in incoming.into_iter().enumerate() {
            rows.push(LinearConstraint::eq(terms, 1));
            rows.push(LinearConstraint::le(vec![(depth + v, 1), (root + v, q)], q));
        }
        if odd {
            rows.push(LinearConstraint::eq(
                (0..m).map(|e| (center + e, 1)).collect(),
                1,
            ));
            rows.extend(
                centers
                    .into_iter()
                    .map(|terms| LinearConstraint::eq(terms, 0)),
            );
        } else {
            rows.push(LinearConstraint::eq(
                (0..n).map(|v| (root + v, 1)).collect(),
                1,
            ));
        }
        rows.push(LinearConstraint::le(
            self.edge_weights().iter().copied().enumerate().collect(),
            *self.weight_bound(),
        ));
        let target = ILP::with_variables(variables, rows, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_edges: m,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "boundeddiameterspanningtree_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(
                BoundedDiameterSpanningTree::new(
                    SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                    vec![1; 3],
                    3,
                    3,
                ),
            )
        },
    }]
}
#[cfg(test)]
#[path = "../unit_tests/rules/boundeddiameterspanningtree_ilp.rs"]
mod tests;
