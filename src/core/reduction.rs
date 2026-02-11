//! v1 reduction contract and legacy adapter.

use crate::core::assignment::Assignment;
use crate::core::problem::{LegacyProblemAdapter, ProblemInstance};
use crate::types::ProblemSize;

/// Reduction between source and target problem instances.
pub trait Reduction<S: ProblemInstance, T: ProblemInstance>: Clone {
    /// Borrow reduced target instance.
    fn target_instance(&self) -> &T;

    /// Project a target assignment back to source assignment space.
    fn project_assignment(&self, target_solution: &Assignment) -> Assignment;

    /// Source size profile at reduction time.
    fn source_size_profile(&self) -> ProblemSize;

    /// Target size profile at reduction time.
    fn target_size_profile(&self) -> ProblemSize;
}

/// Adapter exposing a legacy `ReductionResult` as v1 `Reduction`.
#[derive(Debug, Clone)]
pub struct LegacyReductionAdapter<R, S, T>
where
    S: crate::traits::Problem,
    T: crate::traits::Problem,
    R: crate::rules::ReductionResult<Source = S, Target = T>,
{
    inner: R,
    target: LegacyProblemAdapter<T>,
    _marker: std::marker::PhantomData<S>,
}

impl<R, S, T> LegacyReductionAdapter<R, S, T>
where
    S: crate::traits::Problem,
    T: crate::traits::Problem,
    R: crate::rules::ReductionResult<Source = S, Target = T>,
{
    /// Wrap a legacy reduction result.
    pub fn new(inner: R) -> Self {
        let target = LegacyProblemAdapter::new(inner.target_problem().clone());
        Self {
            inner,
            target,
            _marker: std::marker::PhantomData,
        }
    }

    /// Borrow wrapped legacy reduction.
    pub fn inner(&self) -> &R {
        &self.inner
    }

    /// Consume adapter and return wrapped legacy reduction.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R, S, T> Reduction<LegacyProblemAdapter<S>, LegacyProblemAdapter<T>>
    for LegacyReductionAdapter<R, S, T>
where
    S: crate::traits::Problem,
    T: crate::traits::Problem,
    S::Size: crate::core::objective::ObjectiveValue,
    T::Size: crate::core::objective::ObjectiveValue,
    R: crate::rules::ReductionResult<Source = S, Target = T>,
{
    fn target_instance(&self) -> &LegacyProblemAdapter<T> {
        &self.target
    }

    fn project_assignment(&self, target_solution: &Assignment) -> Assignment {
        Assignment::from(self.inner.extract_solution(target_solution.as_slice()))
    }

    fn source_size_profile(&self) -> ProblemSize {
        self.inner.source_size()
    }

    fn target_size_profile(&self) -> ProblemSize {
        self.inner.target_size()
    }
}
