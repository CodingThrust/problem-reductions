//! Exact minimum decision tree solver using dynamic programming over object subsets.

use crate::models::misc::MinimumDecisionTree;

pub(crate) fn solve(problem: &MinimumDecisionTree) -> Option<Vec<usize>> {
    let n = problem.num_objects();
    let full = (1usize << n) - 1;
    let mut costs = vec![usize::MAX; 1usize << n];
    let mut choices = vec![problem.num_tests(); 1usize << n];
    for object in 0..n {
        costs[1 << object] = 0;
    }

    for subset in 1usize..=full {
        let count = subset.count_ones() as usize;
        if count < 2 {
            continue;
        }
        for test in 0..problem.num_tests() {
            let passed = (0..n).fold(0usize, |mask, object| {
                if subset & (1 << object) != 0 && problem.test_matrix()[test][object] {
                    mask | (1 << object)
                } else {
                    mask
                }
            });
            let failed = subset ^ passed;
            if passed == 0 || failed == 0 {
                continue;
            }
            let cost = count + costs[passed] + costs[failed];
            if cost < costs[subset] {
                costs[subset] = cost;
                choices[subset] = test;
            }
        }
    }

    let slots = (1usize << (n - 1)) - 1;
    let mut solution = vec![problem.num_tests(); slots];
    write_tree(problem, full, 0, &choices, &mut solution);
    Some(solution)
}

fn write_tree(
    problem: &MinimumDecisionTree,
    subset: usize,
    node: usize,
    choices: &[usize],
    solution: &mut [usize],
) {
    if subset.count_ones() == 1 {
        return;
    }
    let test = choices[subset];
    solution[node] = test;
    let passed = (0..problem.num_objects()).fold(0usize, |mask, object| {
        if subset & (1 << object) != 0 && problem.test_matrix()[test][object] {
            mask | (1 << object)
        } else {
            mask
        }
    });
    write_tree(problem, subset ^ passed, 2 * node + 1, choices, solution);
    write_tree(problem, passed, 2 * node + 2, choices, solution);
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/minimum_decision_tree.rs"]
mod tests;
