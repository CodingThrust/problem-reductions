use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

/// H (3×4): [[1,0,1,1],[0,1,1,0],[1,1,0,1]], s = [1,1,0]
fn example_instance() -> MinimumWeightDecoding {
    let matrix = vec![
        vec![true, false, true, true],
        vec![false, true, true, false],
        vec![true, true, false, true],
    ];
    let target = vec![true, true, false];
    MinimumWeightDecoding::new(matrix, target)
}

#[test]
fn test_minimum_weight_decoding_creation() {
    let problem = example_instance();
    assert_eq!(problem.num_rows(), 3);
    assert_eq!(problem.num_cols(), 4);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(
        <MinimumWeightDecoding as Problem>::NAME,
        "MinimumWeightDecoding"
    );
    assert_eq!(<MinimumWeightDecoding as Problem>::variant(), vec![]);
}

#[test]
fn test_minimum_weight_decoding_evaluate_feasible() {
    let problem = example_instance();
    // Config [0,0,1,0] → weight 1
    // Row 0: H[0][2]=1, x[2]=1 → dot=1 mod 2 = 1 = s[0]=true ✓
    // Row 1: H[1][2]=1, x[2]=1 → dot=1 mod 2 = 1 = s[1]=true ✓
    // Row 2: H[2][2]=0 → dot=0 mod 2 = 0 = s[2]=false ✓
    let config = vec![0, 0, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(1)));
}

#[test]
fn test_minimum_weight_decoding_evaluate_infeasible() {
    let problem = example_instance();
    // Config [0,0,0,0] → all zeros, Hx = [0,0,0] but s = [1,1,0] → infeasible
    let config = vec![0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_weight_decoding_evaluate_heavier_feasible() {
    let problem = example_instance();
    // Config [1,0,0,1] → weight 2
    // Row 0: H[0][0]=1, H[0][3]=1 → dot=2 mod 2=0, s[0]=true → 0≠1 infeasible
    let config = vec![1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(None));

    // Config [1,1,0,0] → weight 2
    // Row 0: H[0][0]=1 → dot=1, mod 2=1, s[0]=true ✓
    // Row 1: H[1][1]=1 → dot=1, mod 2=1, s[1]=true ✓
    // Row 2: H[2][0]=1,H[2][1]=1 → dot=2, mod 2=0, s[2]=false ✓
    let config2 = vec![1, 1, 0, 0];
    assert_eq!(problem.evaluate(&config2), Min(Some(2)));
}

#[test]
fn test_minimum_weight_decoding_evaluate_wrong_length() {
    let problem = example_instance();
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1; 5]), Min(None));
}

#[test]
fn test_minimum_weight_decoding_evaluate_invalid_variable() {
    let problem = example_instance();
    let config = vec![2, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_weight_decoding_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    let val = problem.evaluate(&witness);
    // Optimal is weight 1 with config [0,0,1,0]
    assert_eq!(val, Min(Some(1)));
}

#[test]
fn test_minimum_weight_decoding_all_witnesses() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&problem);
    // All witnesses should be feasible and have weight 1
    assert!(!witnesses.is_empty());
    for w in &witnesses {
        assert_eq!(problem.evaluate(w), Min(Some(1)));
    }
}

#[test]
fn test_minimum_weight_decoding_serialization() {
    let problem = example_instance();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [[true, false, true, true], [false, true, true, false], [true, true, false, true]],
            "target": [true, true, false],
        })
    );
    let restored: MinimumWeightDecoding = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_rows(), 3);
    assert_eq!(restored.num_cols(), 4);
}

#[test]
fn test_minimum_weight_decoding_zero_syndrome() {
    // s = [0,0] → x = [0,0,0] is feasible with weight 0
    let matrix = vec![vec![true, false, true], vec![false, true, true]];
    let target = vec![false, false];
    let problem = MinimumWeightDecoding::new(matrix, target);
    let config = vec![0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(0)));
}

#[test]
fn test_minimum_weight_decoding_complexity_metadata() {
    use crate::registry::VariantEntry;

    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == "MinimumWeightDecoding")
        .expect("MinimumWeightDecoding variant entry should exist");

    assert_eq!(entry.complexity, "2^(0.0494 * num_cols)");
}

#[test]
#[should_panic(expected = "at least one row")]
fn test_minimum_weight_decoding_empty_matrix() {
    MinimumWeightDecoding::new(vec![], vec![]);
}

#[test]
#[should_panic(expected = "same length")]
fn test_minimum_weight_decoding_inconsistent_rows() {
    let matrix = vec![vec![true, false], vec![true]];
    MinimumWeightDecoding::new(matrix, vec![true, false]);
}

#[test]
#[should_panic(expected = "Target length")]
fn test_minimum_weight_decoding_target_mismatch() {
    let matrix = vec![vec![true, false], vec![false, true]];
    MinimumWeightDecoding::new(matrix, vec![true]);
}
