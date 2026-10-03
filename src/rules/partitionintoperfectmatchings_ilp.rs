//! Group labels with exact same-group edge indicators and degree one.
use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::PartitionIntoPerfectMatchings;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionPartitionIntoPerfectMatchingsToILP {
    target: ILP<i64, i64, Bounded>,
    num_vertices: usize,
}
impl ReductionResult for ReductionPartitionIntoPerfectMatchingsToILP {
    type Source = PartitionIntoPerfectMatchings<SimpleGraph>;
    type Target = ILP<i64, i64, Bounded>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |v| v.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        crate::rules::ilp_helpers::decode_usize_values(&solution[..self.num_vertices])
    }
}
#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionPartitionIntoPerfectMatchingsToILP {}
#[crate::reduction(transform = upper_bound {
    num_vars = "num_vertices + 2 * num_edges",
    num_constraints = "num_vertices + 4 * num_edges",
    num_nonzeros = "16 * num_edges",
    max_constraint_magnitude_bits = "num_matchings",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for PartitionIntoPerfectMatchings<SimpleGraph> {
    type Result = ReductionPartitionIntoPerfectMatchingsToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let k = Self::exact_i64(self.num_matchings(), "bounding matching-group labels")?;
        // The source counts distinct other neighbors, ignoring loops and multiplicity.
        let mut edges: Vec<_> = self
            .graph()
            .edges()
            .into_iter()
            .filter(|(u, v)| u != v)
            .map(|(u, v)| (u.min(v), u.max(v)))
            .collect();
        edges.sort_unstable();
        edges.dedup();
        let m = edges.len();
        let count = m
            .checked_mul(2)
            .and_then(|v| v.checked_add(n))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting matching-partition variables",
                )
            })?;
        let mut variables =
            vec![IntegerVariable::new(Some(0), Some(k - 1)).map_err(Self::target_construction)?; n];
        variables.resize(count, IntegerVariable::binary());
        let mut degree = vec![Vec::new(); n];
        let mut rows = Vec::new();
        for (i, &(u, v)) in edges.iter().enumerate() {
            let same = n + i;
            let direction = n + m + i;
            degree[u].push((same, 1));
            degree[v].push((same, 1));
            rows.push(LinearConstraint::le(
                vec![(v, 1), (u, -1), (same, k - 1)],
                k - 1,
            ));
            rows.push(LinearConstraint::le(
                vec![(u, 1), (v, -1), (same, k - 1)],
                k - 1,
            ));
            rows.push(LinearConstraint::ge(
                vec![(v, 1), (u, -1), (direction, -k), (same, 1)],
                1 - k,
            ));
            rows.push(LinearConstraint::ge(
                vec![(u, 1), (v, -1), (direction, k), (same, 1)],
                1,
            ));
        }
        rows.extend(
            degree
                .into_iter()
                .map(|terms| LinearConstraint::eq(terms, 1)),
        );
        let target = ILP::with_variables(variables, rows, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_vertices: n,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partitionintoperfectmatchings_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(
                PartitionIntoPerfectMatchings::new(
                    SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
                    2,
                ),
            )
        },
    }]
}
#[cfg(test)]
#[path = "../unit_tests/rules/partitionintoperfectmatchings_ilp.rs"]
mod tests;
