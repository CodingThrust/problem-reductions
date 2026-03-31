//! Set-based problems.
//!
//! This module contains NP-hard problems based on set operations:
//! - [`ConsecutiveSets`]: Consecutive arrangement of subset elements in a string
//! - [`ExactCoverBy3Sets`]: Exact cover by 3-element subsets (X3C)
//! - [`ComparativeContainment`]: Compare containment-weight sums for two set families
//! - [`IntegerKnapsack`]: Maximize value with integer multiplicities subject to capacity
//! - [`MaximumSetPacking`]: Maximum weight set packing
//! - [`MinimumHittingSet`]: Minimum-size universe subset hitting every set
//! - [`MinimumSetCovering`]: Minimum weight set cover
//! - [`PrimeAttributeName`]: Determine if an attribute belongs to any candidate key
//! - [`RootedTreeStorageAssignment`]: Extend subsets to directed tree paths within a total-cost bound
//! - [`SetBasis`]: Minimum-cardinality basis generating all sets by union
//! - [`SetSplitting`]: 2-color universe so every specified subset is non-monochromatic
//! - [`ThreeDimensionalMatching`]: Perfect matching in a tripartite 3-uniform hypergraph
//! - [`ThreeMatroidIntersection`]: Common independent set of size K in three partition matroids
//! - [`TwoDimensionalConsecutiveSets`]: 2D consecutive arrangement of subset elements
//! - [`MinimumCardinalityKey`]: Smallest attribute set that uniquely identifies tuples

pub(crate) mod comparative_containment;
pub(crate) mod consecutive_sets;
pub(crate) mod exact_cover_by_3_sets;
pub(crate) mod integer_knapsack;
pub(crate) mod maximum_set_packing;
pub(crate) mod minimum_cardinality_key;
pub(crate) mod minimum_hitting_set;
pub(crate) mod minimum_set_covering;
pub(crate) mod prime_attribute_name;
pub(crate) mod rooted_tree_storage_assignment;
pub(crate) mod set_basis;
pub(crate) mod set_splitting;
pub(crate) mod three_dimensional_matching;
pub(crate) mod three_matroid_intersection;
pub(crate) mod two_dimensional_consecutive_sets;

pub use comparative_containment::ComparativeContainment;
pub use consecutive_sets::ConsecutiveSets;
pub use exact_cover_by_3_sets::ExactCoverBy3Sets;
pub use integer_knapsack::IntegerKnapsack;
pub use maximum_set_packing::MaximumSetPacking;
pub use minimum_cardinality_key::MinimumCardinalityKey;
pub use minimum_hitting_set::MinimumHittingSet;
pub use minimum_set_covering::MinimumSetCovering;
pub use prime_attribute_name::PrimeAttributeName;
pub use rooted_tree_storage_assignment::RootedTreeStorageAssignment;
pub use set_basis::SetBasis;
pub use set_splitting::SetSplitting;
pub use three_dimensional_matching::ThreeDimensionalMatching;
pub use three_matroid_intersection::ThreeMatroidIntersection;
pub use two_dimensional_consecutive_sets::TwoDimensionalConsecutiveSets;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(comparative_containment::canonical_model_example_specs());
    specs.extend(consecutive_sets::canonical_model_example_specs());
    specs.extend(exact_cover_by_3_sets::canonical_model_example_specs());
    specs.extend(integer_knapsack::canonical_model_example_specs());
    specs.extend(maximum_set_packing::canonical_model_example_specs());
    specs.extend(minimum_cardinality_key::canonical_model_example_specs());
    specs.extend(minimum_hitting_set::canonical_model_example_specs());
    specs.extend(minimum_set_covering::canonical_model_example_specs());
    specs.extend(prime_attribute_name::canonical_model_example_specs());
    specs.extend(rooted_tree_storage_assignment::canonical_model_example_specs());
    specs.extend(set_basis::canonical_model_example_specs());
    specs.extend(set_splitting::canonical_model_example_specs());
    specs.extend(three_dimensional_matching::canonical_model_example_specs());
    specs.extend(three_matroid_intersection::canonical_model_example_specs());
    specs.extend(two_dimensional_consecutive_sets::canonical_model_example_specs());
    specs
}
