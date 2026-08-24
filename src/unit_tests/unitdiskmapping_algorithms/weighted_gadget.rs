//! Boundary-optimum verification for weighted gadgets across grid topologies.

use super::alpha_tensor::alpha_tensor;
use super::common::{ksg_edges, triangular_edges};
use crate::rules::unitdiskmapping::ksg::{
    KsgReflectedGadget, KsgRotatedGadget, Mirror, WeightedKsgBranch, WeightedKsgBranchFix,
    WeightedKsgBranchFixB, WeightedKsgCross, WeightedKsgDanglingLeg, WeightedKsgEndTurn,
    WeightedKsgTCon, WeightedKsgTrivialTurn, WeightedKsgTurn, WeightedKsgWTurn,
};
use crate::rules::unitdiskmapping::triangular::{
    WeightedTriBranch, WeightedTriBranchFix, WeightedTriBranchFixB, WeightedTriCross,
    WeightedTriEndTurn, WeightedTriTConDown, WeightedTriTConLeft, WeightedTriTConUp,
    WeightedTriTrivialTurnLeft, WeightedTriTrivialTurnRight, WeightedTriTurn, WeightedTriWTurn,
    WeightedTriangularGadget,
};
use crate::rules::unitdiskmapping::Pattern;

fn assert_weighted_boundary_equivalent(
    source: (&[(usize, usize)], &mut [i64], &[usize]),
    mapped: (&[(usize, usize)], &mut [i64], &[usize]),
    mis_overhead: i64,
    name: &str,
) {
    let (source_edges, source_weights, source_pins) = source;
    let (mapped_edges, mapped_weights, mapped_pins) = mapped;
    for &pin in source_pins {
        source_weights[pin] = source_weights[pin]
            .checked_sub(1)
            .expect("source pin weight adjustment must fit in i64");
    }
    for &pin in mapped_pins {
        mapped_weights[pin] = mapped_weights[pin]
            .checked_sub(1)
            .expect("mapped pin weight adjustment must fit in i64");
    }

    let source = alpha_tensor(
        source_weights.len(),
        source_edges,
        source_weights,
        source_pins,
    );
    let mapped = alpha_tensor(
        mapped_weights.len(),
        mapped_edges,
        mapped_weights,
        mapped_pins,
    );
    let source_max = source
        .iter()
        .copied()
        .filter(|&value| value != i64::MIN)
        .max()
        .expect("source boundary tensor must contain a feasible pin configuration");
    let mapped_max = mapped
        .iter()
        .copied()
        .filter(|&value| value != i64::MIN)
        .max()
        .expect("mapped boundary tensor must contain a feasible pin configuration");

    assert_eq!(
        source
            .iter()
            .map(|&value| value == source_max)
            .collect::<Vec<_>>(),
        mapped
            .iter()
            .map(|&value| value == mapped_max)
            .collect::<Vec<_>>(),
        "{name}: maximizing pin configurations differ; source={source:?}, mapped={mapped:?}"
    );
    assert_eq!(
        mapped_max
            .checked_sub(source_max)
            .expect("boundary-optimum difference must fit in i64"),
        mis_overhead,
        "{name}: weighted gadget MIS overhead differs"
    );
}

fn assert_weighted_ksg_gadget_equivalent<G: Pattern>(gadget: G, name: &str) {
    let (source_locations, source_edges, source_pins) = gadget.source_graph();
    let (mapped_locations, mapped_pins) = gadget.mapped_graph();
    let mut source_weights = gadget.source_weights();
    let mut mapped_weights = gadget.mapped_weights();
    assert_eq!(source_locations.len(), source_weights.len());
    assert_eq!(mapped_locations.len(), mapped_weights.len());
    assert_weighted_boundary_equivalent(
        (&source_edges, &mut source_weights, &source_pins),
        (
            &ksg_edges(&mapped_locations),
            &mut mapped_weights,
            &mapped_pins,
        ),
        gadget.mis_overhead(),
        name,
    );
}

fn assert_weighted_triangular_gadget_equivalent<G: WeightedTriangularGadget>(
    gadget: G,
    name: &str,
) {
    let (source_locations, source_edges, source_pins) = gadget.source_graph();
    let (mapped_locations, mapped_pins) = gadget.mapped_graph();
    let mut source_weights = gadget.source_weights();
    let mut mapped_weights = gadget.mapped_weights();
    assert_eq!(source_locations.len(), source_weights.len());
    assert_eq!(mapped_locations.len(), mapped_weights.len());

    assert_weighted_boundary_equivalent(
        (&source_edges, &mut source_weights, &source_pins),
        (
            &triangular_edges(&mapped_locations, 1.1),
            &mut mapped_weights,
            &mapped_pins,
        ),
        gadget.mis_overhead(),
        name,
    );
}

#[test]
fn test_weighted_ksg_crossing_gadget_equivalence() {
    assert_weighted_ksg_gadget_equivalent(WeightedKsgCross::<false>, "WeightedKsgCross<false>");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgTurn, "WeightedKsgTurn");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgWTurn, "WeightedKsgWTurn");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgBranch, "WeightedKsgBranch");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgBranchFix, "WeightedKsgBranchFix");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgTCon, "WeightedKsgTCon");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgTrivialTurn, "WeightedKsgTrivialTurn");
    assert_weighted_ksg_gadget_equivalent(
        KsgRotatedGadget::new(WeightedKsgTCon, 1),
        "RotatedWeightedKsgTCon",
    );
    assert_weighted_ksg_gadget_equivalent(
        KsgReflectedGadget::new(WeightedKsgCross::<true>, Mirror::Y),
        "ReflectedWeightedKsgCross<true>",
    );
    assert_weighted_ksg_gadget_equivalent(
        KsgReflectedGadget::new(WeightedKsgTrivialTurn, Mirror::Y),
        "ReflectedWeightedKsgTrivialTurn",
    );
    assert_weighted_ksg_gadget_equivalent(WeightedKsgBranchFixB, "WeightedKsgBranchFixB");
    assert_weighted_ksg_gadget_equivalent(WeightedKsgEndTurn, "WeightedKsgEndTurn");
    assert_weighted_ksg_gadget_equivalent(
        KsgReflectedGadget::new(KsgRotatedGadget::new(WeightedKsgTCon, 1), Mirror::Y),
        "ReflectedRotatedWeightedKsgTCon",
    );
}

#[test]
fn test_weighted_ksg_simplifier_gadget_equivalence() {
    assert_weighted_ksg_gadget_equivalent(WeightedKsgDanglingLeg, "WeightedKsgDanglingLeg");
    assert_weighted_ksg_gadget_equivalent(
        KsgRotatedGadget::new(WeightedKsgDanglingLeg, 1),
        "WeightedKsgDanglingLegRot1",
    );
    assert_weighted_ksg_gadget_equivalent(
        KsgRotatedGadget::new(WeightedKsgDanglingLeg, 2),
        "WeightedKsgDanglingLegRot2",
    );
    assert_weighted_ksg_gadget_equivalent(
        KsgRotatedGadget::new(WeightedKsgDanglingLeg, 3),
        "WeightedKsgDanglingLegRot3",
    );
    assert_weighted_ksg_gadget_equivalent(
        KsgReflectedGadget::new(WeightedKsgDanglingLeg, Mirror::X),
        "WeightedKsgDanglingLegMirrorX",
    );
    assert_weighted_ksg_gadget_equivalent(
        KsgReflectedGadget::new(WeightedKsgDanglingLeg, Mirror::Y),
        "WeightedKsgDanglingLegMirrorY",
    );
}

#[test]
fn test_weighted_triangular_crossing_gadget_equivalence() {
    assert_weighted_triangular_gadget_equivalent(
        WeightedTriCross::<false>,
        "WeightedTriCross<false>",
    );
    assert_weighted_triangular_gadget_equivalent(
        WeightedTriCross::<true>,
        "WeightedTriCross<true>",
    );
    assert_weighted_triangular_gadget_equivalent(WeightedTriTConLeft, "WeightedTriTConLeft");
    assert_weighted_triangular_gadget_equivalent(WeightedTriTConUp, "WeightedTriTConUp");
    assert_weighted_triangular_gadget_equivalent(WeightedTriTConDown, "WeightedTriTConDown");
    assert_weighted_triangular_gadget_equivalent(
        WeightedTriTrivialTurnLeft,
        "WeightedTriTrivialTurnLeft",
    );
    assert_weighted_triangular_gadget_equivalent(
        WeightedTriTrivialTurnRight,
        "WeightedTriTrivialTurnRight",
    );
    assert_weighted_triangular_gadget_equivalent(WeightedTriEndTurn, "WeightedTriEndTurn");
    assert_weighted_triangular_gadget_equivalent(WeightedTriTurn, "WeightedTriTurn");
    assert_weighted_triangular_gadget_equivalent(WeightedTriWTurn, "WeightedTriWTurn");
    assert_weighted_triangular_gadget_equivalent(WeightedTriBranchFix, "WeightedTriBranchFix");
    assert_weighted_triangular_gadget_equivalent(WeightedTriBranchFixB, "WeightedTriBranchFixB");
    assert_weighted_triangular_gadget_equivalent(WeightedTriBranch, "WeightedTriBranch");
}
