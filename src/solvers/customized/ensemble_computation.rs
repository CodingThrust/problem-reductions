//! Direct union chains and bounded subset enumeration with a compact ILP fallback.

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
    solve_with_union_limit(problem, 65_536)
}

fn pad_program(
    problem: &EnsembleComputation,
    mut program: Vec<usize>,
) -> Result<Vec<usize>, SolveError> {
    let length = problem
        .budget()
        .checked_mul(2)
        .ok_or_else(|| SolveError::IntegerOverflow("sizing the ensemble program".into()))?;
    let padding = length.checked_sub(program.len()).ok_or_else(|| {
        SolveError::IntegerOverflow("ensemble program exceeds its operation budget".into())
    })?;
    program.try_reserve_exact(padding)?;
    program.resize(length, 0);
    Ok(program)
}

fn solve_with_union_limit(
    problem: &EnsembleComputation,
    union_limit: usize,
) -> Result<Option<Vec<usize>>, SolveError> {
    // A k-element set needs k-1 disjoint unions of singleton leaves, even
    // when intermediate results are shared with other required sets.
    if problem
        .subsets()
        .iter()
        .any(|set| set.len() < 2 || set.len() - 1 > problem.budget())
    {
        return Ok(None);
    }
    let required: BTreeSet<_> = problem.subsets().iter().collect();
    if required.is_empty() {
        return pad_program(problem, Vec::new()).map(Some);
    }
    if required.len() == 1 {
        // A chain attains the k-1 lower bound for one distinct required set.
        let set = required.first().expect("one required set");
        let mut program = vec![set[0], set[1]];
        for &element in &set[2..] {
            let previous = problem
                .universe_size()
                .checked_add(program.len() / 2 - 1)
                .ok_or_else(|| SolveError::IntegerOverflow("indexing an ensemble union".into()))?;
            program.extend([previous, element]);
        }
        return pad_program(problem, program).map(Some);
    }
    let failure = |source| SolveError::IlpSolve {
        problem: EnsembleComputation::NAME.into(),
        source,
    };
    // All subsets and unordered disjoint partitions of one k-element set
    // require (3^k-1)/2-k variables. Sum this upper bound before allocating;
    // repeated intermediate sets only decrease the actual construction size.
    let fits = required
        .iter()
        .try_fold(union_limit, |remaining, set| {
            let power = 3usize.checked_pow(u32::try_from(set.len()).ok()?)?;
            remaining.checked_sub((power - 1) / 2 - set.len())
        })
        .is_some();
    if !fits {
        // Explicit ILP dispatch uses the compact slot encoding, not this
        // customized solver, so this fallback cannot recurse.
        return match ILPSolver::new().solve(problem) {
            Ok(solution) => Ok(Some(solution)),
            Err(ILPSolveError::Infeasible) => Ok(None),
            Err(error) => Err(failure(error)),
        };
    }
    let mut useful = BTreeSet::new();
    for set in &required {
        useful.extend(subsets(set)?.into_iter().filter(|set| set.len() >= 2));
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
            let choice = useful.len().checked_add(partitions.len()).ok_or_else(|| {
                SolveError::IntegerOverflow("indexing ensemble union choices".into())
            })?;
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
    for set in &required {
        constraints.push(LinearConstraint::eq(vec![(indices[*set], 1)], 1));
    }
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
            program.push(*operands.get(child).ok_or_else(|| {
                failure(ILPSolveError::InvalidSolution(
                    "union operand was not computed".into(),
                ))
            })?);
        }
        operands.insert(
            useful[*index].clone(),
            problem.universe_size() + program.len() / 2 - 1,
        );
    }
    pad_program(problem, program).map(Some)
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/ensemble_computation.rs"]
mod tests;
