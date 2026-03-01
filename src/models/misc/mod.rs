//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling

mod bin_packing;
pub(crate) mod factoring;
pub(crate) mod paintshop;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use paintshop::PaintShop;
