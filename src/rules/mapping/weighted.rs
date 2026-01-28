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

impl Weightable for TriTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,2], mw = [2,2,2,2]
        WeightedGadget::new(self, vec![2, 2, 2, 2], vec![2, 2, 2, 2])
    }
}

impl Weightable for TriBranch {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,3,2,2,2,2,2,2], mw = [2,2,2,3,2,2,2,2,2]
        WeightedGadget::new(
            self,
            vec![2, 2, 3, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 3, 2, 2, 2, 2, 2],
        )
    }
}

impl Weightable for TriCross<true> {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,2,2,2,2,2,2,2], mw = [3,2,3,3,2,2,2,2,2,2,2]
        WeightedGadget::new(
            self,
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![3, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2],
        )
    }
}

impl Weightable for TriCross<false> {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,2,2,2,2,2,2,2,2,2], mw = [3,3,2,4,2,2,2,4,3,2,2,2,2,2,2,2]
        WeightedGadget::new(
            self,
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![3, 3, 2, 4, 2, 2, 2, 4, 3, 2, 2, 2, 2, 2, 2, 2],
        )
    }
}

impl Weightable for TriTConLeft {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,1,2,2,2,2,2], mw = [3,2,3,3,1,3,2,2,2,2,2]
        WeightedGadget::new(
            self,
            vec![2, 1, 2, 2, 2, 2, 2],
            vec![3, 2, 3, 3, 1, 3, 2, 2, 2, 2, 2],
        )
    }
}

impl Weightable for TriTConDown {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,1], mw = [2,2,3,2]
        WeightedGadget::new(self, vec![2, 2, 2, 1], vec![2, 2, 3, 2])
    }
}

impl Weightable for TriTConUp {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [1,2,2,2], mw = [3,2,2,2]
        WeightedGadget::new(self, vec![1, 2, 2, 2], vec![3, 2, 2, 2])
    }
}

impl Weightable for TriTrivialTurnLeft {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [1,1], mw = [1,1]
        WeightedGadget::new(self, vec![1, 1], vec![1, 1])
    }
}

impl Weightable for TriTrivialTurnRight {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [1,1], mw = [1,1]
        WeightedGadget::new(self, vec![1, 1], vec![1, 1])
    }
}

impl Weightable for TriEndTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,1], mw = [1]
        WeightedGadget::new(self, vec![2, 2, 1], vec![1])
    }
}

impl Weightable for TriWTurn {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,2,2], mw = [2,2,2,2,2]
        WeightedGadget::new(self, vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2])
    }
}

impl Weightable for TriBranchFix {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,2,2,2], mw = [2,2,2,2]
        WeightedGadget::new(self, vec![2, 2, 2, 2, 2, 2], vec![2, 2, 2, 2])
    }
}

impl Weightable for TriBranchFixB {
    fn weighted(self) -> WeightedGadget<Self> {
        // Julia: sw = [2,2,2,2], mw = [2,2]
        WeightedGadget::new(self, vec![2, 2, 2, 2], vec![2, 2])
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
pub fn trace_centers(result: &MappingResult) -> Vec<(usize, usize)> {
    // Get center locations for each copy line, sorted by vertex index
    let mut indexed: Vec<_> = result
        .lines
        .iter()
        .map(|line| {
            let center = line.center_location(result.padding, result.spacing);
            (line.vertex, center)
        })
        .collect();
    indexed.sort_by_key(|(v, _)| *v);
    indexed.into_iter().map(|(_, c)| c).collect()
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
}
