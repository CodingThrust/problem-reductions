# Plan: Add ResourceConstrainedScheduling Model

**Issue:** #502 — [Model] ResourceConstrainedScheduling
**Skill:** add-model
**Date:** 2026-03-13

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `ResourceConstrainedScheduling` |
| 2 | Mathematical definition | Given unit-length tasks T, m processors, r resources with bounds B_i, resource requirements R_i(t), and deadline D, find a schedule σ assigning each task to a time slot in {0,...,D-1} such that at most m tasks run per slot and resource usage per slot does not exceed any bound B_i |
| 3 | Problem type | Satisfaction (decision: does a feasible schedule exist?) |
| 4 | Type parameters | None |
| 5 | Struct fields | `num_tasks: usize`, `num_processors: usize`, `resource_bounds: Vec<u64>`, `resource_requirements: Vec<Vec<u64>>` (n×r), `deadline: u64` |
| 6 | Configuration space | `vec![deadline as usize; num_tasks]` — each task assigned to a time slot |
| 7 | Feasibility check | For each time slot u, count tasks scheduled there ≤ m, and for each resource i, sum of R_i(t) for tasks at u ≤ B_i |
| 8 | Objective function | bool — true if schedule is feasible |
| 9 | Best known exact algorithm | O*(D^n) brute-force enumeration (strongly NP-complete) |
| 10 | Solving strategy | BruteForce (enumerate all D^n assignments) |
| 11 | Category | `misc` (unique input structure: tasks + processors + resources + deadline) |

## Steps

### Step 1: Create model file
- Create `src/models/misc/resource_constrained_scheduling.rs`
- Implement struct with `inventory::submit!` for `ProblemSchemaEntry`
- Constructor: `new(num_processors, resource_bounds, resource_requirements, deadline)`
  - Derive `num_tasks` from `resource_requirements.len()`
- Accessor methods: `num_tasks()`, `num_processors()`, `resource_bounds()`, `resource_requirements()`, `deadline()`, `num_resources()`
- Implement `Problem` trait with `Metric = bool`, `SatisfactionProblem`
- `dims()` returns `vec![deadline as usize; num_tasks]`
- `evaluate()` checks: config length, values in range, processor capacity per slot, resource bounds per slot
- `variant()` returns `crate::variant_params![]` (no type parameters)
- `declare_variants!` with complexity `"deadline ^ num_tasks"`

### Step 2: Register the model
- Add to `src/models/misc/mod.rs`
- Add to `src/models/mod.rs` re-exports

### Step 3: Register in CLI
- Add match arm in `dispatch.rs` `load_problem()` using `deser_sat`
- Add match arm in `dispatch.rs` `serialize_any_problem()` using `try_ser`
- Add lowercase alias in `problem_name.rs` `resolve_alias()`
- Add creation handler in `commands/create.rs` (parse `--num-processors`, `--resource-bounds`, `--resource-requirements`, `--deadline` flags)
- Add CLI flags in `cli.rs` (`CreateArgs` struct)
- Update help text in `cli.rs` `after_help`
- Update `all_data_flags_empty()` in `create.rs`

### Step 4: Write unit tests
- Create `src/unit_tests/models/misc/resource_constrained_scheduling.rs`
- Tests: creation, evaluation (valid/invalid), solver, serialization, variant, edge cases

### Step 5: Document in paper
- Add `problem-def("ResourceConstrainedScheduling")` to `docs/paper/reductions.typ`
- Add display name mapping

### Step 6: Verify
- `make test clippy`
