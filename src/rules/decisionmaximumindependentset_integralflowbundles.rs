//! Unit-weight decision independent set to integral flow with bundles.
//!
//! Sahni (1974), Theorem 2.2(b)(iv), uses a cardinality threshold and repeated
//! flow queries to recover an optimum. This direct rule preserves one decision
//! query. An auxiliary binary path shifts the threshold to remain positive,
//! including empty graphs and nonpositive source bounds.

use crate::models::decision::Decision;
use crate::models::graph::{IntegralFlowBundles, MaximumIndependentSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of a unit-weight independent-set decision reduction.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMISToIFB {
    target: IntegralFlowBundles,
    num_source_vertices: usize,
}

impl ReductionResult for ReductionDecisionMISToIFB {
    type Source = Decision<MaximumIndependentSet<SimpleGraph, One>>;
    type Target = IntegralFlowBundles;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let feasible =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !feasible.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target flow must satisfy conservation, bundle capacities, and the requirement",
            ));
        }
        Ok((0..self.num_source_vertices)
            .map(|i| target_solution[2 * i + 1] == 1)
            .collect())
    }
}

/// Check all graph and bundle allocation counts before constructing the network.
fn flow_dimensions(
    n: usize,
    m: usize,
) -> Result<(usize, usize, usize), crate::rules::ReductionError> {
    n.checked_add(3)
        .zip(n.checked_add(1).and_then(|count| count.checked_mul(2)))
        .zip(n.checked_add(m).and_then(|count| count.checked_add(1)))
        .map(|((vertices, arcs), bundles)| (vertices, arcs, bundles))
        .ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                Decision<MaximumIndependentSet<SimpleGraph, One>>,
                IntegralFlowBundles,
            >("counting independent-set flow paths and bundles")
        })
}

/// Thresholds outside [0,n] have constant truth values for cardinality decisions.
/// Map them to 0 and n+1, then account for the auxiliary unit of flow.
fn flow_requirement(n: usize, bound: i64) -> Result<i64, crate::rules::ReductionError> {
    let n = <Decision<MaximumIndependentSet<SimpleGraph, One>> as ReduceTo<
        IntegralFlowBundles,
    >>::exact_i64(n, "converting the independent-set vertex count")?;
    let maximum_requirement = n.checked_add(2).ok_or_else(|| {
        crate::rules::ReductionError::integer_overflow::<
            Decision<MaximumIndependentSet<SimpleGraph, One>>,
            IntegralFlowBundles,
        >("shifting the independent-set decision threshold")
    })?;
    Ok(bound.clamp(0, maximum_requirement - 1) + 1)
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices + 3",
        num_arcs = "2 * num_vertices + 2",
        num_bundles = "num_edges + num_vertices + 1",
    }
)]
impl ReduceTo<IntegralFlowBundles> for Decision<MaximumIndependentSet<SimpleGraph, One>> {
    type Result = ReductionDecisionMISToIFB;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let graph = self.inner().graph();
        let n = graph.num_vertices();
        let (num_vertices, num_arcs, num_bundles) = flow_dimensions(n, graph.num_edges())?;
        let requirement = flow_requirement(n, *self.bound())?;
        let sink = n + 2;
        let mut arcs = Vec::with_capacity(num_arcs);
        // The final path is auxiliary; the other n paths represent source vertices.
        for i in 0..=n {
            arcs.push((0, i + 1));
            arcs.push((i + 1, sink));
        }
        let mut bundles = Vec::with_capacity(num_bundles);
        let mut capacities = Vec::with_capacity(num_bundles);
        // Mixing incoming/outgoing arcs keeps the two bundle members distinct
        // even for a self-loop. Conservation makes its constraint 2*f_i <= 1.
        for (u, v) in graph.edges() {
            bundles.push(vec![2 * u, 2 * v + 1]);
            capacities.push(1);
        }
        for i in 0..=n {
            bundles.push(vec![2 * i, 2 * i + 1]);
            capacities.push(2);
        }
        let target = IntegralFlowBundles::new(
            crate::topology::DirectedGraph::new(num_vertices, arcs),
            0,
            sink,
            bundles,
            capacities,
            requirement,
        );
        Ok(ReductionDecisionMISToIFB {
            target,
            num_source_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionmaximumindependentset_to_integralflowbundles",
        build: || {
            let source = Decision::new(
                MaximumIndependentSet::new(SimpleGraph::path(3), vec![One; 3]),
                2,
            );
            let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source)
                .expect("canonical reduction must succeed");
            let target_witness = BruteForce::new()
                .solve(reduction.target_problem())
                .expect("canonical target evaluation must succeed")
                .expect("the path has an independent set of size two");
            let source_witness = reduction.extract_solution(&target_witness).unwrap();
            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::json!(source_witness),
                    target_config: serde_json::json!(target_witness),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionmaximumindependentset_integralflowbundles.rs"]
mod tests;
