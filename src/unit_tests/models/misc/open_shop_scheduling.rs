use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn encode_permutation(permutation: &[usize]) -> Vec<usize> {
    let mut available: Vec<usize> = (0..permutation.len()).collect();
    let mut digits = Vec::with_capacity(permutation.len());
    for &job in permutation {
        let index = available
            .iter()
            .position(|&candidate| candidate == job)
            .expect("permutation must contain each job exactly once");
        digits.push(index);
        available.remove(index);
    }
    digits
}

fn encode_machine_orders(machine_orders: &[Vec<usize>]) -> Vec<usize> {
    machine_orders
        .iter()
        .flat_map(|order| encode_permutation(order))
        .collect()
}

#[test]
fn test_open_shop_scheduling_creation() {
    let problem = OpenShopScheduling::new(
        3,
        vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
    );

    assert_eq!(problem.num_jobs(), 4);
    assert_eq!(problem.num_machines(), 3);
    assert_eq!(
        problem.processing_times(),
        &[vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]]
    );
    assert_eq!(problem.dims(), vec![4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1]);
    assert_eq!(<OpenShopScheduling as Problem>::NAME, "OpenShopScheduling");
    assert_eq!(<OpenShopScheduling as Problem>::variant(), vec![]);
}

#[test]
fn test_open_shop_scheduling_evaluate_verified_example() {
    let problem = OpenShopScheduling::new(
        3,
        vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
    );
    let config = encode_machine_orders(&[vec![0, 1, 2, 3], vec![1, 0, 3, 2], vec![2, 3, 0, 1]]);

    assert_eq!(problem.evaluate(&config), Min(Some(8)));
}

#[test]
fn test_open_shop_scheduling_invalid_config() {
    let problem = OpenShopScheduling::new(2, vec![vec![1, 2], vec![3, 4]]);

    assert_eq!(problem.evaluate(&[0, 2, 0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(None));
}

#[test]
fn test_open_shop_scheduling_brute_force_solver() {
    let problem = OpenShopScheduling::new(
        3,
        vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
    );
    let known_optimum =
        encode_machine_orders(&[vec![0, 1, 2, 3], vec![1, 0, 3, 2], vec![2, 3, 0, 1]]);
    let solver = BruteForce::new();

    let witness = solver
        .find_witness(&problem)
        .expect("open shop example should have an optimal schedule");

    assert_eq!(problem.evaluate(&witness), Min(Some(8)));
    assert!(solver.find_all_witnesses(&problem).contains(&known_optimum));
}

#[test]
fn test_open_shop_scheduling_serialization() {
    let problem = OpenShopScheduling::new(2, vec![vec![1, 2], vec![3, 4], vec![2, 1]]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: OpenShopScheduling = serde_json::from_value(json).unwrap();

    assert_eq!(restored.num_machines(), problem.num_machines());
    assert_eq!(restored.processing_times(), problem.processing_times());
}

#[test]
fn test_open_shop_scheduling_empty() {
    let problem = OpenShopScheduling::new(3, vec![]);

    assert_eq!(problem.num_jobs(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}
