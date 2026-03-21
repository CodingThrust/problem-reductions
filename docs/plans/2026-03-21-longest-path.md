# LongestPath Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement issue #288 by adding the `LongestPath` model, wiring brute-force and ILP solving support, registering CLI/example-db integration, and documenting the model and new ILP reduction in the paper.

**Architecture:** Model `LongestPath<G, W>` as an edge-selection optimization problem, matching the issue's optimization framing from the `fix-issue` comment. A configuration is valid exactly when the selected undirected edges form one simple path from `source_vertex` to `target_vertex`; the metric is the total selected edge length. Because the issue explicitly opts into ILP solving, add a direct `LongestPath<SimpleGraph, i32> -> ILP<bool>` reduction with a path-ordering formulation that forbids disconnected cycles and extracts the chosen edge set back into the source configuration.

**Tech Stack:** Rust workspace, serde/inventory schema registration, registry-backed CLI loading, `BruteForce` + `ILPSolver`, Typst paper, example-db exports.

---

## Issue Packet Summary

- Issue: `#288 [Model] LongestPath`
- Issue state: open, labeled `Good`
- Existing PRs: none (`action = create-pr`)
- Associated rule already on the board: `#359 [Rule] HAMILTONIAN PATH BETWEEN TWO VERTICES to LONGEST PATH`
- Maintainer guidance from comments:
  - Keep `LongestPath` as an optimization problem (`Direction::Maximize`)
  - Use the single verified example instance with optimum `20`
  - Record ILP support instead of leaving it unspecified

## Concrete Design Choices

1. **Problem shape**
   - File: `src/models/graph/longest_path.rs`
   - Type: `LongestPath<G, W>`
   - Fields:
     - `graph: G`
     - `edge_lengths: Vec<W>`
     - `source_vertex: usize`
     - `target_vertex: usize`
   - Variants:
     - `default opt LongestPath<SimpleGraph, i32> => "num_vertices * 2^num_vertices"`
     - `opt LongestPath<SimpleGraph, One> => "num_vertices * 2^num_vertices"`

2. **Configuration semantics**
   - One binary variable per edge, in graph-edge order
   - Valid iff selected edges induce a single undirected simple path from `source_vertex` to `target_vertex`
   - `source_vertex == target_vertex` should accept only the empty edge set and evaluate to `Valid(0)`
   - Any repeated-edge, branching, disconnected, cyclic, or wrong-endpoint selection is `Invalid`

3. **Example of record**
   - Use the corrected issue instance with 7 vertices, 10 edges, `s = 0`, `t = 6`
   - Optimal path edges correspond to `0 -> 1 -> 3 -> 2 -> 4 -> 5 -> 6`
   - Optimal objective value: `20`
   - Include a suboptimal valid path worth `17`

4. **ILP solver path**
   - File: `src/rules/longestpath_ilp.rs`
   - Restrict initial ILP reduction to `LongestPath<SimpleGraph, i32>`
   - Use binary directed-arc variables plus visited/order variables so the selected solution is a single simple `s-t` path, not a path plus detached cycles
   - Objective: maximize total selected edge length
   - Extraction: recover the selected undirected source edges from the ILP solution

## Batch 1: Implementation

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/graph/longest_path.rs`

**Step 1: Add tests that define the required behavior**

Cover at least these cases:
- `test_longest_path_creation` for constructor/accessors/dimensions
- `test_longest_path_evaluate_valid_and_invalid_configs`
- `test_longest_path_bruteforce_finds_issue_optimum`
- `test_longest_path_serialization`
- `test_longest_path_source_equals_target_only_allows_empty_path`
- `test_longestpath_paper_example`

Use the corrected issue fixture as the canonical positive instance.

**Step 2: Run the new test target and confirm RED**

Run:

```bash
cargo test longest_path --lib
```

Expected: compile failure because `LongestPath` does not exist yet.

**Step 3: Commit the failing-test checkpoint only if the tree is intentionally staged that way**

Do not force an early commit if the repo workflow is smoother with model code immediately after the RED check.

### Task 2: Implement and register the model

**Files:**
- Create: `src/models/graph/longest_path.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement `LongestPath<G, W>`**

Include:
- `inventory::submit!` schema metadata
- constructor validation:
  - `edge_lengths.len() == graph.num_edges()`
  - vertex indices in range
  - positive lengths for weighted variants
- accessors:
  - `graph()`
  - `edge_lengths()`
  - `set_lengths()`
  - `source_vertex()`
  - `target_vertex()`
  - `num_vertices()`
  - `num_edges()`
  - `is_weighted()`
  - `is_valid_solution()`
- `Problem` impl with `Metric = SolutionSize<W::Sum>`
- `OptimizationProblem` impl with `Direction::Maximize`
- internal validation helper that checks the selected edge set forms exactly one simple `s-t` path
- `declare_variants!`
- `canonical_model_example_specs()`
- test link:
  - `#[cfg(test)] #[path = "../../unit_tests/models/graph/longest_path.rs"] mod tests;`

**Step 2: Register the model everywhere it must appear**

Update:
- `src/models/graph/mod.rs`
  - module declaration
  - public re-export
  - example-db spec chain
- `src/models/mod.rs`
  - graph re-export list
- `src/lib.rs`
  - `prelude` exports if needed by current conventions

**Step 3: Re-run the model tests and get GREEN**

Run:

```bash
cargo test longest_path --lib
```

Expected: the new model tests pass.

### Task 3: Add ILP rule support with tests first

**Files:**
- Create: `src/unit_tests/rules/longestpath_ilp.rs`
- Create: `src/rules/longestpath_ilp.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Add failing rule tests**

Cover:
- reduction builds an `ILP<bool>` with nonempty objective/constraints
- `ILPSolver` solution maps back to a valid `LongestPath` configuration
- reduced optimum matches brute-force optimum on the issue example
- extraction rejects or avoids detached cycles by construction
- path metadata is discoverable from the reduction graph if relevant

**Step 2: Run the ILP-focused test target and confirm RED**

Run:

```bash
cargo test longestpath_ilp --lib --features ilp-solver
```

Expected: compile failure because the rule module does not exist yet.

**Step 3: Implement `LongestPath<SimpleGraph, i32> -> ILP<bool>`**

Model the path with:
- directed-arc binary variables for each undirected edge orientation
- vertex-visited binary variables
- integer order variables (or equivalent MTZ-style progression variables)

Enforce:
- source has one outgoing selected arc and no incoming selected arc
- target has one incoming selected arc and no outgoing selected arc
- every nonterminal visited vertex has one incoming and one outgoing selected arc
- every unvisited nonterminal vertex has zero selected incident arcs
- each undirected edge is used in at most one direction
- ordering constraints eliminate directed cycles disconnected from the main path

Objective:
- maximize total selected edge length

Extraction:
- map whichever arc orientation is selected for each undirected edge back to the edge-indexed source config

**Step 4: Register and re-run the ILP tests**

Run:

```bash
cargo test longestpath_ilp --lib --features ilp-solver
```

Expected: rule tests pass.

### Task 4: Wire CLI creation, aliases, and example-db coverage

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Add CLI coverage tests first**

Add tests near the existing `create.rs` test module for:
- successful JSON creation from:
  - `pred create LongestPath --graph ... --edge-lengths ... --source-vertex 0 --target-vertex 6`
- missing required `--edge-lengths`
- misuse of `--weights` instead of `--edge-lengths`

**Step 2: Run the targeted CLI tests and confirm RED**

Run:

```bash
cargo test create_longest_path --package problemreductions-cli
```

Expected: failures until the CLI path exists.

**Step 3: Implement CLI integration**

Update:
- `problemreductions-cli/src/problem_name.rs`
  - allow lowercase alias resolution for `longestpath`
  - do not invent a short literature acronym
- `problemreductions-cli/src/commands/create.rs`
  - add example string
  - add create arm parsing `--graph`, `--edge-lengths`, `--source-vertex`, `--target-vertex`
  - emit helpful errors for missing fields and wrong flag families
- `problemreductions-cli/src/cli.rs`
  - ensure help text lists `LongestPath`
  - ensure `all_data_flags_empty()` already covers the flags this model needs; change only if a new flag is introduced

**Step 4: Re-run the CLI tests**

Run:

```bash
cargo test create_longest_path --package problemreductions-cli
```

Expected: the new CLI tests pass.

### Task 5: Run broader implementation verification for Batch 1

**Files:**
- No new files; verification only

**Step 1: Run focused workspace tests**

Run:

```bash
cargo test longest_path --workspace --features ilp-solver
```

**Step 2: Run the standard repo checks for code touched so far**

Run:

```bash
make test
make clippy
```

If `make test` is too expensive while iterating, keep using the targeted cargo invocations until the batch is stable, then run the full commands before leaving Batch 1.

## Batch 2: Paper and Documentation

### Task 6: Add paper coverage for both the model and the new ILP rule

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add the display name entry**

Register:

```text
"LongestPath": [Longest Path],
```

**Step 2: Add `problem-def("LongestPath")`**

The entry should:
- define the optimization version explicitly
- explain that the decision form is recovered by thresholding the optimum
- cite Garey & Johnson plus the exact-algorithm references already verified in the issue comments
- use the canonical 7-vertex example with highlighted optimal path
- include `pred-commands()` derived from `load-model-example("LongestPath")`

**Step 3: Add a `reduction-rule("LongestPath", "ILP", ...)` entry**

Because Batch 1 registers a real reduction edge, the paper must cover it. Document:
- the path-selection variables
- degree / flow / ordering constraints
- why every feasible ILP solution corresponds to a simple `s-t` path
- why the maximized objective equals the path length

Use exported example data if the existing helper flow supports it; otherwise write the example from the same canonical issue fixture.

**Step 4: Build the paper**

Run:

```bash
make paper
```

Expected: no completeness warnings for `LongestPath` or `LongestPath -> ILP`.

## Final Verification and Cleanup

### Task 7: Full verification before pushing

**Step 1: Run the highest-signal verification commands**

Run:

```bash
make test
make clippy
make paper
```

If runtime permits and coverage impact is uncertain, also run:

```bash
make coverage
```

**Step 2: Inspect the working tree**

Run:

```bash
git status --short
```

Expected:
- only intended tracked changes remain
- ignored generated exports under `docs/src/reductions/` stay unstaged

**Step 3: Keep commits coherent**

Recommended commit sequence:
- `Add plan for #288: [Model] LongestPath`
- `Implement #288: [Model] LongestPath`
- `chore: remove plan file after implementation`

## Expected File Inventory

- `src/models/graph/longest_path.rs`
- `src/unit_tests/models/graph/longest_path.rs`
- `src/rules/longestpath_ilp.rs`
- `src/unit_tests/rules/longestpath_ilp.rs`
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`
- `problemreductions-cli/src/problem_name.rs`
- `problemreductions-cli/src/commands/create.rs`
- `problemreductions-cli/src/cli.rs`
- `docs/paper/reductions.typ`

## References To Use During Implementation

- Issue packet for `#288`, including the `fix-issue` comment and the quality-check comment
- `src/models/graph/maximum_independent_set.rs`
- `src/models/graph/shortest_weight_constrained_path.rs`
- `src/models/graph/traveling_salesman.rs`
- `src/rules/travelingsalesman_ilp.rs`
- `src/unit_tests/models/graph/hamiltonian_path.rs`
- `src/unit_tests/rules/maximumclique_ilp.rs`
- `docs/paper/reductions.typ` entries for `HamiltonianPath` and existing `-> ILP` rules
