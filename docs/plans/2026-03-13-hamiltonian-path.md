# Plan: Add HamiltonianPath Model (#217)

## Overview
Add `HamiltonianPath` — a graph-based satisfaction problem (Metric = bool) that asks whether a graph contains a simple path visiting every vertex exactly once. Follows the `add-model` skill pipeline.

## Step 1: Create Model File
**File:** `src/models/graph/hamiltonian_path.rs`

- Define `HamiltonianPath<G>` struct with a single `graph: G` field
- Implement `new()`, `graph()`, `num_vertices()`, `num_edges()` getters
- Implement `Problem` trait:
  - `NAME = "HamiltonianPath"`
  - `Metric = bool`
  - `variant()` via `variant_params![G]`
  - `dims()` returns `vec![n; n]` where n = num_vertices (each variable = which vertex at position i)
  - `evaluate()` checks: (1) config is a valid permutation of 0..n, (2) consecutive vertices are adjacent
- Implement `SatisfactionProblem` marker trait
- Add `declare_variants!` with `HamiltonianPath<SimpleGraph> => "1.657^num_vertices"`
- Add `#[cfg(test)] #[path]` link to unit tests

## Step 2: Register Module
- Add `pub(crate) mod hamiltonian_path;` and `pub use hamiltonian_path::HamiltonianPath;` to `src/models/graph/mod.rs`
- Add `HamiltonianPath` to re-exports in `src/models/mod.rs`

## Step 3: CLI Dispatch
- Add `"HamiltonianPath" => deser_sat::<HamiltonianPath<SimpleGraph>>(data)` in `problemreductions-cli/src/dispatch.rs`

## Step 4: Unit Tests
**File:** `src/unit_tests/models/graph/hamiltonian_path.rs`

- `test_hamiltonian_path_basic`: create instance, test evaluate on valid/invalid paths
- `test_hamiltonian_path_no_solution`: graph with isolated vertices → no satisfying assignment
- `test_hamiltonian_path_brute_force`: use BruteForce::find_satisfying and find_all_satisfying
- `test_hamiltonian_path_serialization`: serde round-trip

## Step 5: Example
**File:** `examples/hamiltonian_path.rs`

- Create Instance 2 from the issue (6 vertices, 8 edges, non-trivial path)
- Print JSON, verify Hamiltonian path exists
- `pub fn run()` + `fn main() { run() }`

## Step 6: Export & Regenerate
- Run `make export-schemas` to regenerate problem schemas
- Run `make check` to verify fmt + clippy + tests pass

## Step 7: Review
- Run `/review-implementation` for completeness check
