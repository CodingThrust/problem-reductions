//! Weighted gadget support for triangular lattice mapping.

use super::ksg::MappingResult;
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
        WeightedGadget::new(
            self,
            TriCross::<true>.source_weights(),
            TriCross::<true>.mapped_weights(),
        )
    }
}

impl Weightable for TriCross<false> {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(
            self,
            TriCross::<false>.source_weights(),
            TriCross::<false>.mapped_weights(),
        )
    }
}

impl Weightable for TriTConLeft {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(
            self,
            TriTConLeft.source_weights(),
            TriTConLeft.mapped_weights(),
        )
    }
}

impl Weightable for TriTConDown {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(
            self,
            TriTConDown.source_weights(),
            TriTConDown.mapped_weights(),
        )
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
        WeightedGadget::new(
            self,
            TriTrivialTurnLeft.source_weights(),
            TriTrivialTurnLeft.mapped_weights(),
        )
    }
}

impl Weightable for TriTrivialTurnRight {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(
            self,
            TriTrivialTurnRight.source_weights(),
            TriTrivialTurnRight.mapped_weights(),
        )
    }
}

impl Weightable for TriEndTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(
            self,
            TriEndTurn.source_weights(),
            TriEndTurn.mapped_weights(),
        )
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
        WeightedGadget::new(
            self,
            TriBranchFix.source_weights(),
            TriBranchFix.mapped_weights(),
        )
    }
}

impl Weightable for TriBranchFixB {
    fn weighted(self) -> WeightedGadget<Self> {
        use super::triangular::TriangularGadget;
        WeightedGadget::new(
            self,
            TriBranchFixB.source_weights(),
            TriBranchFixB.mapped_weights(),
        )
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
            TriBranch, TriBranchFix, TriBranchFixB, TriCross, TriEndTurn, TriTConDown, TriTConLeft,
            TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn, TriWTurn,
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
            // Simplifier gadgets: DanglingLeg rotations
            // Base DanglingLeg has size (4, 3)
            100 => (4, 3), // DanglingLeg down (no rotation)
            101 => (4, 3), // DanglingLeg up (180° rotation, same size)
            102 => (3, 4), // DanglingLeg right (90° clockwise, swapped)
            103 => (3, 4), // DanglingLeg left (90° counterclockwise, swapped)
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
/// Julia defines center movement for:
/// 1. Triangular crossing gadgets (7-12): TriTurn, TriBranch, etc.
/// 2. Simplifier gadgets (100-103): DanglingLeg rotations
///
/// Gadgets 0-6 (TriCross, TriTCon*, TriTrivialTurn*) have empty centers - no movement.
fn move_center_for_gadget(
    gadget_idx: usize,
    local_pos: (usize, usize),
    gi: usize,
    gj: usize,
) -> Option<(usize, usize)> {
    // Get source_center and mapped_center for this gadget
    // From Julia triangular.jl line 415-417:
    //   source_centers = [cross_location(T()) .+ (0, 1)]
    //   All triangular gadgets have cross_location = (2, 2), so source = (2, 3)
    //   mapped_centers: TriTurn->(1,2), TriBranch->(1,2), TriBranchFix->(3,2),
    //                   TriBranchFixB->(3,2), TriWTurn->(2,3), TriEndTurn->(1,2)
    let (source_center, mapped_center) = match gadget_idx {
        // Triangular crossing gadgets - all have cross_location=(2,2), source=(2,3)
        7 => ((2, 3), (1, 2)),  // TriEndTurn
        8 => ((2, 3), (1, 2)),  // TriTurn
        9 => ((2, 3), (2, 3)),  // TriWTurn (center stays same)
        10 => ((2, 3), (3, 2)), // TriBranchFix
        11 => ((2, 3), (3, 2)), // TriBranchFixB
        12 => ((2, 3), (1, 2)), // TriBranch

        // Simplifier gadgets: DanglingLeg rotations (from simplifiers.jl:107-108)
        // Base DanglingLeg: source_centers=[(2,2)], mapped_centers=[(4,2)]
        // Size (4, 3). When rotated, centers transform accordingly.
        //
        // 100: DanglingLeg down (no rotation) - size (4, 3)
        //      source_center = (2, 2), mapped_center = (4, 2)
        100 => ((2, 2), (4, 2)),

        // 101: DanglingLeg up (180° rotation) - size (4, 3)
        //      Rotation 2: (r, c) -> (m+1-r, n+1-c) where (m,n)=(4,3)
        //      source: (2, 2) -> (4+1-2, 3+1-2) = (3, 2)
        //      mapped: (4, 2) -> (4+1-4, 3+1-2) = (1, 2)
        101 => ((3, 2), (1, 2)),

        // 102: DanglingLeg right (90° clockwise, rotation 1) - size (3, 4)
        //      Rotation 1: (r, c) -> (c, m+1-r) where m=4 (original rows)
        //      source: (2, 2) -> (2, 4+1-2) = (2, 3)
        //      mapped: (4, 2) -> (2, 4+1-4) = (2, 1)
        102 => ((2, 3), (2, 1)),

        // 103: DanglingLeg left (90° counterclockwise, rotation 3) - size (3, 4)
        //      Rotation 3: (r, c) -> (n+1-c, r) where n=3 (original cols)
        //      source: (2, 2) -> (3+1-2, 2) = (2, 2)
        //      mapped: (4, 2) -> (3+1-2, 4) = (2, 4)
        103 => ((2, 2), (2, 4)),

        // Gadgets 0-6 and unknown: no center movement
        _ => return None,
    };

    // Check if local_pos matches source_center
    if local_pos == source_center {
        // Julia: return nodexy .+ mc .- sc
        // global_new = global_old + (mapped_center - source_center)
        let di = mapped_center.0 as isize - source_center.0 as isize;
        let dj = mapped_center.1 as isize - source_center.1 as isize;
        let new_i = (gi as isize + local_pos.0 as isize - 1 + di) as usize;
        let new_j = (gj as isize + local_pos.1 as isize - 1 + dj) as usize;
        return Some((new_i, new_j));
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
    let mut weights: Vec<f64> = result.node_weights.iter().map(|&w| w as f64).collect();

    // Get center locations for each original vertex
    let centers = trace_centers(result);

    // Add source weights at center locations
    for (vertex, &src_weight) in source_weights.iter().enumerate() {
        let center = centers[vertex];
        // Find the node index at this center location
        if let Some(idx) = result
            .positions
            .iter()
            .position(|&(r, c)| r as usize == center.0 && c as usize == center.1)
        {
            weights[idx] += src_weight;
        }
    }

    weights
}

#[cfg(test)]
#[path = "../../unit_tests/rules/unitdiskmapping/weighted.rs"]
mod tests;
