use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_creation_and_getters() {
    let points = vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];
    let problem = MinimumGeometricConnectedDominatingSet::new(points, 1.5);
    assert_eq!(problem.num_points(), 3);
    assert!((problem.radius() - 1.5).abs() < f64::EPSILON);
    assert_eq!(problem.points().len(), 3);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.dims(), vec![2; 3]);
}

#[test]
#[should_panic(expected = "radius must be positive")]
fn test_negative_radius_panics() {
    MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0)], -1.0);
}

#[test]
#[should_panic(expected = "points must be non-empty")]
fn test_empty_points_panics() {
    MinimumGeometricConnectedDominatingSet::new(vec![], 1.0);
}

#[test]
fn test_try_new_errors() {
    assert!(MinimumGeometricConnectedDominatingSet::try_new(vec![], 1.0).is_err());
    assert!(MinimumGeometricConnectedDominatingSet::try_new(vec![(0.0, 0.0)], 0.0).is_err());
    assert!(MinimumGeometricConnectedDominatingSet::try_new(vec![(0.0, 0.0)], -1.0).is_err());
    assert!(MinimumGeometricConnectedDominatingSet::try_new(vec![(0.0, 0.0)], 1.0).is_ok());
}

#[test]
fn test_single_point() {
    let problem = MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0)], 1.0);
    // Selecting the single point is valid
    let result = problem.evaluate(&[1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);
    // Not selecting is invalid (empty set)
    let result = problem.evaluate(&[0]);
    assert!(!result.is_valid());
}

#[test]
fn test_evaluate_domination_failure() {
    // Two points far apart, radius too small
    let problem = MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0), (10.0, 0.0)], 1.0);
    // Only select first point: second point not dominated
    let result = problem.evaluate(&[1, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_evaluate_connectivity_failure() {
    // Three points in a line, select endpoints but not middle
    // With radius=1.5, each point covers its neighbor but endpoints aren't connected
    let problem =
        MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)], 1.5);
    // Select points 0 and 2 (not connected to each other, distance = 2.0 > 1.5)
    let result = problem.evaluate(&[1, 0, 1]);
    assert!(!result.is_valid());
}

#[test]
fn test_evaluate_valid_connected_dominating_set() {
    // Three collinear points, select first two
    let problem =
        MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)], 1.5);
    // Select 0 and 1: they are connected (dist=1.0 <= 1.5), and point 2 is dominated by point 1 (dist=1.0 <= 1.5)
    let result = problem.evaluate(&[1, 1, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_brute_force_line_graph() {
    // Line of 3 points, middle point dominates all
    let problem =
        MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)], 1.5);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness).unwrap();
    // Middle point alone dominates all and is trivially connected
    assert_eq!(value, 1);
}

#[test]
fn test_ladder_example() {
    // 8 points in a ladder: [(0,0),(3,0),(6,0),(9,0),(0,3),(3,3),(6,3),(9,3)], B=3.5
    let problem = MinimumGeometricConnectedDominatingSet::new(
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
    );
    // Bottom row selected: config [1,1,1,1,0,0,0,0]
    let config = vec![1, 1, 1, 1, 0, 0, 0, 0];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 4);

    // Verify with brute force
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let best_value = problem.evaluate(&witness).unwrap();
    assert_eq!(best_value, 4);
}

#[test]
fn test_serialization_roundtrip() {
    let problem =
        MinimumGeometricConnectedDominatingSet::new(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)], 2.0);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumGeometricConnectedDominatingSet = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_points(), 3);
    assert!((deserialized.radius() - 2.0).abs() < f64::EPSILON);
}
