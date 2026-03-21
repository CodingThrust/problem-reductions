# IntegralFlowHomologousArcs Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.
>
> **Codex note:** `run-pipeline` normally hands execution to subagents, but this session does not have explicit delegation approval. Execute this plan locally in the same order, following TDD for every code change.

**Goal:** Add the `IntegralFlowHomologousArcs` satisfaction model, register it for library/CLI/example-db/paper workflows, and validate the canonical YES/NO homologous-constraint instances from issue `#292`.

**Architecture:** Model the instance as a `DirectedGraph` plus per-arc capacities, one source/sink pair, one required sink inflow, and homologous pairs encoded as arc-index pairs in graph arc order. A configuration stores one integer flow value per arc; validity checks enforce per-arc bounds, conservation at non-terminals, homologous equalities, and sink inflow `>= requirement`. For `declare_variants!`, use the general-capacity brute-force bound `"(max_capacity + 1)^num_arcs"` and document `2^num_arcs` as the unit-capacity special case mentioned in the issue.

**Tech Stack:** Rust workspace (`problemreductions`, `problemreductions-cli`), serde, inventory registry metadata, Typst paper docs, existing brute-force solver.

---

## Issue Checklist

| Item | Decision |
|---|---|
| Problem name | `IntegralFlowHomologousArcs` |
| Problem type | Satisfaction (`Metric = bool`) |
| Category | `src/models/graph/` |
| Core fields | `graph`, `capacities`, `source`, `sink`, `requirement`, `homologous_pairs` |
| Config space | One variable per arc, domain `{0, ..., c(a)}` |
| Feasibility | Capacity, conservation, homologous-equality, sink inflow threshold |
| Example outcome | Use the YES and NO instances from issue `#292` / fix-issue changelog |
| Associated rules | Incoming: `#732`, `#365`; outgoing: `#733` |

## Batch 1: Model, Registry, CLI, Tests

### Task 1: Add failing model tests first

**Files:**
- Create: `src/unit_tests/models/graph/integral_flow_homologous_arcs.rs`
- Modify: `src/models/graph/integral_flow_homologous_arcs.rs` (test link will fail until the file exists)

**Step 1: Write the failing tests**

Cover these behaviors in the new test file:
- constructor/accessors/dimension sizes for the issue YES instance
- `evaluate()` accepts the YES configuration from the issue
- `evaluate()` rejects the issue NO instance because the homologous constraint makes it infeasible
- `evaluate()` rejects capacity, conservation, homologous-pair, and wrong-length violations
- `BruteForce::find_satisfying()` returns `Some(_)` for the YES instance and `None` for the NO instance
- serde round-trip and a `test_integral_flow_homologous_arcs_paper_example`

**Step 2: Run the focused test target and verify RED**

Run: `cargo test integral_flow_homologous_arcs --lib`

Expected: compile/test failure because the model module and test link do not exist yet.

**Step 3: Do not add production code yet**

Stop after confirming the failing state. The model implementation belongs to Task 2.

### Task 2: Implement the model and wire it into the library

**Files:**
- Create: `src/models/graph/integral_flow_homologous_arcs.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the minimal model to satisfy Task 1**

In `src/models/graph/integral_flow_homologous_arcs.rs`:
- add `ProblemSchemaEntry` with constructor-facing fields
- add optional `ProblemSizeFieldEntry` for `num_vertices` and `num_arcs`
- define `IntegralFlowHomologousArcs` with `#[derive(Debug, Clone, Serialize, Deserialize)]`
- use `DirectedGraph` plus `Vec<u64>` capacities and `Vec<(usize, usize)>` homologous arc-index pairs
- validate constructor invariants:
  - capacities length matches `graph.num_arcs()`
  - source/sink are in range
  - every homologous pair references valid arc indices
  - each capacity fits into `usize` for `dims()`
- expose getters: `graph()`, `capacities()`, `source()`, `sink()`, `requirement()`, `homologous_pairs()`, `num_vertices()`, `num_arcs()`, `max_capacity()`
- implement `Problem`, `SatisfactionProblem`, `declare_variants!`, and `canonical_model_example_specs()`
- use `"(max_capacity + 1)^num_arcs"` in `declare_variants!`

In the library exports:
- register the module in `src/models/graph/mod.rs`
- extend the graph re-exports and `canonical_model_example_specs()` chain
- add the type to `src/models/mod.rs` and the graph prelude exports in `src/lib.rs`

**Step 2: Run the focused model tests and verify GREEN**

Run: `cargo test integral_flow_homologous_arcs --lib`

Expected: the new model tests pass.

**Step 3: Refactor only if needed**

Keep helper functions small and local. Prefer explicit balance calculations over generic abstractions.

### Task 3: Add CLI creation support with tests first

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the failing CLI tests**

Add coverage for:
- successful `pred create IntegralFlowHomologousArcs --arcs ... --capacities ... --source ... --sink ... --requirement ... --homologous-pairs ...`
- helpful usage errors when `--homologous-pairs` or required terminals are missing
- schema/help wiring so `pred create IntegralFlowHomologousArcs` surfaces the new fields

Use `--homologous-pairs "2=5;4=3"` as the planned CLI format for arc-index equalities.

**Step 2: Run the focused CLI tests and verify RED**

Run: `cargo test -p problemreductions-cli integral_flow_homologous_arcs`

Expected: failure because the CLI flags, parser, and create arm do not exist yet.

**Step 3: Implement the minimal CLI support**

In `problemreductions-cli/src/cli.rs`:
- add `IntegralFlowHomologousArcs` to the "Flags by problem type" and examples
- add `--homologous-pairs` to `CreateArgs`

In `problemreductions-cli/src/commands/create.rs`:
- include `homologous_pairs` in both `all_data_flags_empty()` checks
- parse the new flag into `Vec<(usize, usize)>`
- add a `create()` match arm that builds `IntegralFlowHomologousArcs`
- reuse existing `--arcs`, `--capacities`, `--source`, `--sink`, and `--requirement`

**Step 4: Re-run the focused CLI tests and verify GREEN**

Run: `cargo test -p problemreductions-cli integral_flow_homologous_arcs`

Expected: the targeted CLI tests pass.

### Task 4: Add supporting regression coverage and trait/example checks

**Files:**
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `src/models/graph/integral_flow_homologous_arcs.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs` (if Task 3 only added minimal error coverage)

**Step 1: Add failing regression checks**

Add or extend tests so the new model is covered by:
- trait-consistency name checks
- canonical example-db expectations
- at least one end-to-end CLI create/solve or create/evaluate smoke path if still missing

**Step 2: Run focused tests and verify RED**

Run: `cargo test trait_consistency integral_flow_homologous_arcs`

Expected: failure until the missing registrations or example values are wired correctly.

**Step 3: Implement the minimal missing wiring**

Only add the registrations/tests needed to satisfy the new coverage.

**Step 4: Re-run focused tests and verify GREEN**

Run: `cargo test trait_consistency integral_flow_homologous_arcs`

Expected: the focused regression checks pass.

## Batch 2: Paper Entry

### Task 5: Add the paper documentation after implementation is stable

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the failing paper-adjacent check**

If the model tests do not already assert the issue example used in the paper, extend `test_integral_flow_homologous_arcs_paper_example` first so the paper instance is executable in Rust before editing Typst.

**Step 2: Implement the paper entry**

Add:
- display-name dictionary entry for `IntegralFlowHomologousArcs`
- `#problem-def("IntegralFlowHomologousArcs")[...]` with self-contained notation
- background that cites Garey-Johnson / Sahni and notes the LP-equivalent fractional variant
- a worked example matching the canonical YES instance
- `pred-commands()` using the canonical example-db entry rather than a hand-written bare alias
- wording that records `2^num_arcs` as the unit-capacity special case while the code registers the general-capacity bound

Use the existing directed-flow paper entries as the style reference.

**Step 3: Run the paper build and verify GREEN**

Run: `make paper`

Expected: Typst compiles without warnings/errors attributable to the new entry.

## Final Verification

### Task 6: Run repo verification and prepare the branch for PR push

**Files:**
- Modify only as needed from prior tasks

**Step 1: Run the required verification**

Run:
- `cargo test integral_flow_homologous_arcs`
- `cargo test -p problemreductions-cli integral_flow_homologous_arcs`
- `make test`
- `make clippy`
- `make paper`

**Step 2: Inspect the tree**

Run: `git status --short`

Expected: only intentional tracked changes remain; ignored generated docs under `docs/src/reductions/` may appear but must not be staged accidentally.

**Step 3: Commit coherent implementation changes**

Suggested messages:
- `Add IntegralFlowHomologousArcs model and CLI support`
- `Document IntegralFlowHomologousArcs in paper`

**Step 4: Remove this plan file before final push**

Run:
- `git rm docs/plans/2026-03-22-integral-flow-homologous-arcs-model.md`
- `git commit -m "chore: remove plan file after implementation"`
