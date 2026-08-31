//! Dedicated customized solver backends.
//!
//! Each backend is registered for one exact problem variant. Dispatch is
//! performed by the solver capability registry rather than a downcast chain.

pub(crate) mod closest_vector_problem;
pub(crate) mod fd_subset_search;
pub(crate) mod partial_feedback_edge_set;
pub(crate) mod rooted_tree_arrangement;
mod solver;
