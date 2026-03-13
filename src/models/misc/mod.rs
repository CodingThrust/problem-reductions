//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`ResourceConstrainedScheduling`]: Schedule unit-length tasks on processors with resource constraints
//! - [`SubsetSum`]: Find a subset summing to exactly a target value

mod bin_packing;
pub(crate) mod factoring;
mod knapsack;
pub(crate) mod paintshop;
mod resource_constrained_scheduling;
mod subset_sum;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use knapsack::Knapsack;
pub use paintshop::PaintShop;
pub use resource_constrained_scheduling::ResourceConstrainedScheduling;
pub use subset_sum::SubsetSum;
