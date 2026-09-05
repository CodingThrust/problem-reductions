//! Dedicated customized solver backends.
//!
//! Each backend is registered for one exact problem variant. Dispatch is
//! performed by the solver capability registry rather than a downcast chain.

pub(crate) mod closest_vector_problem;
#[cfg(test)]
pub(crate) mod ensemble_computation;
pub(crate) mod fd_subset_search;
pub(crate) mod grouping_by_swapping;
pub(crate) mod minimum_cost_circulation;
pub(crate) mod minimum_decision_tree;
pub(crate) mod minimum_intersection_graph_basis;
pub(crate) mod partial_feedback_edge_set;
pub(crate) mod rooted_tree_arrangement;
pub(crate) mod shortest_common_superstring;
mod solver;
