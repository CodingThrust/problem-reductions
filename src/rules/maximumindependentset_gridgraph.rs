//! Reduction from unweighted MaximumIndependentSet on SimpleGraph to KingsSubgraph
//! using the King's Subgraph (KSG) unit disk mapping.
//!
//! Maps an arbitrary graph's MIS problem to an equivalent MIS on a grid graph.

use crate::models::graph::MaximumIndependentSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::unitdiskmapping::ksg;
use crate::topology::{Graph, KingsSubgraph, SimpleGraph};
use crate::types::One;

/// Result of reducing MIS<SimpleGraph, One> to MIS<KingsSubgraph, One>.
#[derive(Debug, Clone)]
pub struct ReductionISSimpleOneToGridOne {
    target: MaximumIndependentSet<KingsSubgraph, One>,
    mapping_result: ksg::MappingResult<ksg::KsgTapeEntry>,
}

impl ReductionResult for ReductionISSimpleOneToGridOne {
    type Source = MaximumIndependentSet<SimpleGraph, One>;
    type Target = MaximumIndependentSet<KingsSubgraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let encoded = crate::config::bits_to_config(target_solution);
        let mapped = self.mapping_result.map_config_back(&encoded)?;
        Ok(crate::config::config_to_bits(&mapped))
    }
}

// At order position i, vertical slot span is <=i and horizontal span
// <=n-1-i. With spacing four, copy lines contain <=4n(n-1)+n cells.
// Only disconnected crossing gadgets increase counts: +2 vertices and
// +10 internal edges, consuming one doubled cell each. There are at most
// n(n-1)/2 doubled cells; all gadget boundary nodes are retained source
// nodes, so no outside edges are added. Other gadgets only decrease counts.
// Copy-line edges total <=4n(n-1)+5(n-1) (five extra corner contacts),
// with at most four extra contacts per pair of crossing lines. Thus final
// V<=5n^2-4n and E<=11n(n-1)+5(n-1)<=11n^2. Positive-coefficient
// relaxations preserve monotonic composition; n=0 is rejected by the mapper.
#[reduction(
    transform = upper_bound {
        num_vertices = "5 * num_vertices^2",
        num_edges = "11 * num_vertices^2",
    }
)]
impl ReduceTo<MaximumIndependentSet<KingsSubgraph, One>>
    for MaximumIndependentSet<SimpleGraph, One>
{
    type Result = ReductionISSimpleOneToGridOne;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();
        let result = ksg::map_unweighted(n, &edges).map_err(|error| {
            error.for_reduction::<Self, MaximumIndependentSet<KingsSubgraph, One>>()
        })?;
        let grid = result.to_kings_subgraph();
        let weights = vec![One; grid.num_vertices()];
        let target = MaximumIndependentSet::new(grid, weights);
        Ok(ReductionISSimpleOneToGridOne {
            target,
            mapping_result: result,
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_gridgraph.rs"]
mod tests;
