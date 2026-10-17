//! Numeric variant reductions for MinimumDominatingSet.

use crate::impl_variant_reduction;
use crate::models::graph::MinimumDominatingSet;
use crate::models::Decision;
use crate::rules::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    MinimumDominatingSet,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| MinimumDominatingSet::new(src.graph().clone(), vec![1_i64; src.num_vertices()])
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MinimumDominatingSet<SimpleGraph, One>,
        MinimumDominatingSet<SimpleGraph, i64>,
    >
);

impl_variant_reduction!(
    Decision,
    <MinimumDominatingSet<SimpleGraph, One>> => <MinimumDominatingSet<SimpleGraph, i64>>,
    fields: [num_vertices, num_edges],
    |src| Decision::new(
        ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(src.inner())?.target_problem().clone(),
        *src.bound())
);
type DecisionWeightCast = crate::rules::VariantReductionResult<
    Decision<MinimumDominatingSet<SimpleGraph, One>>,
    Decision<MinimumDominatingSet<SimpleGraph, i64>>,
>;
crate::register_aggregate_reduction!(DecisionWeightCast);
