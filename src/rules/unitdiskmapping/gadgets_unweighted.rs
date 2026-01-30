//! Unweighted square lattice gadgets for resolving crossings.
//!
//! This module contains all gadget implementations for the square lattice
//! unweighted mapping: Cross, Turn, WTurn, Branch, BranchFix, TCon, TrivialTurn,
//! EndTurn, BranchFixB, DanglingLeg, and their rotated/reflected variants.

use super::gadgets::{apply_gadget, map_config_back_pattern, pattern_matches, Pattern, PatternCell};
use super::grid::{CellState, MappingGrid};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Crossing Gadgets - matching Julia's gadgets.jl exactly
// ============================================================================

/// Crossing gadget for resolving two crossing copy-lines.
///
/// `Cross<true>`: connected crossing (edges share a vertex), size (3,3)
/// `Cross<false>`: disconnected crossing, size (4,5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cross<const CON: bool>;

impl Pattern for Cross<true> {
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
            (5, 5), (12, 12), (8, 0), (1, 0), (0, 0), (6, 6), (11, 11),
            (9, 9), (14, 14), (3, 3), (7, 7), (4, 0), (13, 13), (15, 15),
            (2, 0), (10, 10),
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

impl Pattern for Cross<false> {
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
            (2, 1), (2, 2), (2, 3), (2, 4), (2, 5),
            (1, 3), (2, 3), (3, 3), (4, 3),
        ];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (5, 6), (6, 7), (7, 8)];
        let pins = vec![0, 5, 8, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2, 1), (2, 2), (2, 3), (2, 4), (2, 5),
            (1, 3), (3, 3), (4, 3), (3, 2), (3, 4),
        ];
        let pins = vec![0, 5, 7, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (5, 4), (12, 4), (8, 0), (1, 0), (0, 0), (6, 0), (11, 11),
            (9, 9), (14, 2), (3, 2), (7, 2), (4, 4), (13, 13), (15, 11),
            (2, 2), (10, 2),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![
            vec![false, true, false, true, false, false, false, true, false],
            vec![false, true, false, true, false, false, true, false, false],
        ]);
        map.insert(2, vec![vec![false, true, false, true, false, true, false, true, false]]);
        map.insert(4, vec![vec![false, true, false, true, false, false, true, false, true]]);
        map.insert(9, vec![
            vec![true, false, true, false, true, false, false, true, false],
            vec![true, false, true, false, true, false, true, false, false],
        ]);
        map.insert(11, vec![vec![true, false, true, false, true, true, false, true, false]]);
        map.insert(13, vec![vec![true, false, true, false, true, false, true, false, true]]);
        for i in [1, 3, 5, 6, 7, 8, 10, 12, 14, 15] {
            map.entry(i).or_insert_with(Vec::new);
        }
        map
    }
}

/// Turn gadget for 90-degree turns in copy-lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turn;

impl Pattern for Turn {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (3, 2) }
    fn is_connected(&self) -> bool { false }

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

    fn mis_overhead(&self) -> i32 { -1 }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 0), (3, 3), (1, 0)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, true, false, true, false]]);
        map.insert(1, vec![
            vec![true, false, true, false, false],
            vec![true, false, false, true, false],
        ]);
        map.insert(2, vec![
            vec![false, true, false, false, true],
            vec![false, false, true, false, true],
        ]);
        map.insert(3, vec![vec![true, false, true, false, true]]);
        map
    }
}

/// W-shaped turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WTurn;

impl Pattern for WTurn {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

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

    fn mis_overhead(&self) -> i32 { -1 }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 0), (3, 3), (1, 0)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![true, false, true, false, false]]);
        map.insert(1, vec![
            vec![false, true, false, true, false],
            vec![false, true, true, false, false],
        ]);
        map.insert(2, vec![
            vec![false, false, false, true, true],
            vec![true, false, false, false, true],
        ]);
        map.insert(3, vec![vec![false, true, false, true, true]]);
        map
    }
}

/// Branch gadget for T-junctions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Branch;

impl Pattern for Branch {
    fn size(&self) -> (usize, usize) { (5, 4) }
    fn cross_location(&self) -> (usize, usize) { (3, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1, 2), (2, 2), (3, 2), (3, 3), (3, 4), (4, 3), (4, 2), (5, 2),
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

    fn mis_overhead(&self) -> i32 { -1 }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (0, 0), (4, 0), (5, 5), (6, 6), (2, 0), (7, 7), (3, 3), (1, 0),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, true, false, true, false, false, true, false]]);
        map.insert(3, vec![
            vec![true, false, true, false, true, false, true, false],
            vec![true, false, true, false, true, true, false, false],
        ]);
        map.insert(5, vec![vec![true, false, true, false, false, true, false, true]]);
        map.insert(6, vec![
            vec![false, false, true, false, true, true, false, true],
            vec![false, true, false, false, true, true, false, true],
        ]);
        map.insert(7, vec![vec![true, false, true, false, true, true, false, true]]);
        for i in [1, 2, 4] {
            map.insert(i, vec![]);
        }
        map
    }
}

/// Branch fix gadget for simplifying branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFix;

impl Pattern for BranchFix {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

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

    fn mis_overhead(&self) -> i32 { -1 }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 1), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![
            vec![false, true, false, true, false, false],
            vec![false, true, false, false, true, false],
            vec![false, false, true, false, true, false],
        ]);
        map.insert(1, vec![vec![true, false, true, false, true, false]]);
        map.insert(2, vec![vec![false, true, false, true, false, true]]);
        map.insert(3, vec![
            vec![true, false, false, true, false, true],
            vec![true, false, true, false, false, true],
        ]);
        map
    }
}

/// T-connection gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TCon;

impl Pattern for TCon {
    fn size(&self) -> (usize, usize) { (3, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { true }
    fn connected_nodes(&self) -> Vec<usize> { vec![0, 1] }

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

    fn mis_overhead(&self) -> i32 { 0 }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [
            (0, 0), (4, 0), (5, 5), (6, 6), (2, 2), (7, 7), (3, 3), (1, 0),
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
pub struct TrivialTurn;

impl Pattern for TrivialTurn {
    fn size(&self) -> (usize, usize) { (2, 2) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { true }
    fn connected_nodes(&self) -> Vec<usize> { vec![0, 1] }

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

    fn mis_overhead(&self) -> i32 { 0 }

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
pub struct EndTurn;

impl Pattern for EndTurn {
    fn size(&self) -> (usize, usize) { (3, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

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

    fn mis_overhead(&self) -> i32 { -1 }

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
pub struct BranchFixB;

impl Pattern for BranchFixB {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

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

    fn mis_overhead(&self) -> i32 { -1 }

    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 3), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> {
        let mut map = HashMap::new();
        map.insert(0, vec![vec![false, false, true, false], vec![false, true, false, false]]);
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
pub struct RotatedGadget<G: Pattern> {
    pub gadget: G,
    /// Number of 90-degree clockwise rotations (0-3).
    pub n: usize,
}

impl<G: Pattern> RotatedGadget<G> {
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

impl<G: Pattern> Pattern for RotatedGadget<G> {
    fn size(&self) -> (usize, usize) {
        let (m, n) = self.gadget.size();
        if self.n % 2 == 0 { (m, n) } else { (n, m) }
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
        ((rotated.0 + offset_r) as usize, (rotated.1 + offset_c) as usize)
    }

    fn is_connected(&self) -> bool { self.gadget.is_connected() }
    fn is_cross_gadget(&self) -> bool { self.gadget.is_cross_gadget() }
    fn connected_nodes(&self) -> Vec<usize> { self.gadget.connected_nodes() }

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
                ((rotated.0 + offset_r) as usize, (rotated.1 + offset_c) as usize)
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
                ((rotated.0 + offset_r) as usize, (rotated.1 + offset_c) as usize)
            })
            .collect();
        (new_locs, pins)
    }

    fn mis_overhead(&self) -> i32 { self.gadget.mis_overhead() }
    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> { self.gadget.mapped_entry_to_compact() }
    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> { self.gadget.source_entry_to_configs() }
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
pub struct ReflectedGadget<G: Pattern> {
    pub gadget: G,
    pub mirror: Mirror,
}

impl<G: Pattern> ReflectedGadget<G> {
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

fn reflect_around_center(loc: (usize, usize), center: (usize, usize), mirror: Mirror) -> (i32, i32) {
    let dx = loc.0 as i32 - center.0 as i32;
    let dy = loc.1 as i32 - center.1 as i32;
    let (nx, ny) = reflect((dx, dy), mirror);
    (center.0 as i32 + nx, center.1 as i32 + ny)
}

impl<G: Pattern> Pattern for ReflectedGadget<G> {
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
        ((reflected.0 + offset_r) as usize, (reflected.1 + offset_c) as usize)
    }

    fn is_connected(&self) -> bool { self.gadget.is_connected() }
    fn is_cross_gadget(&self) -> bool { self.gadget.is_cross_gadget() }
    fn connected_nodes(&self) -> Vec<usize> { self.gadget.connected_nodes() }

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
                ((reflected.0 + offset_r) as usize, (reflected.1 + offset_c) as usize)
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
                ((reflected.0 + offset_r) as usize, (reflected.1 + offset_c) as usize)
            })
            .collect();
        (new_locs, pins)
    }

    fn mis_overhead(&self) -> i32 { self.gadget.mis_overhead() }
    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize> { self.gadget.mapped_entry_to_compact() }
    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>> { self.gadget.source_entry_to_configs() }
}

// ============================================================================
// Simplifier Patterns
// ============================================================================

/// Dangling leg simplifier pattern.
///
/// Julia pattern:
/// ```text
/// Source:       Mapped:
/// ⋅ ⋅ ⋅         ⋅ ⋅ ⋅
/// ⋅ ● ⋅    =>   ⋅ ⋅ ⋅
/// ⋅ ● ⋅         ⋅ ⋅ ⋅
/// ⋅ ● ⋅         ⋅ ● ⋅
/// ```
/// Removes 2 nodes from a dangling chain, keeping only the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DanglingLeg;

impl Pattern for DanglingLeg {
    fn size(&self) -> (usize, usize) { (4, 3) }
    // Julia: cross_location = size .÷ 2 = (4÷2, 3÷2) = (2, 1)
    fn cross_location(&self) -> (usize, usize) { (2, 1) }
    fn is_connected(&self) -> bool { false }

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

    fn mis_overhead(&self) -> i32 { -1 }

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
// SquarePattern Enum for Dynamic Dispatch
// ============================================================================

/// Enum wrapping all square lattice patterns for dynamic dispatch during unapply.
#[derive(Debug, Clone)]
pub enum SquarePattern {
    CrossFalse(Cross<false>),
    CrossTrue(Cross<true>),
    Turn(Turn),
    WTurn(WTurn),
    Branch(Branch),
    BranchFix(BranchFix),
    TCon(TCon),
    TrivialTurn(TrivialTurn),
    EndTurn(EndTurn),
    BranchFixB(BranchFixB),
    DanglingLeg(DanglingLeg),
    RotatedTCon1(RotatedGadget<TCon>),
    ReflectedCrossTrue(ReflectedGadget<Cross<true>>),
    ReflectedTrivialTurn(ReflectedGadget<TrivialTurn>),
    ReflectedRotatedTCon1(ReflectedGadget<RotatedGadget<TCon>>),
    DanglingLegRot1(RotatedGadget<DanglingLeg>),
    DanglingLegRot2(RotatedGadget<RotatedGadget<DanglingLeg>>),
    DanglingLegRot3(RotatedGadget<RotatedGadget<RotatedGadget<DanglingLeg>>>),
    DanglingLegReflX(ReflectedGadget<DanglingLeg>),
    DanglingLegReflY(ReflectedGadget<DanglingLeg>),
}

impl SquarePattern {
    /// Get pattern from tape index.
    pub fn from_tape_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::CrossFalse(Cross::<false>)),
            1 => Some(Self::Turn(Turn)),
            2 => Some(Self::WTurn(WTurn)),
            3 => Some(Self::Branch(Branch)),
            4 => Some(Self::BranchFix(BranchFix)),
            5 => Some(Self::TCon(TCon)),
            6 => Some(Self::TrivialTurn(TrivialTurn)),
            7 => Some(Self::RotatedTCon1(RotatedGadget::new(TCon, 1))),
            8 => Some(Self::ReflectedCrossTrue(ReflectedGadget::new(Cross::<true>, Mirror::Y))),
            9 => Some(Self::ReflectedTrivialTurn(ReflectedGadget::new(TrivialTurn, Mirror::Y))),
            10 => Some(Self::BranchFixB(BranchFixB)),
            11 => Some(Self::EndTurn(EndTurn)),
            12 => Some(Self::ReflectedRotatedTCon1(ReflectedGadget::new(RotatedGadget::new(TCon, 1), Mirror::Y))),
            100 => Some(Self::DanglingLeg(DanglingLeg)),
            101 => Some(Self::DanglingLegRot1(RotatedGadget::new(DanglingLeg, 1))),
            102 => Some(Self::DanglingLegRot2(RotatedGadget::new(RotatedGadget::new(DanglingLeg, 1), 1))),
            103 => Some(Self::DanglingLegRot3(RotatedGadget::new(RotatedGadget::new(RotatedGadget::new(DanglingLeg, 1), 1), 1))),
            104 => Some(Self::DanglingLegReflX(ReflectedGadget::new(DanglingLeg, Mirror::X))),
            105 => Some(Self::DanglingLegReflY(ReflectedGadget::new(DanglingLeg, Mirror::Y))),
            _ => None,
        }
    }

    /// Apply map_config_back_pattern for this pattern.
    pub fn map_config_back(&self, gi: usize, gj: usize, config: &mut Vec<Vec<usize>>) {
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
pub struct TapeEntry {
    pub pattern_idx: usize,
    pub row: usize,
    pub col: usize,
}

/// Calculate MIS overhead for a tape entry.
pub fn tape_entry_mis_overhead(entry: &TapeEntry) -> i32 {
    match entry.pattern_idx {
        0 => Cross::<false>.mis_overhead(),
        1 => Turn.mis_overhead(),
        2 => WTurn.mis_overhead(),
        3 => Branch.mis_overhead(),
        4 => BranchFix.mis_overhead(),
        5 => TCon.mis_overhead(),
        6 => TrivialTurn.mis_overhead(),
        7 => RotatedGadget::new(TCon, 1).mis_overhead(),
        8 => ReflectedGadget::new(Cross::<true>, Mirror::Y).mis_overhead(),
        9 => ReflectedGadget::new(TrivialTurn, Mirror::Y).mis_overhead(),
        10 => BranchFixB.mis_overhead(),
        11 => EndTurn.mis_overhead(),
        12 => ReflectedGadget::new(RotatedGadget::new(TCon, 1), Mirror::Y).mis_overhead(),
        100..=105 => DanglingLeg.mis_overhead(),
        _ => 0,
    }
}

/// The default crossing ruleset for square lattice.
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
    copylines: &[super::copyline::CopyLine],
) -> Vec<TapeEntry> {
    let mut tape = Vec::new();
    let n = copylines.len();

    let debug = std::env::var("DEBUG_CROSSING").is_ok();

    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat(grid, copylines, i, j);
            if debug {
                eprintln!("Trying crossat ({}, {}) from copylines[{}][{}]", cross_row, cross_col, i, j);
            }
            if let Some((pattern_idx, row, col)) =
                try_match_and_apply_crossing(grid, cross_row, cross_col)
            {
                if debug {
                    eprintln!("  -> Matched pattern {} at ({}, {})", pattern_idx, row, col);
                }
                tape.push(TapeEntry { pattern_idx, row, col });
            }
        }
    }
    tape
}

/// Calculate crossing point for two copylines.
/// Uses grid.cross_at() which implements Julia's crossat formula.
fn crossat(
    grid: &MappingGrid,
    copylines: &[super::copyline::CopyLine],
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
    let patterns: Vec<(usize, Box<dyn Fn() -> Box<dyn PatternBoxed>>)> = vec![
        (0, Box::new(|| Box::new(Cross::<false>))),
        (1, Box::new(|| Box::new(Turn))),
        (2, Box::new(|| Box::new(WTurn))),
        (3, Box::new(|| Box::new(Branch))),
        (4, Box::new(|| Box::new(BranchFix))),
        (5, Box::new(|| Box::new(TCon))),
        (6, Box::new(|| Box::new(TrivialTurn))),
        (7, Box::new(|| Box::new(RotatedGadget::new(TCon, 1)))),
        (8, Box::new(|| Box::new(ReflectedGadget::new(Cross::<true>, Mirror::Y)))),
        (9, Box::new(|| Box::new(ReflectedGadget::new(TrivialTurn, Mirror::Y)))),
        (10, Box::new(|| Box::new(BranchFixB))),
        (11, Box::new(|| Box::new(EndTurn))),
        (12, Box::new(|| Box::new(ReflectedGadget::new(RotatedGadget::new(TCon, 1), Mirror::Y)))),
    ];

    let debug = std::env::var("DEBUG_CROSSING").is_ok();

    for (idx, make_pattern) in patterns {
        let pattern = make_pattern();
        let cl = pattern.cross_location();
        // cross_row/cross_col are 0-indexed, cl is 1-indexed within gadget
        // x = cross_row - (cl.0 - 1) = cross_row + 1 - cl.0, needs x >= 0
        if cross_row + 1 >= cl.0 && cross_col + 1 >= cl.1 {
            let x = cross_row + 1 - cl.0;
            let y = cross_col + 1 - cl.1;
            if debug && (cross_row == 3 && cross_col == 6) && idx == 7 {
                eprintln!("    Pattern {} cross_loc={:?} -> trying at ({}, {})", idx, cl, x, y);
                // Print the source_matrix directly
                let source = pattern.source_matrix();
                let (m, n) = pattern.size_boxed();
                eprintln!("    Source matrix ({}x{}):", m, n);
                for r in 0..m {
                    let row_str: String = source[r].iter().map(|c| match c {
                        PatternCell::Empty => '.',
                        PatternCell::Occupied => 'O',
                        PatternCell::Connected => 'C',
                        PatternCell::Doubled => 'D',
                    }).collect();
                    eprintln!("      Row {}: {}", r, row_str);
                }
                eprintln!("    Grid at position ({}, {}):", x, y);
                for r in 0..m {
                    let row_str: String = (0..n).map(|c| {
                        let gr = x + r;
                        let gc = y + c;
                        match safe_get_pattern_cell(grid, gr, gc) {
                            PatternCell::Empty => '.',
                            PatternCell::Occupied => 'O',
                            PatternCell::Connected => 'C',
                            PatternCell::Doubled => 'D',
                        }
                    }).collect();
                    eprintln!("      Row {}: {}", r, row_str);
                }
            }
            let matches = pattern.pattern_matches_boxed(grid, x, y);
            if debug && (cross_row == 3 && cross_col == 6) && idx == 7 {
                eprintln!("    Pattern {} at ({}, {}) -> matches={}", idx, x, y, matches);
            }
            if matches {
                pattern.apply_gadget_boxed(grid, x, y);
                return Some((idx, x, y));
            }
        }
    }
    None
}

/// Apply simplifier gadgets (DanglingLeg variants).
/// `nrepeat` specifies the number of simplification passes.
pub fn apply_simplifier_gadgets(grid: &mut MappingGrid, nrepeat: usize) -> Vec<TapeEntry> {
    let mut tape = Vec::new();
    let (rows, cols) = grid.size();

    // Get all rotations and reflections of DanglingLeg
    let patterns = rotated_and_reflected_danglinleg();

    for _ in 0..nrepeat {
        for (pattern_idx, pattern) in patterns.iter().enumerate() {
            for j in 0..cols {
                for i in 0..rows {
                    if pattern_matches_boxed(pattern.as_ref(), grid, i, j) {
                        apply_gadget_boxed(pattern.as_ref(), grid, i, j);
                        tape.push(TapeEntry {
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

fn rotated_and_reflected_danglinleg() -> Vec<Box<dyn PatternBoxed>> {
    vec![
        Box::new(DanglingLeg),
        Box::new(RotatedGadget::new(DanglingLeg, 1)),
        Box::new(RotatedGadget::new(DanglingLeg, 2)),
        Box::new(RotatedGadget::new(DanglingLeg, 3)),
        Box::new(ReflectedGadget::new(DanglingLeg, Mirror::X)),
        Box::new(ReflectedGadget::new(DanglingLeg, Mirror::Y)),
    ]
}

/// Check if a boxed pattern matches at position (i, j) in the grid.
#[allow(clippy::needless_range_loop)]
fn pattern_matches_boxed(pattern: &dyn PatternBoxed, grid: &MappingGrid, i: usize, j: usize) -> bool {
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
fn apply_gadget_boxed(pattern: &dyn PatternBoxed, grid: &mut MappingGrid, i: usize, j: usize) {
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

/// Trait for boxed pattern operations.
pub trait PatternBoxed {
    fn size_boxed(&self) -> (usize, usize);
    fn cross_location(&self) -> (usize, usize);
    fn source_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn pattern_matches_boxed(&self, grid: &MappingGrid, i: usize, j: usize) -> bool;
    fn apply_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize);
}

impl<P: Pattern> PatternBoxed for P {
    fn size_boxed(&self) -> (usize, usize) { self.size() }
    fn cross_location(&self) -> (usize, usize) { Pattern::cross_location(self) }
    fn source_matrix(&self) -> Vec<Vec<PatternCell>> { Pattern::source_matrix(self) }
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>> { Pattern::mapped_matrix(self) }
    fn pattern_matches_boxed(&self, grid: &MappingGrid, i: usize, j: usize) -> bool {
        pattern_matches(self, grid, i, j)
    }
    fn apply_gadget_boxed(&self, grid: &mut MappingGrid, i: usize, j: usize) {
        apply_gadget(self, grid, i, j);
    }
}
