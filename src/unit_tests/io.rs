use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::SimpleGraph;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_to_json() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let json = to_json(&problem);
    assert!(json.is_ok());
    let json = json.unwrap();
    assert!(json.contains("graph"));
}

#[test]
fn test_from_json() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let json = to_json(&problem).unwrap();
    let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json).unwrap();
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
}

#[test]
fn test_json_compact() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let compact = to_json_compact(&problem).unwrap();
    let pretty = to_json(&problem).unwrap();
    // Compact should be shorter
    assert!(compact.len() < pretty.len());
}

#[test]
fn test_file_roundtrip() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("test_problem_{ts}.json"));
    let path = path.to_str().unwrap();

    // Write
    write_problem(&problem, path, FileFormat::Json).unwrap();

    // Read back
    let restored: MaximumIndependentSet<SimpleGraph, i32> = read_problem(path, FileFormat::Json).unwrap();
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
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("test_io_{ts}.txt"));
    let path = path.to_str().unwrap();
    let contents = "Hello, World!";

    write_file(path, contents).unwrap();
    let read_back = read_file(path).unwrap();

    assert_eq!(read_back, contents);

    fs::remove_file(path).ok();
}

#[test]
fn test_invalid_json() {
    let result: Result<MaximumIndependentSet<SimpleGraph, i32>> = from_json("not valid json");
    assert!(result.is_err());
}
