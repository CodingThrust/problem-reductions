use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_closed_loop() {
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![3, 2, 1], vec![4, 2, 3], vec![4, 3, 6]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_bf_vs_ilp() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(
        vec![3, 2, 4, 1, 2],
        vec![5, 3, 7, 2, 4],
        vec![6, 4, 10, 2, 8],
    );

    let bf = BruteForce::new();
    let bf_witness = bf.solve(&problem).unwrap().expect("should find a solution");
    let bf_value = problem.evaluate(&bf_witness).unwrap();

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = problem.evaluate(&extracted).unwrap();

    assert_eq!(bf_value, ilp_value);
    assert_eq!(ilp_value.0, Some(3));
}

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_all_on_time() {
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![1, 1, 1], vec![2, 3, 4], vec![10, 10, 10]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = problem.evaluate(&extracted).unwrap();
    assert!(value.is_valid());
    assert_eq!(value.0, Some(0));
}

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_optimal_ordering() {
    // 3 tasks where order matters:
    // t0: length=4, weight=5, deadline=4
    // t1: length=1, weight=1, deadline=5
    // t2: length=2, weight=3, deadline=3
    // Best schedule: [2,0,1] -> t2 completes 2 (ok), t0 completes 6 (tardy wt=5), t1 completes 7 (tardy wt=1)
    // or: [2,1,0] -> t2 completes 2 (ok), t1 completes 3 (ok), t0 completes 7 (tardy wt=5)
    // or: [1,2,0] -> t1 completes 1 (ok), t2 completes 3 (ok), t0 completes 7 (tardy wt=5)
    // or: [0,1,2] -> t0 completes 4 (ok), t1 completes 5 (ok), t2 completes 7 (tardy wt=3) = 3
    // Minimum is 3 (schedule [0,1,2])
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![4, 1, 2], vec![5, 1, 3], vec![4, 5, 3]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = problem.evaluate(&extracted).unwrap();

    let bf = BruteForce::new();
    let bf_witness = bf.solve(&problem).unwrap().expect("should have solution");
    let bf_value = problem.evaluate(&bf_witness).unwrap();

    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_tardy_ilp_signed_permutations_and_all_indicators() {
    for n in 0u32..=3 {
        for mut code in 0..4usize.pow(n) {
            let lengths: Vec<i64> = (0..n)
                .map(|_| {
                    let length = [-2, -1, 1, 2][code % 4];
                    code /= 4;
                    length
                })
                .collect();
            for pattern in 0..5 {
                let weights = (0..n)
                    .map(|j| if (j + pattern) % 2 == 0 { 3 } else { -2 })
                    .collect();
                let deadlines = (0..n)
                    .map(|j| [-4, -1, 0, 2, 5][(j + pattern) as usize % 5])
                    .collect();
                let source =
                    SequencingToMinimizeTardyTaskWeight::new(lengths.clone(), weights, deadlines);
                let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
                let target = reduction.target_problem();
                let count = n as usize;
                assert_eq!(target.num_vars(), count * count + count);
                assert_eq!(target.num_constraints(), 2 * count * count + 2 * count);
                for mut encoded in 0..count.pow(n) {
                    let schedule: Vec<usize> = (0..n)
                        .map(|_| {
                            let job = encoded % count;
                            encoded /= count;
                            job
                        })
                        .collect();
                    let source_value = source.evaluate(&schedule).unwrap();
                    if !source_value.is_valid() {
                        continue;
                    }
                    let mut expected = vec![false; count];
                    let mut elapsed = 0;
                    for &job in &schedule {
                        elapsed += source.lengths()[job];
                        expected[job] = elapsed > source.deadlines()[job];
                    }
                    for mask in 0..1usize << n {
                        let mut bits = vec![0; count * count + count];
                        for (position, &job) in schedule.iter().enumerate() {
                            bits[job * count + position] = 1;
                        }
                        for job in 0..count {
                            bits[count * count + job] = i64::from(mask & (1 << job) != 0);
                        }
                        let exact =
                            (0..count).all(|job| (bits[count * count + job] == 1) == expected[job]);
                        let value = target.evaluate(&bits).unwrap();
                        assert_eq!(value.is_valid(), exact);
                        let extracted = reduction.extract_solution(&bits);
                        assert_eq!(extracted.is_ok(), exact);
                        if exact {
                            assert_eq!(value.value, source_value.0);
                            assert_eq!(extracted.unwrap(), schedule);
                        }
                    }
                }
                assert!(reduction
                    .extract_solution(&vec![0; target.num_vars() + 1])
                    .is_err());
                if count > 0 {
                    assert!(reduction
                        .extract_solution(&vec![0; target.num_vars()])
                        .is_err());
                    assert!(reduction
                        .extract_solution(&vec![2; target.num_vars()])
                        .is_err());
                }
            }
        }
    }
}

#[test]
fn test_tardy_ilp_complete_small_binary_target_space() {
    let source =
        SequencingToMinimizeTardyTaskWeight::new(vec![-2, 1, 3], vec![-3, 2, -1], vec![-1, 0, 2]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let mut feasible = 0;
    for mask in 0..1usize << target.num_vars() {
        let bits = (0..target.num_vars())
            .map(|i| i64::from(mask & (1 << i) != 0))
            .collect();
        let value = target.evaluate(&bits).unwrap();
        let extracted = reduction.extract_solution(&bits);
        assert_eq!(extracted.is_ok(), value.is_valid());
        if let Ok(schedule) = extracted {
            assert_eq!(source.evaluate(&schedule).unwrap().0, value.value);
            feasible += 1;
        }
    }
    assert_eq!(feasible, 6);
}

#[test]
fn test_tardy_ilp_numeric_boundaries_and_representability_errors() {
    for (length, weight, deadline) in [
        (i64::MAX, i64::MAX, i64::MAX),
        (i64::MIN, i64::MIN, i64::MIN),
        (i64::MAX, i64::MIN, i64::MIN),
        (i64::MIN, i64::MAX, i64::MAX),
        (1, -1, 1),
        (1, 1, -1),
        (-1, 1, -2),
    ] {
        let source =
            SequencingToMinimizeTardyTaskWeight::new(vec![length], vec![weight], vec![deadline]);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let bits = vec![1, i64::from(length > deadline)];
        assert_eq!(
            reduction.target_problem().evaluate(&bits).unwrap().value,
            source.evaluate(&vec![0]).unwrap().0
        );
        assert_eq!(reduction.extract_solution(&bits).unwrap(), vec![0]);
    }
    for (lengths, weights, deadlines) in [
        (vec![i64::MAX, i64::MAX], vec![1, 1], vec![0, 0]),
        (vec![i64::MIN, i64::MIN], vec![1, 1], vec![0, 0]),
        (vec![1, 1], vec![i64::MAX, i64::MAX], vec![0, 0]),
        (vec![1, 1], vec![i64::MIN, i64::MIN], vec![0, 0]),
        (vec![1, i64::MAX - 1], vec![1, 1], vec![i64::MAX / 2; 2]),
    ] {
        let source = SequencingToMinimizeTardyTaskWeight::new(lengths, weights, deadlines);
        assert!(matches!(
            ReduceTo::<ILP<bool>>::reduce_to(&source),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
