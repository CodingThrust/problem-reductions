//! Minimum Geometric Connected Dominating Set.
//!
//! Given a set of points P in the plane and a distance threshold B > 0,
//! find a minimum subset P' ⊆ P such that:
//! 1. Every point in P \ P' is within Euclidean distance B of some point in P' (domination).
//! 2. The subgraph induced on P' (edges between points within distance B) is connected.

use crate::registry::{ConstructionError, FieldInfo, ProblemSchemaEntry};
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
        category: crate::registry::ProblemCategory::Graph,
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
/// use problemreductions::{Problem, BruteForce};
///
/// // Four collinear points with spacing 3 and radius 3.5:
/// // each point reaches its immediate neighbor but not two steps away.
/// let points = vec![(0.0, 0.0), (3.0, 0.0), (6.0, 0.0), (9.0, 0.0)];
/// let problem = MinimumGeometricConnectedDominatingSet::new(points, 3.5).unwrap();
///
/// let solver = BruteForce::new();
/// let witness = solver.solve(&problem).unwrap().unwrap();
/// let value = problem.evaluate(&witness).unwrap().unwrap();
/// assert_eq!(value, 2); // Two interior points dominate all and form a connected pair
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MinimumGeometricConnectedDominatingSet {
    /// The set of points in the plane.
    points: Vec<(f64, f64)>,
    /// The distance threshold B.
    radius: f64,
}

impl MinimumGeometricConnectedDominatingSet {
    /// Create a new instance.
    ///
    pub fn new(points: Vec<(f64, f64)>, radius: f64) -> Result<Self, ConstructionError> {
        if points.is_empty() {
            return Err(ConstructionError::Conversion(
                "points must be non-empty".into(),
            ));
        }
        if !radius.is_finite() || radius <= 0.0 {
            if !radius.is_finite() {
                return Err(ConstructionError::NonFiniteFloat(
                    "radius must be finite".into(),
                ));
            }
            return Err(ConstructionError::Conversion(
                "radius must be positive".into(),
            ));
        }
        for (index, &(x, y)) in points.iter().enumerate() {
            if !x.is_finite() || !y.is_finite() {
                return Err(ConstructionError::NonFiniteFloat(format!(
                    "point at index {index} must have finite coordinates"
                )));
            }
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
    fn dist_sq(a: (f64, f64), b: (f64, f64)) -> Result<f64, crate::traits::EvaluationError> {
        let dx = a.0 - b.0;
        let dy = a.1 - b.1;
        let distance = dx * dx + dy * dy;
        distance.is_finite().then_some(distance).ok_or_else(|| {
            crate::traits::EvaluationError::NonFiniteResult(
                "computing geometric squared distance".into(),
            )
        })
    }

    /// Check if two points are within distance B.
    fn within_radius(
        &self,
        i: usize,
        j: usize,
        radius_squared: f64,
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(Self::dist_sq(self.points[i], self.points[j])? <= radius_squared)
    }

    /// Check if a configuration is a valid connected dominating set.
    pub fn is_valid_solution(
        &self,
        config: &[bool],
    ) -> Result<bool, crate::traits::EvaluationError> {
        if config.len() != self.points.len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "geometric connected dominating set expects one Boolean value per point".into(),
            ));
        }
        let radius_squared = self.radius * self.radius;
        if !radius_squared.is_finite() {
            return Err(crate::traits::EvaluationError::NonFiniteResult(
                "squaring the geometric radius".into(),
            ));
        }
        let selected: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(i, _)| i)
            .collect();

        if selected.is_empty() {
            return Ok(false);
        }

        // Check domination: every unselected point must be within distance B
        // of some selected point.
        for (i, &v) in config.iter().enumerate() {
            if !v {
                let mut dominated = false;
                for &s in &selected {
                    if self.within_radius(i, s, radius_squared)? {
                        dominated = true;
                        break;
                    }
                }
                if !dominated {
                    return Ok(false);
                }
            }
        }

        // Check connectivity: BFS on selected points using distance-B edges.
        if selected.len() == 1 {
            return Ok(true);
        }
        let mut visited = vec![false; selected.len()];
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        while let Some(u) = queue.pop_front() {
            for (vi, &vj) in selected.iter().enumerate() {
                if !visited[vi] && self.within_radius(selected[u], vj, radius_squared)? {
                    visited[vi] = true;
                    queue.push_back(vi);
                }
            }
        }
        Ok(visited.iter().all(|&v| v))
    }
}

impl<'de> Deserialize<'de> for MinimumGeometricConnectedDominatingSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            points: Vec<(f64, f64)>,
            radius: f64,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.points, raw.radius).map_err(serde::de::Error::custom)
    }
}

impl Problem for MinimumGeometricConnectedDominatingSet {
    const NAME: &'static str = "MinimumGeometricConnectedDominatingSet";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_points", num_points),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if !self.is_valid_solution(config)? {
                return Ok(Min(None));
            }
            let count = config.iter().filter(|&&v| v).count();
            Min(Some(i64::try_from(count).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting dominating-set cardinality to i64".into(),
                )
            })?))
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumGeometricConnectedDominatingSet {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_points()]
    }
}

crate::declare_variants! {
    default MinimumGeometricConnectedDominatingSet => "2^num_points",
}

crate::register_brute_force! {
    MinimumGeometricConnectedDominatingSet decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_geometric_connected_dominating_set",
        instance: Box::new(
            MinimumGeometricConnectedDominatingSet::new(
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
            )
            .expect("canonical geometric connected-dominating-set instance must be valid"),
        ),
        optimal_config: serde_json::json!(vec![true, true, true, true, false, false, false, false]),
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_geometric_connected_dominating_set.rs"]
mod tests;
