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
//! ```
//! use problemreductions::models::graph::{GraphProblem, GraphConstraint};
//! use problemreductions::types::EnergyMode;
//! use problemreductions::registry::GraphSubcategory;
//! use problemreductions::topology::SimpleGraph;
//!
//! // Define a new graph problem constraint
//! #[derive(Clone)]
//! struct MyConstraint;
//!
//! impl GraphConstraint for MyConstraint {
//!     const NAME: &'static str = "My Problem";
//!     const DESCRIPTION: &'static str = "A custom graph problem";
//!     const ENERGY_MODE: EnergyMode = EnergyMode::LargerSizeIsBetter;
//!     const SUBCATEGORY: GraphSubcategory = GraphSubcategory::Independent;
//!
//!     fn edge_constraint_spec() -> [bool; 4] {
//!         [true, true, true, false]
//!     }
//! }
//!
//! // Create a type alias for convenience (defaults to SimpleGraph and i32)
//! type MyProblem = GraphProblem<MyConstraint, SimpleGraph, i32>;
//!
//! // Use it
//! let problem = MyProblem::new(3, vec![(0, 1)]);
//! assert_eq!(problem.num_vertices(), 3);
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
