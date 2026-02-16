//! Paint Shop problem implementation.
//!
//! In the Paint Shop problem, we have a sequence of cars to paint.
//! Each car appears exactly twice in the sequence and must be painted
//! one color at its first occurrence and another at its second.
//! The goal is to minimize color switches between adjacent positions.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PaintShop",
        module_path: module_path!(),
        description: "Minimize color changes in paint shop sequence",
        fields: &[
            FieldInfo { name: "sequence_indices", type_name: "Vec<usize>", description: "Car sequence as indices" },
            FieldInfo { name: "car_labels", type_name: "Vec<String>", description: "Unique car labels" },
            FieldInfo { name: "is_first", type_name: "Vec<bool>", description: "First occurrence flags" },
            FieldInfo { name: "num_cars", type_name: "usize", description: "Number of unique cars" },
        ],
    }
}

/// The Paint Shop problem.
///
/// Given a sequence where each car appears exactly twice, assign colors
/// (0 or 1) to each car to minimize color switches in the sequence.
///
/// # Example
///
/// ```
/// use problemreductions::models::specialized::PaintShop;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Sequence: a, b, a, c, c, b
/// let problem = PaintShop::new(vec!["a", "b", "a", "c", "c", "b"]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // The minimum number of color switches
/// for sol in &solutions {
///     let switches = problem.count_switches(sol);
///     println!("Switches: {}", switches);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaintShop {
    /// The sequence of car labels (as indices into unique cars).
    sequence_indices: Vec<usize>,
    /// Original car labels.
    car_labels: Vec<String>,
    /// Which positions are the first occurrence of each car.
    is_first: Vec<bool>,
    /// Number of unique cars.
    num_cars: usize,
}

impl PaintShop {
    /// Create a new Paint Shop problem from string labels.
    ///
    /// Each element in the sequence must appear exactly twice.
    pub fn new<S: AsRef<str>>(sequence: Vec<S>) -> Self {
        let sequence: Vec<String> = sequence.iter().map(|s| s.as_ref().to_string()).collect();
        Self::from_strings(sequence)
    }

    /// Create from a vector of strings.
    pub fn from_strings(sequence: Vec<String>) -> Self {
        // Build car-to-index mapping and count occurrences
        let mut car_count: HashMap<String, usize> = HashMap::new();
        let mut car_to_index: HashMap<String, usize> = HashMap::new();
        let mut car_labels: Vec<String> = Vec::new();

        for item in &sequence {
            let count = car_count.entry(item.clone()).or_insert(0);
            if *count == 0 {
                car_to_index.insert(item.clone(), car_labels.len());
                car_labels.push(item.clone());
            }
            *count += 1;
        }

        // Verify each car appears exactly twice
        for (car, count) in &car_count {
            assert_eq!(
                *count, 2,
                "Each car must appear exactly twice, but '{}' appears {} times",
                car, count
            );
        }

        // Convert sequence to indices
        let sequence_indices: Vec<usize> = sequence.iter().map(|item| car_to_index[item]).collect();

        // Determine which positions are first occurrences
        let mut seen: HashSet<usize> = HashSet::new();
        let is_first: Vec<bool> = sequence_indices
            .iter()
            .map(|&idx| seen.insert(idx))
            .collect();

        let num_cars = car_labels.len();

        Self {
            sequence_indices,
            car_labels,
            is_first,
            num_cars,
        }
    }

    /// Get the sequence length.
    pub fn sequence_len(&self) -> usize {
        self.sequence_indices.len()
    }

    /// Get the number of unique cars.
    pub fn num_cars(&self) -> usize {
        self.num_cars
    }

    /// Get the car labels.
    pub fn car_labels(&self) -> &[String] {
        &self.car_labels
    }

    /// Get the coloring of the sequence from a configuration.
    ///
    /// Config assigns a color (0 or 1) to each car for its first occurrence.
    /// The second occurrence gets the opposite color.
    pub fn get_coloring(&self, config: &[usize]) -> Vec<usize> {
        self.sequence_indices
            .iter()
            .enumerate()
            .map(|(i, &car_idx)| {
                let first_color = config.get(car_idx).copied().unwrap_or(0);
                if self.is_first[i] {
                    first_color
                } else {
                    1 - first_color // Opposite color for second occurrence
                }
            })
            .collect()
    }

    /// Count the number of color switches in the sequence.
    pub fn count_switches(&self, config: &[usize]) -> usize {
        let coloring = self.get_coloring(config);
        coloring.windows(2).filter(|w| w[0] != w[1]).count()
    }
}

/// Count color switches in a painted sequence.
pub fn count_paint_switches(coloring: &[usize]) -> usize {
    coloring.windows(2).filter(|w| w[0] != w[1]).count()
}

impl Problem for PaintShop {
    const NAME: &'static str = "PaintShop";
    type Metric = SolutionSize<i32>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_cars]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        // All configurations are valid (no hard constraints).
        SolutionSize::Valid(self.count_switches(config) as i32)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_cars", self.num_cars()),
            ("num_sequence", self.sequence_len()),
        ])
    }
}

impl OptimizationProblem for PaintShop {
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/specialized/paintshop.rs"]
mod tests;
