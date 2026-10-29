//! Canonical problem-instance parameters, shared with the browser.
use serde::{Deserialize, Serialize};
use std::fmt;

/// Canonical named parameters for one concrete problem instance.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemParameters {
    /// Named parameters in canonical declaration order.
    #[serde(deserialize_with = "deserialize_parameter_components")]
    pub(crate) components: Vec<(String, u64)>,
}

impl ProblemParameters {
    /// Create problem parameters in canonical declaration order.
    ///
    /// # Panics
    /// Panics if a parameter name occurs more than once.
    pub fn new(components: Vec<(&str, u64)>) -> Self {
        Self::from_owned(
            components
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
        )
    }

    /// Create problem parameters from owned names.
    ///
    /// # Panics
    /// Panics if a parameter name occurs more than once.
    pub fn from_owned(components: Vec<(String, u64)>) -> Self {
        if let Some(name) = duplicate_parameter_name(&components) {
            panic!("duplicate problem parameter `{name}`");
        }
        Self { components }
    }

    /// Iterate over parameters in canonical declaration order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, u64)> {
        self.components
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
    }

    /// Get a parameter by name.
    pub fn get(&self, name: &str) -> Option<u64> {
        self.components
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| *v)
    }
}

fn duplicate_parameter_name(components: &[(String, u64)]) -> Option<&str> {
    components
        .iter()
        .enumerate()
        .find_map(|(index, (name, _))| {
            components[..index]
                .iter()
                .any(|(previous, _)| previous == name)
                .then_some(name.as_str())
        })
}

fn deserialize_parameter_components<'de, D>(deserializer: D) -> Result<Vec<(String, u64)>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let components = Vec::<(String, u64)>::deserialize(deserializer)?;
    if let Some(name) = duplicate_parameter_name(&components) {
        return Err(serde::de::Error::custom(format!(
            "duplicate problem parameter `{name}`"
        )));
    }
    Ok(components)
}

impl fmt::Display for ProblemParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProblemParameters{{")?;
        for (i, (name, value)) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, value)?;
        }
        write!(f, "}}")
    }
}
