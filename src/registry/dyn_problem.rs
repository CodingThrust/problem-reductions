use serde_json::Value;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;

use crate::traits::EvaluationError;

/// Format a metric for CLI- and registry-facing dynamic dispatch.
///
/// Dynamic formatting uses the problem value's display form directly.
pub fn format_metric<T>(metric: &T) -> String
where
    T: fmt::Display,
{
    metric.to_string()
}

/// Type-erased problem interface for dynamic dispatch.
///
/// Generated for concrete variants at the registration boundary.
pub trait DynProblem: Any {
    /// Evaluate once and return the display value and whether the configuration is feasible.
    fn evaluate_dyn(&self, solution: &Value) -> Result<(String, bool), EvaluationError>;
    /// Evaluate a configuration and return the result as a serializable JSON value.
    fn evaluate_json(&self, solution: &Value) -> Result<Value, EvaluationError>;
    /// Serialize the problem to a JSON value.
    fn serialize_json(&self) -> Value;
    /// Downcast to `&dyn Any` for type recovery.
    fn as_any(&self) -> &dyn Any;
    /// Return the problem name (`Problem::NAME`).
    fn problem_name(&self) -> &'static str;
    /// Return the variant key-value map.
    fn variant_map(&self) -> BTreeMap<String, String>;
    /// Return this problem model's canonical parameter names.
    fn parameter_names_dyn(&self) -> &'static [&'static str];
    /// Measure the complete canonical parameters of this concrete instance.
    fn parameters_dyn(&self) -> crate::types::ProblemParameters;
}

/// Implement the existing dynamic transport boundary for a concrete problem type.
///
/// Concrete value semantics determine feasibility; no solver capability is required.
#[macro_export]
macro_rules! impl_dyn_problem {
    ($ty:ty) => {
        impl $crate::registry::DynProblem for $ty {
            fn evaluate_dyn(
                &self,
                solution: &serde_json::Value,
            ) -> Result<(String, bool), $crate::traits::EvaluationError> {
                let solution = serde::Deserialize::deserialize(solution).map_err(|error| {
                    $crate::traits::EvaluationError::InvalidConfiguration(format!(
                        "invalid solution JSON: {error}"
                    ))
                })?;
                let value = <$ty as $crate::traits::Problem>::evaluate(self, &solution)?;
                Ok((
                    $crate::registry::format_metric(&value),
                    $crate::traits::EvaluationValue::is_valid(&value),
                ))
            }

            fn evaluate_json(
                &self,
                solution: &serde_json::Value,
            ) -> Result<serde_json::Value, $crate::traits::EvaluationError> {
                let solution = serde::Deserialize::deserialize(solution).map_err(|error| {
                    $crate::traits::EvaluationError::InvalidConfiguration(format!(
                        "invalid solution JSON: {error}"
                    ))
                })?;
                Ok(
                    serde_json::to_value(<$ty as $crate::traits::Problem>::evaluate(
                        self, &solution,
                    )?)
                    .expect("serialize metric failed"),
                )
            }

            fn serialize_json(&self) -> serde_json::Value {
                serde_json::to_value(self).expect("serialize failed")
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn problem_name(&self) -> &'static str {
                <$ty as $crate::traits::Problem>::NAME
            }

            fn variant_map(&self) -> std::collections::BTreeMap<String, String> {
                $crate::export::variant_to_map(<$ty as $crate::traits::Problem>::variant())
            }

            fn parameter_names_dyn(&self) -> &'static [&'static str] {
                <$ty as $crate::traits::Problem>::parameter_names()
            }

            fn parameters_dyn(&self) -> $crate::types::ProblemParameters {
                <$ty as $crate::traits::Problem>::parameters(self)
            }
        }
    };
}

/// A loaded type-erased problem.
pub struct LoadedDynProblem {
    inner: Box<dyn DynProblem>,
}

impl std::fmt::Debug for LoadedDynProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadedDynProblem")
            .field("name", &self.inner.problem_name())
            .finish()
    }
}

impl LoadedDynProblem {
    /// Create a new loaded dynamic problem.
    pub(crate) fn new(inner: Box<dyn DynProblem>) -> Self {
        Self { inner }
    }
}

impl std::ops::Deref for LoadedDynProblem {
    type Target = dyn DynProblem;

    fn deref(&self) -> &(dyn DynProblem + 'static) {
        &*self.inner
    }
}
