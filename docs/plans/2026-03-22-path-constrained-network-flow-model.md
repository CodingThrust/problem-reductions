# PathConstrainedNetworkFlow Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `PathConstrainedNetworkFlow` satisfaction model, expose it through registry/CLI/example-db flows, and document the canonical example in the Typst paper for issue #291.

**Architecture:** Implement the model as a graph problem backed by `DirectedGraph`, explicit arc capacities, designated source/sink vertices, and a prescribed collection of s-t paths encoded as arc-index sequences. Reuse the existing satisfaction-problem + brute-force pattern from `DirectedTwoCommodityIntegralFlow`, then extend CLI creation and paper/example plumbing around that core representation.

**Tech Stack:** Rust workspace crate, serde/inventory registry metadata, `pred` CLI, Typst paper, existing brute-force solver and example-db fixtures.

---

### Task 1: Lock down the model behavior with failing tests

**Files:**
- Create: `src/unit_tests/models/graph/path_constrained_network_flow.rs`
- Test: `src/unit_tests/models/graph/path_constrained_network_flow.rs`
- Reference: `src/unit_tests/models/graph/directed_two_commodity_integral_flow.rs`
- Reference: `src/models/graph/directed_two_commodity_integral_flow.rs`

**Step 1: Write the failing tests**

Add tests that cover:
- constructor/accessor behavior for a YES instance based on the issue’s 5-path example
- `dims()` using per-path bottleneck capacities
- satisfying evaluation for one known feasible path-flow vector
- unsatisfying evaluation for requirement failure, capacity overflow, invalid config length, and invalid path descriptions
- brute-force solver behavior on the YES and NO instances
- serde round-trip
- paper/example consistency test that checks the canonical YES instance and verifies the expected satisfying config

**Step 2: Run the targeted test to verify it fails**

Run: `cargo test path_constrained_network_flow --lib`
Expected: FAIL because `PathConstrainedNetworkFlow` does not exist yet.

**Step 3: Record the concrete example data in the test helpers**

Use the fixed issue example from #291 / `/fix-issue`:
- 8 vertices, 10 directed arcs, capacities `[2,1,1,1,1,1,1,1,2,1]`
- 5 prescribed s-t paths encoded as arc-index lists
- YES requirement `3`
- NO requirement `4`

**Step 4: Re-run the targeted test to confirm the failure is still the missing model, not a broken fixture**

Run: `cargo test path_constrained_network_flow --lib`
Expected: FAIL with unresolved import / missing type, not malformed test data.

### Task 2: Implement and register the model

**Files:**
- Create: `src/models/graph/path_constrained_network_flow.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the model**

Create `PathConstrainedNetworkFlow` with:
- `graph: DirectedGraph`
- `capacities: Vec<u64>`
- `source: usize`
- `sink: usize`
- `paths: Vec<Vec<usize>>`
- `requirement: u64`

Implement:
- `inventory::submit!` schema metadata
- constructor assertions for arc count, terminal bounds, and path validity
- accessors and size getters: `num_vertices`, `num_arcs`, `num_paths`, `max_capacity`, `requirement`
- helper(s) for path bottlenecks and prescribed-path validation
- `Problem` + `SatisfactionProblem`
- `declare_variants!` with `"((max_capacity + 1)^num_paths)"` or equivalent valid expression using the existing getter names
- canonical example spec for the YES instance

**Step 2: Register the model**

Wire the new type through:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude exports
- graph example-db spec chain in `src/models/graph/mod.rs`

**Step 3: Run the targeted tests**

Run: `cargo test path_constrained_network_flow --lib`
Expected: PASS for the new model tests.

**Step 4: Refactor only if needed**

Keep refactors local to helper extraction inside the new model/test files. Do not broaden scope beyond what the tests demand.

### Task 3: Add CLI creation support for prescribed paths

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/problem_name.rs` (only if alias coverage is missing after schema registration)

**Step 1: Write or extend failing CLI-facing tests first**

Add focused tests for:
- help text / example text exposing `PathConstrainedNetworkFlow`
- parsing a valid `pred create PathConstrainedNetworkFlow ... --paths ...`
- rejecting malformed `--paths` input and invalid arc references

**Step 2: Run the targeted CLI tests to verify failure**

Run: `cargo test create::tests::path_constrained_network_flow --package problemreductions-cli`
Expected: FAIL because the CLI does not yet know the new problem or `--paths`.

**Step 3: Implement CLI support**

Add:
- `CreateArgs.paths: Option<String>`
- `all_data_flags_empty()` coverage for the new flag
- help-table entry and example string
- parser/helper for semicolon-separated path lists where each path is a comma-separated arc-index sequence
- `create()` match arm that builds `DirectedGraph` from `--arcs`, parses capacities / source / sink / paths / requirement, and serializes the new model

Prefer the existing directed-graph conventions:
- use `--arcs`, not `--graph`
- use singular `--requirement`
- use `--paths "0,2,5,8;1,3,6,8;..."` to match the model’s constructor shape

**Step 4: Re-run the targeted CLI tests**

Run: `cargo test create::tests::path_constrained_network_flow --package problemreductions-cli`
Expected: PASS.

### Task 4: Wire the canonical example and paper entry

**Files:**
- Modify: `docs/paper/reductions.typ`
- Verify via existing example-db exports driven from model registration
- Reference: `.claude/skills/write-model-in-paper/SKILL.md`

**Step 1: Verify the canonical example is the same issue fixture**

Use the example from Task 1 / model registration as the single source of truth for:
- `canonical_model_example_specs()`
- paper example text
- `pred-commands()` snippet

**Step 2: Update the paper**

Add:
- display-name entry for `PathConstrainedNetworkFlow`
- `problem-def("PathConstrainedNetworkFlow")`
- cited background using Garey & Johnson ND34 plus the published Büsing–Stiller 2011 reference
- a worked YES example that explains the feasible integral path-flow assignment
- `pred-commands()` that uses the canonical example fixture and brute-force solving

**Step 3: Build the paper to confirm the entry is valid**

Run: `make paper`
Expected: PASS.

### Task 5: Full verification, PR summary, and cleanup

**Files:**
- Modify: PR body/comment generated during pipeline execution

**Step 1: Run the required verification commands**

Run:
- `cargo test path_constrained_network_flow --lib`
- `cargo test create::tests::path_constrained_network_flow --package problemreductions-cli`
- `make test`
- `make clippy`
- `make paper`

**Step 2: Inspect the tree**

Run: `git status --short`
Expected: only intentional tracked changes; no leftover plan artifacts after cleanup.

**Step 3: Call out the pipeline-specific deviation**

In the PR body or implementation summary comment, note:
- `<!-- WARNING: orphan model — no associated rule issue -->`
- the current repo search found no open rule issue referencing `PathConstrainedNetworkFlow`

**Step 4: Commit, remove the plan file, push, and post the implementation summary**

Follow `.claude/skills/issue-to-pr/SKILL.md` Step 7b-7d exactly after the implementation is verified.
