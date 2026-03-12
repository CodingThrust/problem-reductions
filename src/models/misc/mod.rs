//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`Minesweeper`]: Minesweeper Consistency (determine valid mine assignment)
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling

mod bin_packing;
pub(crate) mod factoring;
mod knapsack;
mod minesweeper;
pub(crate) mod paintshop;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use knapsack::Knapsack;
pub use minesweeper::Minesweeper;
pub use paintshop::PaintShop;
