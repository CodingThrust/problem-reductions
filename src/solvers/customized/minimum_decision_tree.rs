//! Exact minimum decision tree solver using dynamic programming over object subsets.

use crate::models::misc::MinimumDecisionTree;
use crate::solvers::SolveError;

pub(crate) fn solve(problem: &MinimumDecisionTree) -> Result<Vec<usize>, SolveError> {
    let n = problem.num_objects();
    if n >= usize::BITS as usize {
        return Err(SolveError::IntegerOverflow(
            "indexing object subsets with a usize mask".into(),
        ));
    }
    let states = 1usize << n;
    let full = states - 1;
    let mut costs = Vec::new();
    costs.try_reserve_exact(states)?;
    costs.resize(states, usize::MAX);
    let mut choices = Vec::new();
    choices.try_reserve_exact(states)?;
    choices.resize(states, problem.num_tests());
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
    let mut solution = Vec::new();
    solution.try_reserve_exact(slots)?;
    solution.resize(slots, problem.num_tests());
    write_tree(problem, full, 0, &choices, &mut solution);
    Ok(solution)
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
