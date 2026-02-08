//! KSG weighted square lattice gadgets for resolving crossings.
//!
//! This module contains weighted gadget implementations for the King's SubGraph (KSG)
//! weighted mapping. Each weighted gadget implements the Pattern trait directly with
//! actual weight methods, following Julia's formula: mis_overhead(weighted) = mis_overhead(unweighted) * 2.

use super::super::grid::{CellState, MappingGrid};
use super::super::traits::{apply_gadget, pattern_matches, Pattern, PatternCell};
use super::gadgets::{KsgReflectedGadget, KsgRotatedGadget, Mirror};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for weighted pattern factory function used in crossing gadget matching.
type WeightedPatternFactory = Box<dyn Fn() -> Box<dyn WeightedKsgPatternBoxed>>;

/// Type alias for source graph representation: (locations, pin_edges, source_pins).
pub type SourceGraph = (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>);

// ============================================================================
// Weighted Crossing Gadgets
// ============================================================================

/// Weighted crossing gadget for resolving two crossing copy-lines.
///
/// `WeightedKsgCross<true>`: connected crossing (edges share a vertex), size (3,3)
/// `WeightedKsgCross<false>`: disconnected crossing, size (4,5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgCross<const CON: bool>;

impl Pattern for WeightedKsgCross<true> {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 6]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 5]
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

impl Pattern for WeightedKsgCross<false> {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 9]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 10]
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

/// Weighted turn gadget for 90-degree turns in copy-lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgTurn;

impl Pattern for WeightedKsgTurn {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 5]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 3]
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

/// Weighted W-shaped turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgWTurn;

impl Pattern for WeightedKsgWTurn {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 5]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 3]
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

/// Weighted branch gadget for T-junctions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgBranch;

impl Pattern for WeightedKsgBranch {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    // Weighted version: node 3 (0-indexed) has weight 3, others have weight 2
    fn source_weights(&self) -> Vec<i32> {
        vec![2, 2, 2, 3, 2, 2, 2, 2]
    }

    // Weighted version: node 1 (0-indexed) has weight 3, others have weight 2
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

/// Weighted branch fix gadget for simplifying branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgBranchFix;

impl Pattern for WeightedKsgBranchFix {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 6]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 4]
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

/// Weighted T-connection gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgTCon;

impl Pattern for WeightedKsgTCon {
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
        0 // 2x unweighted value (0 * 2)
    }

    // Weighted version: node 1 (0-indexed) has weight 1, others have weight 2
    fn source_weights(&self) -> Vec<i32> {
        vec![2, 1, 2, 2]
    }

    // Weighted version: node 1 (0-indexed) has weight 1, others have weight 2
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

/// Weighted trivial turn gadget for simple diagonal turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgTrivialTurn;

impl Pattern for WeightedKsgTrivialTurn {
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
        0 // 2x unweighted value (0 * 2)
    }

    // Weighted version: both nodes have weight 1
    fn source_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }

    // Weighted version: both nodes have weight 1
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

/// Weighted end turn gadget for line terminations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgEndTurn;

impl Pattern for WeightedKsgEndTurn {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    // Weighted version: node 2 (0-indexed) has weight 1, others have weight 2
    fn source_weights(&self) -> Vec<i32> {
        vec![2, 2, 1]
    }

    // Weighted version: node 0 (0-indexed) has weight 1
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

/// Weighted alternate branch fix gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgBranchFixB;

impl Pattern for WeightedKsgBranchFixB {
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
        -2 // 2x unweighted value (-1 * 2)
    }

    // Weighted version: node 0 (0-indexed) has weight 1, others have weight 2
    fn source_weights(&self) -> Vec<i32> {
        vec![1, 2, 2, 2]
    }

    // Weighted version: node 0 (0-indexed) has weight 1, node 1 has weight 2
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

/// Weighted dangling leg simplifier pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedKsgDanglingLeg;

impl Pattern for WeightedKsgDanglingLeg {
    fn size(&self) -> (usize, usize) {
        (4, 3)
    }
    fn cross_location(&self) -> (usize, usize) {
        (2, 1)
    }
    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 2), (3, 2), (4, 2)];
        let edges = vec![(0, 1), (1, 2)];
        let pins = vec![2];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(4, 2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2 // 2x unweighted value (-1 * 2)
    }

    // Weighted version: node 0 (0-indexed) has weight 1, others have weight 2
    fn source_weights(&self) -> Vec<i32> {
        vec![1, 2, 2]
    }

    // Weighted version: node 0 (0-indexed) has weight 1
    fn mapped_weights(&self) -> Vec<i32> {
        vec![1]
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![true, false, false], vec![false, true, false]]);
        map.insert(1, vec![vec![true, false, true]]);
        map
    }
}

// ============================================================================
// WeightedKsgPattern Enum for Dynamic Dispatch
// ============================================================================

/// Enum wrapping all weighted KSG square lattice patterns for dynamic dispatch during unapply.
#[derive(Debug, Clone)]
pub enum WeightedKsgPattern {
    CrossFalse(WeightedKsgCross<false>),
    CrossTrue(WeightedKsgCross<true>),
    Turn(WeightedKsgTurn),
    WTurn(WeightedKsgWTurn),
    Branch(WeightedKsgBranch),
    BranchFix(WeightedKsgBranchFix),
    TCon(WeightedKsgTCon),
    TrivialTurn(WeightedKsgTrivialTurn),
    EndTurn(WeightedKsgEndTurn),
    BranchFixB(WeightedKsgBranchFixB),
    DanglingLeg(WeightedKsgDanglingLeg),
    RotatedTCon1(KsgRotatedGadget<WeightedKsgTCon>),
    ReflectedCrossTrue(KsgReflectedGadget<WeightedKsgCross<true>>),
    ReflectedTrivialTurn(KsgReflectedGadget<WeightedKsgTrivialTurn>),
    ReflectedRotatedTCon1(KsgReflectedGadget<KsgRotatedGadget<WeightedKsgTCon>>),
    DanglingLegRot1(KsgRotatedGadget<WeightedKsgDanglingLeg>),
    DanglingLegRot2(KsgRotatedGadget<KsgRotatedGadget<WeightedKsgDanglingLeg>>),
    DanglingLegRot3(KsgRotatedGadget<KsgRotatedGadget<KsgRotatedGadget<WeightedKsgDanglingLeg>>>),
    DanglingLegReflX(KsgReflectedGadget<WeightedKsgDanglingLeg>),
    DanglingLegReflY(KsgReflectedGadget<WeightedKsgDanglingLeg>),
}

impl WeightedKsgPattern {
    /// Get pattern from tape index.
    pub fn from_tape_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::CrossFalse(WeightedKsgCross::<false>)),
            1 => Some(Self::Turn(WeightedKsgTurn)),
            2 => Some(Self::WTurn(WeightedKsgWTurn)),
            3 => Some(Self::Branch(WeightedKsgBranch)),
            4 => Some(Self::BranchFix(WeightedKsgBranchFix)),
            5 => Some(Self::TCon(WeightedKsgTCon)),
            6 => Some(Self::TrivialTurn(WeightedKsgTrivialTurn)),
            7 => Some(Self::RotatedTCon1(KsgRotatedGadget::new(
                WeightedKsgTCon,
                1,
            ))),
            8 => Some(Self::ReflectedCrossTrue(KsgReflectedGadget::new(
                WeightedKsgCross::<true>,
                Mirror::Y,
            ))),
            9 => Some(Self::ReflectedTrivialTurn(KsgReflectedGadget::new(
                WeightedKsgTrivialTurn,
                Mirror::Y,
            ))),
            10 => Some(Self::BranchFixB(WeightedKsgBranchFixB)),
            11 => Some(Self::EndTurn(WeightedKsgEndTurn)),
            12 => Some(Self::ReflectedRotatedTCon1(KsgReflectedGadget::new(
                KsgRotatedGadget::new(WeightedKsgTCon, 1),
                Mirror::Y,
            ))),
            100 => Some(Self::DanglingLeg(WeightedKsgDanglingLeg)),
            101 => Some(Self::DanglingLegRot1(KsgRotatedGadget::new(
                WeightedKsgDanglingLeg,
                1,
            ))),
            102 => Some(Self::DanglingLegRot2(KsgRotatedGadget::new(
                KsgRotatedGadget::new(WeightedKsgDanglingLeg, 1),
                1,
            ))),
            103 => Some(Self::DanglingLegRot3(KsgRotatedGadget::new(
                KsgRotatedGadget::new(KsgRotatedGadget::new(WeightedKsgDanglingLeg, 1), 1),
                1,
            ))),
            104 => Some(Self::DanglingLegReflX(KsgReflectedGadget::new(
                WeightedKsgDanglingLeg,
                Mirror::X,
            ))),
            105 => Some(Self::DanglingLegReflY(KsgReflectedGadget::new(
                WeightedKsgDanglingLeg,
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
// Weighted Tape Entry and Apply Functions
// ============================================================================

/// A tape entry recording a weighted gadget application.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeightedKsgTapeEntry {
    pub pattern_idx: usize,
    pub row: usize,
    pub col: usize,
}

/// Calculate MIS overhead for a weighted tape entry.
pub fn weighted_tape_entry_mis_overhead(entry: &WeightedKsgTapeEntry) -> i32 {
    match entry.pattern_idx {
        0 => WeightedKsgCross::<false>.mis_overhead(),
        1 => WeightedKsgTurn.mis_overhead(),
        2 => WeightedKsgWTurn.mis_overhead(),
        3 => WeightedKsgBranch.mis_overhead(),
        4 => WeightedKsgBranchFix.mis_overhead(),
        5 => WeightedKsgTCon.mis_overhead(),
        6 => WeightedKsgTrivialTurn.mis_overhead(),
        7 => KsgRotatedGadget::new(WeightedKsgTCon, 1).mis_overhead(),
        8 => KsgReflectedGadget::new(WeightedKsgCross::<true>, Mirror::Y).mis_overhead(),
        9 => KsgReflectedGadget::new(WeightedKsgTrivialTurn, Mirror::Y).mis_overhead(),
        10 => WeightedKsgBranchFixB.mis_overhead(),
        11 => WeightedKsgEndTurn.mis_overhead(),
        12 => KsgReflectedGadget::new(KsgRotatedGadget::new(WeightedKsgTCon, 1), Mirror::Y)
            .mis_overhead(),
        100..=105 => WeightedKsgDanglingLeg.mis_overhead(),
        _ => 0,
    }
}

/// Trait for boxed weighted pattern operations.
pub trait WeightedKsgPatternBoxed {
    fn size_boxed(&self) -> (usize, usize);
    fn cross_location(&self) -> (usize, usize);
    fn source_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn source_graph_boxed(&self) -> SourceGraph;
    fn mapped_graph_boxed(&self) -> (Vec<(usize, usize)>, Vec<usize>);
    fn source_weights_boxed(&self) -> Vec<i32>;
    fn mapped_weights_boxed(&self) -> Vec<i32>;
    fn pattern_matches_boxed(&self, grid: &MappingGrid, i: usize, j: usize) -> bool;
    fn apply_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize);
    fn apply_weighted_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize);
}

impl<P: Pattern> WeightedKsgPatternBoxed for P {
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
    fn mapped_graph_boxed(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        Pattern::mapped_graph(self)
    }
    fn source_weights_boxed(&self) -> Vec<i32> {
        Pattern::source_weights(self)
    }
    fn mapped_weights_boxed(&self) -> Vec<i32> {
        Pattern::mapped_weights(self)
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

/// Apply all weighted crossing gadgets to the grid.
pub fn apply_weighted_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::super::copyline::CopyLine],
) -> Vec<WeightedKsgTapeEntry> {
    let mut tape = Vec::new();
    let n = copylines.len();

    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat(grid, copylines, i, j);
            if let Some((pattern_idx, row, col)) =
                try_match_and_apply_weighted_crossing(grid, cross_row, cross_col)
            {
                tape.push(WeightedKsgTapeEntry {
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
            grid.cross_at(line_first.vslot, line_second.vslot, line_first.hslot)
        }
        _ => (0, 0),
    }
}

fn try_match_and_apply_weighted_crossing(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<(usize, usize, usize)> {
    // Try each pattern in order
    let patterns: Vec<(usize, WeightedPatternFactory)> = vec![
        (0, Box::new(|| Box::new(WeightedKsgCross::<false>))),
        (1, Box::new(|| Box::new(WeightedKsgTurn))),
        (2, Box::new(|| Box::new(WeightedKsgWTurn))),
        (3, Box::new(|| Box::new(WeightedKsgBranch))),
        (4, Box::new(|| Box::new(WeightedKsgBranchFix))),
        (5, Box::new(|| Box::new(WeightedKsgTCon))),
        (6, Box::new(|| Box::new(WeightedKsgTrivialTurn))),
        (
            7,
            Box::new(|| Box::new(KsgRotatedGadget::new(WeightedKsgTCon, 1))),
        ),
        (
            8,
            Box::new(|| Box::new(KsgReflectedGadget::new(WeightedKsgCross::<true>, Mirror::Y))),
        ),
        (
            9,
            Box::new(|| Box::new(KsgReflectedGadget::new(WeightedKsgTrivialTurn, Mirror::Y))),
        ),
        (10, Box::new(|| Box::new(WeightedKsgBranchFixB))),
        (11, Box::new(|| Box::new(WeightedKsgEndTurn))),
        (
            12,
            Box::new(|| {
                Box::new(KsgReflectedGadget::new(
                    KsgRotatedGadget::new(WeightedKsgTCon, 1),
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

/// Apply weighted simplifier gadgets (WeightedKsgDanglingLeg variants).
pub fn apply_weighted_simplifier_gadgets(
    grid: &mut MappingGrid,
    nrepeat: usize,
) -> Vec<WeightedKsgTapeEntry> {
    let mut tape = Vec::new();
    let (rows, cols) = grid.size();

    let patterns = rotated_and_reflected_weighted_danglingleg();

    for _ in 0..nrepeat {
        for (pattern_idx, pattern) in patterns.iter().enumerate() {
            for j in 0..cols {
                for i in 0..rows {
                    if pattern_matches_weighted(pattern.as_ref(), grid, i, j) {
                        pattern.apply_weighted_gadget_boxed(grid, i, j);
                        tape.push(WeightedKsgTapeEntry {
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
/// For weighted mode, the center node must have weight 1.
fn pattern_matches_weighted(
    pattern: &dyn WeightedKsgPatternBoxed,
    grid: &MappingGrid,
    i: usize,
    j: usize,
) -> bool {
    // First check basic pattern match
    if !pattern.pattern_matches_boxed(grid, i, j) {
        return false;
    }

    // Check that source weights match the grid weights
    let (locs, _, _) = pattern.source_graph_boxed();
    let source_weights = pattern.source_weights_boxed();

    for (idx, (loc_r, loc_c)) in locs.iter().enumerate() {
        let grid_r = i + loc_r - 1;
        let grid_c = j + loc_c - 1;
        if let Some(cell) = grid.get(grid_r, grid_c) {
            let expected_weight = source_weights.get(idx).copied().unwrap_or(2);
            if cell.weight() != expected_weight {
                return false;
            }
        }
    }

    true
}

fn rotated_and_reflected_weighted_danglingleg() -> Vec<Box<dyn WeightedKsgPatternBoxed>> {
    vec![
        Box::new(WeightedKsgDanglingLeg),
        Box::new(KsgRotatedGadget::new(WeightedKsgDanglingLeg, 1)),
        Box::new(KsgRotatedGadget::new(WeightedKsgDanglingLeg, 2)),
        Box::new(KsgRotatedGadget::new(WeightedKsgDanglingLeg, 3)),
        Box::new(KsgReflectedGadget::new(WeightedKsgDanglingLeg, Mirror::X)),
        Box::new(KsgReflectedGadget::new(WeightedKsgDanglingLeg, Mirror::Y)),
    ]
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

#[cfg(test)]
#[path = "../../../unit_tests/rules/unitdiskmapping/ksg/gadgets_weighted.rs"]
mod tests;
