//! Error types for the problemreductions library.

use thiserror::Error;

/// Errors that can occur in the problemreductions library.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProblemError {
    /// Configuration has wrong number of variables.
    #[error("invalid configuration size: expected {expected}, got {got}")]
    InvalidConfigSize { expected: usize, got: usize },

    /// Configuration contains invalid flavor value.
    #[error("invalid flavor value {value} at index {index}: expected 0..{num_flavors}")]
    InvalidFlavor {
        index: usize,
        value: usize,
        num_flavors: usize,
    },

    /// Invalid problem construction.
    #[error("invalid problem: {0}")]
    InvalidProblem(String),

    /// Weight vector has wrong length.
    #[error("invalid weights length: expected {expected}, got {got}")]
    InvalidWeightsLength { expected: usize, got: usize },

    /// Empty problem (no variables or constraints).
    #[error("empty problem: {0}")]
    EmptyProblem(String),

    /// Index out of bounds.
    #[error("index out of bounds: {index} >= {bound}")]
    IndexOutOfBounds { index: usize, bound: usize },
}

/// Result type alias for problemreductions operations.
pub type Result<T> = std::result::Result<T, ProblemError>;
