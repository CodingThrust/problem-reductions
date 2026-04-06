//! Golden-section-style search for optimization via decision queries.

use crate::models::decision::{Decision, DecisionProblemMeta};
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::{Max, Min, OptimizationValue, Or};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;

/// Whether a decision problem has at least one satisfying configuration.
fn is_satisfiable<P>(problem: &P) -> bool
where
    P: Problem<Value = Or>,
{
    BruteForce::new().solve(problem).0
}

fn solve_via_decision_min<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Problem<Value = Min<i32>> + Clone,
{
    if lower > upper {
        return None;
    }

    if !is_satisfiable(&Decision::new(problem.clone(), upper)) {
        return None;
    }

    let mut lo = lower;
    let mut hi = upper;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if is_satisfiable(&Decision::new(problem.clone(), mid)) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    Some(lo)
}

fn solve_via_decision_max<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Problem<Value = Max<i32>> + Clone,
{
    if lower > upper {
        return None;
    }

    if !is_satisfiable(&Decision::new(problem.clone(), lower)) {
        return None;
    }

    let mut lo = lower;
    let mut hi = upper;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if is_satisfiable(&Decision::new(problem.clone(), mid)) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    Some(lo)
}

#[doc(hidden)]
pub trait DecisionSearchValue:
    OptimizationValue<Inner = i32> + Clone + fmt::Debug + Serialize + DeserializeOwned
{
    fn solve_problem<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
    where
        P: DecisionProblemMeta + Problem<Value = Self> + Clone;
}

impl DecisionSearchValue for Min<i32> {
    fn solve_problem<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
    where
        P: DecisionProblemMeta + Problem<Value = Self> + Clone,
    {
        solve_via_decision_min(problem, lower, upper)
    }
}

impl DecisionSearchValue for Max<i32> {
    fn solve_problem<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
    where
        P: DecisionProblemMeta + Problem<Value = Self> + Clone,
    {
        solve_via_decision_max(problem, lower, upper)
    }
}

/// Recover an optimization value by querying the problem's decision wrapper.
pub fn solve_via_decision<P>(problem: &P, lower: i32, upper: i32) -> Option<i32>
where
    P: DecisionProblemMeta + Clone,
    P::Value: DecisionSearchValue,
{
    <P::Value as DecisionSearchValue>::solve_problem(problem, lower, upper)
}

#[cfg(test)]
#[path = "../unit_tests/solvers/golden_section.rs"]
mod tests;
