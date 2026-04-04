//! Minimum Geometric Connected Dominating Set.
//!
//! Given a set of points P in the plane and a distance threshold B > 0,
//! find a minimum subset P' ⊆ P such that:
//! 1. Every point in P \ P' is within Euclidean distance B of some point in P' (domination).
//! 2. The subgraph induced on P' (edges between points within distance B) is connected.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumGeometricConnectedDominatingSet",
        display_name: "Minimum Geometric Connected Dominating Set",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum connected dominating set in a geometric point set",
        fields: &[
            FieldInfo {
                name: "points",
                type_name: "Vec<(f64, f64)>",
                description: "The set of points P in the plane",
            },
            FieldInfo {
                name: "radius",
                type_name: "f64",
                description: "The distance threshold B",
            },
        ],
    }
}

/// Minimum Geometric Connected Dominating Set.
///
/// Given points P in the plane and distance threshold B > 0,
/// find a minimum subset P' ⊆ P such that every point in P \ P'
/// is within distance B of some point in P', and the subgraph
/// induced on P' (edges between points within distance B) is connected.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumGeometricConnectedDominatingSet;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Four collinear points with spacing 3 and radius 3.5:
/// // each point reaches its immediate neighbor but not two steps away.
/// let points = vec![(0.0, 0.0), (3.0, 0.0), (6.0, 0.0), (9.0, 0.0)];
/// let problem = MinimumGeometricConnectedDominatingSet::new(points, 3.5);
///
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem).unwrap();
/// let value = problem.evaluate(&witness).unwrap();
/// assert_eq!(value, 2); // Two interior points dominate all and form a connected pair
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumGeometricConnectedDominatingSet {
    /// The set of points in the plane.
    points: Vec<(f64, f64)>,
    /// The distance threshold B.
    radius: f64,
}

impl MinimumGeometricConnectedDominatingSet {
    /// Create a new instance.
    ///
    /// # Panics
    /// Panics if `radius <= 0.0` or if `points` is empty.
    pub fn new(points: Vec<(f64, f64)>, radius: f64) -> Self {
        assert!(radius > 0.0, "radius must be positive");
        assert!(!points.is_empty(), "points must be non-empty");
        Self { points, radius }
    }

    /// Fallible constructor used by CLI validation and deserialization.
    pub fn try_new(points: Vec<(f64, f64)>, radius: f64) -> Result<Self, String> {
        if radius <= 0.0 {
            return Err("radius must be positive".into());
        }
        if points.is_empty() {
            return Err("points must be non-empty".into());
        }
        Ok(Self { points, radius })
    }

    /// Get the number of points.
    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    /// Get the distance threshold.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Get a reference to the points.
    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }

    /// Squared Euclidean distance between two points.
    fn dist_sq(a: (f64, f64), b: (f64, f64)) -> f64 {
        let dx = a.0 - b.0;
        let dy = a.1 - b.1;
        dx * dx + dy * dy
    }

    /// Check if two points are within distance B.
    fn within_radius(&self, i: usize, j: usize) -> bool {
        Self::dist_sq(self.points[i], self.points[j]) <= self.radius * self.radius
    }

    /// Check if a configuration is a valid connected dominating set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        let selected: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();

        if selected.is_empty() {
            return false;
        }

        // Check domination: every unselected point must be within distance B
        // of some selected point.
        for (i, &v) in config.iter().enumerate() {
            if v == 0 && !selected.iter().any(|&s| self.within_radius(i, s)) {
                return false;
            }
        }

        // Check connectivity: BFS on selected points using distance-B edges.
        if selected.len() == 1 {
            return true;
        }
        let mut visited = vec![false; selected.len()];
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        while let Some(u) = queue.pop_front() {
            for (vi, &vj) in selected.iter().enumerate() {
                if !visited[vi] && self.within_radius(selected[u], vj) {
                    visited[vi] = true;
                    queue.push_back(vi);
                }
            }
        }
        visited.iter().all(|&v| v)
    }
}

impl Problem for MinimumGeometricConnectedDominatingSet {
    const NAME: &'static str = "MinimumGeometricConnectedDominatingSet";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_points()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if !self.is_valid_solution(config) {
            return Min(None);
        }
        let count = config.iter().filter(|&&v| v == 1).count();
        Min(Some(count))
    }
}

crate::declare_variants! {
    default MinimumGeometricConnectedDominatingSet => "2^num_points",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_geometric_connected_dominating_set",
        instance: Box::new(MinimumGeometricConnectedDominatingSet::new(
            vec![
                (0.0, 0.0),
                (3.0, 0.0),
                (6.0, 0.0),
                (9.0, 0.0),
                (0.0, 3.0),
                (3.0, 3.0),
                (6.0, 3.0),
                (9.0, 3.0),
            ],
            3.5,
        )),
        optimal_config: vec![1, 1, 1, 1, 0, 0, 0, 0],
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_geometric_connected_dominating_set.rs"]
mod tests;
