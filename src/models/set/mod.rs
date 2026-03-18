//! Set-based problems.
//!
//! This module contains NP-hard problems based on set operations:
//! - [`MinimumSetCovering`]: Minimum weight set cover
//! - [`MaximumSetPacking`]: Maximum weight set packing
//! - [`ExactCoverBy3Sets`]: Exact cover by 3-element subsets (X3C)
//! - [`ComparativeContainment`]: Compare containment-weight sums for two set families

pub(crate) mod comparative_containment;
pub(crate) mod exact_cover_by_3_sets;
pub(crate) mod maximum_set_packing;
pub(crate) mod minimum_cardinality_key;
pub(crate) mod minimum_set_covering;
pub(crate) mod set_basis;

pub use comparative_containment::ComparativeContainment;
pub use exact_cover_by_3_sets::ExactCoverBy3Sets;
pub use maximum_set_packing::MaximumSetPacking;
pub use minimum_cardinality_key::MinimumCardinalityKey;
pub use minimum_set_covering::MinimumSetCovering;
pub use set_basis::SetBasis;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(comparative_containment::canonical_model_example_specs());
    specs.extend(exact_cover_by_3_sets::canonical_model_example_specs());
    specs.extend(maximum_set_packing::canonical_model_example_specs());
    specs.extend(minimum_set_covering::canonical_model_example_specs());
    specs.extend(minimum_cardinality_key::canonical_model_example_specs());
    specs.extend(set_basis::canonical_model_example_specs());
    specs
}
