//! Assignment wrapper and validation helpers.

use crate::error::{ProblemError, Result};
use serde::{Deserialize, Serialize};

/// A typed wrapper around variable assignments.
///
/// Each value must be in the range `0..num_flavors`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Assignment {
    values: Vec<usize>,
}

impl Assignment {
    /// Create an assignment from raw values.
    pub fn new(values: Vec<usize>) -> Self {
        Self { values }
    }

    /// Create an assignment by cloning a slice.
    pub fn from_slice(values: &[usize]) -> Self {
        Self {
            values: values.to_vec(),
        }
    }

    /// Borrow raw assignment values.
    pub fn as_slice(&self) -> &[usize] {
        &self.values
    }

    /// Consume and return raw assignment values.
    pub fn into_inner(self) -> Vec<usize> {
        self.values
    }

    /// Number of variables in this assignment.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if assignment is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Validate assignment shape and flavor bounds.
    pub fn validate(&self, num_variables: usize, num_flavors: usize) -> Result<()> {
        if self.values.len() != num_variables {
            return Err(ProblemError::InvalidConfigSize {
                expected: num_variables,
                got: self.values.len(),
            });
        }

        for (idx, &value) in self.values.iter().enumerate() {
            if value >= num_flavors {
                return Err(ProblemError::InvalidFlavor {
                    index: idx,
                    value,
                    num_flavors,
                });
            }
        }
        Ok(())
    }

    /// Fast boolean assignment validity check.
    pub fn is_valid(&self, num_variables: usize, num_flavors: usize) -> bool {
        self.validate(num_variables, num_flavors).is_ok()
    }
}

impl From<Vec<usize>> for Assignment {
    fn from(values: Vec<usize>) -> Self {
        Self::new(values)
    }
}

impl From<&[usize]> for Assignment {
    fn from(values: &[usize]) -> Self {
        Self::from_slice(values)
    }
}

impl AsRef<[usize]> for Assignment {
    fn as_ref(&self) -> &[usize] {
        self.as_slice()
    }
}
