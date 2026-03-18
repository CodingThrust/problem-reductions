use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

/// Create a 4x4 test instance matching issue #300's example.
///
/// Cost matrix C (flows between 4 facilities):
/// [[0, 5, 2, 0],
///  [5, 0, 0, 3],
///  [2, 0, 0, 4],
///  [0, 3, 4, 0]]
///
/// Distance matrix D (distances between 4 locations):
/// [[0, 4, 1, 1],
///  [4, 0, 3, 4],
///  [1, 3, 0, 4],
///  [1, 4, 4, 0]]
///
/// Optimal assignment: f* = (3, 0, 1, 2) with cost 56.
fn make_test_instance() -> QuadraticAssignment {
    let cost_matrix = vec![
        vec![0, 5, 2, 0],
        vec![5, 0, 0, 3],
        vec![2, 0, 0, 4],
        vec![0, 3, 4, 0],
    ];
    let distance_matrix = vec![
        vec![0, 4, 1, 1],
        vec![4, 0, 3, 4],
        vec![1, 3, 0, 4],
        vec![1, 4, 4, 0],
    ];
    QuadraticAssignment::new(cost_matrix, distance_matrix)
}

#[test]
fn test_quadratic_assignment_creation() {
    let qap = make_test_instance();
    assert_eq!(qap.num_facilities(), 4);
    assert_eq!(qap.num_locations(), 4);
    assert_eq!(qap.dims(), vec![4, 4, 4, 4]);
    assert_eq!(qap.cost_matrix().len(), 4);
    assert_eq!(qap.distance_matrix().len(), 4);
}

#[test]
fn test_quadratic_assignment_evaluate_identity() {
    let qap = make_test_instance();
    // Identity assignment f = (0, 1, 2, 3):
    // cost = sum_{i != j} C[i][j] * D[i][j]
    //   = 5*4 + 2*1 + 0*1 + 5*4 + 0*3 + 3*4 + 2*1 + 0*3 + 4*4 + 0*1 + 3*4 + 4*4
    //   = 20 + 2 + 0 + 20 + 0 + 12 + 2 + 0 + 16 + 0 + 12 + 16 = 100
    assert_eq!(
        Problem::evaluate(&qap, &[0, 1, 2, 3]),
        SolutionSize::Valid(100)
    );
}

#[test]
fn test_quadratic_assignment_evaluate_swap() {
    let qap = make_test_instance();
    // Assignment f = (0, 2, 1, 3): facility 1 -> loc 2, facility 2 -> loc 1
    // cost = sum_{i != j} C[i][j] * D[config[i]][config[j]]
    //   i=0,j=1: 5*D[0][2]=5*1=5   i=0,j=2: 2*D[0][1]=2*4=8   i=0,j=3: 0*D[0][3]=0
    //   i=1,j=0: 5*D[2][0]=5*1=5   i=1,j=2: 0*D[2][1]=0*3=0   i=1,j=3: 3*D[2][3]=3*4=12
    //   i=2,j=0: 2*D[1][0]=2*4=8   i=2,j=1: 0*D[1][2]=0*3=0   i=2,j=3: 4*D[1][3]=4*4=16
    //   i=3,j=0: 0*D[3][0]=0       i=3,j=1: 3*D[3][2]=3*4=12  i=3,j=2: 4*D[3][1]=4*4=16
    //   Total = 5+8+0+5+0+12+8+0+16+0+12+16 = 82
    assert_eq!(
        Problem::evaluate(&qap, &[0, 2, 1, 3]),
        SolutionSize::Valid(82)
    );
}

#[test]
fn test_quadratic_assignment_evaluate_invalid() {
    let qap = make_test_instance();
    // Duplicate location 0 — not injective, should be Invalid.
    assert_eq!(
        Problem::evaluate(&qap, &[0, 0, 1, 2]),
        SolutionSize::Invalid
    );
    // Out-of-range location index.
    assert_eq!(
        Problem::evaluate(&qap, &[0, 1, 2, 99]),
        SolutionSize::Invalid
    );
    // Wrong config length — too short.
    assert_eq!(Problem::evaluate(&qap, &[0, 1, 2]), SolutionSize::Invalid);
    // Wrong config length — too long.
    assert_eq!(
        Problem::evaluate(&qap, &[0, 1, 2, 3, 0]),
        SolutionSize::Invalid
    );
}

#[test]
fn test_quadratic_assignment_direction() {
    let qap = make_test_instance();
    assert_eq!(qap.direction(), Direction::Minimize);
}

#[test]
fn test_quadratic_assignment_serialization() {
    let qap = make_test_instance();
    let json = serde_json::to_string(&qap).expect("serialize");
    let qap2: QuadraticAssignment = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(qap2.num_facilities(), 4);
    assert_eq!(qap2.num_locations(), 4);
    // Verify functional equivalence after round-trip.
    assert_eq!(
        Problem::evaluate(&qap, &[0, 1, 2, 3]),
        Problem::evaluate(&qap2, &[0, 1, 2, 3])
    );
}

#[test]
fn test_quadratic_assignment_rectangular() {
    // 2 facilities, 3 locations (n < m)
    let cost_matrix = vec![vec![0, 3], vec![3, 0]];
    let distance_matrix = vec![vec![0, 1, 4], vec![1, 0, 2], vec![4, 2, 0]];
    let qap = QuadraticAssignment::new(cost_matrix, distance_matrix);
    assert_eq!(qap.num_facilities(), 2);
    assert_eq!(qap.num_locations(), 3);
    assert_eq!(qap.dims(), vec![3, 3]);
    // Assignment f=(0,1): cost = C[0][1]*D[0][1] + C[1][0]*D[1][0] = 3*1 + 3*1 = 6
    assert_eq!(Problem::evaluate(&qap, &[0, 1]), SolutionSize::Valid(6));
    // Assignment f=(0,2): cost = 3*D[0][2] + 3*D[2][0] = 3*4 + 3*4 = 24
    assert_eq!(Problem::evaluate(&qap, &[0, 2]), SolutionSize::Valid(24));
    // BruteForce should find optimal
    let solver = BruteForce::new();
    let best = solver.find_best(&qap).unwrap();
    assert_eq!(Problem::evaluate(&qap, &best), SolutionSize::Valid(6));
}

#[test]
#[should_panic(expected = "cost_matrix must be square")]
fn test_quadratic_assignment_nonsquare_cost() {
    QuadraticAssignment::new(vec![vec![0, 1]], vec![vec![0, 1], vec![1, 0]]);
}

#[test]
#[should_panic(expected = "num_facilities")]
fn test_quadratic_assignment_too_many_facilities() {
    // 3 facilities, 2 locations (n > m) -- should panic
    let cost = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
    let dist = vec![vec![0, 1], vec![1, 0]];
    QuadraticAssignment::new(cost, dist);
}

#[test]
fn test_quadratic_assignment_solver() {
    let qap = make_test_instance();
    let solver = BruteForce::new();
    let best = solver.find_best(&qap);
    assert!(best.is_some());
    let best_config = best.unwrap();
    // The brute-force solver finds the optimal assignment f* = (3, 0, 1, 2) with cost 56.
    assert_eq!(
        Problem::evaluate(&qap, &best_config),
        SolutionSize::Valid(56)
    );
}
