//! Exact solver for GroupingBySwapping by enumerating symbol-block orders.

use crate::models::misc::GroupingBySwapping;

pub(crate) fn solve(problem: &GroupingBySwapping) -> Option<Vec<usize>> {
    if problem.string().is_empty() {
        return Some(Vec::new());
    }

    let mut symbols = problem.string().to_vec();
    symbols.sort_unstable();
    symbols.dedup();
    permute(problem, &mut symbols, 0)
}

fn permute(
    problem: &GroupingBySwapping,
    symbols: &mut [usize],
    start: usize,
) -> Option<Vec<usize>> {
    if start == symbols.len() {
        let mut target = problem.string().to_vec();
        let mut rank = vec![0; problem.alphabet_size()];
        for (position, &symbol) in symbols.iter().enumerate() {
            rank[symbol] = position;
        }
        target.sort_by_key(|symbol| rank[*symbol]);

        let mut current = problem.string().to_vec();
        let mut swaps = Vec::new();
        for (i, &wanted) in target.iter().enumerate() {
            let mut j = i;
            while current[j] != wanted {
                j += 1;
            }
            while j > i {
                current.swap(j - 1, j);
                swaps.push(j - 1);
                j -= 1;
            }
        }
        if swaps.len() <= problem.budget() {
            swaps.resize(problem.budget(), problem.string_len() - 1);
            return Some(swaps);
        }
        return None;
    }

    for i in start..symbols.len() {
        symbols.swap(start, i);
        if let Some(solution) = permute(problem, symbols, start + 1) {
            return Some(solution);
        }
        symbols.swap(start, i);
    }
    None
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/grouping_by_swapping.rs"]
mod tests;
