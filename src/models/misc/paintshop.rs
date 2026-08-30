//! Paint Shop problem implementation.
//!
//! In the Paint Shop problem, we have a sequence of cars to paint.
//! Each car appears exactly twice in the sequence and must be painted
//! one color at its first occurrence and another at its second.
//! The goal is to minimize color switches between adjacent positions.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PaintShop",
        display_name: "Paint Shop",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Minimize color changes in paint shop sequence",
        fields: &[
            FieldInfo { name: "sequence", type_name: "Vec<String>", description: "Car labels (each must appear exactly twice)" },
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
/// use problemreductions::models::misc::PaintShop;
/// use problemreductions::{Problem, BruteForce};
///
/// // Sequence: a, b, a, c, c, b
/// let problem = PaintShop::new(vec!["a", "b", "a", "c", "c", "b"]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // The minimum number of color switches
/// for sol in &solutions {
///     let switches = problem.count_switches(sol).unwrap();
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

    /// Get the sequence length (alias for `sequence_len()`).
    pub fn num_sequence(&self) -> usize {
        self.sequence_len()
    }

    /// Get the number of unique cars.
    pub fn num_cars(&self) -> usize {
        self.num_cars
    }

    /// Get the car labels.
    pub fn car_labels(&self) -> &[String] {
        &self.car_labels
    }

    /// Get the sequence as car indices.
    pub fn sequence_indices(&self) -> &[usize] {
        &self.sequence_indices
    }

    /// Get whether each position is the first occurrence of its car.
    pub fn is_first(&self) -> &[bool] {
        &self.is_first
    }

    /// Get the coloring of the sequence from a configuration.
    ///
    /// Config assigns a color (0 or 1) to each car for its first occurrence.
    /// The second occurrence gets the opposite color.
    pub fn get_coloring(
        &self,
        config: &[bool],
    ) -> Result<Vec<bool>, crate::traits::EvaluationError> {
        if config.len() != self.num_cars {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "paint assignment length does not match the cars".into(),
            ));
        }
        Ok(self
            .sequence_indices
            .iter()
            .enumerate()
            .map(|(i, &car_idx)| {
                let first_color = config[car_idx];
                if self.is_first[i] {
                    first_color
                } else {
                    !first_color // Opposite color for second occurrence
                }
            })
            .collect())
    }

    /// Count the number of color switches in the sequence.
    pub fn count_switches(&self, config: &[bool]) -> Result<i64, crate::traits::EvaluationError> {
        let coloring = self.get_coloring(config)?;
        let count = coloring.windows(2).filter(|w| w[0] != w[1]).count();
        i64::try_from(count).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow(
                "converting paint-switch count to i64".into(),
            )
        })
    }
}

/// Count color switches in a painted sequence.
#[cfg(test)]
pub(crate) fn count_paint_switches(coloring: &[bool]) -> usize {
    coloring.windows(2).filter(|w| w[0] != w[1]).count()
}

impl Problem for PaintShop {
    const NAME: &'static str = "PaintShop";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_cars", num_cars), ("num_sequence", num_sequence),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            // All configurations are valid (no hard constraints).
            Min(Some(self.count_switches(config)?))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for PaintShop {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_cars]
    }
}

crate::declare_variants! {
    default PaintShop => "2^num_cars",
}

crate::register_brute_force! {
    PaintShop decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "paintshop",
        instance: Box::new(PaintShop::new(vec!["A", "B", "A", "C", "B", "C"])),
        optimal_config: serde_json::json!(vec![false, false, true]),
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/paintshop.rs"]
mod tests;
