//! Exact verification solver: choose useful unions, rather than ordering circuit slots.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::EnsembleComputation;
use crate::solvers::{ILPSolveError, ILPSolver, SolveError};
use crate::traits::Problem;
use std::collections::{BTreeMap, BTreeSet};

fn subsets(set: &[usize]) -> Result<Vec<Vec<usize>>, SolveError> {
    let mut result = vec![vec![]];
    for &element in set {
        let previous = result.len();
        result.try_reserve_exact(previous)?;
        for index in 0..previous {
            let mut next = result[index].clone();
            next.push(element);
            result.push(next);
        }
    }
    Ok(result)
}

pub(crate) fn solve(problem: &EnsembleComputation) -> Result<Option<Vec<usize>>, SolveError> {
    // Computed sets are nonempty disjoint unions of two nonempty operands.
    if problem.subsets().iter().any(|set| set.len() < 2) {
        return Ok(None);
    }
    // ponytail: enumerate subsets of required sets; use implicit subset search
    // if correctness checks must handle large individual required sets.
    let mut useful = BTreeSet::new();
    for required in problem.subsets() {
        useful.extend(subsets(required)?.into_iter().filter(|set| set.len() >= 2));
    }
    let mut useful: Vec<_> = useful.into_iter().collect();
    useful.sort_by_key(Vec::len);
    let indices: BTreeMap<_, _> = useful
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, set)| (set, index))
        .collect();
    let mut partitions = Vec::new();
    let mut constraints = Vec::new();
    for (index, set) in useful.iter().enumerate() {
        let mut choices = vec![(index, -1)];
        for left in subsets(set)?
            .into_iter()
            .filter(|left| !left.is_empty() && left.len() < set.len() && left[0] == set[0])
        {
            let right: Vec<_> = set
                .iter()
                .copied()
                .filter(|element| left.binary_search(element).is_err())
                .collect();
            let choice =
                useful
                    .len()
                    .checked_add(partitions.len())
                    .ok_or(SolveError::IntegerOverflow(
                        "indexing ensemble union choices".into(),
                    ))?;
            choices.push((choice, 1));
            for child in [&left, &right] {
                if child.len() > 1 {
                    constraints.push(LinearConstraint::le(
                        vec![(choice, 1), (indices[child], -1)],
                        0,
                    ));
                }
            }
            partitions.push((index, left, right));
        }
        // A computed set has exactly one disjoint-union definition.
        constraints.push(LinearConstraint::eq(choices, 0));
    }
    for required in problem.subsets() {
        constraints.push(LinearConstraint::eq(vec![(indices[required], 1)], 1));
    }
    let failure = |source| SolveError::IlpSolve {
        problem: EnsembleComputation::NAME.into(),
        source,
    };
    let budget = <EnsembleComputation as crate::rules::ReduceTo<ILP<bool>>>::exact_i64(
        problem.budget(),
        "representing the ensemble operation budget",
    )
    .map_err(ILPSolveError::from)
    .map_err(failure)?;
    let objective: Vec<_> = (0..useful.len()).map(|index| (index, 1)).collect();
    constraints.push(LinearConstraint::le(objective.clone(), budget));
    let target = ILP::<bool>::new(
        useful.len() + partitions.len(),
        constraints,
        objective,
        ObjectiveSense::Minimize,
    )
    .map_err(<EnsembleComputation as crate::rules::ReduceTo<ILP<bool>>>::target_construction)
    .map_err(ILPSolveError::from)
    .map_err(failure)?;
    let solution = match ILPSolver::new().solve(&target) {
        Ok(solution) => solution,
        Err(ILPSolveError::Infeasible) => return Ok(None),
        Err(error) => return Err(failure(error)),
    };
    let mut operands: BTreeMap<Vec<usize>, usize> = problem
        .subsets()
        .iter()
        .flatten()
        .map(|&element| (vec![element], element))
        .collect();
    let mut program = Vec::new();
    // Partitions were generated in increasing parent cardinality. Every
    // selected non-singleton operand therefore already has a program index.
    for (choice, (index, left, right)) in partitions.iter().enumerate() {
        if solution[useful.len() + choice] == 0 {
            continue;
        }
        for child in [left, right] {
            program.push(
                *operands
                    .get(child)
                    .ok_or(failure(ILPSolveError::InvalidSolution(
                        "union operand was not computed".into(),
                    )))?,
            );
        }
        operands.insert(
            useful[*index].clone(),
            problem.universe_size() + program.len() / 2 - 1,
        );
    }
    program.resize(2 * problem.budget(), 0);
    Ok(Some(program))
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/ensemble_computation.rs"]
mod tests;
