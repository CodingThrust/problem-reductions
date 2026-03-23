//! Bin Packing problem implementation.
//!
//! The Bin Packing problem asks for an assignment of items to bins
//! that minimizes the number of bins used while respecting capacity constraints.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::{ObjectiveProblem, Problem};
use crate::types::{ExtremumSense, Min, WeightElement};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "BinPacking",
        display_name: "Bin Packing",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i32", &["i32", "f64"])],
        module_path: module_path!(),
        description: "Assign items to bins minimizing number of bins used, subject to capacity",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<W>", description: "Item sizes s_i for each item" },
            FieldInfo { name: "capacity", type_name: "W", description: "Bin capacity C" },
        ],
    }
}

/// The Bin Packing problem.
///
/// Given `n` items with sizes `s_1, ..., s_n` and bin capacity `C`,
/// find an assignment of items to bins such that:
/// - For each bin `j`, the total size of items assigned to `j` does not exceed `C`
/// - The number of bins used is minimized
///
/// # Representation
///
/// Each item has a variable in `{0, ..., n-1}` representing its bin assignment.
/// The worst case uses `n` bins (one item per bin).
///
/// # Type Parameters
///
/// * `W` - The weight type for sizes and capacity (e.g., `i32`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::BinPacking;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 items with sizes [3, 3, 2, 2], capacity 5
/// let problem = BinPacking::new(vec![3, 3, 2, 2], 5);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinPacking<W> {
    /// Item sizes.
    sizes: Vec<W>,
    /// Bin capacity.
    capacity: W,
}

impl<W: Clone> BinPacking<W> {
    /// Create a Bin Packing problem from item sizes and capacity.
    pub fn new(sizes: Vec<W>, capacity: W) -> Self {
        Self { sizes, capacity }
    }

    /// Get the item sizes.
    pub fn sizes(&self) -> &[W] {
        &self.sizes
    }

    /// Get the bin capacity.
    pub fn capacity(&self) -> &W {
        &self.capacity
    }

    /// Get the number of items.
    pub fn num_items(&self) -> usize {
        self.sizes.len()
    }
}

impl<W> Problem for BinPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: PartialOrd,
{
    const NAME: &'static str = "BinPacking";
    type Value = Min<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.sizes.len();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i32> {
        if !is_valid_packing(&self.sizes, &self.capacity, config) {
            return Min(None);
        }
        let num_bins = count_bins(config);
        Min(Some(num_bins as i32))
    }
}

impl<W> ObjectiveProblem for BinPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: PartialOrd,
{
    type Objective = i32;

    fn direction(&self) -> ExtremumSense {
        ExtremumSense::Minimize
    }
}

/// Check if a configuration is a valid bin packing (all bins within capacity).
fn is_valid_packing<W: WeightElement>(sizes: &[W], capacity: &W, config: &[usize]) -> bool
where
    W::Sum: PartialOrd,
{
    if config.len() != sizes.len() {
        return false;
    }
    let n = sizes.len();
    // Check all bin indices are in range
    if config.iter().any(|&b| b >= n) {
        return false;
    }
    // Compute load per bin
    let cap_sum = capacity.to_sum();
    let mut bin_load: Vec<W::Sum> = vec![W::Sum::default(); n];
    for (i, &bin) in config.iter().enumerate() {
        bin_load[bin] += sizes[i].to_sum();
    }
    // Check capacity constraints
    bin_load.iter().all(|load| *load <= cap_sum)
}

/// Count the number of distinct bins used in a configuration.
fn count_bins(config: &[usize]) -> usize {
    let mut used = vec![false; config.len()];
    for &bin in config {
        if bin < used.len() {
            used[bin] = true;
        }
    }
    used.iter().filter(|&&u| u).count()
}

crate::declare_variants! {
    default opt BinPacking<i32> => "2^num_items",
    opt BinPacking<f64> => "2^num_items",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "bin_packing",
        // 3 items of sizes [3,3,4], capacity 7 → optimal 2 bins
        instance: Box::new(BinPacking::<i32>::new(vec![3, 3, 4], 7)),
        optimal_config: vec![0, 1, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/bin_packing.rs"]
mod tests;
