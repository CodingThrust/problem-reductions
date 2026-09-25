//! Exact shortest common superstring solver using subset dynamic programming.

use crate::models::misc::ShortestCommonSuperstring;
use crate::solvers::SolveError;

pub(crate) fn solve(problem: &ShortestCommonSuperstring) -> Result<Vec<Option<usize>>, SolveError> {
    let mut strings = problem.strings().to_vec();
    strings.sort();
    strings.dedup();
    strings = strings
        .iter()
        .enumerate()
        .filter(|(i, string)| {
            !strings
                .iter()
                .enumerate()
                .any(|(j, other)| i != &j && contains(other, string))
        })
        .map(|(_, string)| string.clone())
        .collect();

    let mut solution = Vec::new();
    solution.try_reserve_exact(problem.max_length())?;
    if strings.is_empty() {
        solution.resize(problem.max_length(), None);
        return Ok(solution);
    }

    let n = strings.len();
    if n >= usize::BITS as usize {
        return Err(SolveError::IntegerOverflow(
            "indexing string subsets with a usize mask".into(),
        ));
    }
    let states = 1usize << n;
    let cells = states.checked_mul(n).ok_or_else(|| {
        SolveError::IntegerOverflow("sizing the superstring dynamic-programming table".into())
    })?;
    let mut dp = Vec::<Option<Vec<usize>>>::new();
    dp.try_reserve_exact(cells)?;
    dp.resize(cells, None);
    for (i, string) in strings.iter().enumerate() {
        dp[(1 << i) * n + i] = Some(string.clone());
    }

    for mask in 1usize..states {
        for last in 0..n {
            let Some(prefix) = dp[mask * n + last].clone() else {
                continue;
            };
            for next in 0..n {
                if mask & (1 << next) != 0 {
                    continue;
                }
                let overlap = overlap(&prefix, &strings[next]);
                let mut candidate = prefix.clone();
                candidate.extend_from_slice(&strings[next][overlap..]);
                let slot = &mut dp[((mask | (1 << next)) * n) + next];
                if slot
                    .as_ref()
                    .is_none_or(|current| candidate.len() < current.len())
                {
                    *slot = Some(candidate);
                }
            }
        }
    }

    let full = states - 1;
    let shortest = (0..n)
        .filter_map(|last| dp[full * n + last].take())
        .min_by_key(Vec::len)
        .unwrap();
    solution.extend(shortest.into_iter().map(Some));
    solution.resize(problem.max_length(), None);
    Ok(solution)
}

fn contains(haystack: &[usize], needle: &[usize]) -> bool {
    needle.is_empty()
        || (needle.len() <= haystack.len()
            && haystack
                .windows(needle.len())
                .any(|window| window == needle))
}

fn overlap(left: &[usize], right: &[usize]) -> usize {
    (0..=left.len().min(right.len()))
        .rev()
        .find(|&length| left[left.len() - length..] == right[..length])
        .unwrap()
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/shortest_common_superstring.rs"]
mod tests;
