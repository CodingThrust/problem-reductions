use super::*;
include!("../../jl_helpers.rs");
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_qubo_from_matrix() {
    let problem = QUBO::from_matrix(vec![vec![1, 2], vec![0, 3]]).unwrap();
    assert_eq!(problem.num_vars(), 2);
    assert_eq!(problem.get(0, 0), Some(1));
    assert_eq!(problem.get(0, 1), Some(2));
    assert_eq!(problem.get(1, 1), Some(3));
}

#[test]
fn test_qubo_new() {
    let problem = QUBO::new(vec![1.0, 2.0], vec![((0, 1), 3.0)]).unwrap();
    assert_eq!(problem.get(0, 0), Some(1.0));
    assert_eq!(problem.get(1, 1), Some(2.0));
    assert_eq!(problem.get(0, 1), Some(3.0));
}

#[test]
fn test_num_variables() {
    let problem = QUBO::<f64>::from_matrix(vec![vec![0.0; 5]; 5]).unwrap();
    assert_eq!(problem.num_variables().unwrap(), 5);
}

#[test]
fn test_matrix_access() {
    let problem = QUBO::from_matrix(vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 4.0, 5.0],
        vec![0.0, 0.0, 6.0],
    ])
    .unwrap();
    let matrix = problem.matrix();
    assert_eq!(matrix.rows(), 3);
    assert_eq!(matrix.outer_view(0).unwrap().data(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_empty_qubo() {
    let problem = QUBO::<f64>::from_matrix(vec![]).unwrap();
    assert_eq!(problem.num_vars(), 0);
    assert_eq!(
        Problem::evaluate(&problem, &vec![]).unwrap(),
        Min(Some(0.0))
    );
}

#[test]
fn test_qubo_rejects_invalid_configurations() {
    let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]).unwrap();
    for solution in [vec![true], vec![true, false, false]] {
        assert!(matches!(
            Problem::evaluate(&problem, &solution),
            Err(crate::traits::EvaluationError::InvalidConfiguration(_))
        ));
    }
    assert!(
        crate::registry::DynProblem::evaluate_dyn(&problem, &serde_json::json!([2, false]))
            .is_err()
    );
}

#[test]
fn test_qubo_new_reverse_indices() {
    // Test the case where (j, i) is provided with i < j
    let problem = QUBO::new(vec![1.0, 2.0], vec![((1, 0), 3.0)]).unwrap(); // j > i
    assert_eq!(problem.get(0, 1), Some(3.0)); // Should be stored at (0, 1)
}

#[test]
fn test_get_out_of_bounds() {
    let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]).unwrap();
    assert_eq!(problem.get(5, 5), None);
    assert_eq!(problem.get(0, 5), None);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/qubo.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let jl_matrix: Vec<Vec<f64>> = instance["instance"]["matrix"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                row.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect()
            })
            .collect();
        let n = jl_matrix.len();
        let mut rust_matrix = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            rust_matrix[i][i] = jl_matrix[i][i];
            for j in (i + 1)..n {
                rust_matrix[i][j] = jl_matrix[i][j] + jl_matrix[j][i];
            }
        }
        let problem = QUBO::from_matrix(rust_matrix).unwrap();
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_bool_config(&eval["config"]);
            let result = Problem::evaluate(&problem, &config).unwrap();
            let jl_size = eval["size"].as_f64().unwrap();
            assert!(result.is_valid(), "QUBO should always be valid");
            assert!(
                (result.unwrap() - jl_size).abs() < 1e-10,
                "QUBO value mismatch for config {:?}",
                config
            );
        }
        let best = BruteForce::new().find_all_witnesses(&problem).unwrap();
        let jl_best = jl_parse_bool_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<bool>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "QUBO best solutions mismatch");
    }
}

#[test]
fn test_qubo_paper_example() {
    // Paper: Q=[[-1,2,0],[0,-1,2],[0,0,-1]], min=-2 at (1,0,1)
    let problem = QUBO::from_matrix(vec![vec![-1, 2, 0], vec![0, -1, 2], vec![0, 0, -1]]).unwrap();
    assert_eq!(
        Problem::evaluate(&problem, &vec![true, false, true]).unwrap(),
        Min(Some(-2))
    );

    let solver = BruteForce::new();
    let best = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(Problem::evaluate(&problem, &best).unwrap(), Min(Some(-2)));
}

#[test]
fn test_qubo_create_spec_derives_num_vars() {
    let problem = QUBO::try_from(QuboCreateSpec {
        matrix: vec![vec![1, 2], vec![0, 3]],
    })
    .unwrap();

    assert_eq!(problem.num_vars(), 2);
    assert_eq!(QuboCreateSpec::<i64>::FIELDS[0].name, "matrix");
    assert_eq!(QuboCreateSpec::<i64>::FIELDS.len(), 1);
}

#[test]
fn test_qubo_f64_create_spec() {
    let problem = QUBO::<f64>::try_from(QuboCreateSpec {
        matrix: vec![vec![0.5, -1.25], vec![0.0, 2.0]],
    })
    .unwrap();

    assert_eq!(problem.get(0, 0), Some(0.5));
    assert_eq!(problem.get(0, 1), Some(-1.25));
}

#[test]
fn test_qubo_rejects_non_square_matrix() {
    let error = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![3.0]]).unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::Conversion(message) if message.contains("row 1")
    ));
}

#[test]
fn test_qubo_rejects_non_finite_coefficients() {
    let error = QUBO::from_matrix(vec![vec![0.0, f64::NAN], vec![0.0, 0.0]]).unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::NonFiniteFloat(message) if message.contains("(0, 1)")
    ));
    let error = QUBO::new(vec![f64::INFINITY], vec![]).unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::NonFiniteFloat(_)
    ));
}

#[test]
fn test_qubo_rejects_out_of_range_quadratic_index() {
    let error = QUBO::new(vec![1.0], vec![((0, 1), 2.0)]).unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::Conversion(message)
            if message.contains("outside 0..1")
    ));
}

#[test]
fn test_integer_qubo_reports_objective_overflow() {
    let problem = QUBO::from_matrix(vec![vec![i64::MAX, 1], vec![0, 0]]).unwrap();
    assert!(matches!(
        problem.evaluate(&vec![true, true]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn sparse_storage_preserves_every_assignment_and_sum_order() {
    let integer = vec![
        vec![3, -5, 0, 2],
        vec![99, 0, 7, -4],
        vec![0, 0, -6, 0],
        vec![0, 0, 0, 1],
    ];
    let floating = vec![
        vec![1e16, 1.0, -1e16, 0.0],
        vec![99.0, 0.5, 0.0, -0.25],
        vec![0.0, 0.0, -2.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];
    let int_problem = QUBO::from_matrix(integer.clone()).unwrap();
    let float_problem = QUBO::from_matrix(floating.clone()).unwrap();
    for mask in 0..16 {
        let solution: Vec<bool> = (0..4).map(|i| mask & (1 << i) != 0).collect();
        let mut int_value = 0i64;
        let mut float_value = 0.0f64;
        for i in 0..4 {
            for j in i..4 {
                if solution[i] && solution[j] {
                    int_value = int_value.checked_add(integer[i][j]).unwrap();
                    float_value += floating[i][j];
                }
            }
        }
        assert_eq!(
            int_problem.evaluate(&solution).unwrap(),
            Min(Some(int_value))
        );
        assert_eq!(
            float_problem
                .evaluate(&solution)
                .unwrap()
                .unwrap()
                .to_bits(),
            float_value.to_bits()
        );
    }
}

#[test]
fn sparse_qubo_keeps_unused_variables_and_last_assignment() {
    let problem = QUBO::new(
        vec![0i64; 10_000],
        vec![((2, 7), i64::MAX), ((7, 2), 5), ((9, 9), 3), ((9, 9), 0)],
    )
    .unwrap();
    assert_eq!(problem.num_vars(), 10_000);
    assert_eq!(problem.matrix().nnz(), 1);
    assert_eq!(problem.get(9, 9), Some(0));
    assert_eq!(problem.get(2, 7), Some(5));
    let json = serde_json::to_string(&problem).unwrap();
    assert!(json.len() < 100_000);
    let restored: QUBO<i64> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.matrix(), problem.matrix());
    let mut solution = vec![false; 10_000];
    solution[2] = true;
    solution[7] = true;
    assert_eq!(restored.evaluate(&solution).unwrap(), Min(Some(5)));
}

#[test]
fn sparse_qubo_validates_shape_values_and_serialized_structure() {
    assert!(QUBO::from_sparse(CsMat::<i64>::zero((2, 3))).is_err());
    let invalid = CsMat::new((1, 1), vec![0, 1], vec![0], vec![f64::INFINITY]);
    assert!(QUBO::from_sparse(invalid).is_err());
    let column_matrix = CsMat::new_csc((2, 2), vec![0, 1, 2], vec![0, 0], vec![2i64, 3]);
    let problem = QUBO::from_sparse(column_matrix).unwrap();
    assert!(problem.matrix().is_csr());
    assert_eq!(problem.evaluate(&vec![true, true]).unwrap(), Min(Some(5)));
    assert_eq!(
        serde_json::to_string(&problem).unwrap(),
        r#"{"num_vars":2,"entries":[[0,0,2],[0,1,3]]}"#
    );
}

fn assignments(n: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << n).map(move |mask| (0..n).map(|i| mask & (1 << i) != 0).collect())
}

fn load_error<W>(json: &str) -> String
where
    W: WeightElement + serde::de::DeserializeOwned,
{
    serde_json::from_str::<QUBO<W>>(json)
        .err()
        .expect("QUBO JSON should be rejected")
        .to_string()
}

#[test]
fn qubo_json_writes_row_major_entries() {
    // The lower-triangle 99 is stored by from_matrix and therefore persisted, though never evaluated.
    let problem = QUBO::from_matrix(vec![vec![3, -5, 0], vec![99, 0, 7], vec![0, 0, -6]]).unwrap();
    assert_eq!(
        serde_json::to_string(&problem).unwrap(),
        r#"{"num_vars":3,"entries":[[0,0,3],[0,1,-5],[1,0,99],[1,2,7],[2,2,-6]]}"#
    );
    let floating = QUBO::new(vec![0.5, 0.0, -2.0], vec![((0, 2), 1e16)]).unwrap();
    assert_eq!(
        serde_json::to_string(&floating).unwrap(),
        r#"{"num_vars":3,"entries":[[0,0,0.5],[0,2,1e+16],[2,2,-2.0]]}"#
    );
    assert_eq!(
        serde_json::to_string(&QUBO::<i64>::from_matrix(vec![]).unwrap()).unwrap(),
        r#"{"num_vars":0,"entries":[]}"#
    );
}

#[test]
fn qubo_json_round_trip_preserves_storage_and_every_evaluation() {
    let integer = QUBO::from_matrix(vec![vec![3, -5, 0], vec![99, 0, 7], vec![0, 0, -6]]).unwrap();
    let restored: QUBO<i64> =
        serde_json::from_str(&serde_json::to_string(&integer).unwrap()).unwrap();
    assert_eq!(restored.matrix(), integer.matrix());
    for solution in assignments(3) {
        assert_eq!(
            restored.evaluate(&solution).unwrap(),
            integer.evaluate(&solution).unwrap()
        );
    }

    let floating = QUBO::from_matrix(vec![
        vec![1e16, 1.0, -1e16, 0.0],
        vec![99.0, 0.5, 0.0, -0.25],
        vec![0.0, 0.0, -2.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ])
    .unwrap();
    let restored: QUBO<f64> =
        serde_json::from_str(&serde_json::to_string(&floating).unwrap()).unwrap();
    assert_eq!(restored.matrix(), floating.matrix());
    for solution in assignments(4) {
        assert_eq!(
            restored.evaluate(&solution).unwrap().unwrap().to_bits(),
            floating.evaluate(&solution).unwrap().unwrap().to_bits()
        );
    }
}

#[test]
fn qubo_json_entries_behave_like_from_sparse() {
    // Any entry order is accepted; explicit zeros and lower-triangle entries stay stored.
    let restored: QUBO<i64> = serde_json::from_str(
        r#"{"num_vars":3,"entries":[[2,2,-6],[1,0,99],[0,1,-5],[1,1,0],[0,0,3]]}"#,
    )
    .unwrap();
    let expected = QUBO::from_sparse(CsMat::new(
        (3, 3),
        vec![0, 2, 4, 5],
        vec![0, 1, 0, 1, 2],
        vec![3i64, -5, 99, 0, -6],
    ))
    .unwrap();
    assert_eq!(restored.matrix(), expected.matrix());
    assert_eq!(
        serde_json::to_string(&restored).unwrap(),
        r#"{"num_vars":3,"entries":[[0,0,3],[0,1,-5],[1,0,99],[1,1,0],[2,2,-6]]}"#
    );
    assert_eq!(
        restored.evaluate(&vec![true, true, false]).unwrap(),
        Min(Some(-2))
    );
}

#[test]
fn qubo_json_legacy_dense_matrix_loads_like_from_matrix() {
    let legacy = include_str!("../../../../tests/data/qubo_legacy_dense.json");
    let restored: QUBO<i64> = serde_json::from_str(legacy).unwrap();
    let expected = QUBO::from_matrix(vec![vec![3, -5, 0], vec![99, 0, 7], vec![0, 0, -6]]).unwrap();
    assert_eq!(restored.matrix(), expected.matrix());
    for solution in assignments(3) {
        assert_eq!(
            restored.evaluate(&solution).unwrap(),
            expected.evaluate(&solution).unwrap()
        );
    }
    let floating: QUBO<f64> =
        serde_json::from_str(r#"{"num_vars":3,"matrix":[[0.5,1,0],[0,0,0],[0,0,-2]]}"#).unwrap();
    assert_eq!(
        serde_json::to_string(&floating).unwrap(),
        r#"{"num_vars":3,"entries":[[0,0,0.5],[0,1,1.0],[2,2,-2.0]]}"#
    );
}

#[test]
fn qubo_json_rejects_malformed_shapes() {
    const SHAPE: &str = "problem construction failed: QUBO JSON must contain exactly one of \
                         `entries` (sparse) or `matrix` (legacy dense)";
    assert_eq!(load_error::<i64>(r#"{"num_vars":3}"#), SHAPE);
    assert_eq!(
        load_error::<i64>(r#"{"num_vars":1,"entries":[],"matrix":[[0]]}"#),
        SHAPE
    );
    assert_eq!(
        load_error::<i64>(r#"{"num_vars":3,"entries":[[0,3,1]]}"#),
        "problem construction failed: QUBO entry index (0, 3) is outside 0..3"
    );
    assert_eq!(
        load_error::<i64>(r#"{"num_vars":3,"entries":[[3,0,1]]}"#),
        "problem construction failed: QUBO entry index (3, 0) is outside 0..3"
    );
    assert_eq!(
        load_error::<i64>(r#"{"num_vars":3,"entries":[[1,2,4],[0,0,1],[1,2,5]]}"#),
        "problem construction failed: QUBO entry (1, 2) is listed more than once"
    );
    assert_eq!(
        load_error::<i64>(r#"{"num_vars":3,"matrix":[[1,2,3],[0,4],[0,0,5]]}"#),
        "problem construction failed: QUBO matrix row 1 has length 2, expected 3"
    );
    assert_eq!(
        load_error::<i64>(r#"{"num_vars":3,"matrix":[[1,2],[0,4]]}"#),
        "problem construction failed: QUBO num_vars is 3, but the dense matrix has 2 rows"
    );
    assert!(load_error::<i64>(r#"{"entries":[]}"#).starts_with("missing field `num_vars`"));
    // The unreleased sprs CSR layout is not a supported shape.
    assert!(load_error::<i64>(
        r#"{"matrix":{"storage":"CSR","nrows":1,"ncols":1,"indptr":[0,1],"indices":[0],"data":[1]}}"#
    )
    .starts_with("invalid type: map, expected a sequence"));
    assert!(load_error::<i64>(r#"{"num_vars":1,"entries":[[0,0,0.5]]}"#)
        .starts_with("invalid type: floating point `0.5`, expected i64"));
}

#[test]
fn qubo_json_rejects_non_finite_coefficients() {
    // JSON cannot spell a non-finite number, so the rejection is checked on the parsed mirror.
    let sparse = QuboData {
        num_vars: 3,
        entries: Some(vec![(0, 0, 1.0), (1, 2, f64::INFINITY)]),
        matrix: None,
    };
    assert_eq!(
        QUBO::try_from(sparse).unwrap_err().to_string(),
        "non-finite floating-point construction value: QUBO coefficient must be finite at (1, 2)"
    );
    let dense = QuboData {
        num_vars: 3,
        entries: None,
        matrix: Some(vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, f64::NAN],
        ]),
    };
    assert_eq!(
        QUBO::try_from(dense).unwrap_err().to_string(),
        "non-finite floating-point construction value: QUBO coefficient must be finite at (2, 2)"
    );
    assert!(
        load_error::<f64>(r#"{"num_vars":1,"entries":[[0,0,1e999]]}"#)
            .starts_with("number out of range")
    );
}

#[test]
fn qubo_json_rejects_unallocatable_num_vars() {
    assert_eq!(
        QUBO::<i64>::from_entries(usize::MAX, vec![])
            .unwrap_err()
            .to_string(),
        "integer overflow during construction: counting QUBO rows"
    );
    assert!(QUBO::<i64>::from_entries(usize::MAX - 1, vec![])
        .unwrap_err()
        .to_string()
        .starts_with("problem construction failed: allocating QUBO rows:"));
}
