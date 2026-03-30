//! Dynamic Storage Allocation problem implementation.
//!
//! Given items each with arrival time, departure time, and size, plus a
//! memory size D, determine whether each item can be assigned a starting
//! address such that every item fits within [0, D-1] and no two
//! time-overlapping items share memory addresses.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "DynamicStorageAllocation",
        display_name: "Dynamic Storage Allocation",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign starting addresses for items with time intervals and sizes within bounded memory",
        fields: &[
            FieldInfo { name: "items", type_name: "Vec<(usize, usize, usize)>", description: "Items as (arrival, departure, size) tuples" },
            FieldInfo { name: "memory_size", type_name: "usize", description: "Total memory size D" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "DynamicStorageAllocation",
        fields: &["num_items", "memory_size"],
    }
}

/// Dynamic Storage Allocation problem.
///
/// Each item `a` has arrival time `r(a)`, departure time `d(a)`, and size `s(a)`.
/// The goal is to find a starting address `σ(a) ∈ {0, ..., D - s(a)}` for each item
/// such that time-overlapping items do not overlap in memory.
#[derive(Debug, Clone, Serialize)]
pub struct DynamicStorageAllocation {
    items: Vec<(usize, usize, usize)>,
    memory_size: usize,
}

impl DynamicStorageAllocation {
    fn validate_inputs(items: &[(usize, usize, usize)], memory_size: usize) -> Result<(), String> {
        if items.is_empty() {
            return Err("DynamicStorageAllocation requires at least one item".to_string());
        }
        if memory_size == 0 {
            return Err("DynamicStorageAllocation requires a positive memory_size".to_string());
        }
        for (i, &(arrival, departure, size)) in items.iter().enumerate() {
            if size == 0 {
                return Err(format!("Item {i} has zero size; all sizes must be >= 1"));
            }
            if departure <= arrival {
                return Err(format!(
                    "Item {i} has departure ({departure}) <= arrival ({arrival}); departure must be strictly greater"
                ));
            }
            if size > memory_size {
                return Err(format!(
                    "Item {i} has size ({size}) > memory_size ({memory_size}); every item must fit in memory"
                ));
            }
        }
        Ok(())
    }

    /// Try to create a new `DynamicStorageAllocation` instance.
    pub fn try_new(items: Vec<(usize, usize, usize)>, memory_size: usize) -> Result<Self, String> {
        Self::validate_inputs(&items, memory_size)?;
        Ok(Self { items, memory_size })
    }

    /// Create a new `DynamicStorageAllocation` instance.
    ///
    /// # Panics
    ///
    /// Panics if any item has zero size, departure <= arrival, or size > memory_size.
    pub fn new(items: Vec<(usize, usize, usize)>, memory_size: usize) -> Self {
        Self::try_new(items, memory_size).unwrap_or_else(|message| panic!("{message}"))
    }

    /// The items as `(arrival, departure, size)` tuples.
    pub fn items(&self) -> &[(usize, usize, usize)] {
        &self.items
    }

    /// The total memory size D.
    pub fn memory_size(&self) -> usize {
        self.memory_size
    }

    /// The number of items.
    pub fn num_items(&self) -> usize {
        self.items.len()
    }
}

#[derive(Deserialize)]
struct DynamicStorageAllocationData {
    items: Vec<(usize, usize, usize)>,
    memory_size: usize,
}

impl<'de> Deserialize<'de> for DynamicStorageAllocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = DynamicStorageAllocationData::deserialize(deserializer)?;
        Self::try_new(data.items, data.memory_size).map_err(D::Error::custom)
    }
}

impl Problem for DynamicStorageAllocation {
    const NAME: &'static str = "DynamicStorageAllocation";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.items
            .iter()
            .map(|&(_, _, s)| self.memory_size - s + 1)
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or({
            if config.len() != self.num_items() {
                return Or(false);
            }

            // Check each item fits within memory
            for (i, &(_, _, size)) in self.items.iter().enumerate() {
                let start = config[i];
                if start + size > self.memory_size {
                    return Or(false);
                }
            }

            // Check all pairs of time-overlapping items for memory non-overlap
            for (i, &(r_i, d_i, s_i)) in self.items.iter().enumerate() {
                let sigma_i = config[i];
                for (j, &(r_j, d_j, s_j)) in self.items.iter().enumerate().skip(i + 1) {
                    // Time overlap: r_i < d_j AND r_j < d_i
                    if r_i < d_j && r_j < d_i {
                        let sigma_j = config[j];
                        // Memory overlap: NOT (sigma_i + s_i <= sigma_j OR sigma_j + s_j <= sigma_i)
                        let no_memory_overlap =
                            sigma_i + s_i <= sigma_j || sigma_j + s_j <= sigma_i;
                        if !no_memory_overlap {
                            return Or(false);
                        }
                    }
                }
            }
            true
        })
    }
}

crate::declare_variants! {
    default DynamicStorageAllocation => "(memory_size + 1)^num_items",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "dynamic_storage_allocation",
        instance: Box::new(DynamicStorageAllocation::new(
            vec![(0, 3, 2), (0, 2, 3), (1, 4, 1), (2, 5, 3), (3, 5, 2)],
            6,
        )),
        optimal_config: vec![0, 2, 5, 2, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/dynamic_storage_allocation.rs"]
mod tests;
