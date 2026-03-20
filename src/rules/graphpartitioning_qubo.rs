//! Reduction from GraphPartitioning to QUBO.
//!
//! This file is intentionally scaffolded for TDD. The concrete QUBO
//! construction is added in the implementation task.

use crate::models::algebraic::QUBO;
use crate::models::graph::GraphPartitioning;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing GraphPartitioning to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionGraphPartitioningToQUBO {
    target: QUBO<f64>,
}

impl ReductionResult for ReductionGraphPartitioningToQUBO {
    type Source = GraphPartitioning<SimpleGraph>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = { num_vars = "num_vertices" })]
impl ReduceTo<QUBO<f64>> for GraphPartitioning<SimpleGraph> {
    type Result = ReductionGraphPartitioningToQUBO;

    fn reduce_to(&self) -> Self::Result {
        todo!("GraphPartitioning -> QUBO reduction not implemented yet")
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    Vec::new()
}

#[cfg(test)]
#[path = "../unit_tests/rules/graphpartitioning_qubo.rs"]
mod tests;
