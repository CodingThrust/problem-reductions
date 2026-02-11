//! v1 problem contracts and legacy adapter.

use crate::core::assignment::Assignment;
use crate::core::evaluation::Evaluation;
use crate::core::objective::{ObjectiveDirection, ObjectiveValue};
use crate::core::variant::{from_legacy_variant, VariantDimension};
use crate::types::ProblemSize;

/// Static problem identity and variant dimensions.
pub trait ProblemSpec {
    /// Stable base name for this problem.
    const NAME: &'static str;

    /// Objective value type for this problem.
    type Value: ObjectiveValue;

    /// Variant dimensions for this concrete type.
    fn variant_dimensions() -> Vec<VariantDimension>;
}

/// Runtime problem instance behavior.
pub trait ProblemInstance: Clone + ProblemSpec {
    /// Number of decision variables.
    fn num_variables(&self) -> usize;

    /// Number of available flavors per variable.
    fn num_flavors(&self) -> usize;

    /// Size profile used by reduction overhead and complexity metadata.
    fn size_profile(&self) -> ProblemSize;

    /// Optimization direction for this instance.
    fn objective_direction(&self) -> ObjectiveDirection;

    /// Evaluate an assignment.
    fn evaluate_assignment(&self, assignment: &Assignment) -> Evaluation<Self::Value>;

    /// Evaluate raw assignment values.
    fn evaluate_config(&self, config: &[usize]) -> Evaluation<Self::Value> {
        self.evaluate_assignment(&Assignment::from_slice(config))
    }

    /// Validate assignment shape and flavor bounds.
    fn validate_assignment(&self, assignment: &Assignment) -> crate::error::Result<()> {
        assignment.validate(self.num_variables(), self.num_flavors())
    }
}

/// Adapter that exposes a legacy `Problem` as a v1 `ProblemInstance`.
#[derive(Debug, Clone)]
pub struct LegacyProblemAdapter<P: crate::traits::Problem> {
    inner: P,
}

impl<P: crate::traits::Problem> LegacyProblemAdapter<P> {
    /// Wrap a legacy problem instance.
    pub fn new(inner: P) -> Self {
        Self { inner }
    }

    /// Borrow wrapped problem.
    pub fn inner(&self) -> &P {
        &self.inner
    }

    /// Consume adapter and return wrapped problem.
    pub fn into_inner(self) -> P {
        self.inner
    }
}

impl<P> ProblemSpec for LegacyProblemAdapter<P>
where
    P: crate::traits::Problem,
    P::Size: ObjectiveValue,
{
    const NAME: &'static str = P::NAME;
    type Value = P::Size;

    fn variant_dimensions() -> Vec<VariantDimension> {
        from_legacy_variant(&P::variant())
    }
}

impl<P> ProblemInstance for LegacyProblemAdapter<P>
where
    P: crate::traits::Problem,
    P::Size: ObjectiveValue,
{
    fn num_variables(&self) -> usize {
        self.inner.num_variables()
    }

    fn num_flavors(&self) -> usize {
        self.inner.num_flavors()
    }

    fn size_profile(&self) -> ProblemSize {
        self.inner.problem_size()
    }

    fn objective_direction(&self) -> ObjectiveDirection {
        self.inner.energy_mode().into()
    }

    fn evaluate_assignment(&self, assignment: &Assignment) -> Evaluation<Self::Value> {
        self.inner.solution_size(assignment.as_slice()).into()
    }
}
