use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn canonical_cvp() -> ClosestVectorProblem<i64> {
    ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2]).unwrap()
}

fn canonical_bits() -> Vec<bool> {
    vec![
        false, false, false, true, true, false, false, true, false, false, true,
    ]
}

#[test]
fn test_closestvectorproblem_determinant_exact_elimination() {
    assert_eq!(determinant(&[]).unwrap(), 1);
    assert_eq!(determinant(&[vec![-7]]).unwrap(), -7);
    assert_eq!(determinant(&[vec![0, 1], vec![1, 0]]).unwrap(), -1);
    assert_eq!(
        determinant(&[vec![0, 0, 1], vec![1, 0, 0], vec![0, 1, 0]]).unwrap(),
        1
    );
    assert_eq!(
        determinant(&[vec![2, 3, 1], vec![4, 1, -3], vec![1, 2, 0]]).unwrap(),
        10
    );
    assert_eq!(determinant(&[vec![0, 1], vec![0, 2]]).unwrap(), 0);
    assert_eq!(determinant(&[vec![1, 2], vec![2, 4]]).unwrap(), 0);
    let large = i64::MAX;
    assert_eq!(
        determinant(&[vec![large, large - 1], vec![large - 1, large - 2]]).unwrap(),
        -1
    );
    assert!(matches!(
        determinant(&[vec![large, 0], vec![0, 2]]),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_closestvectorproblem_to_qubo_twelve_dimensional_identity() {
    let size = 12;
    let basis = (0..size)
        .map(|column| (0..size).map(|row| i64::from(row == column)).collect())
        .collect();
    let source = ClosestVectorProblem::new(basis, vec![1_i64; size]).unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    let mut bits = vec![false; reduction.target_problem().num_vars()];
    for encoding in &reduction.encodings {
        let mut offset = 1 - encoding.lower;
        for (index, &weight) in encoding.weights.iter().enumerate().rev() {
            if weight <= offset {
                bits[encoding.start + index] = true;
                offset -= weight;
            }
        }
        assert_eq!(offset, 0);
    }
    let solution = reduction.extract_solution(&bits).unwrap();
    assert_eq!(solution, vec![1; size]);
    assert_eq!(source.evaluate(&solution).unwrap().0, Some(0.0));
}

#[test]
fn test_closestvectorproblem_to_qubo_closed_loop() {
    let source = canonical_cvp();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    let target_solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert_eq!(source_solution, vec![1, 1]);
    assert_eq!(source.evaluate(&source_solution).unwrap().0, Some(0.0));
    assert_eq!(reduction.target_problem().num_vars(), 11);
}

#[test]
fn test_closestvectorproblem_to_qubo_coefficients() {
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&canonical_cvp()).unwrap();
    let qubo = reduction.target_problem();

    assert_eq!(qubo.get(0, 0), Some(&-248));
    assert_eq!(qubo.get(0, 1), Some(&16));
    assert_eq!(qubo.get(0, 6), Some(&4));
    assert_eq!(qubo.get(6, 6), Some(&-241));
}

#[test]
fn test_closestvectorproblem_to_qubo_exact_range_decoding() {
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&canonical_cvp()).unwrap();
    assert_eq!(
        reduction.extract_solution(&canonical_bits()).unwrap(),
        vec![1, 1]
    );

    let duplicate = vec![
        true, false, false, true, false, true, true, true, true, true, false,
    ];
    assert_eq!(reduction.extract_solution(&duplicate).unwrap(), vec![1, 1]);
    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&canonical_bits())
            .unwrap(),
        reduction.target_problem().evaluate(&duplicate).unwrap()
    );
}

#[test]
fn test_closestvectorproblem_to_qubo_preserves_optimum_outside_old_box() {
    let source = ClosestVectorProblem::new(vec![vec![1]], vec![20_i64]).unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    let target_solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        reduction.extract_solution(&target_solution).unwrap(),
        vec![20]
    );
}

#[test]
fn test_closestvectorproblem_to_qubo_reports_numeric_boundaries() {
    let absolute_value = ClosestVectorProblem::new(vec![vec![1]], vec![i64::MIN]).unwrap();
    assert!(matches!(
        ReduceTo::<QUBO<i64>>::reduce_to(&absolute_value),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));

    let large_exact = ClosestVectorProblem::new(vec![vec![100_000_000]], vec![1_i64]).unwrap();
    assert!(ReduceTo::<QUBO<i64>>::reduce_to(&large_exact).is_ok());
}

#[cfg(feature = "example-db")]
#[test]
fn test_closestvectorproblem_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "closestvectorproblem_to_qubo")
        .unwrap();
    let example = (spec.build)();

    assert_eq!(example.source.problem, "ClosestVectorProblem");
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.target.instance["num_vars"], 11);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([1, 1])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::to_value(canonical_bits()).unwrap()
    );
}
