//! Gadgets for resolving crossings in grid graph embeddings.
//!
//! A gadget transforms a pattern in the source graph to an equivalent pattern
//! in the mapped graph, preserving MIS properties. Gadgets are the building
//! blocks for resolving crossings when copy-lines intersect.

use serde::{Deserialize, Serialize};

/// A gadget pattern that transforms source configurations to mapped configurations.
pub trait Gadget: Clone {
    /// Size of the gadget pattern (rows, cols).
    fn size(&self) -> (usize, usize);

    /// Cross location within the gadget.
    fn cross_location(&self) -> (usize, usize);

    /// Whether this gadget involves connected nodes.
    fn is_connected(&self) -> bool;

    /// Source graph: (locations, pin_indices).
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// Mapped graph: (locations, pin_indices).
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// MIS overhead when applying this gadget.
    fn mis_overhead(&self) -> i32;
}

/// Crossing gadget for resolving two crossing copy-lines.
///
/// `Cross<true>`: connected crossing, size (3,3), overhead 0
/// `Cross<false>`: disconnected crossing, size (4,5), overhead 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cross<const CON: bool>;

impl<const CON: bool> Gadget for Cross<CON> {
    fn size(&self) -> (usize, usize) {
        if CON {
            (3, 3)
        } else {
            (4, 5)
        }
    }

    fn cross_location(&self) -> (usize, usize) {
        if CON {
            (1, 1)
        } else {
            (1, 2)
        }
    }

    fn is_connected(&self) -> bool {
        CON
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        if CON {
            // Connected crossing: single cross point with 4 pins
            let locations = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
            let pins = vec![0, 1, 3, 4]; // top, left, right, bottom
            (locations, pins)
        } else {
            // Disconnected crossing: two separate lines crossing
            let locations = vec![
                (0, 2), // top pin (vertical line)
                (1, 0), // left pin (horizontal line)
                (1, 4), // right pin (horizontal line)
                (3, 2), // bottom pin (vertical line)
            ];
            let pins = vec![0, 1, 2, 3];
            (locations, pins)
        }
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        if CON {
            // Connected: same as source
            let locations = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
            let pins = vec![0, 1, 3, 4];
            (locations, pins)
        } else {
            // Disconnected: elaborate crossing gadget
            let locations = vec![
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 2),
            ];
            let pins = vec![0, 1, 5, 9]; // top, left, right, bottom
            (locations, pins)
        }
    }

    fn mis_overhead(&self) -> i32 {
        if CON {
            0
        } else {
            1
        }
    }
}

/// Turn gadget for 90-degree turns in copy-lines.
///
/// Size (4,4), overhead 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turn;

impl Gadget for Turn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (1, 1)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // L-shaped path with two pins
        let locations = vec![(0, 1), (1, 1), (1, 2), (1, 3)];
        let pins = vec![0, 3]; // top and right
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Expanded turn with additional nodes
        let locations = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (3, 1),
        ];
        let pins = vec![0, 4]; // top and right
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }
}

/// Branch gadget for T-junctions.
///
/// Size (5,4), overhead 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Branch;

impl Gadget for Branch {
    fn size(&self) -> (usize, usize) {
        (5, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 1)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // T-shape: vertical line with horizontal branch
        let locations = vec![(0, 1), (1, 1), (2, 1), (2, 2), (2, 3), (3, 1), (4, 1)];
        let pins = vec![0, 4, 6]; // top, right, bottom
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Expanded T-junction
        let locations = vec![
            (0, 1),
            (1, 1),
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 3),
            (3, 0),
            (3, 1),
            (4, 1),
        ];
        let pins = vec![0, 5, 8]; // top, right, bottom
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// Branch fix gadget for simplifying branches.
///
/// Size (4,4), overhead 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFix;

impl Gadget for BranchFix {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (1, 1)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![(0, 1), (1, 1), (1, 2), (1, 3), (2, 1), (3, 1)];
        let pins = vec![0, 3, 5]; // top, right, bottom
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (3, 1),
        ];
        let pins = vec![0, 4, 7]; // top, right, bottom
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }
}

/// W-shaped turn gadget.
///
/// Size (4,4), overhead 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WTurn;

impl Gadget for WTurn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (1, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // W-shape path
        let locations = vec![(0, 0), (1, 0), (1, 1), (1, 2), (1, 3)];
        let pins = vec![0, 4]; // top-left and right
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![
            (0, 0),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 2),
            (3, 2),
        ];
        let pins = vec![0, 4]; // top-left and right
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }
}

/// T-connection gadget.
///
/// Size (3,4), overhead 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TCon;

impl Gadget for TCon {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (1, 1)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // T-connection pattern
        let locations = vec![(0, 1), (1, 0), (1, 1), (1, 2), (1, 3)];
        let pins = vec![0, 1, 4]; // top, left, right
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 2),
        ];
        let pins = vec![0, 1, 4]; // top, left, right
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }
}

/// Trivial turn gadget for simple diagonal turns.
///
/// Size (2,2), overhead 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrivialTurn;

impl Gadget for TrivialTurn {
    fn size(&self) -> (usize, usize) {
        (2, 2)
    }

    fn cross_location(&self) -> (usize, usize) {
        (0, 0)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Simple L-shape
        let locations = vec![(0, 0), (0, 1), (1, 0)];
        let pins = vec![1, 2]; // right and bottom
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Same as source for trivial turn
        let locations = vec![(0, 0), (0, 1), (1, 0)];
        let pins = vec![1, 2]; // right and bottom
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// End turn gadget for line terminations.
///
/// Size (3,4), overhead 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndTurn;

impl Gadget for EndTurn {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (1, 1)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // End of a line with a turn
        let locations = vec![(0, 1), (1, 1), (1, 2), (1, 3)];
        let pins = vec![0, 3]; // top and right
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
        ];
        let pins = vec![0, 4]; // top and right
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }
}

/// Alternate branch fix gadget.
///
/// Size (4,4), overhead 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFixB;

impl Gadget for BranchFixB {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 1)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![(0, 1), (1, 1), (2, 1), (2, 2), (2, 3), (3, 1)];
        let pins = vec![0, 4, 5]; // top, right, bottom
        (locations, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locations = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 3),
            (3, 1),
        ];
        let pins = vec![0, 6, 7]; // top, right, bottom
        (locations, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }
}

/// The default crossing ruleset for square lattice.
///
/// Returns a vector of boxed gadgets that can be used to resolve
/// crossings in grid graph embeddings.
pub fn crossing_ruleset_square() -> Vec<Box<dyn GadgetBoxed>> {
    vec![
        Box::new(Cross::<false>),
        Box::new(Turn),
        Box::new(WTurn),
        Box::new(Branch),
        Box::new(BranchFix),
        Box::new(TCon),
        Box::new(TrivialTurn),
        Box::new(EndTurn),
        Box::new(BranchFixB),
    ]
}

/// Helper trait for boxing gadgets with object safety.
pub trait GadgetBoxed {
    /// Size of the gadget pattern (rows, cols).
    fn size(&self) -> (usize, usize);

    /// Cross location within the gadget.
    fn cross_location(&self) -> (usize, usize);

    /// Whether this gadget involves connected nodes.
    fn is_connected(&self) -> bool;

    /// Source graph: (locations, pin_indices).
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// Mapped graph: (locations, pin_indices).
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// MIS overhead when applying this gadget.
    fn mis_overhead(&self) -> i32;
}

impl<T: Gadget> GadgetBoxed for T {
    fn size(&self) -> (usize, usize) {
        Gadget::size(self)
    }

    fn cross_location(&self) -> (usize, usize) {
        Gadget::cross_location(self)
    }

    fn is_connected(&self) -> bool {
        Gadget::is_connected(self)
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        Gadget::source_graph(self)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        Gadget::mapped_graph(self)
    }

    fn mis_overhead(&self) -> i32 {
        Gadget::mis_overhead(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_gadget_size() {
        let cross = Cross::<false>;
        assert_eq!(Gadget::size(&cross), (4, 5));

        let cross_con = Cross::<true>;
        assert_eq!(Gadget::size(&cross_con), (3, 3));
    }

    #[test]
    fn test_turn_gadget() {
        let turn = Turn;
        assert_eq!(Gadget::size(&turn), (4, 4));
        let (locs, pins) = Gadget::source_graph(&turn);
        assert_eq!(pins.len(), 2);
        assert!(!locs.is_empty());
    }

    #[test]
    fn test_gadget_mis_overhead() {
        assert_eq!(Gadget::mis_overhead(&Cross::<false>), 1);
        assert_eq!(Gadget::mis_overhead(&Cross::<true>), 0);
        assert_eq!(Gadget::mis_overhead(&Turn), 1);
    }

    #[test]
    fn test_branch_gadget() {
        let branch = Branch;
        assert_eq!(Gadget::size(&branch), (5, 4));
        assert_eq!(Gadget::mis_overhead(&branch), 0);
        let (_, pins) = Gadget::source_graph(&branch);
        assert_eq!(pins.len(), 3); // T-junction has 3 pins
    }

    #[test]
    fn test_trivial_turn_gadget() {
        let trivial = TrivialTurn;
        assert_eq!(Gadget::size(&trivial), (2, 2));
        assert_eq!(Gadget::mis_overhead(&trivial), 0);
        assert!(Gadget::is_connected(&trivial));
    }

    #[test]
    fn test_all_gadgets_have_valid_pins() {
        // Verify that all pin indices are within bounds for all gadgets
        let gadgets: Vec<Box<dyn GadgetBoxed>> = vec![
            Box::new(Cross::<false>),
            Box::new(Cross::<true>),
            Box::new(Turn),
            Box::new(Branch),
            Box::new(BranchFix),
            Box::new(WTurn),
            Box::new(TCon),
            Box::new(TrivialTurn),
            Box::new(EndTurn),
            Box::new(BranchFixB),
        ];

        for gadget in gadgets {
            let (source_locs, source_pins) = gadget.source_graph();
            let (mapped_locs, mapped_pins) = gadget.mapped_graph();

            for &pin in &source_pins {
                assert!(
                    pin < source_locs.len(),
                    "Source pin {} out of bounds (len={})",
                    pin,
                    source_locs.len()
                );
            }

            for &pin in &mapped_pins {
                assert!(
                    pin < mapped_locs.len(),
                    "Mapped pin {} out of bounds (len={})",
                    pin,
                    mapped_locs.len()
                );
            }
        }
    }

    #[test]
    fn test_crossing_ruleset_square() {
        let ruleset = crossing_ruleset_square();
        assert_eq!(ruleset.len(), 9);

        // Check that Cross<false> is first (most common case)
        assert_eq!(ruleset[0].size(), (4, 5));
    }

    #[test]
    fn test_cross_connected_vs_disconnected() {
        let connected = Cross::<true>;
        let disconnected = Cross::<false>;

        assert!(Gadget::is_connected(&connected));
        assert!(!Gadget::is_connected(&disconnected));

        assert_eq!(Gadget::size(&connected), (3, 3));
        assert_eq!(Gadget::size(&disconnected), (4, 5));
    }

    #[test]
    fn test_gadget_serialization() {
        let turn = Turn;
        let json = serde_json::to_string(&turn).unwrap();
        let deserialized: Turn = serde_json::from_str(&json).unwrap();
        assert_eq!(turn, deserialized);

        let cross: Cross<false> = Cross::<false>;
        let json = serde_json::to_string(&cross).unwrap();
        let deserialized: Cross<false> = serde_json::from_str(&json).unwrap();
        assert_eq!(cross, deserialized);
    }

    #[test]
    fn test_tcon_gadget() {
        let tcon = TCon;
        assert_eq!(Gadget::size(&tcon), (3, 4));
        assert_eq!(Gadget::mis_overhead(&tcon), 1);
        let (_, pins) = Gadget::source_graph(&tcon);
        assert_eq!(pins.len(), 3);
    }

    #[test]
    fn test_wturn_gadget() {
        let wturn = WTurn;
        assert_eq!(Gadget::size(&wturn), (4, 4));
        assert_eq!(Gadget::mis_overhead(&wturn), 1);
        let (_, pins) = Gadget::source_graph(&wturn);
        assert_eq!(pins.len(), 2);
    }

    #[test]
    fn test_endturn_gadget() {
        let endturn = EndTurn;
        assert_eq!(Gadget::size(&endturn), (3, 4));
        assert_eq!(Gadget::mis_overhead(&endturn), 1);
        let (_, pins) = Gadget::source_graph(&endturn);
        assert_eq!(pins.len(), 2);
    }

    #[test]
    fn test_branchfix_gadgets() {
        let bf = BranchFix;
        assert_eq!(Gadget::size(&bf), (4, 4));
        assert_eq!(Gadget::mis_overhead(&bf), 1);

        let bfb = BranchFixB;
        assert_eq!(Gadget::size(&bfb), (4, 4));
        assert_eq!(Gadget::mis_overhead(&bfb), 1);
    }
}
