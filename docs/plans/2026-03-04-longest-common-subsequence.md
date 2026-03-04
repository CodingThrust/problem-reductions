# LongestCommonSubsequence Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `LongestCommonSubsequence` problem model — given k strings over an alphabet, find the longest string that is a subsequence of every input string.

**Architecture:** New model in `src/models/misc/` with no type parameters. Configuration space is binary selection over characters of the shortest string (dims = `vec![2; m]` where m = shortest string length). Feasibility checks that the selected subsequence of the shortest string is also a subsequence of all other strings.

**Tech Stack:** Rust, serde, inventory (schema registration)

---

### Task 1: Create the model file with struct and schema registration

**Files:**
- Create: `src/models/misc/longest_common_subsequence.rs`

**Step 1: Write the full model file**

```rust
//! Longest Common Subsequence problem implementation.
//!
//! Given k strings over an alphabet, find the longest string that is a
//! subsequence of every input string. NP-hard for variable k (Maier, 1978).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCommonSubsequence",
        module_path: module_path!(),
        description: "Find the longest string that is a subsequence of every input string",
        fields: &[
            FieldInfo { name: "strings", type_name: "Vec<Vec<u8>>", description: "The input strings" },
        ],
    }
}

/// The Longest Common Subsequence problem.
///
/// Given `k` strings `s_1, ..., s_k` over an alphabet, find a longest
/// string `w` that is a subsequence of every `s_i`.
///
/// A string `w` is a **subsequence** of `s` if `w` can be obtained by
/// deleting zero or more characters from `s` without changing the order
/// of the remaining characters.
///
/// # Representation
///
/// Configuration is binary selection over the characters of the shortest
/// string. Each variable in `{0, 1}` indicates whether the corresponding
/// character of the shortest string is included in the candidate subsequence.
/// The candidate is valid if the resulting subsequence is also a subsequence
/// of every other input string.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::LongestCommonSubsequence;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = LongestCommonSubsequence::new(vec![
///     vec![b'A', b'B', b'C', b'D', b'A', b'B'],
///     vec![b'B', b'D', b'C', b'A', b'B', b'A'],
///     vec![b'B', b'C', b'A', b'D', b'B', b'A'],
/// ]);
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    /// The input strings.
    strings: Vec<Vec<u8>>,
}

impl LongestCommonSubsequence {
    /// Create a new LCS problem from a list of strings.
    ///
    /// # Panics
    ///
    /// Panics if `strings` is empty.
    pub fn new(strings: Vec<Vec<u8>>) -> Self {
        assert!(!strings.is_empty(), "must have at least one string");
        Self { strings }
    }

    /// Get the input strings.
    pub fn strings(&self) -> &[Vec<u8>] {
        &self.strings
    }

    /// Get the number of input strings.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Get the total length of all input strings.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Index of the shortest string.
    fn shortest_index(&self) -> usize {
        self.strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Length of the shortest string.
    fn shortest_len(&self) -> usize {
        self.strings.iter().map(|s| s.len()).min().unwrap_or(0)
    }
}

impl Problem for LongestCommonSubsequence {
    const NAME: &'static str = "LongestCommonSubsequence";
    type Metric = SolutionSize<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.shortest_len()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        let si = self.shortest_index();
        let shortest = &self.strings[si];
        if config.len() != shortest.len() {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v > 1) {
            return SolutionSize::Invalid;
        }
        // Build the candidate subsequence from selected characters
        let candidate: Vec<u8> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| shortest[i])
            .collect();
        // Check that candidate is a subsequence of every other string
        for (j, s) in self.strings.iter().enumerate() {
            if j == si {
                continue;
            }
            if !is_subsequence(&candidate, s) {
                return SolutionSize::Invalid;
            }
        }
        SolutionSize::Valid(candidate.len() as i32)
    }
}

impl OptimizationProblem for LongestCommonSubsequence {
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if `sub` is a subsequence of `full`.
fn is_subsequence(sub: &[u8], full: &[u8]) -> bool {
    let mut it = full.iter();
    for &c in sub {
        if it.find(|&&x| x == c).is_none() {
            return false;
        }
    }
    true
}

crate::declare_variants! {
    LongestCommonSubsequence => "2^total_length",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/longest_common_subsequence.rs"]
mod tests;
```

**Step 2: Commit**

```bash
git add src/models/misc/longest_common_subsequence.rs
git commit -m "feat: add LongestCommonSubsequence model (struct + traits)"
```

---

### Task 2: Register the model in module tree and prelude

**Files:**
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs` (prelude)

**Step 1: Update `src/models/misc/mod.rs`**

Add to the doc comment:
```rust
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence (maximize common subsequence length)
```

Add module declaration and re-export:
```rust
pub(crate) mod longest_common_subsequence;
pub use longest_common_subsequence::LongestCommonSubsequence;
```

**Step 2: Update `src/models/mod.rs` line 18**

Change:
```rust
pub use misc::{BinPacking, Factoring, PaintShop};
```
To:
```rust
pub use misc::{BinPacking, Factoring, LongestCommonSubsequence, PaintShop};
```

**Step 3: Update `src/lib.rs` line 46 prelude**

Change:
```rust
pub use crate::models::misc::{BinPacking, Factoring, PaintShop};
```
To:
```rust
pub use crate::models::misc::{BinPacking, Factoring, LongestCommonSubsequence, PaintShop};
```

**Step 4: Verify compilation**

Run: `cargo build 2>&1 | tail -5`

**Step 5: Commit**

```bash
git add src/models/misc/mod.rs src/models/mod.rs src/lib.rs
git commit -m "feat: register LongestCommonSubsequence in module tree"
```

---

### Task 3: Write unit tests

**Files:**
- Create: `src/unit_tests/models/misc/longest_common_subsequence.rs`

**Step 1: Write the test file**

```rust
use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_lcs_creation() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'C'],
        vec![b'B', b'A', b'C', b'D'],
    ]);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.total_length(), 9);
    // Shortest string is "AC" with length 2
    assert_eq!(problem.dims(), vec![2; 2]);
}

#[test]
fn test_lcs_direction() {
    let problem = LongestCommonSubsequence::new(vec![vec![b'A'], vec![b'A']]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_lcs_evaluate_all_selected() {
    // s1 = "AC", s2 = "ABC" → selecting both chars of s1 gives "AC"
    // "AC" is subsequence of "ABC"? A..C — yes
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'C'],
        vec![b'A', b'B', b'C'],
    ]);
    let result = problem.evaluate(&[1, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_lcs_evaluate_partial_selection() {
    // s1 = "ABC", s2 = "AXC" → select A and C (indices 0, 2)
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'X', b'C'],
    ]);
    // Config [1, 0, 1] selects "AC" — subsequence of both
    let result = problem.evaluate(&[1, 0, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_lcs_evaluate_invalid_not_subsequence() {
    // s1 = "BA", s2 = "AB" → select both chars of s1 gives "BA"
    // "BA" is NOT a subsequence of "AB" (B comes after A in "AB")
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'B', b'A'],
        vec![b'A', b'B'],
    ]);
    let result = problem.evaluate(&[1, 1]);
    assert!(!result.is_valid());
}

#[test]
fn test_lcs_evaluate_empty_selection() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B'],
        vec![b'C', b'D'],
    ]);
    // Select nothing — empty string is always a valid subsequence
    let result = problem.evaluate(&[0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_lcs_evaluate_wrong_config_length() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B'],
        vec![b'A', b'C'],
    ]);
    assert!(!problem.evaluate(&[1]).is_valid());
    assert!(!problem.evaluate(&[1, 0, 1]).is_valid());
}

#[test]
fn test_lcs_problem_name() {
    assert_eq!(LongestCommonSubsequence::NAME, "LongestCommonSubsequence");
}

#[test]
fn test_lcs_variant() {
    let v = <LongestCommonSubsequence as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_lcs_brute_force_issue_example() {
    // Example from issue #108:
    // s1 = "ABCDAB", s2 = "BDCABA", s3 = "BCADBA"
    // Optimal LCS = "BCAB", length 4
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C', b'D', b'A', b'B'],
        vec![b'B', b'D', b'C', b'A', b'B', b'A'],
        vec![b'B', b'C', b'A', b'D', b'B', b'A'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 4);
}

#[test]
fn test_lcs_brute_force_two_strings() {
    // s1 = "ABCBDAB", s2 = "BDCAB" → LCS = "BCAB", length 4
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'B', b'D', b'C', b'A', b'B'],
        vec![b'A', b'B', b'C', b'B', b'D', b'A', b'B'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 4);
}

#[test]
fn test_lcs_single_string() {
    // Single string — LCS is the string itself
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_no_common() {
    // s1 = "AB", s2 = "CD" → LCS = "", length 0
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B'],
        vec![b'C', b'D'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 0);
}

#[test]
fn test_lcs_serialization() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'C'],
    ]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.strings(), problem.strings());
}
```

**Step 2: Run tests**

Run: `cargo test --lib longest_common_subsequence -- --nocapture 2>&1 | tail -20`
Expected: All tests pass.

**Step 3: Commit**

```bash
git add src/unit_tests/models/misc/longest_common_subsequence.rs
git commit -m "test: add LongestCommonSubsequence unit tests"
```

---

### Task 4: Register in CLI dispatch

**Files:**
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/problem_name.rs`

**Step 1: Update `dispatch.rs` — add import**

Add to imports at top:
```rust
use problemreductions::models::misc::LongestCommonSubsequence;
```

**Step 2: Update `dispatch.rs` — `load_problem()` match**

Add after the `"BinPacking"` arm (around line 242):
```rust
"LongestCommonSubsequence" => deser_opt::<LongestCommonSubsequence>(data),
```

**Step 3: Update `dispatch.rs` — `serialize_any_problem()` match**

Add after the `"BinPacking"` arm (around line 301):
```rust
"LongestCommonSubsequence" => try_ser::<LongestCommonSubsequence>(any),
```

**Step 4: Update `problem_name.rs` — ALIASES**

Add to `ALIASES` const:
```rust
("LCS", "LongestCommonSubsequence"),
```

**Step 5: Update `problem_name.rs` — `resolve_alias()`**

Add case:
```rust
"lcs" | "longestcommonsubsequence" => "LongestCommonSubsequence".to_string(),
```

**Step 6: Verify build**

Run: `cargo build --workspace 2>&1 | tail -5`

**Step 7: Commit**

```bash
git add problemreductions-cli/src/dispatch.rs problemreductions-cli/src/problem_name.rs
git commit -m "feat: register LongestCommonSubsequence in CLI dispatch"
```

---

### Task 5: Run full checks

**Step 1: Run formatting, clippy, and tests**

Run: `make fmt && make clippy && make test`
Expected: All pass.

**Step 2: Fix any issues found**

If clippy or tests fail, fix the issues before proceeding.

**Step 3: Commit any fixes**

```bash
git add -u
git commit -m "fix: address clippy/test issues for LongestCommonSubsequence"
```

---

### Task 6: Document in paper

Invoke `/write-model-in-paper` to add the problem-def entry for LongestCommonSubsequence in `docs/paper/reductions.typ`. Include:
- Formal definition referencing Maier (1978) and Garey & Johnson SR10
- Background on 2-string polynomial DP vs k-string NP-hardness
- Example from the issue (3 strings, LCS = "BCAB")
- Algorithm list

---

### Task 7: Final review

Invoke `/review-implementation` to verify all structural and semantic checks pass.
