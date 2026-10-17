//! Numeric variant reductions for MinimumTardinessSequencing.

use crate::impl_variant_reduction;
use crate::models::misc::MinimumTardinessSequencing;
use crate::types::One;

impl_variant_reduction!(
    MinimumTardinessSequencing,
    <One> => <i64>,
    fields: [num_tasks, num_precedences, max_processing_time_bits],
    |src| MinimumTardinessSequencing::with_lengths(vec![1_i64; src.num_tasks()], src.deadlines().to_vec(), src.precedences().to_vec())
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MinimumTardinessSequencing<One>,
        MinimumTardinessSequencing<i64>,
    >
);
