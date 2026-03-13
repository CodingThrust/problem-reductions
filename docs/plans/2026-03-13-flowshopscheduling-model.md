# Plan: Add FlowShopScheduling Model (#507)

## Summary
Add the `FlowShopScheduling` satisfaction problem model — a classic NP-complete scheduling problem from Garey & Johnson (A5 SS15). Given m processors and n jobs (each with m tasks in fixed processor order), determine if all jobs can complete by deadline D.

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `FlowShopScheduling` |
| 2 | Mathematical definition | Given m processors, n jobs each with m tasks of specified lengths, and deadline D, determine if a flow-shop schedule exists meeting D |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `num_processors: usize`, `task_lengths: Vec<Vec<u64>>`, `deadline: u64` |
| 6 | Configuration space | `vec![num_jobs; num_jobs]` — each variable is a permutation position {0..n-1} |
| 7 | Feasibility check | Config must be a valid permutation of jobs; then compute schedule greedily and check makespan <= deadline |
| 8 | Objective function | `bool` — true iff valid permutation schedule meets deadline |
| 9 | Best known exact | For general m: `num_jobs! * num_processors * num_jobs` (brute-force over permutations). Use `"num_jobs! * num_processors * num_jobs"` but since factorial isn't supported in Expr, use a conservative bound. The issue mentions O*(3^n) for m=3 specifically. For general case, use brute-force enumeration. |
| 10 | Solving strategy | BruteForce (enumerate all permutation configs, check feasibility) |
| 11 | Category | `misc` (unique scheduling input structure) |

## Steps

### Step 1: Create model file `src/models/misc/flow_shop_scheduling.rs`

- `FlowShopScheduling` struct with fields: `num_processors`, `task_lengths`, `deadline`
- Constructor `new(num_processors, task_lengths, deadline)` with validation (each job must have exactly m tasks)
- Getters: `num_processors()`, `task_lengths()`, `deadline()`, `num_jobs()`
- `compute_makespan(job_order: &[usize]) -> u64` helper that computes the flow-shop makespan given a job sequence
- `Problem` impl with `Metric = bool`:
  - `NAME = "FlowShopScheduling"`
  - `dims()` returns `vec![num_jobs; num_jobs]` (permutation encoding)
  - `evaluate()`: validate config is a permutation, compute makespan, return makespan <= deadline
  - `variant()` returns `crate::variant_params![]` (no type params)
- `SatisfactionProblem` marker impl
- `declare_variants!` with complexity `"3^num_jobs"` (conservative bound from the O*(3^n) DP for m=3)
- `inventory::submit!` for `ProblemSchemaEntry`
- `#[cfg(test)] #[path]` link to unit tests

### Step 2: Register the model

- `src/models/misc/mod.rs`: add `mod flow_shop_scheduling;` and `pub use flow_shop_scheduling::FlowShopScheduling;`
- `src/models/mod.rs`: add `FlowShopScheduling` to the `pub use misc::` line
- `src/lib.rs`: add `FlowShopScheduling` to the prelude re-export

### Step 3: Register in CLI

- `problemreductions-cli/src/dispatch.rs`:
  - `load_problem()`: add `"FlowShopScheduling" => deser_sat::<FlowShopScheduling>(data)`
  - `serialize_any_problem()`: add `"FlowShopScheduling" => try_ser::<FlowShopScheduling>(any)`
  - Add import for `FlowShopScheduling`
- `problemreductions-cli/src/problem_name.rs`:
  - Add `"flowshopscheduling" => "FlowShopScheduling".to_string()` to `resolve_alias()`
- `problemreductions-cli/src/commands/create.rs`:
  - Add creation handler for FlowShopScheduling using `--task-lengths` (semicolon-separated rows) and `--deadline`
  - Note: `--m` flag already exists (for Factoring), can reuse for num_processors
- `problemreductions-cli/src/cli.rs`:
  - Add `--task-lengths` and `--deadline` flags to `CreateArgs`
  - Update `all_data_flags_empty()` to check new flags
  - Update help table with FlowShopScheduling entry

### Step 4: Write unit tests `src/unit_tests/models/misc/flow_shop_scheduling.rs`

Tests to write:
- `test_flow_shop_scheduling_creation` — construct instance, verify dimensions
- `test_flow_shop_scheduling_evaluate_feasible` — use the issue's example (sequence j4,j1,j5,j3,j2 with makespan 23 <= 25)
- `test_flow_shop_scheduling_evaluate_infeasible` — a sequence exceeding deadline
- `test_flow_shop_scheduling_invalid_config` — non-permutation config returns false
- `test_flow_shop_scheduling_direction` — verify it's a satisfaction problem
- `test_flow_shop_scheduling_serialization` — round-trip serde
- `test_flow_shop_scheduling_solver` — brute force finds a satisfying assignment for small instance
- `test_flow_shop_scheduling_empty` — 0 jobs case
- `test_flow_shop_scheduling_variant` — verify empty variant

Also add `mod flow_shop_scheduling;` to `src/unit_tests/models/misc/mod.rs` (create if needed).

### Step 5: Verify

- `make test` — all tests pass
- `make clippy` — no warnings
- `make fmt` — code formatted
