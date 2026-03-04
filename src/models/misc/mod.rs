//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence (maximize common subsequence length)
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling

mod bin_packing;
pub(crate) mod factoring;
pub(crate) mod longest_common_subsequence;
pub(crate) mod paintshop;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use longest_common_subsequence::LongestCommonSubsequence;
pub use paintshop::PaintShop;
