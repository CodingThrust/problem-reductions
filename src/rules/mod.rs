//! Reduction rules between NP-hard problems.

pub mod analysis;
pub mod cost;
pub mod registry;
pub use cost::{CustomCost, Minimize, MinimizeSteps, PathCostFn};
pub use registry::{ReductionEntry, ReductionOverhead};

pub(crate) mod circuit_spinglass;
pub(crate) mod coloring_qubo;
pub(crate) mod factoring_circuit;
pub(crate) mod graphpartitioning_qubo;
mod graph;
mod kcoloring_casts;
mod knapsack_qubo;
mod ksatisfiability_casts;
pub(crate) mod ksatisfiability_qubo;
pub(crate) mod ksatisfiability_subsetsum;
pub(crate) mod maximumclique_maximumindependentset;
mod maximumindependentset_casts;
mod maximumindependentset_gridgraph;
pub(crate) mod maximumindependentset_maximumclique;
pub(crate) mod maximumindependentset_maximumsetpacking;
mod maximumindependentset_triangular;
pub(crate) mod maximummatching_maximumsetpacking;
mod maximumsetpacking_casts;
pub(crate) mod maximumsetpacking_qubo;
pub(crate) mod minimummultiwaycut_qubo;
pub(crate) mod minimumvertexcover_maximumindependentset;
pub(crate) mod minimumvertexcover_minimumsetcovering;
pub(crate) mod sat_circuitsat;
pub(crate) mod sat_coloring;
pub(crate) mod sat_ksat;
pub(crate) mod sat_maximumindependentset;
pub(crate) mod sat_minimumdominatingset;
mod spinglass_casts;
pub(crate) mod spinglass_maxcut;
pub(crate) mod spinglass_qubo;
#[cfg(test)]
pub(crate) mod test_helpers;
mod traits;
pub(crate) mod travelingsalesman_qubo;

pub mod unitdiskmapping;

#[cfg(feature = "ilp-solver")]
pub(crate) mod binpacking_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod circuit_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod coloring_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod factoring_ilp;
#[cfg(feature = "ilp-solver")]
mod ilp_bool_ilp_i32;
#[cfg(feature = "ilp-solver")]
pub(crate) mod ilp_qubo;
#[cfg(feature = "ilp-solver")]
pub(crate) mod longestcommonsubsequence_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumclique_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximummatching_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumsetpacking_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumdominatingset_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumfeedbackvertexset_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimummultiwaycut_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumsetcovering_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod qubo_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod travelingsalesman_ilp;

pub use graph::{
    NeighborInfo, NeighborTree, ReductionChain, ReductionEdgeInfo, ReductionGraph, ReductionPath,
    ReductionStep, TraversalDirection,
};
pub use traits::{ReduceTo, ReductionAutoCast, ReductionResult};

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(circuit_spinglass::canonical_rule_example_specs());
    specs.extend(coloring_qubo::canonical_rule_example_specs());
    specs.extend(factoring_circuit::canonical_rule_example_specs());
    specs.extend(graphpartitioning_qubo::canonical_rule_example_specs());
    specs.extend(knapsack_qubo::canonical_rule_example_specs());
    specs.extend(ksatisfiability_qubo::canonical_rule_example_specs());
    specs.extend(ksatisfiability_subsetsum::canonical_rule_example_specs());
    specs.extend(maximumclique_maximumindependentset::canonical_rule_example_specs());
    specs.extend(maximumindependentset_maximumclique::canonical_rule_example_specs());
    specs.extend(maximumindependentset_maximumsetpacking::canonical_rule_example_specs());
    specs.extend(maximummatching_maximumsetpacking::canonical_rule_example_specs());
    specs.extend(maximumsetpacking_qubo::canonical_rule_example_specs());
    specs.extend(minimummultiwaycut_qubo::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_maximumindependentset::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_minimumsetcovering::canonical_rule_example_specs());
    specs.extend(sat_circuitsat::canonical_rule_example_specs());
    specs.extend(sat_coloring::canonical_rule_example_specs());
    specs.extend(sat_ksat::canonical_rule_example_specs());
    specs.extend(sat_maximumindependentset::canonical_rule_example_specs());
    specs.extend(sat_minimumdominatingset::canonical_rule_example_specs());
    specs.extend(spinglass_maxcut::canonical_rule_example_specs());
    specs.extend(spinglass_qubo::canonical_rule_example_specs());
    specs.extend(travelingsalesman_qubo::canonical_rule_example_specs());
    #[cfg(feature = "ilp-solver")]
    {
        specs.extend(binpacking_ilp::canonical_rule_example_specs());
        specs.extend(circuit_ilp::canonical_rule_example_specs());
        specs.extend(coloring_ilp::canonical_rule_example_specs());
        specs.extend(factoring_ilp::canonical_rule_example_specs());
        specs.extend(ilp_qubo::canonical_rule_example_specs());
        specs.extend(longestcommonsubsequence_ilp::canonical_rule_example_specs());
        specs.extend(maximumclique_ilp::canonical_rule_example_specs());
        specs.extend(maximummatching_ilp::canonical_rule_example_specs());
        specs.extend(minimummultiwaycut_ilp::canonical_rule_example_specs());
        specs.extend(maximumsetpacking_ilp::canonical_rule_example_specs());
        specs.extend(minimumdominatingset_ilp::canonical_rule_example_specs());
        specs.extend(minimumfeedbackvertexset_ilp::canonical_rule_example_specs());
        specs.extend(minimumsetcovering_ilp::canonical_rule_example_specs());
        specs.extend(qubo_ilp::canonical_rule_example_specs());
        specs.extend(travelingsalesman_ilp::canonical_rule_example_specs());
    }
    specs
}

/// Generates a variant-cast `ReduceTo` impl with `#[reduction]` registration.
///
/// Variant casts convert a problem from one variant to another (e.g.,
/// `MIS<KingsSubgraph, i32>` -> `MIS<UnitDiskGraph, i32>`). The solution
/// mapping is identity -- vertex/element indices are preserved.
///
/// The problem name is specified once, followed by `<SourceParams> => <TargetParams>`.
/// This works with any number of type parameters.
///
/// # Example
///
/// ```text
/// impl_variant_reduction!(
///     MaximumIndependentSet,
///     <KingsSubgraph, i32> => <UnitDiskGraph, i32>,
///     fields: [num_vertices, num_edges],
///     |src| MaximumIndependentSet::new(
///         src.graph().cast_to_parent(), src.weights())
/// );
/// ```
#[macro_export]
macro_rules! impl_variant_reduction {
    ($problem:ident,
     < $($src_param:ty),+ > => < $($dst_param:ty),+ >,
     fields: [$($field:ident),+],
     |$src:ident| $body:expr) => {
        #[$crate::reduction(
            overhead = {
                $crate::rules::registry::ReductionOverhead::identity(
                    &[$(stringify!($field)),+]
                )
            }
        )]
        impl $crate::rules::ReduceTo<$problem<$($dst_param),+>>
            for $problem<$($src_param),+>
        {
            type Result = $crate::rules::ReductionAutoCast<
                $problem<$($src_param),+>,
                $problem<$($dst_param),+>,
            >;
            fn reduce_to(&self) -> Self::Result {
                let $src = self;
                $crate::rules::ReductionAutoCast::new($body)
            }
        }
    };
}
