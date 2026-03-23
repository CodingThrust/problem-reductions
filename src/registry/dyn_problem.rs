use serde::Serialize;
use serde_json::Value;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;

use crate::traits::Problem;

/// Type-erased problem interface for dynamic dispatch.
///
/// Implemented via blanket impl for any `T: Problem + Serialize + 'static`.
pub trait DynProblem: Any {
    /// Evaluate a configuration and return the result as a debug string.
    fn evaluate_dyn(&self, config: &[usize]) -> String;
    /// Evaluate a configuration and return the result as a serializable JSON value.
    fn evaluate_json(&self, config: &[usize]) -> Value;
    /// Serialize the problem to a JSON value.
    fn serialize_json(&self) -> Value;
    /// Downcast to `&dyn Any` for type recovery.
    fn as_any(&self) -> &dyn Any;
    /// Return the configuration space dimensions.
    fn dims_dyn(&self) -> Vec<usize>;
    /// Return the problem name (`Problem::NAME`).
    fn problem_name(&self) -> &'static str;
    /// Return the variant key-value map.
    fn variant_map(&self) -> BTreeMap<String, String>;
    /// Return the number of variables.
    fn num_variables_dyn(&self) -> usize;
}

impl<T> DynProblem for T
where
    T: Problem + Serialize + 'static,
    T::Value: fmt::Debug + Serialize,
{
    fn evaluate_dyn(&self, config: &[usize]) -> String {
        format!("{:?}", self.evaluate(config))
    }

    fn evaluate_json(&self, config: &[usize]) -> Value {
        serde_json::to_value(self.evaluate(config)).expect("serialize metric failed")
    }

    fn serialize_json(&self) -> Value {
        serde_json::to_value(self).expect("serialize failed")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dims_dyn(&self) -> Vec<usize> {
        self.dims()
    }

    fn problem_name(&self) -> &'static str {
        T::NAME
    }

    fn variant_map(&self) -> BTreeMap<String, String> {
        crate::export::variant_to_map(T::variant())
    }

    fn num_variables_dyn(&self) -> usize {
        self.num_variables()
    }
}

/// Function pointer type for brute-force solve dispatch.
pub type SolveFn = fn(&dyn Any) -> Option<(Vec<usize>, String)>;

/// A loaded problem with type-erased solve capability.
///
/// Wraps a `Box<dyn DynProblem>` with a brute-force solve function pointer.
pub struct LoadedDynProblem {
    inner: Box<dyn DynProblem>,
    solve_fn: SolveFn,
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
    pub fn new(inner: Box<dyn DynProblem>, solve_fn: SolveFn) -> Self {
        Self { inner, solve_fn }
    }

    /// Solve the problem using brute force.
    pub fn solve_brute_force(&self) -> Option<(Vec<usize>, String)> {
        (self.solve_fn)(self.inner.as_any())
    }
}

impl std::ops::Deref for LoadedDynProblem {
    type Target = dyn DynProblem;

    fn deref(&self) -> &(dyn DynProblem + 'static) {
        &*self.inner
    }
}
