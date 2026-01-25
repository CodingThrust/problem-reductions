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
/// use problemreductions::models::graph::IndependentSet;
///
/// let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
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
/// use problemreductions::models::graph::IndependentSet;
///
/// let problem: IndependentSet<i32> = read_problem("problem.json", FileFormat::Json).unwrap();
/// ```
pub fn read_problem<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
    format: FileFormat,
) -> Result<T> {
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
mod tests {
    use super::*;
    use crate::models::graph::IndependentSet;
    use std::fs;

    #[test]
    fn test_to_json() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let json = to_json(&problem);
        assert!(json.is_ok());
        let json = json.unwrap();
        assert!(json.contains("graph"));
    }

    #[test]
    fn test_from_json() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let json = to_json(&problem).unwrap();
        let restored: IndependentSet<i32> = from_json(&json).unwrap();
        assert_eq!(restored.num_vertices(), 3);
        assert_eq!(restored.num_edges(), 2);
    }

    #[test]
    fn test_json_compact() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1)]);
        let compact = to_json_compact(&problem).unwrap();
        let pretty = to_json(&problem).unwrap();
        // Compact should be shorter
        assert!(compact.len() < pretty.len());
    }

    #[test]
    fn test_file_roundtrip() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let path = "/tmp/test_problem.json";

        // Write
        write_problem(&problem, path, FileFormat::Json).unwrap();

        // Read back
        let restored: IndependentSet<i32> = read_problem(path, FileFormat::Json).unwrap();
        assert_eq!(restored.num_vertices(), 4);
        assert_eq!(restored.num_edges(), 3);

        // Cleanup
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_file_format_from_extension() {
        assert_eq!(
            FileFormat::from_extension(Path::new("test.json")),
            Some(FileFormat::Json)
        );
        assert_eq!(
            FileFormat::from_extension(Path::new("test.JSON")),
            Some(FileFormat::Json)
        );
        assert_eq!(FileFormat::from_extension(Path::new("test.txt")), None);
        assert_eq!(FileFormat::from_extension(Path::new("noext")), None);
    }

    #[test]
    fn test_read_write_file() {
        let path = "/tmp/test_io.txt";
        let contents = "Hello, World!";

        write_file(path, contents).unwrap();
        let read_back = read_file(path).unwrap();

        assert_eq!(read_back, contents);

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_invalid_json() {
        let result: Result<IndependentSet<i32>> = from_json("not valid json");
        assert!(result.is_err());
    }
}
