# ConsistencyOfDatabaseFrequencyTables Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `ConsistencyOfDatabaseFrequencyTables` satisfaction model, its canonical example/docs/CLI support, and an ILP reduction so the problem is discoverable, testable, and solvable through the existing pipeline.

**Architecture:** Model the problem as one categorical variable per `(object, attribute)` pair, with per-variable domain sizes taken from the attribute domains. Keep the core model in `src/models/misc/consistency_of_database_frequency_tables.rs`, register it through the normal schema/variant/example-db paths, then add a binary `ILP<bool>` reduction using one-hot assignment variables plus McCormick-linearized pair-count constraints for each published frequency table. Prefer explicit helper structs for frequency tables and known values so the JSON schema, CLI parsing, and paper example stay readable.

**Tech Stack:** Rust workspace, serde, inventory schema registry, reduction macros, example-db exporters, Typst paper, existing CLI `pred create` infrastructure.

---

### Task 1: Add the model with TDD

**Files:**
- Create: `src/models/misc/consistency_of_database_frequency_tables.rs`
- Create: `src/unit_tests/models/misc/consistency_of_database_frequency_tables.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing model tests**

Add tests in `src/unit_tests/models/misc/consistency_of_database_frequency_tables.rs` for:
- constructor/getters/schema-facing shape on the issue’s YES instance
- `dims()` returning one variable per `(object, attribute)` with the correct per-attribute domain sizes
- `evaluate()` accepting the flattened YES witness from the issue
- `evaluate()` rejecting wrong config length, out-of-range values, conflicting known values, and table-mismatch assignments
- `BruteForce::find_satisfying()` finding a satisfying assignment for the YES instance
- `BruteForce::find_satisfying()` returning `None` on a minimally inconsistent instance
- serde round-trip

Use the issue’s worked example as the paper/example-db source of truth. Flatten the witness object-major:
`[0,0,0, 0,1,1, 0,2,1, 1,0,1, 1,1,1, 1,2,0]`.

**Step 2: Run the new test file and confirm RED**

Run:
```bash
cargo test test_consistency_of_database_frequency_tables -- --nocapture
```

Expected: compile failure because the model does not exist yet.

**Step 3: Implement the model minimally**

Create `src/models/misc/consistency_of_database_frequency_tables.rs` with:
- `ProblemSchemaEntry` metadata
- explicit serializable helper structs for frequency tables / known values if that makes JSON and docs clearer
- constructor validation:
  - `attribute_domains` non-empty and every domain size positive
  - frequency-table attribute indices distinct and in range
  - each table has row count `attribute_domains[a]`, each row has column count `attribute_domains[b]`
  - each table sum equals `num_objects`
  - known values are in range
- getters needed for complexity / overhead:
  - `num_objects()`
  - `num_attributes()`
  - `domain_size_product()` or equivalent exact brute-force complexity helper
  - any extra count getters needed by the ILP reduction overhead
- `dims()` returning `attribute_domains` repeated once per object
- `evaluate()` that:
  - rejects malformed configs
  - checks all known values
  - counts each published pair table exactly
- `SatisfactionProblem` impl
- `declare_variants!` with an exact multi-domain brute-force expression such as `domain_size_product^num_objects` rather than the issue’s binary-only shorthand if needed for correctness
- canonical model example spec using the issue’s YES instance

Register the model in the misc/module/prelude exports.

**Step 4: Run the model tests and get GREEN**

Run:
```bash
cargo test test_consistency_of_database_frequency_tables -- --nocapture
```

Expected: the new model tests pass.

**Step 5: Refactor only after green**

Extract small helpers only if they reduce duplication in evaluation/validation, then re-run the same test command.

### Task 2: Add CLI and example-db support with TDD

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `src/example_db/model_builders.rs`

**Step 1: Write the failing CLI / example-db tests**

Add or extend the most local existing tests for:
- alias/discovery/help exposure if covered in this repo’s CLI tests
- `pred create ConsistencyOfDatabaseFrequencyTables ...` producing the expected JSON payload
- `pred create --example ConsistencyOfDatabaseFrequencyTables` using the canonical example

If no focused CLI unit test file exists yet for `create`, add the smallest relevant coverage next to current create-command tests instead of inventing a new harness.

**Step 2: Run the focused tests and confirm RED**

Run the narrowest affected command, for example:
```bash
cargo test create:: --package problemreductions-cli -- --nocapture
```

Expected: failure because the problem is not yet wired into CLI construction/help.

**Step 3: Implement minimal CLI support**

Add create-flag support using explicit, parseable fields:
- `--num-objects 6`
- `--attribute-domains "2,3,2"`
- `--frequency-tables "0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1"`
- `--known-values "0,0,0;3,0,1;1,2,1"` (optional)

Update:
- `CreateArgs` with the new fields
- `all_data_flags_empty()`
- help tables / usage examples / problem-specific examples
- `create()` match arm to parse and validate the compact table syntax into the model structs

Also ensure `print_problem_help` shows a realistic usage example for this problem.

**Step 4: Run the focused tests and get GREEN**

Repeat the CLI/example-db test command until it passes.

### Task 3: Add the ILP reduction with TDD

**Files:**
- Create: `src/rules/consistencyofdatabasefrequencytables_ilp.rs`
- Create: `src/unit_tests/rules/consistencyofdatabasefrequencytables_ilp.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write the failing reduction tests**

Follow the project’s closed-loop reduction reference and add tests for:
- reduction shape on the issue’s YES example
- ILP feasibility witness extracts back to the original categorical assignment
- a closed-loop test using brute force on the source and brute force / direct feasibility on the reduced ILP
- the inconsistent instance reducing to an infeasible ILP
- canonical rule example registration if needed

**Step 2: Run the reduction tests and confirm RED**

Run:
```bash
cargo test test_consistencyofdatabasefrequencytables_to_ilp --features ilp-solver -- --nocapture
```

Expected: compile failure because the rule does not exist.

**Step 3: Implement the reduction minimally**

Create a binary `ILP<bool>` reduction with:
- assignment variables `y_{v,a,x}` for each object/attribute/value
- auxiliary McCormick variables `z_{t,v,x,y}` for each frequency-table cell and object
- constraints:
  - exactly one value for each `(v,a)`
  - known values fixed to `1`
  - McCormick linearization for every `z`
  - exact frequency-table counts via `sum_v z = f_{a,b}(x,y)`
- zero objective with `ObjectiveSense::Minimize`
- solution extraction that reads the chosen `y_{v,a,x}` variables back into the flattened source config
- reduction overhead expressed via concrete getters added in Task 1
- canonical rule example based on the same issue example

Register the rule in `src/rules/mod.rs` under `#[cfg(feature = "ilp-solver")]`.

**Step 4: Run the reduction tests and get GREEN**

Run the focused reduction test command again until it passes.

### Task 4: Add paper coverage and final verification

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add the paper entry after code is green**

Add:
- `display-name` entry for `ConsistencyOfDatabaseFrequencyTables`
- `problem-def("ConsistencyOfDatabaseFrequencyTables")` with the formal database-frequency-table definition
- short background and cited complexity discussion
- the issue’s worked example in tutorial style, including the satisfying assignment and explicit verification narrative

Use prose and a table-based presentation rather than forcing a graph-style figure.

**Step 2: Run paper verification**

Run:
```bash
make paper
```

Expected: clean Typst build.

**Step 3: Run fresh repo verification before commit/push**

Run:
```bash
make test
make clippy
```

If the ILP reduction or paper/export paths require additional targeted commands, run them too and record the exact outcome before claiming completion.

**Step 4: Review issue compliance and cleanup**

Confirm before the implementation summary comment / push:
- the canonical example matches the issue’s Expected Outcome
- the model is exported/discoverable
- the ILP reduction exists so `pred solve` has a path
- the plan file is removed in the cleanup commit
