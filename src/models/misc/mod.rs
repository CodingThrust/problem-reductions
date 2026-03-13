//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`SequencingWithReleaseTimesAndDeadlines`]: Single-machine scheduling feasibility
//! - [`SubsetSum`]: Find a subset summing to exactly a target value

mod bin_packing;
pub(crate) mod factoring;
mod knapsack;
mod longest_common_subsequence;
pub(crate) mod paintshop;
mod sequencing_with_release_times_and_deadlines;
mod subset_sum;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use knapsack::Knapsack;
pub use longest_common_subsequence::LongestCommonSubsequence;
pub use paintshop::PaintShop;
pub use sequencing_with_release_times_and_deadlines::SequencingWithReleaseTimesAndDeadlines;
pub use subset_sum::SubsetSum;
