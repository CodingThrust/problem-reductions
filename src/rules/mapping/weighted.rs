//! Weighted gadget support for triangular lattice mapping.

use super::map_graph::MappingResult;
use super::triangular::{
    TriBranch, TriBranchFix, TriBranchFixB, TriCross, TriEndTurn, TriTConDown, TriTConLeft,
    TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn, TriWTurn,
};
use serde::{Deserialize, Serialize};

/// Weighted gadget wrapper that adds weight vectors to base gadgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedGadget<G> {
    /// The underlying gadget.
    pub gadget: G,
    /// Weights for each node in the source graph.
    pub source_weights: Vec<i32>,
    /// Weights for each node in the mapped graph.
    pub mapped_weights: Vec<i32>,
}

impl<G> WeightedGadget<G> {
    /// Create a new weighted gadget.
    pub fn new(gadget: G, source_weights: Vec<i32>, mapped_weights: Vec<i32>) -> Self {
        Self {
            gadget,
            source_weights,
            mapped_weights,
        }
    }

    /// Get the source weights.
    pub fn source_weights(&self) -> &[i32] {
        &self.source_weights
    }

    /// Get the mapped weights.
    pub fn mapped_weights(&self) -> &[i32] {
        &self.mapped_weights
    }
}

/// Trait for gadgets that can be converted to weighted versions.
pub trait Weightable: Sized {
    /// Convert to a weighted gadget with appropriate weight vectors.
    fn weighted(self) -> WeightedGadget<Self>;
}

// NOTE: All Weightable implementations delegate to TriangularGadget trait methods
// to ensure consistency between the gadget structure and its weights.

impl Weightable for TriTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriTurn.source_weights(), TriTurn.mapped_weights())
    }
}

impl Weightable for TriBranch {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriBranch.source_weights(), TriBranch.mapped_weights())
    }
}

impl Weightable for TriCross<true> {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriCross::<true>.source_weights(), TriCross::<true>.mapped_weights())
    }
}

impl Weightable for TriCross<false> {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriCross::<false>.source_weights(), TriCross::<false>.mapped_weights())
    }
}

impl Weightable for TriTConLeft {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriTConLeft.source_weights(), TriTConLeft.mapped_weights())
    }
}

impl Weightable for TriTConDown {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriTConDown.source_weights(), TriTConDown.mapped_weights())
    }
}

impl Weightable for TriTConUp {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriTConUp.source_weights(), TriTConUp.mapped_weights())
    }
}

impl Weightable for TriTrivialTurnLeft {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriTrivialTurnLeft.source_weights(), TriTrivialTurnLeft.mapped_weights())
    }
}

impl Weightable for TriTrivialTurnRight {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriTrivialTurnRight.source_weights(), TriTrivialTurnRight.mapped_weights())
    }
}

impl Weightable for TriEndTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriEndTurn.source_weights(), TriEndTurn.mapped_weights())
    }
}

impl Weightable for TriWTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriWTurn.source_weights(), TriWTurn.mapped_weights())
    }
}

impl Weightable for TriBranchFix {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriBranchFix.source_weights(), TriBranchFix.mapped_weights())
    }
}

impl Weightable for TriBranchFixB {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(self, TriBranchFixB.source_weights(), TriBranchFixB.mapped_weights())
    }
}

/// Enum wrapper for weighted triangular gadgets to enable dynamic dispatch.
#[derive(Debug, Clone)]
pub enum WeightedTriangularGadget {
    CrossFalse(WeightedGadget<TriCross<false>>),
    CrossTrue(WeightedGadget<TriCross<true>>),
    TConLeft(WeightedGadget<TriTConLeft>),
    TConUp(WeightedGadget<TriTConUp>),
    TConDown(WeightedGadget<TriTConDown>),
    TrivialTurnLeft(WeightedGadget<TriTrivialTurnLeft>),
    TrivialTurnRight(WeightedGadget<TriTrivialTurnRight>),
    EndTurn(WeightedGadget<TriEndTurn>),
    Turn(WeightedGadget<TriTurn>),
    WTurn(WeightedGadget<TriWTurn>),
    BranchFix(WeightedGadget<TriBranchFix>),
    BranchFixB(WeightedGadget<TriBranchFixB>),
    Branch(WeightedGadget<TriBranch>),
}

impl WeightedTriangularGadget {
    /// Get source weights for this gadget.
    pub fn source_weights(&self) -> &[i32] {
        match self {
            Self::CrossFalse(g) => g.source_weights(),
            Self::CrossTrue(g) => g.source_weights(),
            Self::TConLeft(g) => g.source_weights(),
            Self::TConUp(g) => g.source_weights(),
            Self::TConDown(g) => g.source_weights(),
            Self::TrivialTurnLeft(g) => g.source_weights(),
            Self::TrivialTurnRight(g) => g.source_weights(),
            Self::EndTurn(g) => g.source_weights(),
            Self::Turn(g) => g.source_weights(),
            Self::WTurn(g) => g.source_weights(),
            Self::BranchFix(g) => g.source_weights(),
            Self::BranchFixB(g) => g.source_weights(),
            Self::Branch(g) => g.source_weights(),
        }
    }

    /// Get mapped weights for this gadget.
    pub fn mapped_weights(&self) -> &[i32] {
        match self {
            Self::CrossFalse(g) => g.mapped_weights(),
            Self::CrossTrue(g) => g.mapped_weights(),
            Self::TConLeft(g) => g.mapped_weights(),
            Self::TConUp(g) => g.mapped_weights(),
            Self::TConDown(g) => g.mapped_weights(),
            Self::TrivialTurnLeft(g) => g.mapped_weights(),
            Self::TrivialTurnRight(g) => g.mapped_weights(),
            Self::EndTurn(g) => g.mapped_weights(),
            Self::Turn(g) => g.mapped_weights(),
            Self::WTurn(g) => g.mapped_weights(),
            Self::BranchFix(g) => g.mapped_weights(),
            Self::BranchFixB(g) => g.mapped_weights(),
            Self::Branch(g) => g.mapped_weights(),
        }
    }

    /// Get mis_overhead for this gadget.
    pub fn mis_overhead(&self) -> i32 {
        use super::triangular::TriangularGadget;
        match self {
            Self::CrossFalse(g) => g.gadget.mis_overhead(),
            Self::CrossTrue(g) => g.gadget.mis_overhead(),
            Self::TConLeft(g) => g.gadget.mis_overhead(),
            Self::TConUp(g) => g.gadget.mis_overhead(),
            Self::TConDown(g) => g.gadget.mis_overhead(),
            Self::TrivialTurnLeft(g) => g.gadget.mis_overhead(),
            Self::TrivialTurnRight(g) => g.gadget.mis_overhead(),
            Self::EndTurn(g) => g.gadget.mis_overhead(),
            Self::Turn(g) => g.gadget.mis_overhead(),
            Self::WTurn(g) => g.gadget.mis_overhead(),
            Self::BranchFix(g) => g.gadget.mis_overhead(),
            Self::BranchFixB(g) => g.gadget.mis_overhead(),
            Self::Branch(g) => g.gadget.mis_overhead(),
        }
    }
}

/// Get the weighted triangular crossing ruleset.
/// This matches Julia's `crossing_ruleset_triangular_weighted`.
pub fn triangular_weighted_ruleset() -> Vec<WeightedTriangularGadget> {
    vec![
        WeightedTriangularGadget::CrossFalse(TriCross::<false>.weighted()),
        WeightedTriangularGadget::CrossTrue(TriCross::<true>.weighted()),
        WeightedTriangularGadget::TConLeft(TriTConLeft.weighted()),
        WeightedTriangularGadget::TConUp(TriTConUp.weighted()),
        WeightedTriangularGadget::TConDown(TriTConDown.weighted()),
        WeightedTriangularGadget::TrivialTurnLeft(TriTrivialTurnLeft.weighted()),
        WeightedTriangularGadget::TrivialTurnRight(TriTrivialTurnRight.weighted()),
        WeightedTriangularGadget::EndTurn(TriEndTurn.weighted()),
        WeightedTriangularGadget::Turn(TriTurn.weighted()),
        WeightedTriangularGadget::WTurn(TriWTurn.weighted()),
        WeightedTriangularGadget::BranchFix(TriBranchFix.weighted()),
        WeightedTriangularGadget::BranchFixB(TriBranchFixB.weighted()),
        WeightedTriangularGadget::Branch(TriBranch.weighted()),
    ]
}

/// Trace center locations through gadget transformations.
/// Returns the final center location for each original vertex.
///
/// This matches Julia's `trace_centers` function which:
/// 1. Gets initial center locations with (0, 1) offset
/// 2. Applies `move_center` for each gadget in the tape
pub fn trace_centers(result: &MappingResult) -> Vec<(usize, usize)> {
    // Get gadget sizes for bounds checking
    fn get_gadget_size(gadget_idx: usize) -> (usize, usize) {
        use super::triangular::TriangularGadget;
        use super::triangular::{
            TriBranch, TriBranchFix, TriBranchFixB, TriCross, TriEndTurn, TriTConDown,
            TriTConLeft, TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn, TriWTurn,
        };
        match gadget_idx {
            0 => TriCross::<false>.size(),
            1 => TriCross::<true>.size(),
            2 => TriTConLeft.size(),
            3 => TriTConUp.size(),
            4 => TriTConDown.size(),
            5 => TriTrivialTurnLeft.size(),
            6 => TriTrivialTurnRight.size(),
            7 => TriEndTurn.size(),
            8 => TriTurn.size(),
            9 => TriWTurn.size(),
            10 => TriBranchFix.size(),
            11 => TriBranchFixB.size(),
            12 => TriBranch.size(),
            _ => (0, 0),
        }
    }

    // Get center locations for each copy line with (0, 1) offset (matching Julia)
    let mut centers: Vec<(usize, usize)> = result
        .lines
        .iter()
        .map(|line| {
            let (row, col) = line.center_location(result.padding, result.spacing);
            (row, col + 1) // Julia adds (0, 1) offset
        })
        .collect();

    // Apply gadget transformations from tape
    for entry in &result.tape {
        let gadget_idx = entry.pattern_idx;
        let gi = entry.row;
        let gj = entry.col;

        // Get gadget size
        let (m, n) = get_gadget_size(gadget_idx);
        if m == 0 || n == 0 {
            continue; // Unknown gadget
        }

        // For each center location, check if it's within this gadget's area
        for center in centers.iter_mut() {
            let (ci, cj) = *center;

            // Check if center is within gadget bounds (using >= for lower and < for upper)
            if ci >= gi && ci < gi + m && cj >= gj && cj < gj + n {
                // Local coordinates within gadget (1-indexed as in Julia)
                let local_i = ci - gi + 1;
                let local_j = cj - gj + 1;

                // Apply gadget-specific center movement
                if let Some(new_pos) =
                    move_center_for_gadget(gadget_idx, (local_i, local_j), gi, gj)
                {
                    *center = new_pos;
                }
            }
        }
    }

    // Sort by vertex index and return
    let mut indexed: Vec<_> = result
        .lines
        .iter()
        .enumerate()
        .map(|(idx, line)| (line.vertex, centers[idx]))
        .collect();
    indexed.sort_by_key(|(v, _)| *v);
    indexed.into_iter().map(|(_, c)| c).collect()
}

/// Move a center through a specific gadget transformation.
/// Returns the new global position if the gadget affects this center.
///
/// The center location includes the (0, 1) offset from Julia's trace_centers,
/// so it's at cross_location + (0, 1) within the gadget.
fn move_center_for_gadget(
    gadget_idx: usize,
    local_pos: (usize, usize),
    gi: usize,
    gj: usize,
) -> Option<(usize, usize)> {
    use super::triangular::TriangularGadget;
    use super::triangular::{
        TriBranch, TriBranchFix, TriBranchFixB, TriCross, TriEndTurn, TriTConDown, TriTConLeft,
        TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn, TriWTurn,
    };

    // The center is at cross_location + (0, 1) for most gadgets.
    // We need to find where it maps to in the transformed pattern.
    // The general rule is: center stays at same row, moves to column of nearest mapped node.

    let (m, n, cross_loc) = match gadget_idx {
        0 => (TriCross::<false>.size().0, TriCross::<false>.size().1, TriCross::<false>.cross_location()),
        1 => (TriCross::<true>.size().0, TriCross::<true>.size().1, TriCross::<true>.cross_location()),
        2 => (TriTConLeft.size().0, TriTConLeft.size().1, TriTConLeft.cross_location()),
        3 => (TriTConUp.size().0, TriTConUp.size().1, TriTConUp.cross_location()),
        4 => (TriTConDown.size().0, TriTConDown.size().1, TriTConDown.cross_location()),
        5 => (TriTrivialTurnLeft.size().0, TriTrivialTurnLeft.size().1, TriTrivialTurnLeft.cross_location()),
        6 => (TriTrivialTurnRight.size().0, TriTrivialTurnRight.size().1, TriTrivialTurnRight.cross_location()),
        7 => (TriEndTurn.size().0, TriEndTurn.size().1, TriEndTurn.cross_location()),
        8 => (TriTurn.size().0, TriTurn.size().1, TriTurn.cross_location()),
        9 => (TriWTurn.size().0, TriWTurn.size().1, TriWTurn.cross_location()),
        10 => (TriBranchFix.size().0, TriBranchFix.size().1, TriBranchFix.cross_location()),
        11 => (TriBranchFixB.size().0, TriBranchFixB.size().1, TriBranchFixB.cross_location()),
        12 => (TriBranch.size().0, TriBranch.size().1, TriBranch.cross_location()),
        _ => return None, // Unknown gadget or simplifier
    };

    let (li, lj) = local_pos;

    // Check bounds
    if li < 1 || li > m || lj < 1 || lj > n {
        return None;
    }

    // The center is expected to be at cross_location + (0, 1)
    let expected_center = (cross_loc.0, cross_loc.1 + 1);

    // For most gadgets, if the center is at the expected position,
    // it maps to a specific location in the mapped pattern.
    // The mapped center is typically at cross_location (the gadget's anchor point).
    if local_pos == expected_center {
        // Map center from cross_location + (0, 1) to cross_location + (0, 1) in mapped
        // But if that position doesn't exist in mapped, use cross_location
        let mapped_pos = match gadget_idx {
            // TriCross<false>: center at (2, 4) maps to (2, 4) - stays same
            0 => (cross_loc.0, cross_loc.1 + 1),
            // TriCross<true>: center at (2, 3) maps to (2, 3) - stays same
            1 => (cross_loc.0, cross_loc.1 + 1),
            // TriTConLeft: center at (2, 3) maps to (2, 3)
            2 => (cross_loc.0, cross_loc.1 + 1),
            // TriTConUp: center at (2, 3) maps to (2, 3)
            3 => (cross_loc.0, cross_loc.1 + 1),
            // TriTConDown: center at (2, 3) maps to (3, 2) - moves to different position
            4 => (cross_loc.0 + 1, cross_loc.1),
            // TriTrivialTurnLeft: center at (2, 3) - but size is (2, 2), so this doesn't apply
            5 => (cross_loc.0, cross_loc.1 + 1),
            // TriTrivialTurnRight: center at (1, 3) - but size is (2, 2), so this doesn't apply
            6 => (cross_loc.0 + 1, cross_loc.1 + 1),
            // TriEndTurn: center at (2, 3) maps to (1, 2) - center moves to first node
            7 => (1, 2),
            // TriTurn: center at (3, 3) maps to (2, 3) - follows the turn
            8 => (2, 3),
            // TriWTurn: center at (2, 3) maps to (3, 3)
            9 => (3, 3),
            // TriBranchFix: center at (2, 3) maps to (2, 2) - straightens to column 2
            10 => (cross_loc.0, cross_loc.1),
            // TriBranchFixB: center at (2, 3) maps to (3, 2) - moves down
            11 => (3, 2),
            // TriBranch: center at (3, 3) maps to (2, 3)
            12 => (2, 3),
            _ => return None,
        };
        // Convert to global coordinates
        return Some((gi + mapped_pos.0 - 1, gj + mapped_pos.1 - 1));
    }

    // Also check if center is at cross_location (without the +1 offset)
    // This can happen if the offset wasn't applied or gadgets shifted things
    if local_pos == cross_loc {
        // Return cross_location in global coords
        return Some((gi + cross_loc.0 - 1, gj + cross_loc.1 - 1));
    }

    None
}

/// Map source vertex weights to grid graph weights.
///
/// # Arguments
/// * `result` - The mapping result from map_graph_triangular
/// * `source_weights` - Weights for each original vertex (should be in [0, 1])
///
/// # Returns
/// A vector of weights for each node in the grid graph.
pub fn map_weights(result: &MappingResult, source_weights: &[f64]) -> Vec<f64> {
    assert!(
        source_weights.iter().all(|&w| (0.0..=1.0).contains(&w)),
        "all weights must be in range [0, 1]"
    );
    assert_eq!(
        source_weights.len(),
        result.lines.len(),
        "source_weights length must match number of vertices"
    );

    // Start with base weights from grid nodes
    let mut weights: Vec<f64> = result
        .grid_graph
        .nodes()
        .iter()
        .map(|n| n.weight as f64)
        .collect();

    // Get center locations for each original vertex
    let centers = trace_centers(result);

    // Add source weights at center locations
    for (vertex, &src_weight) in source_weights.iter().enumerate() {
        let center = centers[vertex];
        // Find the node index at this center location
        if let Some(idx) = result
            .grid_graph
            .nodes()
            .iter()
            .position(|n| n.row as usize == center.0 && n.col as usize == center.1)
        {
            weights[idx] += src_weight;
        }
    }

    weights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triturn_weighted() {
        let weighted = TriTurn.weighted();
        assert_eq!(weighted.source_weights, vec![2, 2, 2, 2]);
        assert_eq!(weighted.mapped_weights, vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_tribranch_weighted() {
        let weighted = TriBranch.weighted();
        // Julia: sw = [2,2,3,2,2,2,2,2,2], mw = [2,2,2,3,2,2,2,2,2]
        assert_eq!(weighted.source_weights, vec![2, 2, 3, 2, 2, 2, 2, 2, 2]);
        assert_eq!(weighted.mapped_weights, vec![2, 2, 2, 3, 2, 2, 2, 2, 2]);
    }

    #[test]
    fn test_tricross_true_weighted() {
        let weighted = TriCross::<true>.weighted();
        // Julia: sw = [2,2,2,2,2,2,2,2,2,2], mw = [3,2,3,3,2,2,2,2,2,2,2]
        assert_eq!(weighted.source_weights, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
        assert_eq!(
            weighted.mapped_weights,
            vec![3, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2]
        );
    }

    #[test]
    fn test_tricross_false_weighted() {
        let weighted = TriCross::<false>.weighted();
        // Julia: sw = [2,2,2,2,2,2,2,2,2,2,2,2], mw = [3,3,2,4,2,2,2,4,3,2,2,2,2,2,2,2]
        assert_eq!(
            weighted.source_weights,
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
        );
        assert_eq!(
            weighted.mapped_weights,
            vec![3, 3, 2, 4, 2, 2, 2, 4, 3, 2, 2, 2, 2, 2, 2, 2]
        );
    }

    #[test]
    fn test_all_weighted_gadgets_have_correct_lengths() {
        use super::super::triangular::TriangularGadget;

        fn check<G: TriangularGadget + Weightable + Clone>(g: G, name: &str) {
            let weighted = g.clone().weighted();
            let (src_locs, _, _) = g.source_graph();
            let (map_locs, _) = g.mapped_graph();
            assert_eq!(
                weighted.source_weights.len(),
                src_locs.len(),
                "{}: source weights length mismatch",
                name
            );
            assert_eq!(
                weighted.mapped_weights.len(),
                map_locs.len(),
                "{}: mapped weights length mismatch",
                name
            );
        }

        check(TriTurn, "TriTurn");
        check(TriBranch, "TriBranch");
        check(TriCross::<true>, "TriCross<true>");
        check(TriCross::<false>, "TriCross<false>");
        check(TriTConLeft, "TriTConLeft");
        check(TriTConDown, "TriTConDown");
        check(TriTConUp, "TriTConUp");
        check(TriTrivialTurnLeft, "TriTrivialTurnLeft");
        check(TriTrivialTurnRight, "TriTrivialTurnRight");
        check(TriEndTurn, "TriEndTurn");
        check(TriWTurn, "TriWTurn");
        check(TriBranchFix, "TriBranchFix");
        check(TriBranchFixB, "TriBranchFixB");
    }

    #[test]
    fn test_triangular_weighted_ruleset_has_13_gadgets() {
        let ruleset = super::triangular_weighted_ruleset();
        assert_eq!(ruleset.len(), 13);
    }

    #[test]
    fn test_trace_centers_basic() {
        use crate::rules::mapping::map_graph_triangular;

        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph_triangular(3, &edges);

        let centers = super::trace_centers(&result);
        assert_eq!(centers.len(), 3);

        // Centers should be valid grid positions
        for (row, col) in &centers {
            assert!(*row > 0);
            assert!(*col > 0);
        }
    }

    #[test]
    fn test_map_weights_basic() {
        use crate::rules::mapping::map_graph_triangular;
        use crate::topology::Graph;

        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph_triangular(3, &edges);

        let source_weights = vec![0.5, 0.3, 0.7];
        let grid_weights = super::map_weights(&result, &source_weights);

        // Should have same length as grid nodes
        assert_eq!(grid_weights.len(), result.grid_graph.num_vertices());

        // All weights should be positive
        assert!(grid_weights.iter().all(|&w| w > 0.0));
    }

    #[test]
    #[should_panic(expected = "all weights must be in range")]
    fn test_map_weights_rejects_invalid() {
        use crate::rules::mapping::map_graph_triangular;

        let edges = vec![(0, 1)];
        let result = map_graph_triangular(2, &edges);

        let source_weights = vec![1.5, 0.3]; // Invalid: > 1
        super::map_weights(&result, &source_weights);
    }
}
