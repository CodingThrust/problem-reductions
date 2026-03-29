use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Issue #530 example: 3x6 matrix, rhs=[7,5,3], S={0,1}.
fn issue_example() -> FeasibleBasisExtension {
    FeasibleBasisExtension::new(
        vec![
            vec![1, 0, 1, 2, -1, 0],
            vec![0, 1, 0, 1, 1, 2],
            vec![0, 0, 1, 1, 0, 1],
        ],
        vec![7, 5, 3],
        vec![0, 1],
    )
}

#[test]
fn test_feasible_basis_extension_creation() {
    let problem = issue_example();
    assert_eq!(problem.num_rows(), 3);
    assert_eq!(problem.num_columns(), 6);
    assert_eq!(problem.num_required(), 2);
    assert_eq!(problem.dims(), vec![2; 4]); // 6 - 2 = 4 free columns
    assert_eq!(
        <FeasibleBasisExtension as Problem>::NAME,
        "FeasibleBasisExtension"
    );
    assert_eq!(<FeasibleBasisExtension as Problem>::variant(), vec![]);
}

#[test]
fn test_feasible_basis_extension_evaluate_satisfying() {
    let problem = issue_example();
    // Free columns are [2, 3, 4, 5]. Select col 2 (index 0 in free list).
    // B = {0, 1, 2}. A_B = I_3. x = (7, 5, 3) but actually A_B = [[1,0,1],[0,1,0],[0,0,1]]
    // A_B^{-1} a_bar: solve [[1,0,1],[0,1,0],[0,0,1]] x = [7,5,3] => x = (4, 5, 3) >= 0
    assert!(problem.evaluate(&[1, 0, 0, 0]));
}

#[test]
fn test_feasible_basis_extension_evaluate_satisfying_col3() {
    let problem = issue_example();
    // Select col 3 (index 1 in free list). B = {0, 1, 3}.
    // A_B = [[1,0,2],[0,1,1],[0,0,1]]. Solve: x = (1, 2, 3) >= 0
    assert!(problem.evaluate(&[0, 1, 0, 0]));
}

#[test]
fn test_feasible_basis_extension_evaluate_singular() {
    let problem = issue_example();
    // Select col 4 (index 2 in free list). B = {0, 1, 4}.
    // A_B = [[1,0,-1],[0,1,1],[0,0,0]] => singular
    assert!(!problem.evaluate(&[0, 0, 1, 0]));
}

#[test]
fn test_feasible_basis_extension_evaluate_infeasible_negative() {
    let problem = issue_example();
    // Select col 5 (index 3 in free list). B = {0, 1, 5}.
    // A_B = [[1,0,0],[0,1,2],[0,0,1]]. Solve: x = (7, -1, 3). x_1 = -1 < 0
    assert!(!problem.evaluate(&[0, 0, 0, 1]));
}

#[test]
fn test_feasible_basis_extension_evaluate_wrong_count() {
    let problem = issue_example();
    // Need exactly 1 column selected (m - |S| = 3 - 2 = 1)
    assert!(!problem.evaluate(&[1, 1, 0, 0])); // too many
    assert!(!problem.evaluate(&[0, 0, 0, 0])); // too few
}

#[test]
fn test_feasible_basis_extension_evaluate_wrong_config_length() {
    let problem = issue_example();
    assert!(!problem.evaluate(&[1, 0])); // too short
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0])); // too long
}

#[test]
fn test_feasible_basis_extension_evaluate_invalid_variable_value() {
    let problem = issue_example();
    assert!(!problem.evaluate(&[2, 0, 0, 0]));
}

#[test]
fn test_feasible_basis_extension_brute_force() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_feasible_basis_extension_brute_force_all() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // From the issue: B={0,1,2} and B={0,1,3} are feasible, so 2 solutions
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_feasible_basis_extension_unsatisfiable() {
    // Construct an instance where no feasible basis exists.
    // 2x3 matrix, rhs=[1,-1], S={} (need 2 columns from 3).
    // A = [[1,0,1],[0,1,-1]], rhs = [1,-1]
    // B={0,1}: solve [[1,0],[0,1]]x=[1,-1] => x=(1,-1), x_1<0
    // B={0,2}: solve [[1,1],[0,-1]]x=[1,-1] => x=(0,1), x>=0 => feasible!
    // Let's try: A = [[1,1,1],[1,1,1]], rhs = [1,1]. All 2x2 submatrices are singular.
    let problem =
        FeasibleBasisExtension::new(vec![vec![1, 1, 1], vec![1, 1, 1]], vec![1, 1], vec![]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_feasible_basis_extension_serialization() {
    let problem = issue_example();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [
                [1, 0, 1, 2, -1, 0],
                [0, 1, 0, 1, 1, 2],
                [0, 0, 1, 1, 0, 1],
            ],
            "rhs": [7, 5, 3],
            "required_columns": [0, 1],
        })
    );
    let restored: FeasibleBasisExtension = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_rows(), 3);
    assert_eq!(restored.num_columns(), 6);
    assert_eq!(restored.num_required(), 2);
}

#[test]
fn test_feasible_basis_extension_paper_example() {
    let problem = issue_example();
    // Verify B={0,1,2} is satisfying (config [1,0,0,0])
    assert!(problem.evaluate(&[1, 0, 0, 0]));
    // Verify B={0,1,3} is satisfying (config [0,1,0,0])
    assert!(problem.evaluate(&[0, 1, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 2);
}

#[test]
fn test_feasible_basis_extension_complexity_metadata() {
    use crate::registry::VariantEntry;

    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == "FeasibleBasisExtension")
        .expect("FeasibleBasisExtension variant entry should exist");

    assert_eq!(entry.complexity, "2^num_columns * num_rows^3");
}

#[test]
#[should_panic(expected = "must be less than")]
fn test_feasible_basis_extension_m_ge_n() {
    // 3x3 matrix: m not < n
    FeasibleBasisExtension::new(
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
        vec![1, 1, 1],
        vec![],
    );
}

#[test]
#[should_panic(expected = "rhs length")]
fn test_feasible_basis_extension_rhs_length_mismatch() {
    FeasibleBasisExtension::new(
        vec![vec![1, 0, 1], vec![0, 1, 0]],
        vec![1, 2, 3], // length 3, but m=2
        vec![],
    );
}

#[test]
#[should_panic(expected = "|S|")]
fn test_feasible_basis_extension_too_many_required() {
    // m=2, |S|=2 is not < m
    FeasibleBasisExtension::new(vec![vec![1, 0, 1], vec![0, 1, 0]], vec![1, 2], vec![0, 1]);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_feasible_basis_extension_required_out_of_bounds() {
    FeasibleBasisExtension::new(
        vec![vec![1, 0, 1], vec![0, 1, 0]],
        vec![1, 2],
        vec![5], // out of bounds
    );
}

#[test]
#[should_panic(expected = "Duplicate")]
fn test_feasible_basis_extension_duplicate_required() {
    // 3x5 matrix so |S|=2 < m=3, but S has duplicates
    FeasibleBasisExtension::new(
        vec![
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 0],
            vec![1, 1, 0, 0, 1],
        ],
        vec![1, 2, 3],
        vec![0, 0],
    );
}
