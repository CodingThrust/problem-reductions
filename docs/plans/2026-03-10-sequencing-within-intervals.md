# Plan: Add SequencingWithinIntervals Model

**Issue:** #219 — [Model] SequencingWithinIntervals
**Skill:** add-model (Steps 1–7)

## Overview

Add a `SequencingWithinIntervals` satisfaction model: given n tasks, each with a release time r(t), deadline d(t), and processing length l(t), determine whether all tasks can be scheduled non-overlappingly such that each task runs entirely within its allowed time window. This is problem SS1 from Garey & Johnson, NP-complete via Theorem 3.8 (reduction from PARTITION).

## Design Decisions

- **Category:** `misc/` — scheduling input (list of tasks with release times, deadlines, lengths); does not fit `graph/`, `set/`, `algebraic/`, or `formula/`.
- **Struct:** `SequencingWithinIntervals` with fields `release_times: Vec<u64>`, `deadlines: Vec<u64>`, `lengths: Vec<u64>`. No type parameters (all times are plain non-negative integers).
- **Problem type:** Satisfaction (`Metric = bool`, implements `SatisfactionProblem`).
- **dims():** For each task i, the number of valid start times is `d(i) - r(i) - l(i) + 1` (range `[r(i), d(i) - l(i)]`). So `dims()` returns `vec![d[i] - r[i] - l[i] + 1 for each i]` (as `usize`). Each variable selects an index into the valid start time range for that task.
- **evaluate():** Map each variable index back to an actual start time: `start_i = r[i] + config[i]`. Check: (1) each task finishes before deadline (`start_i + l[i] <= d[i]`), and (2) no two tasks overlap. Return `true` if feasible, `false` otherwise.
- **Constructor precondition:** Assert `r[i] + l[i] <= d[i]` for every task (otherwise domain is empty).
- **variant():** `variant_params![]` — no type parameters.
- **Getters:** `num_tasks()` (for complexity expression variable).
- **Complexity:** `2^num_tasks` — NP-complete, brute-force over orderings is O(n! * n) but the configuration space enumeration is exponential in n. The best known exact algorithms remain exponential.
- **Solver:** BruteForce (existing) — enumerates all configurations.

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `SequencingWithinIntervals` |
| 2 | Mathematical definition | Given tasks T with release times r(t), deadlines d(t), lengths l(t), find schedule sigma: T -> Z_>=0 s.t. sigma(t) >= r(t), sigma(t)+l(t) <= d(t), and no two tasks overlap |
| 3 | Problem type | Satisfaction (bool) |
| 4 | Type parameters | None |
| 5 | Struct fields | `release_times: Vec<u64>`, `deadlines: Vec<u64>`, `lengths: Vec<u64>` |
| 6 | Configuration space | `dims[i] = d[i] - r[i] - l[i] + 1` (number of valid start times per task) |
| 7 | Feasibility check | All tasks within window + no pairwise overlap |
| 8 | Objective function | N/A (satisfaction: returns bool) |
| 9 | Best known exact algorithm | NP-complete (GJ Thm 3.8, 1979). O*(2^n) brute force. |
| 10 | Solving strategy | BruteForce works; ILP reduction possible |
| 11 | Category | `misc/` |

## Steps

### Step 1: Determine category
Category: `misc/` — scheduling problem with unique input structure (task list with time windows).

### Step 1.5: Infer problem size getters
From complexity `O*(2^n)` where n = |T|:
- `num_tasks()` -> number of tasks

### Step 2: Implement the model
Create `src/models/misc/sequencing_within_intervals.rs`:

```rust
// Structure:
// 1. inventory::submit! for ProblemSchemaEntry
// 2. SequencingWithinIntervals struct with release_times, deadlines, lengths (all Vec<u64>)
// 3. Constructor: new(release_times, deadlines, lengths) — panics if r[i]+l[i] > d[i] or lengths mismatch
// 4. Accessors: release_times(), deadlines(), lengths(), num_tasks()
// 5. Problem impl: NAME="SequencingWithinIntervals", Metric=bool, dims(), evaluate()
// 6. SatisfactionProblem impl (marker trait)
// 7. declare_variants!
// 8. #[cfg(test)] #[path] link
```

Key implementation details for `evaluate()`:
```
1. For each task i, compute start_time = release_times[i] + config[i]
2. Check start_time + lengths[i] <= deadlines[i] (should always hold if dims is correct)
3. For all pairs (i, j), check non-overlap:
   either start_i + l_i <= start_j OR start_j + l_j <= start_i
4. Return true iff all constraints satisfied
```

### Step 2.5: Register variant complexity
```rust
crate::declare_variants! {
    SequencingWithinIntervals => "2^num_tasks",
}
```

### Step 3: Register the model
1. `src/models/misc/mod.rs` — add `mod sequencing_within_intervals;` and `pub use sequencing_within_intervals::SequencingWithinIntervals;`
2. `src/models/mod.rs` — add `SequencingWithinIntervals` to the `misc` re-export line

### Step 4: Register in CLI
1. `problemreductions-cli/src/dispatch.rs`:
   - `load_problem()`: add `"SequencingWithinIntervals" => deser_sat::<SequencingWithinIntervals>(data)`
   - `serialize_any_problem()`: add `"SequencingWithinIntervals" => try_ser::<SequencingWithinIntervals>(any)`
2. `problemreductions-cli/src/problem_name.rs`:
   - `resolve_alias()`: add `"sequencingwithinintervals" => "SequencingWithinIntervals".to_string()`
   - No short alias — no well-established abbreviation in the literature

### Step 4.5: Add CLI creation support
Add a match arm in `commands/create.rs` for `"SequencingWithinIntervals"` that parses:
- `--release-times` (or reuse an appropriate flag)
- `--deadlines`
- `--lengths`

Add any needed CLI flags in `cli.rs` (`CreateArgs`).

### Step 5: Write unit tests
Create `src/unit_tests/models/misc/sequencing_within_intervals.rs`:

Tests:
- `test_sequencingwithinintervals_creation` — construct instance, verify num_tasks, dims
- `test_sequencingwithinintervals_evaluation_feasible` — valid schedule returns true
- `test_sequencingwithinintervals_evaluation_infeasible` — overlapping schedule returns false
- `test_sequencingwithinintervals_solver` — BruteForce finds a satisfying assignment for the example
- `test_sequencingwithinintervals_serialization` — round-trip serde test
- `test_sequencingwithinintervals_no_solution` — instance with no feasible schedule returns None from solver

Example instance from issue (PARTITION reduction):
- 5 tasks: release_times = [0, 0, 0, 0, 5], deadlines = [11, 11, 11, 11, 6], lengths = [3, 1, 2, 4, 1]
- Feasible schedule: sigma = [0, 3, 3, 7, 5] -> starts at [0, 3, 3, 7, 5]
  Wait — need to recheck. Config values are offsets from release time.
  - Task 0: r=0, d=11, l=3 -> valid starts: 0..=8, dims=9, config=0 -> start=0, runs [0,3)
  - Task 1: r=0, d=11, l=1 -> valid starts: 0..=10, dims=11, config=6 -> start=6, runs [6,7)
  - Task 2: r=0, d=11, l=2 -> valid starts: 0..=9, dims=10, config=3 -> start=3, runs [3,5)
  - Task 3: r=0, d=11, l=4 -> valid starts: 0..=7, dims=8, config=7 -> start=7, runs [7,11)
  - Task 4: r=5, d=6, l=1 -> valid starts: 5..=5, dims=1, config=0 -> start=5, runs [5,6)
  - No overlaps. Feasible.

### Step 6: Document in paper
Invoke `/write-model-in-paper` to add:
1. `display-name` entry: `"SequencingWithinIntervals": [Sequencing Within Intervals]`
2. `#problem-def("SequencingWithinIntervals")[...]` with formal definition from GJ

### Step 7: Verify
```bash
make check  # fmt + clippy + test
```
Then run `/review-implementation` to verify completeness.

## Files Changed

| File | Action |
|------|--------|
| `src/models/misc/sequencing_within_intervals.rs` | **Create** — model implementation |
| `src/unit_tests/models/misc/sequencing_within_intervals.rs` | **Create** — unit tests |
| `src/models/misc/mod.rs` | **Edit** — register module |
| `src/models/mod.rs` | **Edit** — add re-export |
| `problemreductions-cli/src/dispatch.rs` | **Edit** — CLI dispatch |
| `problemreductions-cli/src/problem_name.rs` | **Edit** — alias |
| `problemreductions-cli/src/commands/create.rs` | **Edit** — CLI create support |
| `problemreductions-cli/src/cli.rs` | **Edit** — CLI flags (if needed) |
| `docs/paper/reductions.typ` | **Edit** — paper definition |
