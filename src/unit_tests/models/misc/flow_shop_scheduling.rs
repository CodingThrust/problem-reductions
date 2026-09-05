use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;

#[test]
fn test_flow_shop_scheduling_creation() {
    let problem = FlowShopScheduling::new(
        3,
        vec![
            vec![3, 4, 2],
            vec![2, 3, 5],
            vec![4, 1, 3],
            vec![1, 5, 4],
            vec![3, 2, 3],
        ],
        25,
    );
    assert_eq!(problem.num_jobs(), 5);
    assert_eq!(problem.num_processors(), 3);
    assert_eq!(problem.deadline(), 25);
    assert_eq!(problem.dimensions().len(), 5);
    // Lehmer code encoding: dims = [5, 4, 3, 2, 1]
    assert_eq!(problem.dimensions(), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_flow_shop_scheduling_evaluate_feasible() {
    // From issue example: 3 machines, 5 jobs
    // Job 0: [3, 4, 2], Job 1: [2, 3, 5], Job 2: [4, 1, 3], Job 3: [1, 5, 4], Job 4: [3, 2, 3]
    // Sequence j4, j1, j5, j3, j2 = jobs [3, 0, 4, 2, 1] (0-indexed)
    // This has makespan 23 <= 25
    let problem = FlowShopScheduling::new(
        3,
        vec![
            vec![3, 4, 2],
            vec![2, 3, 5],
            vec![4, 1, 3],
            vec![1, 5, 4],
            vec![3, 2, 3],
        ],
        25,
    );

    let config = vec![3, 0, 4, 2, 1];
    assert!(problem.evaluate(&config).unwrap());
}

#[test]
fn test_flow_shop_scheduling_evaluate_infeasible() {
    // Same instance, deadline of 15 (below the best makespan of 23)
    let problem = FlowShopScheduling::new(
        3,
        vec![
            vec![3, 4, 2],
            vec![2, 3, 5],
            vec![4, 1, 3],
            vec![1, 5, 4],
            vec![3, 2, 3],
        ],
        15, // Very tight deadline, likely infeasible
    );

    // The sequence j4,j1,j5,j3,j2 gives makespan 23 > 15
    let config = vec![3, 0, 4, 2, 1];
    assert!(!problem.evaluate(&config).unwrap());
}

#[test]
fn test_flow_shop_scheduling_invalid_config() {
    let problem = FlowShopScheduling::new(2, vec![vec![1, 2], vec![3, 4]], 10);

    assert!(matches!(
        problem.evaluate(&vec![2, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(!problem.evaluate(&vec![0, 0]).unwrap());
    // Wrong length
    assert!(matches!(
        problem.evaluate(&vec![0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_flow_shop_scheduling_problem_name() {
    assert_eq!(<FlowShopScheduling as Problem>::NAME, "FlowShopScheduling");
}

#[test]
fn test_flow_shop_scheduling_variant() {
    let v = <FlowShopScheduling as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_flow_shop_scheduling_serialization() {
    let problem = FlowShopScheduling::new(2, vec![vec![1, 2], vec![3, 4], vec![2, 1]], 10);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: FlowShopScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_processors(), problem.num_processors());
    assert_eq!(restored.task_lengths(), problem.task_lengths());
    assert_eq!(restored.deadline(), problem.deadline());
}

#[test]
fn test_flow_shop_scheduling_compute_makespan() {
    // 2 machines, 3 jobs
    // Job 0: [3, 2], Job 1: [2, 4], Job 2: [1, 3]
    let problem = FlowShopScheduling::new(2, vec![vec![3, 2], vec![2, 4], vec![1, 3]], 20);

    // Order: job 0, job 1, job 2
    // Machine 0: j0[0,3], j1[3,5], j2[5,6]
    // Machine 1: j0[3,5], j1[5,9], j2[9,12]
    // Makespan = 12
    assert_eq!(problem.compute_makespan(&[0, 1, 2]).unwrap(), 12);
}

#[test]
fn test_flow_shop_scheduling_brute_force_solver() {
    // Small instance: 2 machines, 3 jobs, generous deadline
    let problem = FlowShopScheduling::new(2, vec![vec![3, 2], vec![2, 4], vec![1, 3]], 20);
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap();
    assert!(solution.is_some());
    let config = solution.unwrap();
    assert!(problem.evaluate(&config).unwrap());
}

#[test]
fn test_flow_shop_scheduling_brute_force_unsatisfiable() {
    // 2 machines, 2 jobs with impossible deadline
    // Job 0: [5, 5], Job 1: [5, 5]
    // Best makespan: min of two orders:
    //   [0,1]: M0: 0-5, 5-10; M1: 5-10, 10-15 -> 15
    //   [1,0]: same by symmetry -> 15
    // Deadline 10 < 15 => unsatisfiable
    let problem = FlowShopScheduling::new(2, vec![vec![5, 5], vec![5, 5]], 10);
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap();
    assert!(solution.is_none());
}

#[test]
fn test_flow_shop_scheduling_empty() {
    let problem = FlowShopScheduling::new(3, vec![], 0);
    assert_eq!(problem.num_jobs(), 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    // Empty config should be satisfying (no jobs to schedule)
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_flow_shop_scheduling_find_all_witnesses() {
    // Issue #507 example: 3 machines, 5 jobs, D=25
    // Search space = 5! = 120 permutations
    let problem = FlowShopScheduling::new(
        3,
        vec![
            vec![3, 4, 2],
            vec![2, 3, 5],
            vec![4, 1, 3],
            vec![1, 5, 4],
            vec![3, 2, 3],
        ],
        25,
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
    // The issue witness sequence [3,0,4,2,1] gives makespan 23 ≤ 25.
    assert!(solutions.contains(&vec![3, 0, 4, 2, 1]));
    // 99 out of 120 permutations have makespan ≤ 25
    assert_eq!(solutions.len(), 99);
}

#[test]
fn test_flow_shop_scheduling_find_all_witnesses_empty() {
    // 2 machines, 2 symmetric jobs [5,5], deadline 10
    // Both orderings give makespan 15 > 10
    let problem = FlowShopScheduling::new(2, vec![vec![5, 5], vec![5, 5]], 10);
    let solver = BruteForce::new();
    assert!(solver.find_all_witnesses(&problem).unwrap().is_empty());
}

#[test]
fn test_flow_shop_scheduling_single_job() {
    // 3 machines, 1 job: [2, 3, 4]
    // Makespan = 2 + 3 + 4 = 9
    let problem = FlowShopScheduling::new(3, vec![vec![2, 3, 4]], 10);
    assert!(problem.evaluate(&vec![0]).unwrap()); // makespan 9 <= 10
    let tight = FlowShopScheduling::new(3, vec![vec![2, 3, 4]], 8);
    assert!(!tight.evaluate(&vec![0]).unwrap()); // makespan 9 > 8
}
