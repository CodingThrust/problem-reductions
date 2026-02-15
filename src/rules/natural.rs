//! Natural-edge reductions via graph subtype relaxation.
//!
//! Natural reductions (e.g., a problem on `Triangular` solved as `SimpleGraph`)
//! are handled automatically by [`ReductionGraph::resolve_path`], which inserts
//! `NaturalCast` steps based on the registered variant subtype hierarchies.
//!
//! No explicit `ReduceTo` impls are needed for natural edges — the resolver
//! computes them from `VariantTypeEntry` registrations.

#[cfg(test)]
#[path = "../unit_tests/rules/natural.rs"]
mod tests;
