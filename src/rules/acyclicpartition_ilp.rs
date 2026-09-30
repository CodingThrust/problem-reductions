//! Bounded partition labels with exact crossing flags and occupied-part budgets.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::AcyclicPartition;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionAcyclicPartitionToILP {
    target: ILP<i64, i64, Bounded>,
    n: usize,
}

impl ReductionResult for ReductionAcyclicPartitionToILP {
    type Source = AcyclicPartition<i64>;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    /// One-hot decode: for each vertex v, output the unique c with x_{v,c} = 1.
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

        crate::rules::ilp_helpers::one_hot_decode_rows(target_solution, self.n, self.n, 0)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionAcyclicPartitionToILP {}

#[reduction(transform = {
    exact {
        num_vars = "num_vertices^2 + 2 * num_vertices + num_arcs",
        num_constraints = "num_vertices^2 + 4 * num_vertices + 2 * num_arcs + 1",
    },
    upper_bound {
        max_constraint_magnitude_bits = "max_numeric_magnitude_bits + num_vertices + 1",
        num_nonzeros = "6 * num_vertices^2 + 2 * num_vertices + 7 * num_arcs",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for AcyclicPartition<i64> {
    type Result = ReductionAcyclicPartitionToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let arcs = self.graph().arcs();
        let m = arcs.len();

        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                "counting acyclic partition variables",
            )
        };
        let square = n.checked_mul(n).ok_or_else(overflow)?;
        let labels = square.checked_add(n).ok_or_else(overflow)?;
        let crossing = labels.checked_add(n).ok_or_else(overflow)?;
        let num_vars = crossing.checked_add(m).ok_or_else(overflow)?;
        let last_label = Self::exact_i64(n.saturating_sub(1), "bounding partition labels")?;
        let x_idx = |v: usize, c: usize| v * n + c;
        let empty_idx = |c: usize| square + c;
        let label_idx = |v: usize| labels + v;
        let y_idx = |t: usize| crossing + t;
        let mut constraints = Vec::new();
        let vertex_weights = self.vertex_weights();
        let arc_costs = self.arc_costs();
        let weight_bound = *self.weight_bound();
        let cost_bound = *self.cost_bound();

        // Assignment: Σ_c x_{v,c} = 1 for each vertex v.
        for v in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|c| (x_idx(v, c), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
            let mut label = vec![(label_idx(v), 1)];
            for c in 1..n {
                label.push((
                    x_idx(v, c),
                    -Self::exact_i64(c, "representing a partition label")?,
                ));
            }
            constraints.push(LinearConstraint::eq(label, 0));
        }

        // Only occupied classes must meet the weight bound, which can be negative.
        for c in 0..n {
            let mut membership = vec![(empty_idx(c), 1)];
            for v in 0..n {
                constraints.push(LinearConstraint::le(
                    vec![(x_idx(v, c), 1), (empty_idx(c), 1)],
                    1,
                ));
                membership.push((x_idx(v, c), 1));
            }
            constraints.push(LinearConstraint::ge(membership, 1));
            let mut terms: Vec<(usize, i64)> = vertex_weights
                .iter()
                .enumerate()
                .map(|(vertex, &weight)| (x_idx(vertex, c), weight))
                .collect();
            // Keep the bound on the RHS to preserve representable source sums.
            terms.push((empty_idx(c), weight_bound.min(0)));
            constraints.push(LinearConstraint::le(terms, weight_bound));
        }

        // A crossing arc increases its part label by at least one; an internal
        // arc has equal labels. This equivalence also handles negative costs.
        for (t, &(u, v)) in arcs.iter().enumerate() {
            constraints.push(LinearConstraint::ge(
                vec![(label_idx(v), 1), (label_idx(u), -1), (y_idx(t), -1)],
                0,
            ));
            constraints.push(LinearConstraint::le(
                vec![
                    (label_idx(v), 1),
                    (label_idx(u), -1),
                    (y_idx(t), -last_label),
                ],
                0,
            ));
        }

        // Cost bound: Σ_t cost(a_t) * y_t ≤ K.
        let cost_terms: Vec<(usize, i64)> = arc_costs
            .iter()
            .enumerate()
            .map(|(arc, &cost)| (y_idx(arc), cost))
            .collect();
        constraints.push(LinearConstraint::le(cost_terms, cost_bound));

        let mut variables = vec![IntegerVariable::binary(); num_vars];
        variables[labels..crossing].fill(
            IntegerVariable::new(Some(0), Some(last_label)).map_err(Self::target_construction)?,
        );
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionAcyclicPartitionToILP { target, n })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "acyclicpartition_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(AcyclicPartition::new(
                DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1, 1, 1, 1],
                vec![1, 1, 1],
                3,
                2,
            ))
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/acyclicpartition_ilp.rs"]
mod tests;
