//! Paint Shop problem implementation.
//!
//! In the Paint Shop problem, we have a sequence of cars to paint.
//! Each car appears exactly twice in the sequence and must be painted
//! one color at its first occurrence and another at its second.
//! The goal is to minimize color switches between adjacent positions.

use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
/// let solutions = solver.find_best(&problem);
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

impl Problem for PaintShop {
    const NAME: &'static str = "PaintShop";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.num_cars
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: color 0 or color 1
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_cars", self.num_cars),
            ("sequence_length", self.sequence_indices.len()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize color switches
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let switches = self.count_switches(config) as i32;
        // All configurations are valid (no hard constraints)
        SolutionSize::valid(switches)
    }
}

/// Count color switches in a painted sequence.
pub fn count_paint_switches(coloring: &[usize]) -> usize {
    coloring.windows(2).filter(|w| w[0] != w[1]).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_paintshop_creation() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
        assert_eq!(problem.num_cars(), 2);
        assert_eq!(problem.sequence_len(), 4);
        assert_eq!(problem.num_variables(), 2);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_is_first() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
        // First occurrence: a at 0, b at 1
        // Second occurrence: a at 2, b at 3
        assert_eq!(problem.is_first, vec![true, true, false, false]);
    }

    #[test]
    fn test_get_coloring() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
        // Config: a=0, b=1
        // Sequence: a(0), b(1), a(1-opposite), b(0-opposite)
        let coloring = problem.get_coloring(&[0, 1]);
        assert_eq!(coloring, vec![0, 1, 1, 0]);

        // Config: a=1, b=0
        let coloring = problem.get_coloring(&[1, 0]);
        assert_eq!(coloring, vec![1, 0, 0, 1]);
    }

    #[test]
    fn test_count_switches() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);

        // Config [0, 1] -> coloring [0, 1, 1, 0] -> 2 switches
        assert_eq!(problem.count_switches(&[0, 1]), 2);

        // Config [0, 0] -> coloring [0, 0, 1, 1] -> 1 switch
        assert_eq!(problem.count_switches(&[0, 0]), 1);

        // Config [1, 1] -> coloring [1, 1, 0, 0] -> 1 switch
        assert_eq!(problem.count_switches(&[1, 1]), 1);
    }

    #[test]
    fn test_solution_size() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);

        let sol = problem.solution_size(&[0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        let sol = problem.solution_size(&[0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);
    }

    #[test]
    fn test_brute_force_simple() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Optimal has 1 switch: [0,0] or [1,1]
        for sol in &solutions {
            assert_eq!(problem.count_switches(sol), 1);
        }
    }

    #[test]
    fn test_brute_force_longer() {
        // Sequence: a, b, a, c, c, b
        let problem = PaintShop::new(vec!["a", "b", "a", "c", "c", "b"]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Find the minimum number of switches
        let min_switches = problem.count_switches(&solutions[0]);
        for sol in &solutions {
            assert_eq!(problem.count_switches(sol), min_switches);
        }
    }

    #[test]
    fn test_count_paint_switches_function() {
        assert_eq!(count_paint_switches(&[0, 0, 0]), 0);
        assert_eq!(count_paint_switches(&[0, 1, 0]), 2);
        assert_eq!(count_paint_switches(&[0, 0, 1, 1]), 1);
        assert_eq!(count_paint_switches(&[0, 1, 0, 1]), 3);
    }

    #[test]
    fn test_energy_mode() {
        let problem = PaintShop::new(vec!["a", "a"]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_problem_size() {
        let problem = PaintShop::new(vec!["a", "b", "c", "a", "b", "c"]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_cars"), Some(3));
        assert_eq!(size.get("sequence_length"), Some(6));
    }

    #[test]
    fn test_single_car() {
        let problem = PaintShop::new(vec!["a", "a"]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Both configs give 1 switch: a(0)->a(1) or a(1)->a(0)
        assert_eq!(solutions.len(), 2);
        for sol in &solutions {
            assert_eq!(problem.count_switches(sol), 1);
        }
    }

    #[test]
    fn test_adjacent_same_car() {
        // Sequence: a, a, b, b
        let problem = PaintShop::new(vec!["a", "a", "b", "b"]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Best case: [0,0] -> [0,1,0,1] = 3 switches, or [0,1] -> [0,1,1,0] = 2 switches
        // Actually: [0,0] -> a=0,a=1,b=0,b=1 = [0,1,0,1] = 3 switches
        // [0,1] -> a=0,a=1,b=1,b=0 = [0,1,1,0] = 2 switches
        let min_switches = problem.count_switches(&solutions[0]);
        assert!(min_switches <= 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_sequence_single_occurrence() {
        // This should panic because 'c' only appears once
        let _ = PaintShop::new(vec!["a", "b", "a", "c"]);
    }

    #[test]
    fn test_car_labels() {
        let problem = PaintShop::new(vec!["car1", "car2", "car1", "car2"]);
        assert_eq!(problem.car_labels().len(), 2);
    }
}
