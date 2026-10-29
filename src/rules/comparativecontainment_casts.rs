//! Numeric variant reductions for ComparativeContainment.

use crate::impl_variant_reduction;
use crate::models::set::ComparativeContainment;
use crate::rules::{ReductionError, weight_casts::exact_float_weights};
use crate::types::One;

impl_variant_reduction!(
    ComparativeContainment,
    <One> => <i64>,
    fields: [universe_size, num_r_sets, num_s_sets],
    |src| ComparativeContainment::new(
        src.universe_size(), src.r_sets().to_vec(), src.s_sets().to_vec(),
    ).map_err(ReductionError::construction::<ComparativeContainment<One>, ComparativeContainment<i64>>)?
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<ComparativeContainment<One>, ComparativeContainment<i64>>
);

impl_variant_reduction!(
    ComparativeContainment,
    <i64> => <f64>,
    fields: [universe_size, num_r_sets, num_s_sets],
    |src| ComparativeContainment::with_weights(
        src.universe_size(), src.r_sets().to_vec(), src.s_sets().to_vec(),
        exact_float_weights::<ComparativeContainment<i64>, ComparativeContainment<f64>>(src.r_weights())?,
        exact_float_weights::<ComparativeContainment<i64>, ComparativeContainment<f64>>(src.s_weights())?,
    ).map_err(ReductionError::construction::<ComparativeContainment<i64>, ComparativeContainment<f64>>)?
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<ComparativeContainment<i64>, ComparativeContainment<f64>>
);
