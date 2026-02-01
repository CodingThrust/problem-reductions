//! KSG unweighted square lattice gadgets for resolving crossings.
//!
//! This module contains all gadget implementations for the King's SubGraph (KSG)
//! unweighted mapping: KsgCross, KsgTurn, KsgWTurn, KsgBranch, KsgBranchFix, KsgTCon,
//! KsgTrivialTurn, KsgEndTurn, KsgBranchFixB, KsgDanglingLeg, and their rotated/reflected variants.

use super::super::grid::{CellState, MappingGrid};
use super::super::traits::{apply_gadget, pattern_matches, Pattern, PatternCell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for pattern factory function used in crossing gadget matching.
type PatternFactory = Box<dyn Fn() -> Box<dyn KsgPatternBoxed>>;

/// Type alias for source graph representation: (locations, pin_edges, source_pins).
pub type SourceGraph = (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>);

// ============================================================================
// Crossing Gadgets - matching Julia's gadgets.jl exactly
// ============================================================================

/// Crossing gadget for resolving two crossing copy-lines.
///
/// `KsgCross<true>`: connected crossing (edges share a vertex), size (3,3)
/// `KsgCross<false>`: disconnected crossing, size (4,5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgCross<const CON: bool>;

impl Pattern for KsgCross<true> {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn is_cross_gadget(&self) -> bool {
        true
    }

    fn connected_nodes(&self) -> Vec<usize> {
        vec![0, 5]
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 1), (2, 2), (2, 3), (1, 2), (2, 2), (3, 2)];
        let edges = vec![(0, 1), (1, 2), (3, 4), (4, 5), (0, 5)];
        let pins = vec![0, 3, 5, 2];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 1), (2, 2), (2, 3), (1, 2), (3, 2)];
        let pins = vec![0, 3, 4, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (5, 5),
            (12, 12),
            (8, 0),
            (1, 0),
            (0, 0),
            (6, 6),
            (11, 11),
            (9, 9),
            (14, 14),
            (3, 3),
            (7, 7),
            (4, 0),
            (13, 13),
            (15, 15),
            (2, 0),
            (10, 10),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, true, false, false, true, false]]);
        map.insert(1, vec![vec![true, false, false, false, true, false]]);
        map.insert(3, vec![vec![true, false, false, true, false, false]]);
        map.insert(4, vec![vec![false, true, false, false, false, true]]);
        map.insert(6, vec![vec![false, true, false, true, false, true]]);
        map.insert(8, vec![vec![false, false, true, false, true, false]]);
        map.insert(9, vec![vec![true, false, true, false, true, false]]);
        map.insert(10, vec![vec![false, false, true, true, false, false]]);
        map.insert(11, vec![vec![true, false, true, true, false, false]]);
        map.insert(12, vec![vec![false, false, true, false, false, true]]);
        map.insert(14, vec![vec![false, false, true, true, false, true]]);
        map.insert(5, vec![]);
        map.insert(7, vec![]);
        map.insert(13, vec![]);
        map.insert(15, vec![]);
        map.insert(2, vec![vec![false, true, false, true, false, false]]);
        map
    }
}

impl Pattern for KsgCross<false> {
    fn size(&self) -> (usize, usize) {
        (4, 5)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 3)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn is_cross_gadget(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
        ];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (5, 6), (6, 7), (7, 8)];
        let pins = vec![0, 5, 8, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (1, 3),
            (3, 3),
            (4, 3),
            (3, 2),
            (3, 4),
        ];
        let pins = vec![0, 5, 7, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (5, 4),
            (12, 4),
            (8, 0),
            (1, 0),
            (0, 0),
            (6, 0),
            (11, 11),
            (9, 9),
            (14, 2),
            (3, 2),
            (7, 2),
            (4, 4),
            (13, 13),
            (15, 11),
            (2, 2),
            (10, 2),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(
            0,
            vec![
                vec![false, true, false, true, false, false, false, true, false],
                vec![false, true, false, true, false, false, true, false, false],
            ],
        );
        map.insert(
            2,
            vec![vec![
                false, true, false, true, false, true, false, true, false,
            ]],
        );
        map.insert(
            4,
            vec![vec![
                false, true, false, true, false, false, true, false, true,
            ]],
        );
        map.insert(
            9,
            vec![
                vec![true, false, true, false, true, false, false, true, false],
                vec![true, false, true, false, true, false, true, false, false],
            ],
        );
        map.insert(
            11,
            vec![vec![
                true, false, true, false, true, true, false, true, false,
            ]],
        );
        map.insert(
            13,
            vec![vec![
                true, false, true, false, true, false, true, false, true,
            ]],
        );
        for i in [1, 3, 5, 6, 7, 8, 10, 12, 14, 15] {
            map.entry(i).or_insert_with(Vec::new);
        }
        map
    }
}

/// Turn gadget for 90-degree turns in copy-lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgTurn;

impl Pattern for KsgTurn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (3, 2)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (3, 2), (3, 3), (3, 4)];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let pins = vec![0, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 3), (3, 4)];
        let pins = vec![0, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 0), (3, 3), (1, 0)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, true, false, true, false]]);
        map.insert(
            1,
            vec![
                vec![true, false, true, false, false],
                vec![true, false, false, true, false],
            ],
        );
        map.insert(
            2,
            vec![
                vec![false, true, false, false, true],
                vec![false, false, true, false, true],
            ],
        );
        map.insert(3, vec![vec![true, false, true, false, true]]);
        map
    }
}

/// W-shaped turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgWTurn;

impl Pattern for KsgWTurn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 3), (2, 4), (3, 2), (3, 3), (4, 2)];
        let edges = vec![(0, 1), (0, 3), (2, 3), (2, 4)];
        let pins = vec![1, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 4), (3, 3), (4, 2)];
        let pins = vec![0, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 0), (3, 3), (1, 0)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![true, false, true, false, false]]);
        map.insert(
            1,
            vec![
                vec![false, true, false, true, false],
                vec![false, true, true, false, false],
            ],
        );
        map.insert(
            2,
            vec![
                vec![false, false, false, true, true],
                vec![true, false, false, false, true],
            ],
        );
        map.insert(3, vec![vec![false, true, false, true, true]]);
        map
    }
}

/// Branch gadget for T-junctions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgBranch;

impl Pattern for KsgBranch {
    fn size(&self) -> (usize, usize) {
        (5, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (3, 2)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
            (3, 4),
            (4, 3),
            (4, 2),
            (5, 2),
        ];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (3, 5), (5, 6), (6, 7)];
        let pins = vec![0, 4, 7];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 3), (3, 2), (3, 4), (4, 3), (5, 2)];
        let pins = vec![0, 3, 5];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    // Julia: sw[[4]] .= 3 (node 4 = 0-indexed 3 has weight 3)
    fn source_weights(&self) -> Vec<i32> {
        vec![2, 2, 2, 3, 2, 2, 2, 2]
    }
    // Julia: mw[[2]] .= 3 (mapped node 2 = 0-indexed 1 has weight 3)
    fn mapped_weights(&self) -> Vec<i32> {
        vec![2, 3, 2, 2, 2, 2]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (0, 0),
            (4, 0),
            (5, 5),
            (6, 6),
            (2, 0),
            (7, 7),
            (3, 3),
            (1, 0),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(
            0,
            vec![vec![false, true, false, true, false, false, true, false]],
        );
        map.insert(
            3,
            vec![
                vec![true, false, true, false, true, false, true, false],
                vec![true, false, true, false, true, true, false, false],
            ],
        );
        map.insert(
            5,
            vec![vec![true, false, true, false, false, true, false, true]],
        );
        map.insert(
            6,
            vec![
                vec![false, false, true, false, true, true, false, true],
                vec![false, true, false, false, true, true, false, true],
            ],
        );
        map.insert(
            7,
            vec![vec![true, false, true, false, true, true, false, true]],
        );
        for i in [1, 2, 4] {
            map.insert(i, vec![]);
        }
        map
    }
}

/// Branch fix gadget for simplifying branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgBranchFix;

impl Pattern for KsgBranchFix {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (2, 3), (3, 3), (3, 2), (4, 2)];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        let pins = vec![0, 5];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (3, 2), (4, 2)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 1), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(
            0,
            vec![
                vec![false, true, false, true, false, false],
                vec![false, true, false, false, true, false],
                vec![false, false, true, false, true, false],
            ],
        );
        map.insert(1, vec![vec![true, false, true, false, true, false]]);
        map.insert(2, vec![vec![false, true, false, true, false, true]]);
        map.insert(
            3,
            vec![
                vec![true, false, false, true, false, true],
                vec![true, false, true, false, false, true],
            ],
        );
        map
    }
}

/// T-connection gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgTCon;

impl Pattern for KsgTCon {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }
    fn is_connected(&self) -> bool {
        true
    }
    fn connected_nodes(&self) -> Vec<usize> {
        vec![0, 1]
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1), (2, 2), (3, 2)];
        let edges = vec![(0, 1), (0, 2), (2, 3)];
        let pins = vec![0, 1, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1), (2, 3), (3, 2)];
        let pins = vec![0, 1, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    // Julia: sw[[2]] .= 1 (node 2 = 0-indexed 1 has weight 1)
    fn source_weights(&self) -> Vec<i32> {
        vec![2, 1, 2, 2]
    }
    // Julia: mw[[2]] .= 1 (mapped node 2 = 0-indexed 1 has weight 1)
    fn mapped_weights(&self) -> Vec<i32> {
        vec![2, 1, 2, 2]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (0, 0),
            (4, 0),
            (5, 5),
            (6, 6),
            (2, 2),
            (7, 7),
            (3, 3),
            (1, 0),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, false, true, false]]);
        map.insert(1, vec![vec![true, false, false, false]]);
        map.insert(2, vec![vec![false, true, true, false]]);
        map.insert(4, vec![vec![false, false, false, true]]);
        map.insert(5, vec![vec![true, false, false, true]]);
        map.insert(6, vec![vec![false, true, false, true]]);
        map.insert(3, vec![]);
        map.insert(7, vec![]);
        map
    }
}

/// Trivial turn gadget for simple diagonal turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgTrivialTurn;

impl Pattern for KsgTrivialTurn {
    fn size(&self) -> (usize, usize) {
        (2, 2)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }
    fn is_connected(&self) -> bool {
        true
    }
    fn connected_nodes(&self) -> Vec<usize> {
        vec![0, 1]
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1)];
        let edges = vec![(0, 1)];
        let pins = vec![0, 1];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    // Julia: sw[[1,2]] .= 1 (nodes 1,2 have weight 1)
    fn source_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }
    // Julia: mw[[1,2]] .= 1 (mapped nodes 1,2 have weight 1)
    fn mapped_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 3), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, false]]);
        map.insert(1, vec![vec![true, false]]);
        map.insert(2, vec![vec![false, true]]);
        map.insert(3, vec![]);
        map
    }
}

/// End turn gadget for line terminations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgEndTurn;

impl Pattern for KsgEndTurn {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (2, 3)];
        let edges = vec![(0, 1), (1, 2)];
        let pins = vec![0];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    // Julia: sw[[3]] .= 1 (node 3 = 0-indexed 2 has weight 1)
    fn source_weights(&self) -> Vec<i32> {
        vec![2, 2, 1]
    }
    // Julia: mw[[1]] .= 1 (mapped node 1 = 0-indexed 0 has weight 1)
    fn mapped_weights(&self) -> Vec<i32> {
        vec![1]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, false, true], vec![false, true, false]]);
        map.insert(1, vec![vec![true, false, true]]);
        map
    }
}

/// Alternate branch fix gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgBranchFixB;

impl Pattern for KsgBranchFixB {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 3), (3, 2), (3, 3), (4, 2)];
        let edges = vec![(0, 2), (1, 2), (1, 3)];
        let pins = vec![0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(3, 2), (4, 2)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    // Julia: sw[[1]] .= 1 (node 1 = 0-indexed 0 has weight 1)
    fn source_weights(&self) -> Vec<i32> {
        vec![1, 2, 2, 2]
    }
    // Julia: mw[[1]] .= 1 (mapped node 1 = 0-indexed 0 has weight 1)
    fn mapped_weights(&self) -> Vec<i32> {
        vec![1, 2]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 3), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(
            0,
            vec![
                vec![false, false, true, false],
                vec![false, true, false, false],
            ],
        );
        map.insert(1, vec![vec![true, true, false, false]]);
        map.insert(2, vec![vec![false, false, true, true]]);
        map.insert(3, vec![vec![true, false, false, true]]);
        map
    }
}

// ============================================================================
// Rotated and Reflected Gadgets
// ============================================================================

/// A rotated version of a gadget.
#[derive(Debug, Clone)]
pub struct KsgRotatedGadget<G: Pattern> {
    pub gadget: G,
    /// Number of 90-degree clockwise rotations (0-3).
    pub n: usize,
}

impl<G: Pattern> KsgRotatedGadget<G> {
    pub fn new(gadget: G, n: usize) -> Self {
        Self { gadget, n: n % 4 }
    }
}

fn rotate90(loc: (i32, i32)) -> (i32, i32) {
    (-loc.1, loc.0)
}

fn rotate_around_center(loc: (usize, usize), center: (usize, usize), n: usize) -> (i32, i32) {
    let mut dx = loc.0 as i32 - center.0 as i32;
    let mut dy = loc.1 as i32 - center.1 as i32;
    for _ in 0..n {
        let (nx, ny) = rotate90((dx, dy));
        dx = nx;
        dy = ny;
    }
    (center.0 as i32 + dx, center.1 as i32 + dy)
}

impl<G: Pattern> Pattern for KsgRotatedGadget<G> {
    fn size(&self) -> (usize, usize) {
        let (m, n) = self.gadget.size();
        if self.n % 2 == 0 {
            (m, n)
        } else {
            (n, m)
        }
    }

    fn cross_location(&self) -> (usize, usize) {
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();
        let rotated = rotate_around_center(center, center, self.n);
        let corners = [(1, 1), (m, n)];
        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|&c| rotate_around_center(c, center, self.n))
            .collect();
        let min_r = rotated_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = rotated_corners.iter().map(|c| c.1).min().unwrap();
        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;
        (
            (rotated.0 + offset_r) as usize,
            (rotated.1 + offset_c) as usize,
        )
    }

    fn is_connected(&self) -> bool {
        self.gadget.is_connected()
    }
    fn is_cross_gadget(&self) -> bool {
        self.gadget.is_cross_gadget()
    }
    fn connected_nodes(&self) -> Vec<usize> {
        self.gadget.connected_nodes()
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let (locs, edges, pins) = self.gadget.source_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();
        let corners = [(1usize, 1usize), (m, n)];
        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|&c| rotate_around_center(c, center, self.n))
            .collect();
        let min_r = rotated_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = rotated_corners.iter().map(|c| c.1).min().unwrap();
        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;
        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let rotated = rotate_around_center(loc, center, self.n);
                (
                    (rotated.0 + offset_r) as usize,
                    (rotated.1 + offset_c) as usize,
                )
            })
            .collect();
        (new_locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let (locs, pins) = self.gadget.mapped_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();
        let corners = [(1usize, 1usize), (m, n)];
        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|&c| rotate_around_center(c, center, self.n))
            .collect();
        let min_r = rotated_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = rotated_corners.iter().map(|c| c.1).min().unwrap();
        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;
        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let rotated = rotate_around_center(loc, center, self.n);
                (
                    (rotated.0 + offset_r) as usize,
                    (rotated.1 + offset_c) as usize,
                )
            })
            .collect();
        (new_locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        self.gadget.mis_overhead()
    }
    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        self.gadget.mapped_entry_to_compact()
    }
    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        self.gadget.source_entry_to_configs()
    }

    // Weights don't change with rotation - delegate to inner gadget
    fn source_weights(&self) -> Vec<i32> {
        self.gadget.source_weights()
    }
    fn mapped_weights(&self) -> Vec<i32> {
        self.gadget.mapped_weights()
    }
}

/// Mirror axis for reflection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirror {
    X,
    Y,
    Diag,
    OffDiag,
}

/// A reflected version of a gadget.
#[derive(Debug, Clone)]
pub struct KsgReflectedGadget<G: Pattern> {
    pub gadget: G,
    pub mirror: Mirror,
}

impl<G: Pattern> KsgReflectedGadget<G> {
    pub fn new(gadget: G, mirror: Mirror) -> Self {
        Self { gadget, mirror }
    }
}

fn reflect(loc: (i32, i32), mirror: Mirror) -> (i32, i32) {
    match mirror {
        Mirror::X => (loc.0, -loc.1),
        Mirror::Y => (-loc.0, loc.1),
        Mirror::Diag => (-loc.1, -loc.0),
        Mirror::OffDiag => (loc.1, loc.0),
    }
}

fn reflect_around_center(
    loc: (usize, usize),
    center: (usize, usize),
    mirror: Mirror,
) -> (i32, i32) {
    let dx = loc.0 as i32 - center.0 as i32;
    let dy = loc.1 as i32 - center.1 as i32;
    let (nx, ny) = reflect((dx, dy), mirror);
    (center.0 as i32 + nx, center.1 as i32 + ny)
}

impl<G: Pattern> Pattern for KsgReflectedGadget<G> {
    fn size(&self) -> (usize, usize) {
        let (m, n) = self.gadget.size();
        match self.mirror {
            Mirror::X | Mirror::Y => (m, n),
            Mirror::Diag | Mirror::OffDiag => (n, m),
        }
    }

    fn cross_location(&self) -> (usize, usize) {
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();
        let reflected = reflect_around_center(center, center, self.mirror);
        let corners = [(1, 1), (m, n)];
        let reflected_corners: Vec<_> = corners
            .iter()
            .map(|&c| reflect_around_center(c, center, self.mirror))
            .collect();
        let min_r = reflected_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = reflected_corners.iter().map(|c| c.1).min().unwrap();
        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;
        (
            (reflected.0 + offset_r) as usize,
            (reflected.1 + offset_c) as usize,
        )
    }

    fn is_connected(&self) -> bool {
        self.gadget.is_connected()
    }
    fn is_cross_gadget(&self) -> bool {
        self.gadget.is_cross_gadget()
    }
    fn connected_nodes(&self) -> Vec<usize> {
        self.gadget.connected_nodes()
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let (locs, edges, pins) = self.gadget.source_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();
        let corners = [(1usize, 1usize), (m, n)];
        let reflected_corners: Vec<_> = corners
            .iter()
            .map(|&c| reflect_around_center(c, center, self.mirror))
            .collect();
        let min_r = reflected_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = reflected_corners.iter().map(|c| c.1).min().unwrap();
        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;
        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let reflected = reflect_around_center(loc, center, self.mirror);
                (
                    (reflected.0 + offset_r) as usize,
                    (reflected.1 + offset_c) as usize,
                )
            })
            .collect();
        (new_locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let (locs, pins) = self.gadget.mapped_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();
        let corners = [(1usize, 1usize), (m, n)];
        let reflected_corners: Vec<_> = corners
            .iter()
            .map(|&c| reflect_around_center(c, center, self.mirror))
            .collect();
        let min_r = reflected_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = reflected_corners.iter().map(|c| c.1).min().unwrap();
        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;
        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let reflected = reflect_around_center(loc, center, self.mirror);
                (
                    (reflected.0 + offset_r) as usize,
                    (reflected.1 + offset_c) as usize,
                )
            })
            .collect();
        (new_locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        self.gadget.mis_overhead()
    }
    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        self.gadget.mapped_entry_to_compact()
    }
    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        self.gadget.source_entry_to_configs()
    }

    // Weights don't change with reflection - delegate to inner gadget
    fn source_weights(&self) -> Vec<i32> {
        self.gadget.source_weights()
    }
    fn mapped_weights(&self) -> Vec<i32> {
        self.gadget.mapped_weights()
    }
}

// ============================================================================
// Simplifier Patterns
// ============================================================================

/// Dangling leg simplifier pattern.
///
/// Julia pattern:
/// ```text
/// Source:       Mapped:
/// . . .         . . .
/// . X .    =>   . . .
/// . X .         . . .
/// . X .         . X .
/// ```
/// Removes 2 nodes from a dangling chain, keeping only the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KsgDanglingLeg;

impl Pattern for KsgDanglingLeg {
    fn size(&self) -> (usize, usize) {
        (4, 3)
    }
    // Julia: cross_location = size ./ 2 = (4/2, 3/2) = (2, 1)
    fn cross_location(&self) -> (usize, usize) {
        (2, 1)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: 3 nodes at (2,2), (3,2), (4,2) - vertical chain in column 2
        let locs = vec![(2, 2), (3, 2), (4, 2)];
        let edges = vec![(0, 1), (1, 2)];
        // Boundary node: only (4,2) is on boundary (row 4 = m for 4x3 pattern)
        let pins = vec![2];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: 1 node at (4,2) - the bottom endpoint
        let locs = vec![(4, 2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    // Julia: sw[[1]] .= 1 (node 1 = 0-indexed 0 has weight 1)
    fn source_weights(&self) -> Vec<i32> {
        vec![1, 2, 2]
    }
    // Julia: mw[[1]] .= 1 (mapped node 1 = 0-indexed 0 has weight 1)
    fn mapped_weights(&self) -> Vec<i32> {
        vec![1]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        // Julia: Dict([0 => 0, 1 => 1])
        [(0, 0), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        // Julia: 0 => [[1,0,0], [0,1,0]], 1 => [[1,0,1]]
        // Entry 0 (mapped node not selected): select node 0 OR node 1
        // Entry 1 (mapped node selected): select nodes 0 and 2
        let mut map = HashMap::new();
        map.insert(0, vec![vec![true, false, false], vec![false, true, false]]);
        map.insert(1, vec![vec![true, false, true]]);
        map
    }
}

// ============================================================================
// KsgPattern Enum for Dynamic Dispatch
// ============================================================================

/// Enum wrapping all KSG square lattice patterns for dynamic dispatch during unapply.
#[derive(Debug, Clone)]
pub enum KsgPattern {
    CrossFalse(KsgCross<false>),
    CrossTrue(KsgCross<true>),
    Turn(KsgTurn),
    WTurn(KsgWTurn),
    Branch(KsgBranch),
    BranchFix(KsgBranchFix),
    TCon(KsgTCon),
    TrivialTurn(KsgTrivialTurn),
    EndTurn(KsgEndTurn),
    BranchFixB(KsgBranchFixB),
    DanglingLeg(KsgDanglingLeg),
    RotatedTCon1(KsgRotatedGadget<KsgTCon>),
    ReflectedCrossTrue(KsgReflectedGadget<KsgCross<true>>),
    ReflectedTrivialTurn(KsgReflectedGadget<KsgTrivialTurn>),
    ReflectedRotatedTCon1(KsgReflectedGadget<KsgRotatedGadget<KsgTCon>>),
    DanglingLegRot1(KsgRotatedGadget<KsgDanglingLeg>),
    DanglingLegRot2(KsgRotatedGadget<KsgRotatedGadget<KsgDanglingLeg>>),
    DanglingLegRot3(KsgRotatedGadget<KsgRotatedGadget<KsgRotatedGadget<KsgDanglingLeg>>>),
    DanglingLegReflX(KsgReflectedGadget<KsgDanglingLeg>),
    DanglingLegReflY(KsgReflectedGadget<KsgDanglingLeg>),
}

impl KsgPattern {
    /// Get pattern from tape index.
    pub fn from_tape_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::CrossFalse(KsgCross::<false>)),
            1 => Some(Self::Turn(KsgTurn)),
            2 => Some(Self::WTurn(KsgWTurn)),
            3 => Some(Self::Branch(KsgBranch)),
            4 => Some(Self::BranchFix(KsgBranchFix)),
            5 => Some(Self::TCon(KsgTCon)),
            6 => Some(Self::TrivialTurn(KsgTrivialTurn)),
            7 => Some(Self::RotatedTCon1(KsgRotatedGadget::new(KsgTCon, 1))),
            8 => Some(Self::ReflectedCrossTrue(KsgReflectedGadget::new(
                KsgCross::<true>,
                Mirror::Y,
            ))),
            9 => Some(Self::ReflectedTrivialTurn(KsgReflectedGadget::new(
                KsgTrivialTurn,
                Mirror::Y,
            ))),
            10 => Some(Self::BranchFixB(KsgBranchFixB)),
            11 => Some(Self::EndTurn(KsgEndTurn)),
            12 => Some(Self::ReflectedRotatedTCon1(KsgReflectedGadget::new(
                KsgRotatedGadget::new(KsgTCon, 1),
                Mirror::Y,
            ))),
            100 => Some(Self::DanglingLeg(KsgDanglingLeg)),
            101 => Some(Self::DanglingLegRot1(KsgRotatedGadget::new(
                KsgDanglingLeg,
                1,
            ))),
            102 => Some(Self::DanglingLegRot2(KsgRotatedGadget::new(
                KsgRotatedGadget::new(KsgDanglingLeg, 1),
                1,
            ))),
            103 => Some(Self::DanglingLegRot3(KsgRotatedGadget::new(
                KsgRotatedGadget::new(KsgRotatedGadget::new(KsgDanglingLeg, 1), 1),
                1,
            ))),
            104 => Some(Self::DanglingLegReflX(KsgReflectedGadget::new(
                KsgDanglingLeg,
                Mirror::X,
            ))),
            105 => Some(Self::DanglingLegReflY(KsgReflectedGadget::new(
                KsgDanglingLeg,
                Mirror::Y,
            ))),
            _ => None,
        }
    }

    /// Apply map_config_back_pattern for this pattern.
    pub fn map_config_back(&self, gi: usize, gj: usize, config: &mut [Vec<usize>]) {
        match self {
            Self::CrossFalse(p) => map_config_back_pattern(p, gi, gj, config),
            Self::CrossTrue(p) => map_config_back_pattern(p, gi, gj, config),
            Self::Turn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::WTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::Branch(p) => map_config_back_pattern(p, gi, gj, config),
            Self::BranchFix(p) => map_config_back_pattern(p, gi, gj, config),
            Self::TCon(p) => map_config_back_pattern(p, gi, gj, config),
            Self::TrivialTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::EndTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::BranchFixB(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLeg(p) => map_config_back_pattern(p, gi, gj, config),
            Self::RotatedTCon1(p) => map_config_back_pattern(p, gi, gj, config),
            Self::ReflectedCrossTrue(p) => map_config_back_pattern(p, gi, gj, config),
            Self::ReflectedTrivialTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::ReflectedRotatedTCon1(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegRot1(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegRot2(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegRot3(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegReflX(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegReflY(p) => map_config_back_pattern(p, gi, gj, config),
        }
    }
}

// ============================================================================
// Crossing ruleset and apply functions
// ============================================================================

/// A tape entry recording a gadget application.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KsgTapeEntry {
    pub pattern_idx: usize,
    pub row: usize,
    pub col: usize,
}

/// Calculate MIS overhead for a tape entry.
pub fn tape_entry_mis_overhead(entry: &KsgTapeEntry) -> i32 {
    match entry.pattern_idx {
        0 => KsgCross::<false>.mis_overhead(),
        1 => KsgTurn.mis_overhead(),
        2 => KsgWTurn.mis_overhead(),
        3 => KsgBranch.mis_overhead(),
        4 => KsgBranchFix.mis_overhead(),
        5 => KsgTCon.mis_overhead(),
        6 => KsgTrivialTurn.mis_overhead(),
        7 => KsgRotatedGadget::new(KsgTCon, 1).mis_overhead(),
        8 => KsgReflectedGadget::new(KsgCross::<true>, Mirror::Y).mis_overhead(),
        9 => KsgReflectedGadget::new(KsgTrivialTurn, Mirror::Y).mis_overhead(),
        10 => KsgBranchFixB.mis_overhead(),
        11 => KsgEndTurn.mis_overhead(),
        12 => KsgReflectedGadget::new(KsgRotatedGadget::new(KsgTCon, 1), Mirror::Y).mis_overhead(),
        100..=105 => KsgDanglingLeg.mis_overhead(),
        _ => 0,
    }
}

/// The default crossing ruleset for KSG square lattice.
#[allow(dead_code)]
pub fn crossing_ruleset_indices() -> Vec<usize> {
    (0..13).collect()
}

/// Apply all crossing gadgets to the grid.
/// Follows Julia's algorithm: iterate over all (i,j) pairs and try all patterns.
/// Note: Unlike the previous version, we don't skip based on crossat position
/// because different (i,j) pairs with the same crossat can match different patterns
/// at different positions (since each pattern has a different cross_location).
pub fn apply_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::super::copyline::CopyLine],
) -> Vec<KsgTapeEntry> {
    let mut tape = Vec::new();
    let n = copylines.len();

    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat(grid, copylines, i, j);
            if let Some((pattern_idx, row, col)) =
                try_match_and_apply_crossing(grid, cross_row, cross_col)
            {
                tape.push(KsgTapeEntry {
                    pattern_idx,
                    row,
                    col,
                });
            }
        }
    }
    tape
}

/// Calculate crossing point for two copylines.
/// Uses grid.cross_at() which implements Julia's crossat formula.
fn crossat(
    grid: &MappingGrid,
    copylines: &[super::super::copyline::CopyLine],
    v: usize,
    w: usize,
) -> (usize, usize) {
    let line_v = copylines.get(v);
    let line_w = copylines.get(w);

    match (line_v, line_w) {
        (Some(lv), Some(lw)) => {
            let (line_first, line_second) = if lv.vslot < lw.vslot {
                (lv, lw)
            } else {
                (lw, lv)
            };
            // Delegate to grid.cross_at() - single source of truth for crossat formula
            grid.cross_at(line_first.vslot, line_second.vslot, line_first.hslot)
        }
        _ => (0, 0),
    }
}

fn try_match_and_apply_crossing(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<(usize, usize, usize)> {
    // Try each pattern in order
    let patterns: Vec<(usize, PatternFactory)> = vec![
        (0, Box::new(|| Box::new(KsgCross::<false>))),
        (1, Box::new(|| Box::new(KsgTurn))),
        (2, Box::new(|| Box::new(KsgWTurn))),
        (3, Box::new(|| Box::new(KsgBranch))),
        (4, Box::new(|| Box::new(KsgBranchFix))),
        (5, Box::new(|| Box::new(KsgTCon))),
        (6, Box::new(|| Box::new(KsgTrivialTurn))),
        (7, Box::new(|| Box::new(KsgRotatedGadget::new(KsgTCon, 1)))),
        (
            8,
            Box::new(|| Box::new(KsgReflectedGadget::new(KsgCross::<true>, Mirror::Y))),
        ),
        (
            9,
            Box::new(|| Box::new(KsgReflectedGadget::new(KsgTrivialTurn, Mirror::Y))),
        ),
        (10, Box::new(|| Box::new(KsgBranchFixB))),
        (11, Box::new(|| Box::new(KsgEndTurn))),
        (
            12,
            Box::new(|| {
                Box::new(KsgReflectedGadget::new(
                    KsgRotatedGadget::new(KsgTCon, 1),
                    Mirror::Y,
                ))
            }),
        ),
    ];

    for (idx, make_pattern) in patterns {
        let pattern = make_pattern();
        let cl = pattern.cross_location();
        // cross_row/cross_col are 0-indexed, cl is 1-indexed within gadget
        // x = cross_row - (cl.0 - 1) = cross_row + 1 - cl.0, needs x >= 0
        if cross_row + 1 >= cl.0 && cross_col + 1 >= cl.1 {
            let x = cross_row + 1 - cl.0;
            let y = cross_col + 1 - cl.1;
            if pattern.pattern_matches_boxed(grid, x, y) {
                pattern.apply_gadget_boxed(grid, x, y);
                return Some((idx, x, y));
            }
        }
    }
    None
}

/// Apply crossing gadgets with proper weights for weighted mode.
/// Uses apply_weighted_gadget which respects mapped_weights() for each gadget.
pub fn apply_weighted_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::super::copyline::CopyLine],
) -> Vec<KsgTapeEntry> {
    let mut tape = Vec::new();
    let n = copylines.len();

    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat(grid, copylines, i, j);
            if let Some((pattern_idx, row, col)) =
                try_match_and_apply_weighted_crossing(grid, cross_row, cross_col)
            {
                tape.push(KsgTapeEntry {
                    pattern_idx,
                    row,
                    col,
                });
            }
        }
    }
    tape
}

fn try_match_and_apply_weighted_crossing(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<(usize, usize, usize)> {
    // Try each pattern in order - same order as try_match_and_apply_crossing
    let patterns: Vec<(usize, PatternFactory)> = vec![
        (0, Box::new(|| Box::new(KsgCross::<false>))),
        (1, Box::new(|| Box::new(KsgTurn))),
        (2, Box::new(|| Box::new(KsgWTurn))),
        (3, Box::new(|| Box::new(KsgBranch))),
        (4, Box::new(|| Box::new(KsgBranchFix))),
        (5, Box::new(|| Box::new(KsgTCon))),
        (6, Box::new(|| Box::new(KsgTrivialTurn))),
        (7, Box::new(|| Box::new(KsgRotatedGadget::new(KsgTCon, 1)))),
        (
            8,
            Box::new(|| Box::new(KsgReflectedGadget::new(KsgCross::<true>, Mirror::Y))),
        ),
        (
            9,
            Box::new(|| Box::new(KsgReflectedGadget::new(KsgTrivialTurn, Mirror::Y))),
        ),
        (10, Box::new(|| Box::new(KsgBranchFixB))),
        (11, Box::new(|| Box::new(KsgEndTurn))),
        (
            12,
            Box::new(|| {
                Box::new(KsgReflectedGadget::new(
                    KsgRotatedGadget::new(KsgTCon, 1),
                    Mirror::Y,
                ))
            }),
        ),
    ];

    for (idx, make_pattern) in patterns {
        let pattern = make_pattern();
        let cl = pattern.cross_location();
        if cross_row + 1 >= cl.0 && cross_col + 1 >= cl.1 {
            let x = cross_row + 1 - cl.0;
            let y = cross_col + 1 - cl.1;
            let matches = pattern.pattern_matches_boxed(grid, x, y);
            if matches {
                pattern.apply_weighted_gadget_boxed(grid, x, y);
                return Some((idx, x, y));
            }
        }
    }
    None
}

/// Apply simplifier gadgets (KsgDanglingLeg variants).
/// `nrepeat` specifies the number of simplification passes.
pub fn apply_simplifier_gadgets(grid: &mut MappingGrid, nrepeat: usize) -> Vec<KsgTapeEntry> {
    let mut tape = Vec::new();
    let (rows, cols) = grid.size();

    // Get all rotations and reflections of KsgDanglingLeg
    let patterns = rotated_and_reflected_danglinleg();

    for _ in 0..nrepeat {
        for (pattern_idx, pattern) in patterns.iter().enumerate() {
            for j in 0..cols {
                for i in 0..rows {
                    if pattern_matches_boxed(pattern.as_ref(), grid, i, j) {
                        apply_gadget_boxed(pattern.as_ref(), grid, i, j);
                        tape.push(KsgTapeEntry {
                            pattern_idx: 100 + pattern_idx, // Offset to distinguish from crossing gadgets
                            row: i,
                            col: j,
                        });
                    }
                }
            }
        }
    }

    tape
}

/// Apply weighted simplifier gadgets (KsgDanglingLeg variants with weight checking).
/// For weighted mode, KsgDanglingLeg requires the center node to have weight 1.
/// Julia's WeightedGadget{DanglingLeg}: source_centers = [(2,2)] means node at (2,2) has weight 1.
pub fn apply_weighted_simplifier_gadgets(
    grid: &mut MappingGrid,
    nrepeat: usize,
) -> Vec<KsgTapeEntry> {
    let mut tape = Vec::new();
    let (rows, cols) = grid.size();

    let patterns = rotated_and_reflected_danglinleg();

    for _ in 0..nrepeat {
        for (pattern_idx, pattern) in patterns.iter().enumerate() {
            for j in 0..cols {
                for i in 0..rows {
                    if pattern_matches_weighted(pattern.as_ref(), grid, i, j) {
                        pattern.apply_weighted_gadget_boxed(grid, i, j);
                        tape.push(KsgTapeEntry {
                            pattern_idx: 100 + pattern_idx,
                            row: i,
                            col: j,
                        });
                    }
                }
            }
        }
    }

    tape
}

/// Check if a weighted KsgDanglingLeg pattern matches.
/// For weighted mode, the center node (at source_centers position) must have weight 1,
/// and other nodes must have weight 2.
fn pattern_matches_weighted(
    pattern: &dyn KsgPatternBoxed,
    grid: &MappingGrid,
    i: usize,
    j: usize,
) -> bool {
    // First check basic pattern match
    if !pattern_matches_boxed(pattern, grid, i, j) {
        return false;
    }

    // For weighted KsgDanglingLeg, check that the center node has weight 1
    // KsgDanglingLeg source_centers = [(2,2)] (1-indexed), which is (1,1) 0-indexed in 4x3 pattern
    // After rotation/reflection, the center position changes
    let (locs, _, _) = pattern.source_graph_boxed();
    // The first node in source_graph is at (2,2), which should have weight 1
    // Node positions in source_graph are 1-indexed, convert to 0-indexed and add to (i,j)
    if let Some((loc_r, loc_c)) = locs.first() {
        let grid_r = i + loc_r - 1;
        let grid_c = j + loc_c - 1;
        if let Some(cell) = grid.get(grid_r, grid_c) {
            // Center node must have weight 1
            if cell.weight() != 1 {
                return false;
            }
        }
    }

    // Check other nodes have weight 2
    for (_idx, (loc_r, loc_c)) in locs.iter().enumerate().skip(1) {
        let grid_r = i + loc_r - 1;
        let grid_c = j + loc_c - 1;
        if let Some(cell) = grid.get(grid_r, grid_c) {
            if cell.weight() != 2 {
                return false;
            }
        }
    }

    true
}

fn rotated_and_reflected_danglinleg() -> Vec<Box<dyn KsgPatternBoxed>> {
    vec![
        Box::new(KsgDanglingLeg),
        Box::new(KsgRotatedGadget::new(KsgDanglingLeg, 1)),
        Box::new(KsgRotatedGadget::new(KsgDanglingLeg, 2)),
        Box::new(KsgRotatedGadget::new(KsgDanglingLeg, 3)),
        Box::new(KsgReflectedGadget::new(KsgDanglingLeg, Mirror::X)),
        Box::new(KsgReflectedGadget::new(KsgDanglingLeg, Mirror::Y)),
    ]
}

/// Check if a boxed pattern matches at position (i, j) in the grid.
#[allow(clippy::needless_range_loop)]
fn pattern_matches_boxed(
    pattern: &dyn KsgPatternBoxed,
    grid: &MappingGrid,
    i: usize,
    j: usize,
) -> bool {
    let source = pattern.source_matrix();
    let (m, n) = pattern.size_boxed();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let expected = source[r][c];
            let actual = safe_get_pattern_cell(grid, grid_r, grid_c);

            // Connected cells in pattern match both Connected and Occupied in grid
            // (Connected is just a marker for edge connection points)
            let matches = match (expected, actual) {
                (a, b) if a == b => true,
                (PatternCell::Connected, PatternCell::Occupied) => true,
                (PatternCell::Occupied, PatternCell::Connected) => true,
                _ => false,
            };
            if !matches {
                return false;
            }
        }
    }
    true
}

fn safe_get_pattern_cell(grid: &MappingGrid, row: usize, col: usize) -> PatternCell {
    let (rows, cols) = grid.size();
    if row >= rows || col >= cols {
        return PatternCell::Empty;
    }
    match grid.get(row, col) {
        Some(CellState::Empty) => PatternCell::Empty,
        Some(CellState::Occupied { .. }) => PatternCell::Occupied,
        Some(CellState::Doubled { .. }) => PatternCell::Doubled,
        Some(CellState::Connected { .. }) => PatternCell::Connected,
        None => PatternCell::Empty,
    }
}

/// Apply a boxed gadget pattern at position (i, j).
#[allow(clippy::needless_range_loop)]
fn apply_gadget_boxed(pattern: &dyn KsgPatternBoxed, grid: &mut MappingGrid, i: usize, j: usize) {
    let mapped = pattern.mapped_matrix();
    let (m, n) = pattern.size_boxed();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let cell = mapped[r][c];
            let state = match cell {
                PatternCell::Empty => CellState::Empty,
                PatternCell::Occupied => CellState::Occupied { weight: 1 },
                PatternCell::Doubled => CellState::Doubled { weight: 2 },
                PatternCell::Connected => CellState::Connected { weight: 1 },
            };
            grid.set(grid_r, grid_c, state);
        }
    }
}

/// Apply a boxed gadget pattern at position (i, j) with proper weights.
#[allow(dead_code)]
fn apply_weighted_gadget_boxed_fn(
    pattern: &dyn KsgPatternBoxed,
    grid: &mut MappingGrid,
    i: usize,
    j: usize,
) {
    pattern.apply_weighted_gadget_boxed(grid, i, j);
}

/// Trait for boxed pattern operations.
pub trait KsgPatternBoxed {
    fn size_boxed(&self) -> (usize, usize);
    fn cross_location(&self) -> (usize, usize);
    fn source_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn source_graph_boxed(&self) -> SourceGraph;
    fn pattern_matches_boxed(&self, grid: &MappingGrid, i: usize, j: usize) -> bool;
    fn apply_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize);
    fn apply_weighted_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize);
}

impl<P: Pattern> KsgPatternBoxed for P {
    fn size_boxed(&self) -> (usize, usize) {
        self.size()
    }
    fn cross_location(&self) -> (usize, usize) {
        Pattern::cross_location(self)
    }
    fn source_matrix(&self) -> Vec<Vec<PatternCell>> {
        Pattern::source_matrix(self)
    }
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>> {
        Pattern::mapped_matrix(self)
    }
    fn source_graph_boxed(&self) -> SourceGraph {
        Pattern::source_graph(self)
    }
    fn pattern_matches_boxed(&self, grid: &MappingGrid, i: usize, j: usize) -> bool {
        pattern_matches(self, grid, i, j)
    }
    fn apply_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize) {
        apply_gadget(self, grid, i, j);
    }
    fn apply_weighted_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize) {
        apply_weighted_gadget(self, grid, i, j);
    }
}

/// Apply a weighted gadget pattern at position (i, j) with proper weights.
/// Uses mapped_graph locations and mapped_weights for each node.
#[allow(clippy::needless_range_loop)]
pub fn apply_weighted_gadget<P: Pattern>(pattern: &P, grid: &mut MappingGrid, i: usize, j: usize) {
    let (m, n) = pattern.size();
    let (mapped_locs, _) = pattern.mapped_graph();
    let mapped_weights = pattern.mapped_weights();

    // First clear the gadget area
    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;
            grid.set(grid_r, grid_c, CellState::Empty);
        }
    }

    // Build a map of (row, col) -> accumulated weight for doubled nodes
    let mut weight_map: HashMap<(usize, usize), i32> = HashMap::new();
    for (idx, &(r, c)) in mapped_locs.iter().enumerate() {
        let weight = mapped_weights.get(idx).copied().unwrap_or(2);
        *weight_map.entry((r, c)).or_insert(0) += weight;
    }

    // Count occurrences to detect doubled nodes
    let mut count_map: HashMap<(usize, usize), usize> = HashMap::new();
    for &(r, c) in &mapped_locs {
        *count_map.entry((r, c)).or_insert(0) += 1;
    }

    // Set cells with proper weights
    for (&(r, c), &total_weight) in &weight_map {
        let grid_r = i + r - 1; // Convert 1-indexed to 0-indexed
        let grid_c = j + c - 1;
        let count = count_map.get(&(r, c)).copied().unwrap_or(1);

        let state = if count > 1 {
            CellState::Doubled {
                weight: total_weight,
            }
        } else {
            CellState::Occupied {
                weight: total_weight,
            }
        };
        grid.set(grid_r, grid_c, state);
    }
}

/// Map configuration back through a single gadget.
pub fn map_config_back_pattern<P: Pattern>(
    pattern: &P,
    gi: usize,
    gj: usize,
    config: &mut [Vec<usize>],
) {
    let (m, n) = pattern.size();
    let (mapped_locs, mapped_pins) = pattern.mapped_graph();
    let (source_locs, _, _) = pattern.source_graph();

    // Step 1: Extract config at mapped locations
    let mapped_config: Vec<usize> = mapped_locs
        .iter()
        .map(|&(r, c)| {
            let row = gi + r - 1;
            let col = gj + c - 1;
            config
                .get(row)
                .and_then(|row_vec| row_vec.get(col))
                .copied()
                .unwrap_or(0)
        })
        .collect();

    // Step 2: Compute boundary config
    let bc = {
        let mut result = 0usize;
        for (i, &pin_idx) in mapped_pins.iter().enumerate() {
            if pin_idx < mapped_config.len() && mapped_config[pin_idx] > 0 {
                result |= 1 << i;
            }
        }
        result
    };

    // Step 3: Look up source config
    let d1 = pattern.mapped_entry_to_compact();
    let d2 = pattern.source_entry_to_configs();

    let compact = d1.get(&bc).copied();
    debug_assert!(
        compact.is_some(),
        "Boundary config {} not found in mapped_entry_to_compact",
        bc
    );
    let compact = compact.unwrap_or(0);

    let source_configs = d2.get(&compact).cloned();
    debug_assert!(
        source_configs.is_some(),
        "Compact {} not found in source_entry_to_configs",
        compact
    );
    let source_configs = source_configs.unwrap_or_default();

    debug_assert!(
        !source_configs.is_empty(),
        "Empty source configs for compact {}.",
        compact
    );
    let new_config = if source_configs.is_empty() {
        vec![false; source_locs.len()]
    } else {
        source_configs[0].clone()
    };

    // Step 4: Clear gadget area
    for row in gi..gi + m {
        for col in gj..gj + n {
            if let Some(row_vec) = config.get_mut(row) {
                if let Some(cell) = row_vec.get_mut(col) {
                    *cell = 0;
                }
            }
        }
    }

    // Step 5: Write source config
    for (k, &(r, c)) in source_locs.iter().enumerate() {
        let row = gi + r - 1;
        let col = gj + c - 1;
        if let Some(rv) = config.get_mut(row) {
            if let Some(cv) = rv.get_mut(col) {
                *cv += if new_config.get(k).copied().unwrap_or(false) {
                    1
                } else {
                    0
                };
            }
        }
    }
}
