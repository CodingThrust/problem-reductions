//! Graph-based optimization problems.
//!
//! This module contains NP-hard problems defined on graphs:
//! - [`IndependentSet`]: Maximum weight independent set
//! - [`MaximalIS`]: Maximal independent set
//! - [`VertexCovering`]: Minimum weight vertex cover
//! - [`DominatingSet`]: Minimum dominating set
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`Coloring`]: K-vertex coloring
//! - [`Matching`]: Maximum weight matching
//!
//! ## Using the Template
//!
//! New graph problems can be defined using the [`GraphProblem`] template by
//! implementing the [`GraphConstraint`] trait:
//!
//! ```rust,ignore
//! use problemreductions::models::graph::{GraphProblem, GraphConstraint};
//!
//! // Define a new graph problem constraint
//! struct MyConstraint;
//! impl GraphConstraint for MyConstraint {
//!     // ... implement required methods
//! }
//!
//! // Create a type alias for convenience
//! type MyProblem<W = i32> = GraphProblem<MyConstraint, W>;
//! ```

mod coloring;
mod dominating_set;
mod independent_set;
mod matching;
mod max_cut;
mod maximal_is;
pub mod template;
mod vertex_covering;

pub use coloring::{is_valid_coloring, Coloring};
pub use dominating_set::{is_dominating_set, DominatingSet};
pub use independent_set::{is_independent_set, IndependentSet};
pub use matching::{is_matching, Matching};
pub use max_cut::{cut_size, MaxCut};
pub use maximal_is::{is_maximal_independent_set, MaximalIS};
pub use template::{
    CliqueConstraint, CliqueT, GraphConstraint, GraphProblem, IndependentSetConstraint,
    IndependentSetT, VertexCoverConstraint, VertexCoverT,
};
pub use vertex_covering::{is_vertex_cover, VertexCovering};
