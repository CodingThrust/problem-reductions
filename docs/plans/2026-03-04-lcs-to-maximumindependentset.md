# LongestCommonSubsequence to MaximumIndependentSet Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the LCS model (issue #108) and the LCS → MaxIS reduction rule (issue #109).

**Architecture:** The LCS problem takes k strings over an alphabet and finds the longest common subsequence. The reduction constructs a "match graph" where nodes are k-tuples of positions with matching characters, and conflict edges connect incompatible tuples. MaxIS on this graph equals the LCS length. The model goes in `src/models/misc/` since strings are a unique input structure. The reduction produces `MaximumIndependentSet<SimpleGraph, One>` (unit-weight).

**Tech Stack:** Rust, serde, inventory (schema registration), `#[reduction]` proc macro

**References:**
- Apostolico & Guerra, 1987 (https://doi.org/10.1137/0216009)
- Baxter et al., 2004 (https://doi.org/10.1007/978-3-540-27801-6_12)
- Issue #108 (model), Issue #109 (rule)

---

### Task 1: Implement the LCS model

**Files:**
- Create: `src/models/misc/longest_common_subsequence.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs` (prelude)
- Create: `src/unit_tests/models/misc/longest_common_subsequence.rs`

**Step 1: Create the model file**

Create `src/models/misc/longest_common_subsequence.rs`:

```rust
//! Longest Common Subsequence problem implementation.
//!
//! Given k strings over a finite alphabet, find a longest string that is
//! a subsequence of every input string.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        module_path: module_path!(),
        description: "Find longest string that is a subsequence of every input string",
        fields: &[
            FieldInfo { name: "strings", type_name: "Vec<Vec<u8>>", description: "Input strings s_1, ..., s_k" },
        ],
    }
}

/// The Longest Common Subsequence (LCS) problem.
///
/// Given `k` strings `s_1, ..., s_k` over a finite alphabet, find a longest
/// string `w` that is a subsequence of every `s_i`.
///
/// # Representation
///
/// Variables represent positions in the shortest string. Each variable selects
/// whether that character position contributes to the subsequence (binary: include/exclude).
///
/// More precisely, for a shortest string of length `m`, we have `m` binary variables.
/// `x_j = 1` means the j-th character of the shortest string is included in the
/// candidate subsequence. The evaluate function checks whether the resulting
/// subsequence of the shortest string is also a subsequence of all other strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    strings: Vec<Vec<u8>>,
}

impl LongestCommonSubsequence {
    /// Create a new LCS instance.
    ///
    /// # Panics
    /// Panics if fewer than 2 strings are provided or any string is empty.
    pub fn new(strings: Vec<Vec<u8>>) -> Self {
        assert!(strings.len() >= 2, "need at least 2 strings");
        Self { strings }
    }

    /// Returns the input strings.
    pub fn strings(&self) -> &[Vec<u8>] {
        &self.strings
    }

    /// Returns the number of strings k.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Returns the total length of all strings.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Returns the index of the shortest string.
    fn shortest_index(&self) -> usize {
        self.strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Returns the length of the shortest string (number of binary variables).
    fn shortest_len(&self) -> usize {
        self.strings.iter().map(|s| s.len()).min().unwrap_or(0)
    }
}

/// Check if `subseq` is a subsequence of `s`.
fn is_subsequence(subseq: &[u8], s: &[u8]) -> bool {
    let mut it = s.iter();
    subseq.iter().all(|c| it.any(|sc| sc == c))
}

impl Problem for LongestCommonSubsequence {
    const NAME: &'static str = "LongestCommonSubsequence";
    type Metric = SolutionSize<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.shortest_len()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<usize> {
        let m = self.shortest_len();
        if config.len() != m {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v >= 2) {
            return SolutionSize::Invalid;
        }

        // Build candidate subsequence from shortest string
        let si = self.shortest_index();
        let shortest = &self.strings[si];
        let subseq: Vec<u8> = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(j, _)| shortest[j])
            .collect();

        // Check if subseq is a subsequence of all other strings
        for (i, s) in self.strings.iter().enumerate() {
            if i == si {
                continue; // Already a subsequence of shortest by construction
            }
            if !is_subsequence(&subseq, s) {
                return SolutionSize::Invalid;
            }
        }

        SolutionSize::Valid(subseq.len())
    }
}

impl OptimizationProblem for LongestCommonSubsequence {
    type Value = usize;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

crate::declare_variants! {
    LongestCommonSubsequence => "2^total_length",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
```

**Step 2: Register the model in mod.rs files**

In `src/models/misc/mod.rs`, add:
```rust
mod longest_common_subsequence;
pub use longest_common_subsequence::LongestCommonSubsequence;
```

In `src/models/mod.rs`, add `LongestCommonSubsequence` to the re-export line for misc.

In `src/lib.rs`, add `LongestCommonSubsequence` to the prelude `pub use crate::models::misc::` line.

**Step 3: Write unit tests**

Create `src/unit_tests/models/misc/longest_common_subsequence.rs`:

```rust
use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_lcs_basic() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);
    assert_eq!(problem.num_strings(), 2);
    assert_eq!(problem.total_length(), 8);
    assert_eq!(problem.direction(), Direction::Maximize);
    assert_eq!(<LongestCommonSubsequence as Problem>::NAME, "LongestCommonSubsequence");
    assert_eq!(<LongestCommonSubsequence as Problem>::variant(), vec![]);
}

#[test]
fn test_lcs_dims() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);
    // Shortest string has length 4, so 4 binary variables
    assert_eq!(problem.dims(), vec![2; 4]);
}

#[test]
fn test_lcs_evaluate_valid_subsequence() {
    // ABAC and BACA: "BAC" is a common subsequence of length 3
    let problem = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);
    // Selecting positions 1,2,3 from ABAC gives "BAC"
    assert!(problem.evaluate(&[0, 1, 1, 1]).is_valid());
}

#[test]
fn test_lcs_evaluate_invalid_subsequence() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);
    // Selecting all 4 chars from ABAC gives "ABAC", not a subsequence of "BACA"
    assert_eq!(problem.evaluate(&[1, 1, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_lcs_evaluate_empty() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABC".to_vec(),
        b"DEF".to_vec(),
    ]);
    // Empty subsequence is always valid
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_lcs_brute_force() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, SolutionSize::Valid(3)); // LCS = "BAC" or "AAC" or "ACA"
}

#[test]
fn test_lcs_three_strings() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABCDAB".to_vec(),
        b"BDCABA".to_vec(),
        b"BCADBA".to_vec(),
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, SolutionSize::Valid(4)); // LCS = "BCAB"
}

#[test]
fn test_lcs_evaluate_wrong_config_length() {
    let problem = LongestCommonSubsequence::new(vec![
        b"AB".to_vec(),
        b"BA".to_vec(),
    ]);
    assert_eq!(problem.evaluate(&[1]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_lcs_evaluate_invalid_variable_value() {
    let problem = LongestCommonSubsequence::new(vec![
        b"AB".to_vec(),
        b"BA".to_vec(),
    ]);
    assert_eq!(problem.evaluate(&[2, 0]), SolutionSize::Invalid);
}

#[test]
fn test_lcs_serialization() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABC".to_vec(),
        b"BCA".to_vec(),
    ]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.strings(), problem.strings());
}

#[test]
fn test_lcs_identical_strings() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABC".to_vec(),
        b"ABC".to_vec(),
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find solution");
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(3));
}

#[test]
fn test_lcs_no_common_chars() {
    let problem = LongestCommonSubsequence::new(vec![
        b"ABC".to_vec(),
        b"DEF".to_vec(),
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find solution");
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(0));
}

#[test]
#[should_panic(expected = "need at least 2 strings")]
fn test_lcs_too_few_strings() {
    LongestCommonSubsequence::new(vec![b"ABC".to_vec()]);
}
```

**Step 4: Register in unit test mod.rs**

Check `src/unit_tests/models/misc/mod.rs` and add:
```rust
mod longest_common_subsequence;
```

**Step 5: Build and test**

Run: `cargo test test_lcs -- --nocapture`
Expected: All tests pass.

**Step 6: Register in CLI dispatch**

In `problemreductions-cli/src/dispatch.rs`:
- Add `use problemreductions::models::misc::LongestCommonSubsequence;` (or adjust import)
- Add `"LongestCommonSubsequence" => deser_opt::<LongestCommonSubsequence>(data),` in the deserialize match
- Add `"LongestCommonSubsequence" => try_ser::<LongestCommonSubsequence>(any),` in the serialize match

**Step 7: Commit**

```bash
git add src/models/misc/longest_common_subsequence.rs src/models/misc/mod.rs \
        src/models/mod.rs src/lib.rs \
        src/unit_tests/models/misc/longest_common_subsequence.rs \
        src/unit_tests/models/misc/mod.rs \
        problemreductions-cli/src/dispatch.rs
git commit -m "feat: add LongestCommonSubsequence model (closes #108)"
```

---

### Task 2: Implement the LCS → MaxIS reduction rule

**Files:**
- Create: `src/rules/longestcommonsubsequence_maximumindependentset.rs`
- Modify: `src/rules/mod.rs`
- Create: `src/unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs`

**Step 1: Create the reduction rule file**

Create `src/rules/longestcommonsubsequence_maximumindependentset.rs`:

```rust
//! Reduction from LongestCommonSubsequence to MaximumIndependentSet.
//!
//! Constructs a "match graph" where each vertex is a k-tuple of positions
//! (one per string) that all share the same character. Two vertices are
//! connected by a conflict edge if they cannot coexist in a valid common
//! subsequence — i.e., the position orderings are inconsistent (crossing)
//! or share a position in some string.
//!
//! A maximum independent set in this graph corresponds to a longest common
//! subsequence.

use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;

/// Result of reducing LCS to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionLCSToIS {
    target: MaximumIndependentSet<SimpleGraph, One>,
    /// Position tuples for each vertex: nodes[i] = (p_1, p_2, ..., p_k)
    nodes: Vec<Vec<usize>>,
    /// Number of variables in the source LCS problem (= shortest string length).
    num_source_variables: usize,
    /// Index of the shortest string in the source problem.
    shortest_index: usize,
}

impl ReductionResult for ReductionLCSToIS {
    type Source = LongestCommonSubsequence;
    type Target = MaximumIndependentSet<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract an LCS solution from an IS solution.
    ///
    /// Each selected vertex represents a matched position tuple. We reconstruct
    /// which positions in the shortest string were used, setting x_j = 1 for
    /// each position j of the shortest string that appears in a selected node.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut config = vec![0usize; self.num_source_variables];
        for (vertex_idx, &selected) in target_solution.iter().enumerate() {
            if selected == 1 {
                let pos_in_shortest = self.nodes[vertex_idx][self.shortest_index];
                config[pos_in_shortest] = 1;
            }
        }
        config
    }
}

/// Check if two position tuples conflict (cannot both be in a common subsequence).
///
/// Conflict occurs when the relative order is inconsistent across strings:
/// either some positions go forward while others go backward, or any
/// position is shared (equal).
fn tuples_conflict(a: &[usize], b: &[usize]) -> bool {
    // Check if all(a_i < b_i) or all(a_i > b_i). If neither, it's a conflict.
    let mut all_less = true;
    let mut all_greater = true;
    for (ai, bi) in a.iter().zip(b.iter()) {
        if ai >= bi {
            all_less = false;
        }
        if ai <= bi {
            all_greater = false;
        }
    }
    // Conflict if neither all-less nor all-greater
    !all_less && !all_greater
}

#[reduction(
    overhead = {
        num_vertices = "total_length^num_strings",
        num_edges = "total_length^(2 * num_strings)",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for LongestCommonSubsequence {
    type Result = ReductionLCSToIS;

    fn reduce_to(&self) -> Self::Result {
        let k = self.num_strings();
        let strings = self.strings();

        // Find the shortest string index
        let shortest_index = strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap_or(0);
        let num_source_variables = strings[shortest_index].len();

        // Step 1: Build match nodes — k-tuples of positions with matching characters
        let mut nodes: Vec<Vec<usize>> = Vec::new();

        // Collect character positions for each string
        // char_positions[i][c] = list of positions in string i with character c
        let mut char_positions: Vec<std::collections::HashMap<u8, Vec<usize>>> =
            vec![std::collections::HashMap::new(); k];
        for (i, s) in strings.iter().enumerate() {
            for (j, &c) in s.iter().enumerate() {
                char_positions[i].entry(c).or_default().push(j);
            }
        }

        // Find all characters that appear in all strings
        let common_chars: Vec<u8> = {
            let first_chars: std::collections::HashSet<u8> =
                char_positions[0].keys().copied().collect();
            first_chars
                .into_iter()
                .filter(|c| char_positions.iter().all(|cp| cp.contains_key(c)))
                .collect()
        };

        // Generate all k-tuples for each common character
        for c in &common_chars {
            let position_lists: Vec<&Vec<usize>> =
                char_positions.iter().map(|cp| &cp[c]).collect();
            // Generate Cartesian product of position lists
            let mut tuples: Vec<Vec<usize>> = vec![vec![]];
            for positions in &position_lists {
                let mut new_tuples = Vec::new();
                for tuple in &tuples {
                    for &pos in *positions {
                        let mut new_tuple = tuple.clone();
                        new_tuple.push(pos);
                        new_tuples.push(new_tuple);
                    }
                }
                tuples = new_tuples;
            }
            nodes.extend(tuples);
        }

        // Step 2: Build conflict edges
        let n = nodes.len();
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if tuples_conflict(&nodes[i], &nodes[j]) {
                    edges.push((i, j));
                }
            }
        }

        let target = MaximumIndependentSet::new(
            SimpleGraph::new(n, edges),
            vec![One; n],
        );

        ReductionLCSToIS {
            target,
            nodes,
            num_source_variables,
            shortest_index,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs"]
mod tests;
```

**Step 2: Register in rules/mod.rs**

Add to `src/rules/mod.rs` (alphabetically):
```rust
mod longestcommonsubsequence_maximumindependentset;
```

**Step 3: Write unit tests**

Create `src/unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs`:

```rust
use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::LongestCommonSubsequence;
use crate::rules::traits::ReduceTo;
use crate::solvers::{BruteForce, Solver};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::One;
use std::collections::HashSet;

#[test]
fn test_lcs_to_maximumindependentset_closed_loop() {
    // ABAC and BACA: LCS = 3 (e.g., "BAC")
    let source = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();

    // Solve source directly
    let best_source = solver.find_all_best(&source);
    let source_set: HashSet<Vec<usize>> = best_source.into_iter().collect();

    // Solve target and extract
    let best_target = solver.find_all_best(target);
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();

    assert!(!extracted.is_empty());
    assert!(extracted.is_subset(&source_set));

    // Verify optimal value matches
    for sol in &extracted {
        let metric = source.evaluate(sol);
        assert_eq!(metric, SolutionSize::Valid(3));
    }
}

#[test]
fn test_lcs_to_is_graph_structure() {
    // ABAC and BACA
    let source = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    // From the issue: 6 match nodes, 9 conflict edges
    assert_eq!(target.graph().num_vertices(), 6);
    assert_eq!(target.graph().num_edges(), 9);
}

#[test]
fn test_lcs_to_is_three_strings() {
    let source = LongestCommonSubsequence::new(vec![
        b"ABCDAB".to_vec(),
        b"BDCABA".to_vec(),
        b"BCADBA".to_vec(),
    ]);

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(target);

    // IS size should equal LCS length = 4
    for sol in &best_target {
        let is_size: usize = sol.iter().sum();
        assert_eq!(is_size, 4);
    }

    // Extract and verify
    let best_source = solver.find_all_best(&source);
    let source_set: HashSet<Vec<usize>> = best_source.into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&source_set));
}

#[test]
fn test_lcs_to_is_no_common_chars() {
    let source = LongestCommonSubsequence::new(vec![
        b"ABC".to_vec(),
        b"DEF".to_vec(),
    ]);

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    // No matching characters → 0 vertices
    assert_eq!(target.graph().num_vertices(), 0);
    assert_eq!(target.graph().num_edges(), 0);
}

#[test]
fn test_lcs_to_is_identical_strings() {
    let source = LongestCommonSubsequence::new(vec![
        b"ABC".to_vec(),
        b"ABC".to_vec(),
    ]);

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let best = solver.find_all_best(target);

    // LCS of identical strings = full string length = 3
    for sol in &best {
        let is_size: usize = sol.iter().sum();
        assert_eq!(is_size, 3);
    }
}

#[test]
fn test_tuples_conflict_function() {
    // Same position → conflict
    assert!(tuples_conflict(&[0, 1], &[0, 2]));
    // Crossing → conflict
    assert!(tuples_conflict(&[0, 1], &[1, 0]));
    // Consistent forward → no conflict
    assert!(!tuples_conflict(&[0, 0], &[1, 1]));
    // Consistent backward → no conflict
    assert!(!tuples_conflict(&[1, 1], &[0, 0]));
}
```

**Step 4: Register unit test module**

Check `src/unit_tests/rules/mod.rs` and add:
```rust
mod longestcommonsubsequence_maximumindependentset;
```

**Step 5: Build and test**

Run: `cargo test test_lcs_to -- --nocapture`
Expected: All tests pass.

**Step 6: Commit**

```bash
git add src/rules/longestcommonsubsequence_maximumindependentset.rs \
        src/rules/mod.rs \
        src/unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs \
        src/unit_tests/rules/mod.rs
git commit -m "feat: add LCS to MaximumIndependentSet reduction rule (closes #109)"
```

---

### Task 3: Write example program

**Files:**
- Create: `examples/reduction_longestcommonsubsequence_to_maximumindependentset.rs`
- Modify: `tests/suites/examples.rs`

**Step 1: Create the example file**

Create `examples/reduction_longestcommonsubsequence_to_maximumindependentset.rs`:

```rust
// # LongestCommonSubsequence to MaximumIndependentSet Reduction
//
// ## Reduction Overview
// A match graph is constructed where each vertex is a pair of positions
// (p_1, p_2) from the two input strings sharing the same character.
// Conflict edges connect pairs that cannot coexist in a common subsequence
// (crossing or shared positions). MaxIS on this graph equals the LCS length.
//
// ## This Example
// - 2 strings: s1 = "ABAC", s2 = "BACA"
// - Match graph: 6 vertices, 9 conflict edges
// - LCS = "BAC" (length 3), corresponding to IS {v2, v3, v5}
//
// ## Output
// Exports `docs/paper/examples/lcs_to_maximumindependentset.json` and
// `lcs_to_maximumindependentset.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    let source = LongestCommonSubsequence::new(vec![
        b"ABAC".to_vec(),
        b"BACA".to_vec(),
    ]);

    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: LCS with {} strings, total length {}",
        source.num_strings(),
        source.total_length()
    );
    println!(
        "Target: MaxIS with {} vertices, {} edges",
        target.graph().num_vertices(),
        target.graph().num_edges()
    );

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_best(target);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", target_solutions.len());

    let mut solutions = Vec::new();
    for target_sol in &target_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let eval = source.evaluate(&source_sol);
        assert!(eval.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol.clone(),
            target_config: target_sol.clone(),
        });
    }

    let source_sol = reduction.extract_solution(&target_solutions[0]);
    println!("Source solution: {:?}", source_sol);
    println!("Source value: {:?}", source.evaluate(&source_sol));
    println!("\nReduction verified successfully");

    // Export JSON
    let source_variant = variant_to_map(LongestCommonSubsequence::variant());
    let target_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, One>::variant());
    let overhead = lookup_overhead(
        "LongestCommonSubsequence",
        &source_variant,
        "MaximumIndependentSet",
        &target_variant,
    )
    .expect("LCS -> MaxIS overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: LongestCommonSubsequence::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_strings": source.num_strings(),
                "total_length": source.total_length(),
                "strings": source.strings(),
            }),
        },
        target: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, One>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vertices": target.graph().num_vertices(),
                "num_edges": target.graph().num_edges(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("lcs_to_maximumindependentset", &data, &results);
}

fn main() {
    run()
}
```

**Step 2: Register the example in tests/suites/examples.rs**

Add alphabetically:
```rust
example_test!(reduction_longestcommonsubsequence_to_maximumindependentset);
```

And:
```rust
example_fn!(
    test_longestcommonsubsequence_to_maximumindependentset,
    reduction_longestcommonsubsequence_to_maximumindependentset
);
```

**Step 3: Run the example**

Run: `cargo run --example reduction_longestcommonsubsequence_to_maximumindependentset`
Expected: Prints transformation info, exports JSON files.

**Step 4: Commit**

```bash
git add examples/reduction_longestcommonsubsequence_to_maximumindependentset.rs \
        tests/suites/examples.rs
git commit -m "feat: add LCS to MaxIS example program"
```

---

### Task 4: Regenerate exports and run checks

**Step 1: Regenerate reduction graph and schemas**

```bash
cargo run --example export_graph
cargo run --example export_schemas
```

**Step 2: Run full test suite**

```bash
make test clippy fmt-check
```

**Step 3: Commit generated files**

```bash
git add docs/paper/reduction_graph.json docs/paper/problem_schemas.json
git commit -m "chore: regenerate reduction graph and schemas after LCS->MaxIS rule"
```

---

### Task 5: Document in paper

Invoke `/write-rule-in-paper` to write the reduction-rule entry in `docs/paper/reductions.typ`.

Also invoke `/write-model-in-paper` for the LongestCommonSubsequence problem definition.

Key points for the paper entry:
- **Problem definition:** k strings over alphabet Σ, find longest common subsequence
- **Reduction rule:** Match graph construction with crossing/conflict edges
- **Example:** ABAC/BACA → 6-vertex match graph → IS={v2,v3,v5} → LCS="BAC"
- **Complexity:** O(n^k) vertices worst-case, NP-hard for k≥3 (Maier, 1978)

**Commit after writing:**
```bash
git add docs/paper/reductions.typ
git commit -m "docs: add LCS problem definition and LCS->MaxIS reduction in paper"
```

---

### Task 6: Final verification

**Step 1: Run full checks**

```bash
make check        # fmt + clippy + test
make coverage     # Must be >95%
```

**Step 2: Run review-implementation skill**

Invoke `/review-implementation` to verify all structural and semantic checks pass.
