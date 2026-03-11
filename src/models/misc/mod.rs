//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`Partition`]: Partition a multiset into two equal-sum subsets
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling

mod bin_packing;
pub(crate) mod factoring;
mod knapsack;
pub(crate) mod paintshop;
mod partition;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use knapsack::Knapsack;
pub use paintshop::PaintShop;
pub use partition::Partition;
