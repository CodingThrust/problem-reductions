//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`FlowShopScheduling`]: Flow Shop Scheduling (meet deadline on m processors)
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence
//! - [`MinimumTardinessSequencing`]: Minimize tardy tasks in single-machine scheduling
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`ShortestCommonSupersequence`]: Find a common supersequence of bounded length
//! - [`SubsetSum`]: Find a subset summing to exactly a target value

mod bin_packing;
pub(crate) mod factoring;
mod flow_shop_scheduling;
mod knapsack;
mod longest_common_subsequence;
mod minimum_tardiness_sequencing;
pub(crate) mod paintshop;
pub(crate) mod shortest_common_supersequence;
mod subset_sum;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use flow_shop_scheduling::FlowShopScheduling;
pub use knapsack::Knapsack;
pub use longest_common_subsequence::LongestCommonSubsequence;
pub use minimum_tardiness_sequencing::MinimumTardinessSequencing;
pub use paintshop::PaintShop;
pub use shortest_common_supersequence::ShortestCommonSupersequence;
pub use subset_sum::SubsetSum;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(factoring::canonical_model_example_specs());
    specs.extend(longest_common_subsequence::canonical_model_example_specs());
    specs.extend(paintshop::canonical_model_example_specs());
    specs.extend(shortest_common_supersequence::canonical_model_example_specs());
    specs.extend(minimum_tardiness_sequencing::canonical_model_example_specs());
    specs
}
