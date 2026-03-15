//! Partially Ordered Knapsack problem implementation.
//!
//! A knapsack variant where items are subject to a partial order: including
//! an item requires including all its predecessors (downward-closed set).
//! NP-complete in the strong sense (Garey & Johnson, A6 MP12).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartiallyOrderedKnapsack",
        display_name: "Partially Ordered Knapsack",
        aliases: &["POK"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Select items to maximize total value subject to precedence constraints and weight capacity",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Item sizes s(u) for each item" },
            FieldInfo { name: "values", type_name: "Vec<i64>", description: "Item values v(u) for each item" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (a, b) meaning a must be included before b" },
            FieldInfo { name: "capacity", type_name: "i64", description: "Knapsack capacity B" },
        ],
    }
}

/// The Partially Ordered Knapsack problem.
///
/// Given `n` items, each with size `s(u)` and value `v(u)`, a partial order
/// on the items (given as precedence pairs), and a capacity `B`, find a subset
/// `U' ⊆ U` that is downward-closed (if `u ∈ U'` and `u' < u`, then `u' ∈ U'`),
/// satisfies `∑_{u∈U'} s(u) ≤ B`, and maximizes `∑_{u∈U'} v(u)`.
///
/// # Representation
///
/// Each item has a binary variable: `x_u = 1` if item `u` is selected, `0` otherwise.
/// Precedences are stored as `(a, b)` pairs meaning item `a` must be included
/// whenever item `b` is included.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::PartiallyOrderedKnapsack;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = PartiallyOrderedKnapsack::new(
///     vec![2, 3, 4, 1, 2, 3],  // sizes
///     vec![3, 2, 5, 4, 3, 8],  // values
///     vec![(0, 2), (0, 3), (1, 4), (3, 5), (4, 5)],  // precedences
///     11,  // capacity
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
/// Raw serialization helper for [`PartiallyOrderedKnapsack`].
#[derive(Serialize, Deserialize)]
struct PartiallyOrderedKnapsackRaw {
    sizes: Vec<i64>,
    values: Vec<i64>,
    precedences: Vec<(usize, usize)>,
    capacity: i64,
}

#[derive(Debug, Clone)]
pub struct PartiallyOrderedKnapsack {
    sizes: Vec<i64>,
    values: Vec<i64>,
    precedences: Vec<(usize, usize)>,
    capacity: i64,
    /// Precomputed transitive predecessors for each item.
    /// `predecessors[b]` contains all items that must be selected when `b` is selected.
    predecessors: Vec<Vec<usize>>,
}

impl Serialize for PartiallyOrderedKnapsack {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        PartiallyOrderedKnapsackRaw {
            sizes: self.sizes.clone(),
            values: self.values.clone(),
            precedences: self.precedences.clone(),
            capacity: self.capacity,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PartiallyOrderedKnapsack {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = PartiallyOrderedKnapsackRaw::deserialize(deserializer)?;
        Ok(Self::new(raw.sizes, raw.values, raw.precedences, raw.capacity))
    }
}

impl PartiallyOrderedKnapsack {
    /// Create a new PartiallyOrderedKnapsack instance.
    ///
    /// # Arguments
    /// * `sizes` - Size s(u) for each item
    /// * `values` - Value v(u) for each item
    /// * `precedences` - Precedence pairs `(a, b)` meaning item `a` must be included before item `b`
    /// * `capacity` - Knapsack capacity B
    ///
    /// # Panics
    /// Panics if `sizes` and `values` have different lengths, if any size or
    /// capacity is negative, if any precedence index is out of bounds, or if
    /// the precedences contain a cycle.
    pub fn new(
        sizes: Vec<i64>,
        values: Vec<i64>,
        precedences: Vec<(usize, usize)>,
        capacity: i64,
    ) -> Self {
        assert_eq!(
            sizes.len(),
            values.len(),
            "sizes and values must have the same length"
        );
        assert!(capacity >= 0, "capacity must be non-negative");
        for (i, &s) in sizes.iter().enumerate() {
            assert!(s >= 0, "size[{i}] must be non-negative, got {s}");
        }
        let n = sizes.len();
        for &(a, b) in &precedences {
            assert!(a < n, "precedence index {a} out of bounds (n={n})");
            assert!(b < n, "precedence index {b} out of bounds (n={n})");
        }
        let predecessors = Self::compute_predecessors(&precedences, n);
        // Check for cycles: if any item is its own transitive predecessor, the DAG has a cycle
        for i in 0..n {
            assert!(
                !predecessors[i].contains(&i),
                "precedences contain a cycle involving item {i}"
            );
        }
        Self {
            sizes,
            values,
            precedences,
            capacity,
            predecessors,
        }
    }

    /// Compute transitive predecessors for each item via Floyd-Warshall.
    fn compute_predecessors(precedences: &[(usize, usize)], n: usize) -> Vec<Vec<usize>> {
        let mut reachable = vec![vec![false; n]; n];
        for &(a, b) in precedences {
            reachable[a][b] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if reachable[i][k] && reachable[k][j] {
                        reachable[i][j] = true;
                    }
                }
            }
        }
        (0..n)
            .map(|b| (0..n).filter(|&a| reachable[a][b]).collect())
            .collect()
    }

    /// Returns the item sizes.
    pub fn sizes(&self) -> &[i64] {
        &self.sizes
    }

    /// Returns the item values.
    pub fn values(&self) -> &[i64] {
        &self.values
    }

    /// Returns the precedence pairs.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Returns the knapsack capacity.
    pub fn capacity(&self) -> i64 {
        self.capacity
    }

    /// Returns the number of items.
    pub fn num_items(&self) -> usize {
        self.sizes.len()
    }

    /// Returns the number of precedence relations.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    /// Check if the selected items form a downward-closed set.
    ///
    /// Uses precomputed transitive predecessors: if item `b` is selected,
    /// all its predecessors must also be selected.
    fn is_downward_closed(&self, config: &[usize]) -> bool {
        for (b, preds) in self.predecessors.iter().enumerate() {
            if config[b] == 1 {
                for &a in preds {
                    if config[a] != 1 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Problem for PartiallyOrderedKnapsack {
    const NAME: &'static str = "PartiallyOrderedKnapsack";
    type Metric = SolutionSize<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_items()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i64> {
        if config.len() != self.num_items() {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v >= 2) {
            return SolutionSize::Invalid;
        }
        // Check downward-closure (precedence constraints)
        if !self.is_downward_closed(config) {
            return SolutionSize::Invalid;
        }
        // Check capacity constraint
        let total_size: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.sizes[i])
            .sum();
        if total_size > self.capacity {
            return SolutionSize::Invalid;
        }
        // Compute total value
        let total_value: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.values[i])
            .sum();
        SolutionSize::Valid(total_value)
    }
}

impl OptimizationProblem for PartiallyOrderedKnapsack {
    type Value = i64;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

crate::declare_variants! {
    default opt PartiallyOrderedKnapsack => "2^num_items",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partially_ordered_knapsack",
        build: || {
            use crate::solvers::BruteForce;
            let problem = PartiallyOrderedKnapsack::new(
                vec![2, 3, 4, 1, 2, 3],
                vec![3, 2, 5, 4, 3, 8],
                vec![(0, 2), (0, 3), (1, 4), (3, 5), (4, 5)],
                11,
            );
            let sample = BruteForce::new()
                .find_all_best(&problem)
                .into_iter()
                .next()
                .expect("partially_ordered_knapsack example should solve");
            crate::example_db::specs::optimization_example(problem, vec![sample])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/partially_ordered_knapsack.rs"]
mod tests;
