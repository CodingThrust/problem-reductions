//! Natural-edge reductions via graph subtype relaxation.
//!
//! Natural reductions (e.g., a problem on `Triangular` solved as `SimpleGraph`)
//! are handled automatically by [`ReductionGraph::resolve_path`], which inserts
//! `NaturalCast` steps based on the registered graph/weight subtype hierarchies.
//!
//! No explicit `ReduceTo` impls are needed for natural edges — the resolver
//! computes them from `GraphSubtypeEntry` and `WeightSubtypeEntry` registrations.

#[cfg(test)]
#[path = "../unit_tests/rules/natural.rs"]
mod tests;
