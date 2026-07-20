# Exact and Approximate Path Search — Product Design

Status: implemented.

Amendment (2026-07-18): intermediate strict dominance pruning is removed. Reduction
overheads may be non-monotone (for example graph-complement size formulas subtract the
current edge count), so the package cannot establish the isotonicity required by a
label-setting dominance proof. The current labels do not carry complete constructed
problems, so equal size, cost, or growth summaries do not coalesce intermediate states.
Pareto dominance is applied only to completed destination labels.

This design refines the path-search portion of
[`symbolic-growth-domain.md`](symbolic-growth-domain.md). It supersedes that document's
implicit global hop and per-node bag caps; it does not change the symbolic `Growth`
domain or measured-size semantics introduced there.

## Need

The reduction graph currently exposes APIs whose names imply a complete optimum or
Pareto front, while the shared Pareto kernel always stops extending after 16 hops and
retains at most 32 labels per node. Those deterministic caps keep interactive searches
small, but they can discard the only feasible path, a true scalar winner, or a distinct
Pareto point. Callers receive no indication that this happened.

The library needs one explicit completeness contract across formula-ranked,
asymptotic, and measured path search:

- **Exact** returns a complete result for the declared finite search space or an error;
  it never silently drops a candidate because of a resource cap.
- **Approximate** may stop or truncate according to caller-provided limits, always
  returns valid best-so-far candidates, and reports every limit that affected
  completeness.

Symbolic versus measured remains a separate semantic choice. `SearchMode` answers
"how complete is the search?", not "what does a label mean?".

**Users:** library callers, the ILP reduction solver, CLI users of `pred path` and
`pred reduce`, and MCP clients.

**Success criteria:**

1. Every public optimum/front API requires an explicit `SearchMode`.
2. Exact mode finds paths longer than the former hop cap and winners that require more
   than the former per-node bag cap.
3. Exact mode terminates on cyclic reduction graphs by searching elementary (simple)
   paths, without intermediate strict dominance pruning.
4. Approximate mode reports whether a hop, per-node label, expanded-state, or time limit
   changed the explored search space. If no limit is hit, its outcome is reported as
   exact.
5. Equal coarse labels remain distinct at intermediate nodes; only completed labels are
   Pareto-filtered.
6. CLI text and JSON and MCP responses expose completeness; no approximate answer is
   presented as an unqualified optimum or Pareto front.
7. Search remains deterministic for all count-based limits. Timeout-limited searches
   are explicitly exempt because elapsed time is machine-dependent.

**Constraints:**

- Rust 2021 and the repository's existing dependencies only.
- No single test may exceed five seconds.
- Internal and public Rust APIs may break under the crate's 0.x version policy.
- Existing reduction declarations and overhead syntax remain unchanged.
- Exactness is relative to the selected label semantics, feasibility policy, and
  elementary-path search space.

## Prior art and landscape

The design follows established multiobjective and resource-constrained shortest-path
practice:

| Source | Adopted lesson |
|---|---|
| Martins-style label setting and the Multiobjective Dijkstra Algorithm ([Maristany de las Casas et al., 2021](https://doi.org/10.1016/j.cor.2021.105424)) | An exact result is a complete set of efficient labels; performance pruning must preserve completeness or be identified separately. |
| Boost Graph Library `r_c_shortest_paths` ([documentation](https://www.boost.org/doc/libs/1_84_0/libs/graph/doc/r_c_shortest_paths.html)) | Dominance pruning is appropriate only when labels contain continuation-relevant resources and extension preserves the order. This package does not assume that property for arbitrary reductions. |
| Papadimitriou and Yannakakis, *On the Approximability of Trade-offs* ([paper](https://www.cs.purdue.edu/homes/yexiang/courses/18fall-cs590/papers/papadimitriou2000.pdf)) | A formal epsilon-Pareto approximation has a coverage guarantee. A fixed bag width without such a guarantee is best-effort bounded search, not epsilon approximation. |
| Elementary resource-constrained shortest-path labeling | When visited vertices affect future feasibility, the visited set is part of the state. Equal resource summaries alone do not identify the same continuation state. |

No external path-search crate matches the repository's path-dependent symbolic labels,
variant graph, and concrete reduction execution. The project should keep its small
kernel and adopt the contracts above rather than add a dependency.

## Features

Selected features and rough agentic-coding-adjusted effort:

| # | Feature | User value | Effort |
|---|---|---|---|
| F1 | Explicit `Exact` / `Approximate` mode and typed limits | Callers choose the completeness contract instead of inheriting hidden caps | ~0.5–1 day |
| F2 | `SearchOutcome<T>` with completeness reasons and statistics | Every consumer can distinguish complete from best-so-far results | ~0.5–1 day |
| F3 | Elementary exact multi-label kernel with terminal Pareto filtering | Exact mode terminates without arbitrary hop/bag truncation or unproved intermediate pruning | ~1.5–2.5 days |
| F4 | Formula, asymptotic, and measured integration | One contract across all search semantics | ~1–1.5 days |
| F5 | CLI/MCP and ILP policy migration | Interactive users retain bounded latency without misleading output | ~1–1.5 days |
| F6 | Behavioural regressions, documentation, and full migration | Prevents the old hidden-cap behaviour from returning | ~1–1.5 days |

Total rough effort: **~5.5–9 days**.

Deferred:

- **Epsilon-Pareto approximation** — requires a real objective-space discretization
  algorithm and proof; add later as another `ApproximationPolicy` variant.
- **Fallible reduction execution (`Result` instead of caught panic)** — desirable Rust
  API work, but independent of completeness.
- **Final-only versus every-intermediate measured budget policies** — separate
  feasibility design.
- **Certified overhead monotonicity metadata** — separate symbolic trust-contract work.

Dropped:

- A third top-level `Bounded` mode. Bounding is the first implementation of
  `Approximate`, not a separate user concept.
- Hidden legacy defaults in the Rust library. Compatibility wrappers would preserve the
  ambiguity this design removes.

## Semantic contract

### Orthogonal axes

The API distinguishes two independent choices:

```text
Search semantics                         Completeness
────────────────────────────────────    ──────────────────────
Formula-evaluated / symbolic / measured  Exact / Approximate
```

`Exact` does not mean that a formula estimate equals a constructed instance. It means
the path search is complete for the selected semantics. Likewise, `Growth::Unknown` or
sound widening may reduce abstract precision without making route enumeration
incomplete.

### Exact search space

Exact mode searches **elementary paths**: no variant-level graph node occurs twice in
one path. This makes the search space finite and matches the existing public
`find_all_paths*` interpretation of a reduction path.

Every path prefix remains a distinct intermediate state. Reaching the same graph node
with equal `ProblemSize`, accumulated cost, or growth vector does not prove that the
constructed problem is identical: hidden instance structure and the visited-node set can
change future reductions. The current label domains carry no certified full-instance
identity, so the kernel performs no intermediate coalescing.

A future label domain may deduplicate only by a certified exact problem-state identity
that includes all continuation-relevant state. This is intentionally not approximated by
summary equality. Strict Pareto dominance is evaluated only after labels reach the
destination, where no future reduction can reverse their order.

### Approximate search

Approximate mode searches the same elementary-path space but may:

- stop extending at a configured hop count;
- truncate a per-node bag deterministically;
- stop after a configured number of expanded states; or
- stop after a configured duration.

Returned paths and labels remain feasible. The result is not claimed to cover the true
front or optimum unless no limit affected exploration. Initial bounded search has no
multiplicative or additive error guarantee.

A timeout is checked between state expansions. It cannot interrupt an in-progress
reduction constructor and is not deterministic across machines.

### Measured feasibility

Measured `budget` remains a feasibility constraint applied after constructing every
intermediate target. It is not an approximation limit and does not change the outcome's
completeness classification. Exact measured search is therefore complete over
elementary paths whose constructed intermediates all satisfy that budget and whose edge
executions succeed.

## Modules

### M1 — Search contract (`src/rules/search.rs`, one new module)

Purpose: own caller intent, outcome metadata, and shared accounting without coupling
them to a label domain.

Normative API shape:

```rust
use std::collections::BTreeSet;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum SearchMode {
    Exact,
    Approximate(ApproximationPolicy),
}

#[derive(Clone, Debug)]
pub enum ApproximationPolicy {
    Bounded(SearchLimits),
}

#[derive(Clone, Debug, Default)]
pub struct SearchLimits {
    pub max_hops: Option<usize>,
    pub max_labels_per_node: Option<usize>,
    pub max_expanded_states: Option<usize>,
    pub timeout: Option<Duration>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LimitReached {
    HopLimit,
    LabelsPerNodeLimit,
    ExpandedStatesLimit,
    Timeout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchCompleteness {
    Exact,
    Approximate {
        reasons: BTreeSet<LimitReached>,
    },
}

#[derive(Clone, Debug, Default)]
pub struct SearchStats {
    pub generated_states: usize,
    pub expanded_states: usize,
    pub dominated_states: usize,
    pub infeasible_extensions: usize,
    pub peak_labels_per_node: usize,
    pub elapsed: Duration,
}

#[must_use]
pub struct SearchOutcome<T> {
    pub value: T,
    pub completeness: SearchCompleteness,
    pub stats: SearchStats,
}
```

`BTreeSet` makes reason serialization deterministic. `Duration` is used instead of a
unit-ambiguous integer. Zero-valued count limits are valid and mean no corresponding
state may be expanded/retained; they are useful negative controls rather than invalid
configuration. `SearchStats::elapsed` remains available to Rust callers but is omitted
from serialized responses because wall-clock timing would break count-limited output
determinism.

Internal `SearchTracker` owns the start `Instant`, counters, and reached limits. `Instant`
does not cross the public or serialization boundary.

Dependencies: standard library only.

### M2 — Pareto kernel (`src/rules/graph.rs`, in place)

Purpose: enumerate elementary labels, filter the terminal Pareto front, and obey the
selected completeness policy.

Changes:

1. Give `PathLabel` a `final_dominates` operation used only at the destination.
2. Exact mode uses deterministic DFS backtracking with one mutable path and `Vec<bool>`
   visited set, streaming completed labels into the terminal front. Its working memory is
   proportional to path depth plus the terminal front rather than all generated prefixes.
   Approximate mode retains arena entries because deterministic bag truncation needs a
   live candidate set.
3. Reject an extension whose target node is already visited.
4. Retain every intermediate label; do not infer problem identity from label equality.
5. In exact mode, remove hop and bag truncation entirely.
6. In approximate mode, apply configured limits and notify `SearchTracker` whenever a
   candidate is skipped or evicted because of a limit.
7. Filter completed destination labels by `final_dominates`, including equality, and
   retain deterministic representatives.
8. Keep scalar `cost()` as agenda ordering only. It never proves intermediate dominance or
   completeness.

The kernel returns its destination front plus tracker outcome; wrapper APIs perform
domain-specific final sorting and deduplication.

### M3 — Label domains (`src/rules/pareto.rs`, in place)

Purpose: define domain-specific extension and terminal dominance, not resource limits.

- `CostLabel`: componentwise `(accumulated cost, predicted size) <=` is terminal-only.
- `GrowthLabel`: fieldwise asymptotic `<=` is terminal-only.
- `MeasuredLabel`: remains outside `PathLabel`; no concrete dominance is introduced.

Global `HOP_CAP` and `BAG_CAP` exports are removed. An interactive legacy preset may
live beside `SearchLimits`, for example `SearchLimits::interactive()`, containing the
old 16/32 values and no timeout.

### M4 — Public graph APIs (`src/rules/graph.rs` and `src/rules/mod.rs`)

Purpose: make completeness impossible to omit at the Rust call site.

The following APIs gain an explicit `search_mode: SearchMode` and return
`SearchOutcome<...>`:

```rust
find_cheapest_path(...)              -> SearchOutcome<Option<ReductionPath>>
find_cheapest_path_mode(...)         -> SearchOutcome<Option<ReductionPath>>
asymptotic_front(...)                -> SearchOutcome<Vec<(ReductionPath, GrowthLabel)>>
find_measured_best_path(...)         -> SearchOutcome<Option<MeasuredPath>>
find_measured_best_path_to_name(...) -> SearchOutcome<Option<MeasuredPath>>
```

No `Default` implementation is provided for `SearchMode`: callers must choose. Domain
configuration (`ReductionMode`, source size, measured budget) remains separate.

Measured search to any target variant shares one `SearchTracker`; counters and timeout
must not reset for every variant. Prefer one traversal with a target-node predicate so
common prefixes are constructed once. If the implementation keeps per-variant
traversals, they must share limits and aggregate statistics exactly.

### M5 — Consumers

#### ILP solver

- Preferred shortest formulation: `Approximate(Bounded(interactive limits))`.
- Execution-aware fallback before `NoReductionPath`: `Exact` measured search.
- A preferred formulation that constructs and solves remains sufficient; the solver is
  not required to prove the smallest formulation.

#### CLI

Use a typed Clap value enum:

```text
--search-mode exact|approximate
```

Interactive default: `approximate` with the legacy 16-hop/32-label count limits and no
timeout. Limit flags are accepted only with approximate mode:

```text
--max-hops
--max-labels-per-node
--max-expanded-states
--timeout
```

Human output prints a warning only when completeness is approximate. JSON always
includes `completeness`, `limit_reasons`, and `stats`.

#### MCP

Request schemas mirror `search_mode` and bounded limits. Responses always include
structured completeness and stats. Unknown enum values fail schema validation rather
than silently selecting a default.

### M6 — Documentation and migration

- Update this design's predecessor where it describes deterministic caps as part of the
  core Pareto algorithm.
- Update rustdoc with the exact elementary-path and approximate best-so-far contracts.
- Migrate every library, test, example, CLI, MCP, and solver call site explicitly.
- Document that formula exactness is exact for the formula model, not concrete target
  size, and that measured exactness is conditional on its intermediate budget.

## Technical approaches considered

### Exact termination

**Chosen: elementary paths without intermediate pruning.** This is finite, matches
current path-enumeration semantics, and requires no assumption that label summaries
identify constructed problems or that reduction overheads preserve an order.

Alternatives:

- Remove caps and allow walks: rejected because incomparable or zero-growth cycles can
  create unbounded labels without a no-beneficial-cycle theorem.
- Keep a graph-wide hop bound in exact mode: rejected because no theorem establishes a
  universal constant smaller than the number of variant nodes.
- Enumerate and store all simple paths before filtering: semantically equivalent but uses
  exponential result memory; the chosen exact DFS filters terminal labels as it goes.

### API compatibility

**Chosen: breaking explicit mode parameters.** The crate is 0.x, the current contract is
misleading, and an implicit wrapper would preserve that ambiguity.

Alternatives:

- Keep old APIs defaulting to approximate: rejected because callers can still consume an
  incomplete result unknowingly.
- Keep old APIs defaulting to exact: rejected because it silently changes latency and
  memory behaviour.

### Approximation representation

**Chosen: one `Approximate(ApproximationPolicy)` top-level variant.** Bounded best-effort
search is the initial policy; epsilon approximation can be added without creating a
third completeness mode.

Alternatives:

- `Exact | Bounded | EpsilonApproximate`: rejected because bounding is a mechanism, while
  exact versus approximate is the user-facing guarantee.
- A boolean `exact`: rejected because it cannot carry limits and ages poorly as policies
  grow.

### Limit accounting

**Chosen: one tracker per public search request.** It produces honest aggregate status
across target variants and keeps limit checks consistent.

Alternatives:

- Per-target counters: rejected because a request could exceed its advertised limits by
  the number of target variants.
- Global mutable counters: rejected because they break reentrancy and concurrency.

## Quality requirements

### Correctness

- Exact mode never invokes a configurable truncation path.
- Exact mode performs no intermediate eviction or coalescing.
- Exact mode does not retain completed or dead path prefixes outside the terminal front.
- Strict dominance is applied only to completed destination labels.
- Every approximate truncation records a reason before its candidate is discarded.
- Approximate outcomes upgrade to `Exact` when no limit affects exploration.
- Returned paths are always feasible under their reduction capability and domain
  constraints, regardless of completeness.

### Determinism

- Edge order, agenda tie-breaks, terminal representatives, bag truncation, and
  reason ordering are deterministic.
- Count-limited searches are byte-stable across Linux and macOS.
- Timeout-limited searches make no cross-machine byte-stability promise and say so in
  their outcome.

### Performance

- Approximate interactive defaults preserve or improve current CLI latency.
- Exact tests use hand-built graphs that establish correctness without exponential test
  fixtures.
- Visited state adds no external dependency and remains proportional to graph node count
  per live label.

### Rust API quality

- Use enums instead of boolean mode flags.
- Use `Duration`, `Instant`, and typed outcome/reason values instead of unit-ambiguous
  integers or strings.
- Mark `SearchOutcome` as `#[must_use]`.
- Do not use global mutable policy or thread-local search state.
- Keep public intent immutable; mutable counters live in an internal tracker.
- Document failure/completeness semantics in rustdoc and serialize structured fields for
  non-Rust consumers.

### Compatibility

- The Rust API break is deliberate and all repository call sites migrate in one change.
- CLI and MCP response additions are structured; existing path fields retain their
  meaning.
- No reduction rule, model, or overhead declaration changes.

## Verification design

Add one hand-built regression fixture that contains both old failure modes:

1. A unique source-to-target path with 17 edges.
2. A second branch whose hub receives at least 33 pairwise-incomparable labels, with the
   true target winner deliberately ordered after the first 32.

The fixture drives one contract test:

```text
test_search_mode_exact_and_approximate_contract
```

Assertions:

- Exact finds the 17-edge path and the post-32 winner and reports `Exact`.
- Approximate with `max_hops = 16` does not claim the long path and reports
  `HopLimit`.
- Approximate with `max_labels_per_node = 32` reports `LabelsPerNodeLimit` and never
  reports `Exact`.
- Approximate limits larger than the fixture require reports `Exact` and returns the
  same value as Exact mode.
- Reversing equivalent-edge insertion order does not change the terminal representative or
  serialized outcome.

Add focused tests proving equal coarse intermediate labels remain distinct,
non-monotone overhead order reversal, Growth terminal equality, timeout/state accounting,
measured shared limits, and CLI/MCP serialization. Run the repository's normal
`make check` after the contract test.

## Out of scope

- Proving or implementing an epsilon approximation ratio.
- Changing concrete reduction failure from panic to `Result`.
- Interrupting an in-progress reduction constructor on timeout.
- Guaranteeing that measured budgets prevent allocation failure.
- Changing the `Growth` abstract domain, its sound widening, or overhead grammar.
