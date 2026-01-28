//! Weighted gadget support for triangular lattice mapping.

use super::triangular::{TriBranch, TriCross, TriTurn};
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
}
