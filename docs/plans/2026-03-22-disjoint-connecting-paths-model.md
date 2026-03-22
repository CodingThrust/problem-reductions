# Plan: Add DisjointConnectingPaths Model

**Issue:** #297 — [Model] DisjointConnectingPaths
**Skill:** add-model
**Execution:** superpowers:subagent-driven-development

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `DisjointConnectingPaths` |
| 2 | Mathematical definition | Given an undirected graph `G = (V, E)` and pairwise disjoint terminal pairs `(s_1, t_1), ..., (s_k, t_k)`, determine whether `G` contains `k` mutually vertex-disjoint paths, one connecting each pair |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | `G: Graph` |
| 5 | Struct fields | `graph: G`, `terminal_pairs: Vec<(usize, usize)>` |
| 6 | Configuration space | `vec![2; num_edges]` — one binary variable per edge in a canonical sorted edge list |
| 7 | Feasibility check | Selected edges must induce exactly `k` pairwise vertex-disjoint simple paths whose endpoint pairs match the requested terminal pairs and whose internal vertices are non-terminals |
| 8 | Objective function | `bool` — `true` iff the selected edge subset realizes all terminal pairs simultaneously |
| 9 | Best known exact algorithm | Brute-force over all edge subsets; complexity string: `"2^num_edges"` |
| 10 | Solving strategy | Existing `BruteForce` solver is sufficient for the model; no ILP rule is required in this issue because brute-force already gives a valid solver path |
| 11 | Category | `graph` |
| 12 | Expected outcome from the issue | YES example: edges `{0,1}, {1,3}, {2,4}, {4,5}` connect `(0,3)` and `(2,5)` with vertex-disjoint paths; NO example: pairs `(0,4)` and `(1,5)` cannot be routed disjointly through the cut vertices |

## Associated Rule Check

- Planned inbound rule already exists in the issue body: `3SAT -> DisjointConnectingPaths`.
- No orphan-model warning is needed.

## Design Decisions

### Canonical edge order

The issue explicitly fixes an edge-based encoding. To make that encoding deterministic across serialization, example-db fixtures, CLI evaluation, and tests, the model will define a helper that normalizes each undirected edge to `(min(u,v), max(u,v))` and sorts the list lexicographically. Config index `i` refers to the `i`-th edge in that canonical order, not to `petgraph`'s internal storage order.

### Validity semantics

`evaluate(config)` should accept a config iff:

1. `config.len() == num_edges` and every entry is binary.
2. The selected-edge subgraph decomposes into exactly `num_pairs()` connected components.
3. Every selected component is a simple path:
   - endpoints have degree 1,
   - internal vertices have degree 2,
   - the component is connected and acyclic.
4. The endpoints of each component match one requested terminal pair (in either orientation).
5. No terminal vertex is used as an internal path vertex, and every requested pair is realized exactly once.

### CLI shape

Add a dedicated `--terminal-pairs` flag using `u-v,u-v,...` syntax, with validation that:

- every vertex index exists in the graph,
- every pair uses distinct endpoints,
- no vertex appears in more than one pair.

This keeps the CLI aligned with the schema field name instead of overloading the existing `--terminals` flag.

### Canonical example

Use the issue's repaired YES instance as the canonical example:

- graph edges: `(0,1), (1,3), (0,2), (1,4), (2,4), (3,5), (4,5)`
- terminal pairs: `(0,3), (2,5)`
- satisfying config over sorted edges: `[1, 0, 1, 0, 1, 0, 1]`

Also keep the issue's NO instance in unit tests to pin down the cut-vertex failure mode.

## Batch 1: Implementation and Registration

### Step 1: Implement the model

**Files:**
- Create: `src/models/graph/disjoint_connecting_paths.rs`
- Create: `src/unit_tests/models/graph/disjoint_connecting_paths.rs`
- Reference: `src/models/graph/length_bounded_disjoint_paths.rs`
- Reference: `src/models/graph/hamiltonian_path.rs`

Work items:

1. Add `ProblemSchemaEntry` metadata with graph-only variants and constructor-facing fields `graph` and `terminal_pairs`.
2. Implement `DisjointConnectingPaths<G>` with constructor validation, accessors, size getters, `is_valid_solution`, and a canonical-edge helper.
3. Implement `Problem` and `SatisfactionProblem`, with `dims()` based on `num_edges()` and `evaluate()` using the path-component validation logic above.
4. Add `declare_variants!` for `DisjointConnectingPaths<SimpleGraph>` with complexity `"2^num_edges"`.
5. Add `canonical_model_example_specs()` using the repaired YES instance from the issue.
6. Write tests first for:
   - constructor validation,
   - YES instance,
   - NO/cut-vertex instance,
   - malformed configs,
   - brute-force solver,
   - serialization,
   - canonical paper/example instance.

### Step 2: Register the model

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

Work items:

1. Add the new module and public re-export in the graph model tree.
2. Add top-level exports so the type is reachable from `problemreductions::models::*` and the prelude.
3. Extend the graph example-db aggregation chain.

### Step 3: Add CLI discovery and creation support

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/problem_name.rs`

Work items:

1. Add `DisjointConnectingPaths` to the imported graph model list.
2. Add `--terminal-pairs` to `CreateArgs`, `all_data_flags_empty()`, the create help table, and any field-to-flag mapping helpers.
3. Add a parser/validator for terminal-pair lists.
4. Add the `create()` match arm for `pred create DisjointConnectingPaths --graph ... --terminal-pairs ...`.
5. Add a canonical example string and any parser-focused CLI tests needed for the new flag.
6. Add alias resolution for the full lowercase name only; do not invent a short alias.

### Step 4: Verify Batch 1

Run targeted commands while implementing, then finish with:

```bash
cargo test disjoint_connecting_paths --lib
cargo test create_disjoint_connecting_paths --package problemreductions-cli
make test
make clippy
```

## Batch 2: Paper and Fixture Integration

### Step 5: Document the model in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib` (only if the Robertson--Seymour / Kawarabayashi--Kobayashi--Reed citations are not already present)

Work items:

1. Add `"DisjointConnectingPaths": [Disjoint Connecting Paths]` to the display-name map.
2. Add a `problem-def("DisjointConnectingPaths")` entry with:
   - the formal definition from the issue,
   - background linking the problem to routing / VLSI,
   - NP-completeness context from Garey & Johnson / Karp,
   - fixed-`k` polynomial-time results with citations to Robertson--Seymour and Kawarabayashi--Kobayashi--Reed,
   - the repaired YES example rendered with a graph figure and the selected edge subset,
   - `pred-commands()` driven by the canonical example-db entry.
3. Rebuild the paper/example exports and verify the example matches the issue's expected outcome.

### Step 6: Verify Batch 2

```bash
make paper
make test
make clippy
```

## Exit Criteria

- `DisjointConnectingPaths<SimpleGraph>` is registered, exported, serializable, and solvable by brute force.
- `pred create DisjointConnectingPaths --graph ... --terminal-pairs ...` works.
- The canonical example-db entry and paper example use the issue's repaired YES instance.
- Tests cover both the YES and NO issue instances.
