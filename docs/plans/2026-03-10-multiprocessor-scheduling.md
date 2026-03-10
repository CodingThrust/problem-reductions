# Plan: [Model] MultiprocessorScheduling (#212)

## Problem Summary

**Name:** `MultiprocessorScheduling`
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS8
**Category:** `misc/` (scheduling input: list of processing times + number of machines)

### Mathematical Definition

INSTANCE: Set T of tasks, number m of processors, length l(t) for each t in T, and a deadline D.
QUESTION: Is there an assignment of tasks to processors such that the total load on each processor does not exceed D?

Equivalently: given n jobs with integer processing times and m identical parallel machines, assign each job to a machine such that for every machine i, the sum of processing times of jobs assigned to i is at most D.

### Problem Type

**Satisfaction problem** (`Metric = bool`, implements `SatisfactionProblem`).

The issue defines this as a decision problem: "Is there an m-processor schedule for T that meets the overall deadline D?" A configuration is feasible (true) iff for every processor, the total load does not exceed D.

### Variables

- **Count:** n = |T| (one variable per task)
- **Per-variable domain:** {0, 1, ..., m-1} -- the processor index assigned to the task
- **dims():** `vec![num_processors; num_tasks]`

### Evaluation

```
evaluate(config):
  for each processor i in 0..m:
    load_i = sum of lengths[j] for all j where config[j] == i
    if load_i > deadline: return false
  return true
```

### Struct Fields

| Field            | Type       | Description                         |
|------------------|------------|-------------------------------------|
| `lengths`        | `Vec<u64>` | Processing time l(t) for each task  |
| `num_processors` | `u64`      | Number of identical processors m    |
| `deadline`       | `u64`      | Global deadline D                   |

### Getter Methods (for overhead system)

- `num_tasks() -> usize` -- returns `lengths.len()`
- `num_processors() -> u64` -- returns `self.num_processors`

### Complexity

- For general m (part of input): strongly NP-hard. No known improvement over O*(m^n) brute-force enumeration.
- For fixed m: weakly NP-hard, solvable by pseudo-polynomial DP in O(n * D^(m-1)).
- Complexity string: `"num_processors ^ num_tasks"` (general case brute force)
- References: Garey & Johnson 1979; Lenstra, Rinnooy Kan & Brucker, *Annals of Discrete Mathematics*, 1977.

### Solving Strategy

- BruteForce: enumerate all m^n assignments, check if max load <= D.
- ILP: binary variables x_{t,i}, constraints sum_i x_{t,i} = 1, sum_t x_{t,i} * l(t) <= D.

### Example Instance

T = {t1, t2, t3, t4, t5}, lengths = [4, 5, 3, 2, 6], m = 2, D = 10.
Feasible assignment: config = [0, 1, 1, 1, 0] (processor 0 gets {t1, t5} load=10, processor 1 gets {t2, t3, t4} load=10).
Answer: true.

---

## Implementation Steps

### Step 1: Category

`misc/` -- scheduling input does not fit graph, formula, set, or algebraic categories.

### Step 2: Implement the model

Create `src/models/misc/multiprocessor_scheduling.rs`:

1. `inventory::submit!` for `ProblemSchemaEntry` with fields: `lengths`, `num_processors`, `deadline`
2. Struct `MultiprocessorScheduling` with `#[derive(Debug, Clone, Serialize, Deserialize)]`
3. Constructor `new(lengths: Vec<u64>, num_processors: u64, deadline: u64) -> Self`
4. Accessors: `lengths()`, `num_processors()`, `deadline()`, `num_tasks()`
5. `Problem` impl:
   - `NAME = "MultiprocessorScheduling"`
   - `Metric = bool`
   - `variant() -> crate::variant_params![]` (no type parameters)
   - `dims() -> vec![self.num_processors as usize; self.num_tasks()]`
   - `evaluate()`: compute load per processor, return true iff all loads <= deadline
6. `SatisfactionProblem` impl (marker trait)
7. `declare_variants! { MultiprocessorScheduling => "num_processors ^ num_tasks" }`
8. `#[cfg(test)] #[path = "..."] mod tests;`

### Step 2.5: Register variant complexity

```rust
crate::declare_variants! {
    MultiprocessorScheduling => "num_processors ^ num_tasks",
}
```

### Step 3: Register the model

1. `src/models/misc/mod.rs`: add `mod multiprocessor_scheduling;` and `pub use multiprocessor_scheduling::MultiprocessorScheduling;`
2. `src/models/mod.rs`: add to the misc re-export line

### Step 4: Register in CLI

1. `problemreductions-cli/src/dispatch.rs`:
   - `load_problem()`: add match arm `"MultiprocessorScheduling" => deser_sat::<MultiprocessorScheduling>(json)`
   - `serialize_any_problem()`: add match arm `"MultiprocessorScheduling" => try_ser::<MultiprocessorScheduling>(json)`
2. `problemreductions-cli/src/problem_name.rs`:
   - `resolve_alias()`: add `"multiprocessorscheduling" => "MultiprocessorScheduling".to_string()`
   - No short alias (no well-established abbreviation in the literature)

### Step 4.5: Add CLI creation support

Add match arm in `problemreductions-cli/src/commands/create.rs` for `"MultiprocessorScheduling"`:
- Parse `--lengths`, `--num-processors`, `--deadline` flags
- Add required flags to `cli.rs` `CreateArgs` if not already present
- Update help text

### Step 5: Write unit tests

Create `src/unit_tests/models/misc/multiprocessor_scheduling.rs`:

- `test_multiprocessor_scheduling_creation`: construct instance, verify dims = [2, 2, 2, 2, 2] for 5 tasks, 2 processors
- `test_multiprocessor_scheduling_evaluation`: test feasible (true) and infeasible (false) configs
- `test_multiprocessor_scheduling_serialization`: round-trip serde
- `test_multiprocessor_scheduling_solver`: brute-force finds a satisfying assignment for the example

### Step 6: Document in paper

Invoke `/write-model-in-paper` to add `#problem-def("MultiprocessorScheduling")` to `docs/paper/reductions.typ`:
- Add `"MultiprocessorScheduling": [Multiprocessor Scheduling]` to `display-name` dict
- Formal definition from Garey & Johnson
- Example with visualization of the 5-task, 2-processor instance
- Reference: Garey & Johnson 1979, Lenstra et al. 1977

### Step 7: Verify

```bash
make test clippy
```

Run `/review-implementation` to verify structural and semantic checks.
