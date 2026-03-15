//! Set-based problems.
//!
//! This module contains NP-hard problems based on set operations:
//! - [`MinimumSetCovering`]: Minimum weight set cover
//! - [`MaximumSetPacking`]: Maximum weight set packing
//! - [`ExactCoverBy3Sets`]: Exact cover by 3-element subsets (X3C)

pub(crate) mod exact_cover_by_3_sets;
pub(crate) mod maximum_set_packing;
pub(crate) mod minimum_set_covering;

pub use exact_cover_by_3_sets::ExactCoverBy3Sets;
pub use maximum_set_packing::MaximumSetPacking;
pub use minimum_set_covering::MinimumSetCovering;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(exact_cover_by_3_sets::canonical_model_example_specs());
    specs.extend(maximum_set_packing::canonical_model_example_specs());
    specs.extend(minimum_set_covering::canonical_model_example_specs());
    specs
}
