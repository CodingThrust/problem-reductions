use super::*;
use crate::registry::load_dyn;
use crate::solvers::{BruteForce, SolveOutcome, SolverExecution, SolverRequest};
use crate::traits::Problem;

#[test]
fn test_subset_dp_shortest_common_superstring_matches_brute_force() {
    let candidates = [vec![], vec![0], vec![1], vec![0, 0], vec![0, 1], vec![1, 0]];
    for first in &candidates {
        for second in &candidates {
            for third in &candidates {
                let problem = ShortestCommonSuperstring::new(
                    2,
                    vec![first.clone(), second.clone(), third.clone()],
                );
                let expected = BruteForce::new().solve(&problem).unwrap().unwrap();
                let actual = solve(&problem).unwrap();
                assert_eq!(
                    problem.evaluate(&actual).unwrap(),
                    problem.evaluate(&expected).unwrap()
                );
            }
        }
    }
}

#[test]
fn test_subset_dp_shortest_common_superstring_handles_containment_and_scale() {
    let problem = ShortestCommonSuperstring::new(
        4,
        vec![
            vec![0, 1, 2, 3],
            vec![1, 2],
            vec![2, 3, 0],
            vec![3, 0, 1],
            vec![0, 1],
        ],
    );
    let loaded = load_dyn(
        ShortestCommonSuperstring::NAME,
        &Default::default(),
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();
    let result = crate::solvers::solve(&loaded, SolverRequest::Default).unwrap();
    assert!(matches!(
        result.solver,
        SolverExecution::Customized {
            implementation: "subset-dp"
        }
    ));
    let SolveOutcome::Optimal {
        solution,
        evaluation,
    } = result.outcome
    else {
        panic!("the instance has a solution");
    };
    assert_eq!(evaluation, "Min(6)");
    let solution = serde_json::from_value(solution).unwrap();
    assert_eq!(problem.evaluate(&solution).unwrap().0, Some(6));
}

#[test]
fn subset_dp_reports_mask_and_table_size_overflow() {
    for count in [usize::BITS as usize, usize::BITS as usize - 1] {
        let problem =
            ShortestCommonSuperstring::new(count, (0..count).map(|symbol| vec![symbol]).collect());
        let loaded = load_dyn(
            ShortestCommonSuperstring::NAME,
            &Default::default(),
            serde_json::to_value(&problem).unwrap(),
        )
        .unwrap();
        assert!(matches!(
            crate::solvers::solve(&loaded, SolverRequest::Default),
            Err(SolveError::IntegerOverflow(_))
        ));
    }
}
