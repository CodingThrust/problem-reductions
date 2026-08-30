use serde::Serialize;
use serde_json::Value;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;

use crate::traits::{EvaluationError, Problem};
use crate::types::Aggregate;

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
/// Implemented via blanket impl for any `T: Problem + Serialize + 'static`.
pub trait DynProblem: Any {
    /// Evaluate a configuration and return the CLI-facing metric string.
    fn evaluate_dyn(&self, solution: &Value) -> Result<String, EvaluationError>;
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

impl<T> DynProblem for T
where
    T: Problem + Serialize + 'static,
    T::Solution: serde::de::DeserializeOwned,
    T::Value: Aggregate + fmt::Display + Serialize,
{
    fn evaluate_dyn(&self, solution: &Value) -> Result<String, EvaluationError> {
        let solution = serde::Deserialize::deserialize(solution).map_err(|error| {
            EvaluationError::InvalidConfiguration(format!("invalid solution JSON: {error}"))
        })?;
        Ok(format_metric(&self.evaluate(&solution)?))
    }

    fn evaluate_json(&self, solution: &Value) -> Result<Value, EvaluationError> {
        let solution = serde::Deserialize::deserialize(solution).map_err(|error| {
            EvaluationError::InvalidConfiguration(format!("invalid solution JSON: {error}"))
        })?;
        Ok(serde_json::to_value(self.evaluate(&solution)?).expect("serialize metric failed"))
    }

    fn serialize_json(&self) -> Value {
        serde_json::to_value(self).expect("serialize failed")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn problem_name(&self) -> &'static str {
        T::NAME
    }

    fn variant_map(&self) -> BTreeMap<String, String> {
        crate::export::variant_to_map(T::variant())
    }

    fn parameter_names_dyn(&self) -> &'static [&'static str] {
        T::parameter_names()
    }

    fn parameters_dyn(&self) -> crate::types::ProblemParameters {
        self.parameters()
    }
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
