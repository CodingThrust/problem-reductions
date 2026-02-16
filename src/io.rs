//! File I/O utilities for problem serialization.
//!
//! This module provides functions for reading and writing problems
//! to various file formats using serde.

use crate::error::{ProblemError, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Supported file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// JSON format (human-readable).
    Json,
    /// Compact JSON format (no pretty-printing).
    JsonCompact,
}

impl FileFormat {
    /// Detect file format from file extension.
    pub fn from_extension(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        match ext.as_str() {
            "json" => Some(FileFormat::Json),
            _ => None,
        }
    }
}

/// Write a problem to a file.
///
/// # Arguments
///
/// * `problem` - The problem to write
/// * `path` - The file path to write to
/// * `format` - The file format to use
///
/// # Example
///
/// ```no_run
/// use problemreductions::io::{write_problem, FileFormat};
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
///
/// let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
/// write_problem(&problem, "problem.json", FileFormat::Json).unwrap();
/// ```
pub fn write_problem<T: Serialize, P: AsRef<Path>>(
    problem: &T,
    path: P,
    format: FileFormat,
) -> Result<()> {
    let file = File::create(path.as_ref())
        .map_err(|e| ProblemError::IoError(format!("Failed to create file: {}", e)))?;
    let writer = BufWriter::new(file);

    match format {
        FileFormat::Json => serde_json::to_writer_pretty(writer, problem)
            .map_err(|e| ProblemError::SerializationError(format!("Failed to write JSON: {}", e))),
        FileFormat::JsonCompact => serde_json::to_writer(writer, problem)
            .map_err(|e| ProblemError::SerializationError(format!("Failed to write JSON: {}", e))),
    }
}

/// Read a problem from a file.
///
/// # Arguments
///
/// * `path` - The file path to read from
/// * `format` - The file format to use
///
/// # Example
///
/// ```no_run
/// use problemreductions::io::{read_problem, FileFormat};
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
///
/// let problem: MaximumIndependentSet<SimpleGraph, i32> = read_problem("problem.json", FileFormat::Json).unwrap();
/// ```
pub fn read_problem<T: DeserializeOwned, P: AsRef<Path>>(path: P, format: FileFormat) -> Result<T> {
    let file = File::open(path.as_ref())
        .map_err(|e| ProblemError::IoError(format!("Failed to open file: {}", e)))?;
    let reader = BufReader::new(file);

    match format {
        FileFormat::Json | FileFormat::JsonCompact => serde_json::from_reader(reader)
            .map_err(|e| ProblemError::SerializationError(format!("Failed to parse JSON: {}", e))),
    }
}

/// Serialize a problem to a JSON string.
pub fn to_json<T: Serialize>(problem: &T) -> Result<String> {
    serde_json::to_string_pretty(problem)
        .map_err(|e| ProblemError::SerializationError(format!("Failed to serialize: {}", e)))
}

/// Serialize a problem to a compact JSON string.
pub fn to_json_compact<T: Serialize>(problem: &T) -> Result<String> {
    serde_json::to_string(problem)
        .map_err(|e| ProblemError::SerializationError(format!("Failed to serialize: {}", e)))
}

/// Deserialize a problem from a JSON string.
pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T> {
    serde_json::from_str(json)
        .map_err(|e| ProblemError::SerializationError(format!("Failed to parse JSON: {}", e)))
}

/// Read a file to a string.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path.as_ref())
        .map_err(|e| ProblemError::IoError(format!("Failed to open file: {}", e)))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| ProblemError::IoError(format!("Failed to read file: {}", e)))?;
    Ok(contents)
}

/// Write a string to a file.
pub fn write_file<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    let mut file = File::create(path.as_ref())
        .map_err(|e| ProblemError::IoError(format!("Failed to create file: {}", e)))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| ProblemError::IoError(format!("Failed to write file: {}", e)))?;
    Ok(())
}

#[cfg(test)]
#[path = "unit_tests/io.rs"]
mod tests;
