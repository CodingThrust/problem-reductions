//! Gate selection and source reachability, including shared or cyclic subgraphs.
use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumWeightAndOrGraph;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionMinimumWeightAndOrGraphToILP {
    target: ILP<i64, i64, Bounded>,
    num_arcs: usize,
}
impl ReductionResult for ReductionMinimumWeightAndOrGraphToILP {
    type Source = MinimumWeightAndOrGraph;
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
        Ok(solution[..self.num_arcs].iter().map(|&v| v == 1).collect())
    }
}
#[crate::reduction(transform = {
    exact { num_vars = "num_vertices + 2 * num_arcs", },
    upper_bound {
        num_constraints = "2 * num_vertices + 4 * num_arcs",
        num_nonzeros = "2 * num_vertices + 10 * num_arcs",
        max_constraint_magnitude_bits = "num_vertices",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for MinimumWeightAndOrGraph {
    type Result = ReductionMinimumWeightAndOrGraphToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let m = self.num_arcs();
        let count = m
            .checked_mul(2)
            .and_then(|v| v.checked_add(n))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting AND/OR flow variables",
                )
            })?;
        let capacity = Self::exact_i64(n - 1, "bounding reachability flow")?;
        let flow = m + n;
        let mut variables = Vec::with_capacity(count);
        variables.resize(flow, IntegerVariable::binary());
        let mut outgoing = vec![Vec::new(); n];
        let mut balance: Vec<_> = (0..n).map(|v| vec![(m + v, -1)]).collect();
        let mut rows = vec![LinearConstraint::eq(vec![(m + self.source(), 1)], 1)];
        for (e, &(u, v)) in self.arcs().iter().enumerate() {
            let gate = self.gate_types()[u].is_some();
            variables.push(
                IntegerVariable::new(Some(0), Some(if gate { capacity } else { 0 }))
                    .map_err(Self::target_construction)?,
            );
            outgoing[u].push((e, 1));
            rows.push(LinearConstraint::le(vec![(e, 1), (m + u, -1)], 0));
            if gate {
                rows.push(LinearConstraint::le(vec![(e, 1), (m + v, -1)], 0));
                rows.push(LinearConstraint::le(vec![(flow + e, 1), (e, -capacity)], 0));
                balance[v].push((flow + e, 1));
                balance[u].push((flow + e, -1));
            }
        }
        for (u, terms) in outgoing.into_iter().enumerate() {
            match self.gate_types()[u] {
                Some(true) => rows.extend(
                    terms
                        .into_iter()
                        .map(|(e, _)| LinearConstraint::le(vec![(m + u, 1), (e, -1)], 0)),
                ),
                Some(false) => {
                    let mut terms = terms;
                    terms.push((m + u, -1));
                    rows.push(LinearConstraint::ge(terms, 0));
                }
                None => {}
            }
        }
        for (v, terms) in balance.into_iter().enumerate() {
            if v != self.source() {
                rows.push(LinearConstraint::ge(terms, 0));
            }
        }
        let objective = self.arc_weights().iter().copied().enumerate().collect();
        let target = ILP::with_variables(variables, rows, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_arcs: m,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumweightandorgraph_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(MinimumWeightAndOrGraph::new(
                3,
                vec![(0, 1), (0, 2), (1, 2)],
                0,
                vec![Some(true), Some(false), None],
                vec![1, 2, -4],
            ))
        },
    }]
}
#[cfg(test)]
#[path = "../unit_tests/rules/minimumweightandorgraph_ilp.rs"]
mod tests;
