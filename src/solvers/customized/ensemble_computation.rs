//! Exact ensemble computation solver using breadth-first search over computed sets.

use crate::models::misc::EnsembleComputation;
use std::collections::{HashSet, VecDeque};

pub(crate) fn solve(problem: &EnsembleComputation) -> Option<Vec<usize>> {
    if problem.subsets().is_empty() {
        return Some(vec![0; 2 * problem.budget()]);
    }

    let singletons = (0..problem.universe_size())
        .map(|element| vec![element])
        .collect::<Vec<_>>();
    let mut queue = VecDeque::from([(Vec::<Vec<usize>>::new(), Vec::<usize>::new())]);
    let mut seen = HashSet::from([Vec::<Vec<usize>>::new()]);

    while let Some((computed, program)) = queue.pop_front() {
        if computed.len() == problem.budget() {
            continue;
        }
        let mut available = singletons.iter().collect::<Vec<_>>();
        available.extend(computed.iter());

        for left in 0..available.len() {
            for right in (left + 1)..available.len() {
                if !disjoint(available[left], available[right]) {
                    continue;
                }
                let result = union(available[left], available[right]);
                if computed.contains(&result)
                    || !problem.subsets().iter().any(|required| {
                        result
                            .iter()
                            .all(|element| required.binary_search(element).is_ok())
                    })
                {
                    continue;
                }

                let mut next_computed = computed.clone();
                next_computed.push(result);
                let mut next_program = program.clone();
                next_program.extend([left, right]);
                if problem
                    .subsets()
                    .iter()
                    .all(|required| next_computed.contains(required))
                {
                    next_program.resize(2 * problem.budget(), 0);
                    return Some(next_program);
                }

                let mut key = next_computed.clone();
                key.sort();
                if seen.insert(key) {
                    queue.push_back((next_computed, next_program));
                }
            }
        }
    }
    None
}

fn disjoint(left: &[usize], right: &[usize]) -> bool {
    !left
        .iter()
        .any(|element| right.binary_search(element).is_ok())
}

fn union(left: &[usize], right: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    result.extend_from_slice(left);
    result.extend_from_slice(right);
    result.sort_unstable();
    result
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/ensemble_computation.rs"]
mod tests;
