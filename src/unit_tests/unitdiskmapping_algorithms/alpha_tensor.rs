//! Compactified alpha-tensor verification for unweighted KSG gadgets.
//!
//! Topology and weight mode are independent. This currently covers only KSG
//! because an unweighted triangular gadget ruleset has not been implemented.

use super::common::ksg_edges;
use crate::rules::unitdiskmapping::ksg::{
    KsgBranch, KsgBranchFix, KsgBranchFixB, KsgCross, KsgDanglingLeg, KsgEndTurn,
    KsgReflectedGadget, KsgRotatedGadget, KsgTCon, KsgTrivialTurn, KsgTurn, KsgWTurn, Mirror,
};
use crate::rules::unitdiskmapping::Pattern;
use std::collections::HashSet;

fn weighted_mis_with_fixed_pins(
    num_vertices: usize,
    edges: &[(usize, usize)],
    weights: &[i64],
    pins: &[usize],
    pin_config: usize,
) -> i64 {
    let forced_in: HashSet<usize> = pins
        .iter()
        .enumerate()
        .filter_map(|(index, &pin)| ((pin_config >> index) & 1 == 1).then_some(pin))
        .collect();
    let forced_out: HashSet<usize> = pins
        .iter()
        .enumerate()
        .filter_map(|(index, &pin)| ((pin_config >> index) & 1 == 0).then_some(pin))
        .collect();

    if edges
        .iter()
        .any(|(u, v)| forced_in.contains(u) && forced_in.contains(v))
    {
        return i64::MIN;
    }

    let blocked: HashSet<usize> = edges
        .iter()
        .flat_map(|&(u, v)| {
            [
                forced_in.contains(&u).then_some(v),
                forced_in.contains(&v).then_some(u),
            ]
        })
        .flatten()
        .collect();
    let free_vertices: Vec<usize> = (0..num_vertices)
        .filter(|vertex| {
            !forced_in.contains(vertex) && !forced_out.contains(vertex) && !blocked.contains(vertex)
        })
        .collect();
    let subset_count = 1usize
        .checked_shl(u32::try_from(free_vertices.len()).expect("free-vertex count must fit in u32"))
        .expect("gadget must be small enough for exhaustive MIS verification");

    let free_mis = (0..subset_count)
        .filter(|subset| {
            edges.iter().all(|&(u, v)| {
                let u_selected = free_vertices
                    .iter()
                    .position(|&vertex| vertex == u)
                    .is_some_and(|index| (subset >> index) & 1 == 1);
                let v_selected = free_vertices
                    .iter()
                    .position(|&vertex| vertex == v)
                    .is_some_and(|index| (subset >> index) & 1 == 1);
                !u_selected || !v_selected
            })
        })
        .map(|subset| {
            free_vertices
                .iter()
                .enumerate()
                .filter(|(index, _)| (subset >> index) & 1 == 1)
                .try_fold(0_i64, |total, (_, &vertex)| {
                    total.checked_add(weights[vertex])
                })
                .expect("gadget MIS weight must fit in i64")
        })
        .max()
        .unwrap_or(0);
    let forced_weight = forced_in
        .iter()
        .try_fold(0_i64, |total, &vertex| total.checked_add(weights[vertex]))
        .expect("forced pin weight must fit in i64");

    forced_weight
        .checked_add(free_mis)
        .expect("gadget MIS weight must fit in i64")
}

pub(super) fn alpha_tensor(
    num_vertices: usize,
    edges: &[(usize, usize)],
    weights: &[i64],
    pins: &[usize],
) -> Vec<i64> {
    let config_count = 1usize
        .checked_shl(u32::try_from(pins.len()).expect("pin count must fit in u32"))
        .expect("gadget must have few enough pins for exhaustive verification");

    (0..config_count)
        .map(|config| weighted_mis_with_fixed_pins(num_vertices, edges, weights, pins, config))
        .collect()
}

fn compactify(tensor: &mut [i64]) {
    for entry in 0..tensor.len() {
        if tensor[entry] == i64::MIN {
            continue;
        }
        if (0..tensor.len()).any(|other| {
            entry != other
                && tensor[other] != i64::MIN
                && tensor[entry] <= tensor[other]
                && (other & entry) == other
        }) {
            tensor[entry] = i64::MIN;
        }
    }
}

fn assert_unweighted_alpha_equivalent<G: Pattern>(gadget: G, name: &str) {
    let (source_locations, source_edges, source_pins) = gadget.source_graph();
    let (mapped_locations, mapped_pins) = gadget.mapped_graph();
    let mut source = alpha_tensor(
        source_locations.len(),
        &source_edges,
        &vec![1; source_locations.len()],
        &source_pins,
    );
    let mut mapped = alpha_tensor(
        mapped_locations.len(),
        &ksg_edges(&mapped_locations),
        &vec![1; mapped_locations.len()],
        &mapped_pins,
    );
    compactify(&mut source);
    compactify(&mut mapped);

    assert_eq!(
        source.len(),
        mapped.len(),
        "{name}: source and mapped gadgets expose different pin counts"
    );
    for (configuration, (&source_value, &mapped_value)) in source.iter().zip(&mapped).enumerate() {
        assert_eq!(
            source_value == i64::MIN,
            mapped_value == i64::MIN,
            "{name}: compactified alpha tensors disagree at pin configuration {configuration:#b}; source={source:?}, mapped={mapped:?}"
        );
        if source_value != i64::MIN {
            assert_eq!(
                mapped_value
                    .checked_sub(source_value)
                    .expect("alpha-tensor difference must fit in i64"),
                gadget.mis_overhead(),
                "{name}: alpha-tensor overhead differs at pin configuration {configuration:#b}; source={source:?}, mapped={mapped:?}"
            );
        }
    }
}

#[test]
fn test_unweighted_crossing_gadget_alpha_tensors() {
    assert_unweighted_alpha_equivalent(KsgCross::<false>, "KsgCross<false>");
    assert_unweighted_alpha_equivalent(KsgTurn, "KsgTurn");
    assert_unweighted_alpha_equivalent(KsgWTurn, "KsgWTurn");
    assert_unweighted_alpha_equivalent(KsgBranch, "KsgBranch");
    assert_unweighted_alpha_equivalent(KsgBranchFix, "KsgBranchFix");
    assert_unweighted_alpha_equivalent(KsgTCon, "KsgTCon");
    assert_unweighted_alpha_equivalent(KsgTrivialTurn, "KsgTrivialTurn");
    assert_unweighted_alpha_equivalent(KsgRotatedGadget::new(KsgTCon, 1), "RotatedKsgTCon");
    assert_unweighted_alpha_equivalent(
        KsgReflectedGadget::new(KsgCross::<true>, Mirror::Y),
        "ReflectedKsgCross<true>",
    );
    assert_unweighted_alpha_equivalent(
        KsgReflectedGadget::new(KsgTrivialTurn, Mirror::Y),
        "ReflectedKsgTrivialTurn",
    );
    assert_unweighted_alpha_equivalent(KsgBranchFixB, "KsgBranchFixB");
    assert_unweighted_alpha_equivalent(KsgEndTurn, "KsgEndTurn");
    assert_unweighted_alpha_equivalent(
        KsgReflectedGadget::new(KsgRotatedGadget::new(KsgTCon, 1), Mirror::Y),
        "ReflectedRotatedKsgTCon",
    );
}

#[test]
fn test_unweighted_simplifier_gadget_alpha_tensors() {
    assert_unweighted_alpha_equivalent(KsgDanglingLeg, "KsgDanglingLeg");
    assert_unweighted_alpha_equivalent(
        KsgRotatedGadget::new(KsgDanglingLeg, 1),
        "KsgDanglingLegRot1",
    );
    assert_unweighted_alpha_equivalent(
        KsgRotatedGadget::new(KsgDanglingLeg, 2),
        "KsgDanglingLegRot2",
    );
    assert_unweighted_alpha_equivalent(
        KsgRotatedGadget::new(KsgDanglingLeg, 3),
        "KsgDanglingLegRot3",
    );
    assert_unweighted_alpha_equivalent(
        KsgReflectedGadget::new(KsgDanglingLeg, Mirror::X),
        "KsgDanglingLegMirrorX",
    );
    assert_unweighted_alpha_equivalent(
        KsgReflectedGadget::new(KsgDanglingLeg, Mirror::Y),
        "KsgDanglingLegMirrorY",
    );
}
