# Redundant Rule Removal Proposal

**Date:** 2026-03-09
**Issue:** #193
**Status:** Proposal only

## Goal

Remove a small set of primitive rules that are redundant under currently trusted symbolic comparisons, while preserving:
- at least one canonical direct path for common target families (`ILP`, `QUBO`)
- existing educational value where the direct encoding is materially clearer
- soundness with respect to the known `ILP -> QUBO` metadata gap

## Recommendation

Adopt a **conservative removal set** rather than removing every rule listed in issue #193.

### Remove

1. `MinimumVertexCover -> ILP`
   - canonical replacement: `MinimumVertexCover -> MaximumIndependentSet -> ILP`
   - trusted because both edges have complete symbolic overhead

2. `MinimumVertexCover -> QUBO`
   - canonical replacement: `MinimumVertexCover -> MaximumIndependentSet -> QUBO`
   - trusted because both edges have complete symbolic overhead

3. `MaximumSetPacking -> ILP`
   - canonical replacement: `MaximumSetPacking -> MaximumIndependentSet -> ILP`
   - trusted because both edges have complete symbolic overhead

4. Variant-graph registrations for `KSatisfiability<K2> -> Satisfiability`
   - canonical replacement: `KSatisfiability<K2> -> KSatisfiability<KN> -> Satisfiability`

5. Variant-graph registrations for `KSatisfiability<K3> -> Satisfiability`
   - canonical replacement: `KSatisfiability<K3> -> KSatisfiability<KN> -> Satisfiability`

For `K2` and `K3`, keep the concrete `ReduceTo<Satisfiability>` impls if tests or direct typed usage still want them, but stop registering them as primitive graph edges.

### Keep

1. `MaximumIndependentSet -> ILP`
   - keep as the canonical graph/set-to-ILP entry point

2. `MaximumIndependentSet -> QUBO`
   - keep as the canonical graph-to-QUBO entry point

3. `KColoring -> QUBO`
   - do **not** remove
   - the only candidate shortcut in issue #193 uses `KColoring -> ILP -> QUBO`
   - `ILP -> QUBO` is not currently trustworthy for symbolic shortcut analysis because slack growth is not captured by the exposed symbolic metadata

4. Variant cast infrastructure
   - do not remove casts just because they create trivial composed paths

## Why this split

The issue’s original list mixes two different situations:

1. **Trusted symbolic redundancies**
   - paths whose composed overhead is representable and comparable with current metadata

2. **Apparent redundancies that rely on incomplete metadata**
   - especially any shortcut through `ILP -> QUBO`

We should only remove rules from category (1).

## Concrete Work Plan

### Phase 1: Remove duplicate KSAT graph registrations

Files:
- Modify `src/rules/sat_ksat.rs`
- Modify tests that assert direct `K2 -> SAT` or `K3 -> SAT` graph edges

Changes:
- Keep the registered `KSatisfiability<KN> -> Satisfiability` edge
- Keep the `K2` and `K3` `ReduceTo<Satisfiability>` impls for typed use
- Remove `#[reduction]` registration from `K2` and `K3`

Acceptance:
- Exact typed reduction still works for `K2` and `K3`
- Variant graph no longer contains separate primitive `K2 -> SAT` and `K3 -> SAT` edges

### Phase 2: Remove `MinimumVertexCover -> QUBO`

Files:
- Delete `src/rules/minimumvertexcover_qubo.rs`
- Remove module registration in `src/rules/mod.rs`
- Remove unit test module `src/unit_tests/rules/minimumvertexcover_qubo.rs`
- Remove example `examples/reduction_minimumvertexcover_to_qubo.rs`
- Update `tests/suites/examples.rs`
- Update paper entry `docs/paper/reductions.typ`

Changes:
- Make `MinimumVertexCover -> MaximumIndependentSet -> QUBO` the canonical route
- Replace the direct example with either:
  - a chained reduction example, or
  - no dedicated example if MIS->QUBO already covers the penalty-method exposition sufficiently

Acceptance:
- No direct MVC->QUBO edge remains in the graph
- `ReductionGraph` still finds a route from MVC to QUBO
- docs/examples no longer reference the deleted primitive rule

### Phase 3: Remove `MinimumVertexCover -> ILP`

Files:
- Delete `src/rules/minimumvertexcover_ilp.rs`
- Remove module registration in `src/rules/mod.rs`
- Remove unit test module `src/unit_tests/rules/minimumvertexcover_ilp.rs`
- Remove example `examples/reduction_minimumvertexcover_to_ilp.rs`
- Update `tests/suites/examples.rs`
- Update paper entry `docs/paper/reductions.typ`

Changes:
- Make `MinimumVertexCover -> MaximumIndependentSet -> ILP` the canonical route

Acceptance:
- No direct MVC->ILP edge remains in the graph
- `ReductionGraph` still finds a route from MVC to ILP

### Phase 4: Remove `MaximumSetPacking -> ILP`

Files:
- Delete `src/rules/maximumsetpacking_ilp.rs`
- Remove module registration in `src/rules/mod.rs`
- Remove unit test module `src/unit_tests/rules/maximumsetpacking_ilp.rs`
- Remove example `examples/reduction_maximumsetpacking_to_ilp.rs`
- Update `tests/suites/examples.rs`
- Update paper entry `docs/paper/reductions.typ`

Changes:
- Make `MaximumSetPacking -> MaximumIndependentSet -> ILP` the canonical route

Acceptance:
- No direct MaxSetPacking->ILP edge remains in the graph
- `ReductionGraph` still finds a route from MaximumSetPacking to ILP

## Explicit Non-Removal

Do not remove:
- `src/rules/coloring_qubo.rs`
- `src/rules/maximumindependentset_ilp.rs`
- `src/rules/maximumindependentset_qubo.rs`

Reason:
- these are the remaining canonical direct formulations for their target families
- removing them would either rely on the untrusted `ILP -> QUBO` symbolic shortcut or make the graph harder to understand

## Acceptance Criteria

After all phases:
- the removed primitive rules are absent from the reduction graph
- the graph still provides routes for the affected source/target pairs through composed paths
- examples, tests, and paper entries are updated to match the surviving canonical rules
- issue #193’s detector should no longer report the removed trusted redundancies
- `KColoring -> QUBO` remains present and is not treated as removable until `ILP -> QUBO` metadata is fixed
