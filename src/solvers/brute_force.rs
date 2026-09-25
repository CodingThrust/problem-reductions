//! Registered brute-force reference solver.

use std::any::Any;

use crate::solvers::SolveError;
use crate::traits::Problem;
use crate::types::{Aggregate, SolutionAggregate};

type CartesianWitness<P> = Option<(<P as Problem>::Solution, <P as Problem>::Value)>;

#[doc(hidden)]
pub type BruteForceDimensionsFn = fn(&dyn Any) -> Vec<usize>;
#[doc(hidden)]
pub type BruteForceSolveFn =
    fn(&dyn Any) -> Result<Option<(serde_json::Value, String)>, SolveError>;
#[doc(hidden)]
pub type BruteForceSolveTypedFn = fn(&dyn Any) -> Result<Option<Box<dyn Any>>, SolveError>;
#[doc(hidden)]
pub type BruteForceSolveTypedWithWitnessesFn = fn(&dyn Any) -> Result<Box<dyn Any>, SolveError>;

/// Type-erased registration for one finite Cartesian reference solver.
#[derive(Debug)]
#[doc(hidden)]
pub struct BruteForceRegistration {
    pub source_name: &'static str,
    pub source_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub dimensions_fn: BruteForceDimensionsFn,
    pub solve_fn: BruteForceSolveFn,
    pub solve_typed_fn: BruteForceSolveTypedFn,
    pub solve_typed_with_witnesses_fn: BruteForceSolveTypedWithWitnessesFn,
}

inventory::collect!(BruteForceRegistration);

/// A problem with a finite Cartesian coordinate space for reference solving.
pub trait BruteForceProblem: Problem {
    /// Cardinality of each coordinate in the brute-force search space.
    fn dimensions(&self) -> Vec<usize>;

    /// Number of coordinates in the brute-force search space.
    fn num_variables(&self) -> usize {
        self.dimensions().len()
    }
}

pub(crate) struct CartesianIndices {
    dimensions: Vec<usize>,
    current: Option<Vec<usize>>,
}

impl CartesianIndices {
    pub(crate) fn new(dimensions: Vec<usize>) -> Result<Self, SolveError> {
        let current = if dimensions.contains(&0) {
            None
        } else {
            let mut current = Vec::new();
            current.try_reserve_exact(dimensions.len())?;
            current.resize(dimensions.len(), 0);
            Some(current)
        };
        Ok(Self {
            current,
            dimensions,
        })
    }
}

impl Iterator for CartesianIndices {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        let mut next = current.clone();
        for index in (0..self.dimensions.len()).rev() {
            next[index] += 1;
            if next[index] < self.dimensions[index] {
                self.current = Some(next);
                break;
            }
            next[index] = 0;
        }
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.current.is_some() {
            (1, None)
        } else {
            (0, Some(0))
        }
    }
}

/// Exact reference solver for variants with a registered finite enumeration.
#[derive(Debug, Clone, Default)]
pub struct BruteForce;

impl BruteForce {
    /// Create a new brute-force reference solver.
    pub fn new() -> Self {
        Self
    }

    fn registration<P: Problem>(&self) -> Result<&'static BruteForceRegistration, SolveError> {
        let key = crate::solvers::ExactProblemKey::new(
            P::NAME,
            P::variant()
                .into_iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
        );
        crate::solvers::registry::brute_force_registration(&key)
            .map_err(SolveError::InvalidRegistry)?
            .ok_or_else(|| SolveError::MissingRegistration(P::NAME.to_string()))
    }

    /// Solve a registered finite problem and return one solution when feasible.
    pub fn solve<P>(&self, problem: &P) -> Result<Option<P::Solution>, SolveError>
    where
        P: Problem + 'static,
        P::Solution: 'static,
        P::Value: SolutionAggregate + 'static,
    {
        let solution = (self.registration::<P>()?.solve_typed_fn)(problem as &dyn Any)?;
        solution
            .map(|solution| {
                solution
                    .downcast::<P::Solution>()
                    .map(|value| *value)
                    .map_err(|_| {
                        SolveError::RegistrationTypeMismatch(format!(
                            "{} solution registration returned the wrong type",
                            P::NAME
                        ))
                    })
            })
            .transpose()
    }

    /// Find all witnesses for a registered finite reference solve.
    pub fn find_all_witnesses<P>(&self, problem: &P) -> Result<Vec<P::Solution>, SolveError>
    where
        P: Problem + 'static,
        P::Solution: 'static,
        P::Value: SolutionAggregate + 'static,
    {
        self.solve_with_witnesses(problem)
            .map(|(_, witnesses)| witnesses)
    }

    /// Solve a problem and collect every contributing witness.
    pub fn solve_with_witnesses<P>(
        &self,
        problem: &P,
    ) -> Result<(P::Value, Vec<P::Solution>), SolveError>
    where
        P: Problem + 'static,
        P::Solution: 'static,
        P::Value: SolutionAggregate + 'static,
    {
        (self.registration::<P>()?.solve_typed_with_witnesses_fn)(problem as &dyn Any)?
            .downcast::<(P::Value, Vec<P::Solution>)>()
            .map(|result| *result)
            .map_err(|_| {
                SolveError::RegistrationTypeMismatch(format!(
                    "{} aggregate-and-witness registration returned the wrong type",
                    P::NAME
                ))
            })
    }

    pub(crate) fn solve_cartesian<P, F>(
        &self,
        problem: &P,
        decode: F,
    ) -> Result<P::Value, SolveError>
    where
        P: BruteForceProblem,
        P::Value: Aggregate,
        F: Fn(Vec<usize>) -> P::Solution,
    {
        let mut total = P::Value::identity();
        for indices in CartesianIndices::new(problem.dimensions())? {
            total = total.combine(problem.evaluate(&decode(indices))?)?;
            if total.is_absorbing() {
                break;
            }
        }
        Ok(total)
    }

    pub(crate) fn solve_with_witnesses_cartesian<P, F>(
        &self,
        problem: &P,
        decode: F,
    ) -> Result<(P::Value, Vec<P::Solution>), SolveError>
    where
        P: BruteForceProblem,
        P::Value: SolutionAggregate,
        F: Fn(Vec<usize>) -> P::Solution,
    {
        let total = self.solve_cartesian(problem, &decode)?;
        let mut witnesses = Vec::new();
        for indices in CartesianIndices::new(problem.dimensions())? {
            let solution = decode(indices);
            let value = problem.evaluate(&solution)?;
            if P::Value::contributes_to_solution(&value, &total) {
                witnesses.push(solution);
            }
        }
        Ok((total, witnesses))
    }

    pub(crate) fn find_cartesian<P, F>(
        &self,
        problem: &P,
        decode: F,
    ) -> Result<CartesianWitness<P>, SolveError>
    where
        P: BruteForceProblem,
        P::Value: SolutionAggregate,
        F: Fn(Vec<usize>) -> P::Solution,
    {
        let total = self.solve_cartesian(problem, &decode)?;
        for indices in CartesianIndices::new(problem.dimensions())? {
            let solution = decode(indices);
            let value = problem.evaluate(&solution)?;
            if P::Value::contributes_to_solution(&value, &total) {
                return Ok(Some((solution, value)));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/brute_force.rs"]
mod tests;
