//! Reduction from unweighted MaximumIndependentSet on SimpleGraph to TriangularSubgraph
//! using the triangular unit disk mapping.
//!
//! Maps an arbitrary graph's MIS problem to an equivalent weighted MIS on a
//! triangular lattice grid graph.

use crate::models::graph::MaximumIndependentSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::unitdiskmapping::ksg;
use crate::rules::unitdiskmapping::triangular;
use crate::topology::{Graph, SimpleGraph, TriangularSubgraph};
use crate::types::One;

/// Result of reducing MIS<SimpleGraph, One> to MIS<TriangularSubgraph, i64>.
#[derive(Debug, Clone)]
pub struct ReductionISSimpleToTriangular {
    target: MaximumIndependentSet<TriangularSubgraph, i64>,
    mapping_result: ksg::MappingResult<ksg::KsgTapeEntry>,
}

impl ReductionResult for ReductionISSimpleToTriangular {
    type Source = MaximumIndependentSet<SimpleGraph, One>;
    type Target = MaximumIndependentSet<TriangularSubgraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let encoded = crate::config::bits_to_config(target_solution);
        let mapped = triangular::map_config_back(&self.mapping_result, &encoded)?;
        Ok(crate::config::config_to_bits(&mapped))
    }
}

#[reduction(
    size = upper_bound {
        num_vertices = "36 * num_vertices^2 + 36 * num_vertices",
        num_edges = "108 * num_vertices^2 + 108 * num_vertices",
    }
)]
impl ReduceTo<MaximumIndependentSet<TriangularSubgraph, i64>>
    for MaximumIndependentSet<SimpleGraph, One>
{
    type Result = ReductionISSimpleToTriangular;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();
        let mapping_error = |error: crate::rules::ReductionError| {
            error.for_reduction::<Self, MaximumIndependentSet<TriangularSubgraph, i64>>()
        };
        let result = triangular::map_weighted(n, &edges).map_err(&mapping_error)?;
        let weights = triangular::map_unit_weights(&result).map_err(mapping_error)?;
        let grid = result.to_triangular_subgraph();
        let target = MaximumIndependentSet::new(grid, weights);
        Ok(ReductionISSimpleToTriangular {
            target,
            mapping_result: result,
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_triangular.rs"]
mod tests;
