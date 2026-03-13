# Plan: Add ShortestCommonSupersequence Model (#412)

## Overview
Add the Shortest Common Supersequence (SCS) satisfaction problem to the codebase. SCS asks: given a set of strings R over alphabet Sigma and a bound K, does there exist a string w with |w| <= K such that every string in R is a subsequence of w?

## Information Checklist
| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `ShortestCommonSupersequence` |
| 2 | Mathematical definition | Given alphabet Sigma, strings R, bound K: does w exist with \|w\| <= K containing all strings in R as subsequences? |
| 3 | Problem type | Satisfaction (decision) |
| 4 | Type parameters | None |
| 5 | Struct fields | `alphabet_size: usize`, `strings: Vec<Vec<usize>>`, `bound: usize` |
| 6 | Configuration space | `vec![alphabet_size; bound]` — each position picks an alphabet symbol |
| 7 | Feasibility check | Each string in R must be a subsequence of the constructed supersequence w |
| 8 | Objective function | `bool` — true iff all strings are subsequences of w |
| 9 | Best known exact | Brute force O(alphabet_size^bound); for 2 strings O(\|x1\|*\|x2\|) via LCS duality |
| 10 | Solving strategy | BruteForce (enumerate all strings of length bound over alphabet) |
| 11 | Category | `misc` (string sequences, unique input structure) |

## Size Getters
- `alphabet_size()` -> usize (|Sigma|)
- `num_strings()` -> usize (|R|)
- `bound()` -> usize (K)
- `total_length()` -> usize (sum of |x| for x in R)

## Complexity String
`"alphabet_size ^ bound"` — brute-force enumeration of all candidate strings.

## Steps

### Step 1: Implement the model
**File:** `src/models/misc/shortest_common_supersequence.rs`

Following `SubsetSum` as the reference satisfaction problem:
1. `inventory::submit!` with ProblemSchemaEntry (fields: `alphabet_size`, `strings`, `bound`)
2. Struct with `#[derive(Debug, Clone, Serialize, Deserialize)]`
3. Constructor `new(alphabet_size: usize, strings: Vec<Vec<usize>>, bound: usize)`
4. Accessor methods: `alphabet_size()`, `strings()`, `bound()`, `num_strings()`, `total_length()`
5. `Problem` trait: NAME = "ShortestCommonSupersequence", Metric = bool
6. `dims()` returns `vec![alphabet_size; bound]`
7. `evaluate()`: construct w from config, check each string in R is a subsequence of w
8. `SatisfactionProblem` marker trait impl
9. `declare_variants! { ShortestCommonSupersequence => "alphabet_size ^ bound" }`
10. `#[cfg(test)] #[path]` link to test file

**Subsequence check logic:** For each string s in R, greedily match characters left-to-right in w. If all characters of s are matched, s is a subsequence. All strings must be subsequences.

**Config interpretation:** config[i] is the alphabet index at position i of w. If config[i] >= alphabet_size, evaluate returns false (out-of-range).

### Step 2: Register the model
1. `src/models/misc/mod.rs` — add `pub(crate) mod shortest_common_supersequence;` and `pub use shortest_common_supersequence::ShortestCommonSupersequence;`
2. `src/models/mod.rs` — add `ShortestCommonSupersequence` to misc re-export

### Step 3: Register in CLI
1. `problemreductions-cli/src/dispatch.rs`:
   - `load_problem()`: add `"ShortestCommonSupersequence" => deser_sat::<ShortestCommonSupersequence>(data)`
   - `serialize_any_problem()`: add `"ShortestCommonSupersequence" => try_ser::<ShortestCommonSupersequence>(any)`
2. `problemreductions-cli/src/problem_name.rs`:
   - `resolve_alias()`: add `"shortestcommonsupersequence" => "ShortestCommonSupersequence"` and `"scs" => "ShortestCommonSupersequence"`
3. `problemreductions-cli/src/commands/create.rs`:
   - Add match arm for "ShortestCommonSupersequence" that parses `--strings` (semicolon-separated sequences of alphabet indices) and `--bound`
4. `problemreductions-cli/src/cli.rs`:
   - Add `--strings` flag if not already present, update `all_data_flags_empty()` and help table

### Step 4: Write unit tests
**File:** `src/unit_tests/models/misc/shortest_common_supersequence.rs`

Tests:
- `test_shortestcommonsupersequence_basic` — construct instance, verify dims
- `test_shortestcommonsupersequence_evaluate_yes` — test satisfying config (from issue example 1)
- `test_shortestcommonsupersequence_evaluate_no` — test unsatisfying config
- `test_shortestcommonsupersequence_out_of_range` — config values >= alphabet_size return false
- `test_shortestcommonsupersequence_wrong_length` — wrong config length returns false
- `test_shortestcommonsupersequence_brute_force` — BruteForce solver finds solution for small instance
- `test_shortestcommonsupersequence_serialization` — serde round-trip

### Step 5: Document in paper
Invoke `/write-model-in-paper` for ShortestCommonSupersequence.

### Step 6: Verify
Run `make check` (fmt + clippy + test). Fix any issues.
