# Types and Interfaces Redesign (v1 Contract) - Full Implementation Plan

**Date:** 2026-02-11  
**Status:** Draft for alignment  
**Decision Context:** Full redesign, spec-first, with priorities:
- Internal architecture purity
- Long-term API stability

---

## 1. Problem Statement

The current crate works functionally, but contributor cognitive load is higher than necessary because core concepts are spread across overlapping abstractions and repeated implementation patterns:
- Graph concepts are split across two systems (`src/topology` runtime graph trait and `src/graph_types` marker hierarchy).
- Variant identity is stringly typed (`Vec<(&'static str, &'static str)>`) and repeated manually.
- Numeric and objective bounds are repeated in many model impls instead of centralized semantic aliases.
- Contributor docs and architecture docs are partially out of sync with actual trait shapes, which increases onboarding friction.
- Reduction metadata registration, variant keys, and runtime path discovery depend on conventions that are not codified in one stable contract.

This plan defines a **new v1 concept contract** first, then migrates implementation to match that contract while preserving predictable, durable API surfaces.

---

## 2. Goals and Non-Goals

### Goals

1. Define a single canonical conceptual model for problems, variants, assignments, evaluation, and reductions.
2. Replace duplicated and stringly-typed interfaces with typed primitives and semantic bounds.
3. Unify graph type taxonomy and runtime graph interface into one coherent model.
4. Publish a stable v1 API boundary that contributors can learn once and reuse.
5. Reduce boilerplate for new model/reduction implementation by at least 30% (measured by repeated trait-bound and variant code removal in representative models).
6. Align code, docs, and contributor templates with one source of truth.

### Non-Goals

1. Adding new reduction algorithms.
2. Optimizing solver performance beyond interface-driven refactors.
3. Preserving old API names in perpetuity (this is a redesign; compatibility shims are temporary and explicitly time-bounded).

---

## 3. Approaches Considered

### A. Spec-first (Chosen)

Define and freeze a new v1 contract first, then migrate internals and implementations to it.

Pros:
- Strongest API stability outcome.
- Internal refactors stay accountable to a fixed conceptual target.
- Cleaner contributor docs and templates from day one of migration.

Cons:
- Requires upfront design and contract test scaffolding.
- Initial velocity lower before migration starts.

### B. Refactor-first

Clean internals first, finalize API contract later.

Pros:
- Faster early code movement.

Cons:
- High risk of API drift during migration.
- Docs and contributor guidance stay unstable longer.

### C. Parallel API + internals iteration

Evolve contract and internals simultaneously.

Pros:
- Potentially shorter elapsed time.

Cons:
- Hardest to reason about stability.
- Highest review complexity and rework risk.

**Recommendation:** A (Spec-first), already aligned with project decisions.

---

## 4. v1 Contract (Frozen Before Migration)

### 4.1 Core Concepts

1. `ProblemSpec`: static identity and variant dimensions.
2. `ProblemInstance`: runtime instance data and evaluation behavior.
3. `Assignment`: typed wrapper for decision vectors.
4. `Evaluation<T>`: objective value + feasibility.
5. `Reduction<S, T>`: transform + extraction mapping.
6. `ObjectiveValue`: semantic alias for objective numeric bounds.

### 4.2 Contract Sketch (Target Public Surface)

```rust
pub trait ObjectiveValue:
    Clone + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static {}

pub struct Assignment {
    values: Vec<usize>,
}

pub struct Evaluation<T> {
    pub objective: T,
    pub feasible: bool,
}

pub enum ObjectiveSense {
    Maximize,
    Minimize,
}

pub enum VariantKey {
    Graph,
    Weight,
    ConstParam(&'static str),
    Domain(&'static str),
}

pub struct VariantDimension {
    pub key: VariantKey,
    pub value: &'static str,
}

pub trait ProblemSpec {
    const NAME: &'static str;
    type Value: ObjectiveValue;
    fn objective_sense() -> ObjectiveSense;
    fn variant() -> &'static [VariantDimension];
}

pub trait ProblemInstance: Clone + ProblemSpec {
    fn num_variables(&self) -> usize;
    fn num_flavors(&self) -> usize;
    fn size_profile(&self) -> ProblemSize;
    fn evaluate(&self, assignment: &Assignment) -> Evaluation<Self::Value>;
}

pub trait Reduction<S: ProblemInstance, T: ProblemInstance>: Clone {
    fn target(&self) -> &T;
    fn project_solution(&self, target_solution: &Assignment) -> Assignment;
    fn source_size(&self) -> ProblemSize;
    fn target_size(&self) -> ProblemSize;
}
```

Note: exact names may differ, but concept roles and boundaries are frozen in Phase 0.

### 4.3 Graph Model Unification

Unify `src/topology/*` and `src/graph_types.rs` into:
- A single runtime graph trait (`topology::Graph` stays canonical).
- A typed graph taxonomy (`GraphKind`, subtype registry) colocated under `topology` and used by variant metadata + reduction applicability.

Remove duplicate marker-only graph hierarchy once adapters are no longer needed.

### 4.4 Registry and Macro Contract

`#[reduction]` macro output must target typed v1 variant metadata and stable registry records. Registry data should be normalized around:
- source spec id
- target spec id
- typed variant dimensions
- overhead polynomial metadata
- module path

---

## 5. Execution Plan (Phased)

## Phase 0 - Freeze Contract and Guardrails

### Deliverables

1. v1 contract RFC in repo docs.
2. Contract tests that assert semantics independent of specific problems.
3. Migration tracking checklist.

### File Targets

- `docs/plans/2026-02-11-types-interfaces-redesign-design.md` (this file)
- New: `docs/architecture/v1-contract.md`
- New: `src/unit_tests/contracts/*`

### Exit Criteria

1. Contract reviewed and approved.
2. No migration coding starts before contract tests compile.

---

## Phase 1 - Introduce v1 Core Modules (Additive)

### Deliverables

1. New core module layout:
- `src/core/objective.rs`
- `src/core/assignment.rs`
- `src/core/evaluation.rs`
- `src/core/variant.rs`
- `src/core/problem.rs`
- `src/core/reduction.rs`
2. Re-export strategy in `src/lib.rs` and `prelude`.

### Work

1. Implement v1 types and traits.
2. Keep existing traits (`Problem`, `ReduceTo`, `ReductionResult`) operational through adapters.
3. Add conversion helpers:
- `impl From<Vec<usize>> for Assignment`
- `Assignment::as_slice()`
- mapping between `SolutionSize<T>` and `Evaluation<T>`

### Exit Criteria

1. Build passes with both old and new abstractions present.
2. Contract tests pass for dummy test problems.

---

## Phase 2 - Typed Variant and Graph Taxonomy Migration

### Deliverables

1. Typed variant dimension model used in new APIs.
2. Unified graph subtype registry integrated into topology.

### Work

1. Move graph subtype entries into `topology` namespace.
2. Replace raw `("graph", "...")` string pairs in new interfaces with `VariantDimension`.
3. Add normalization utilities to preserve current variant-id output for graph export.
4. Adapt reduction graph applicability checks to use unified graph taxonomy.

### Exit Criteria

1. `ReductionGraph` behavior unchanged on existing test suite.
2. JSON graph export remains schema-compatible unless explicitly versioned.

---

## Phase 3 - Macro and Registry Refactor

### Deliverables

1. `problemreductions-macros` emits v1-compatible registry records.
2. Registry structs updated for typed variant dimensions.

### Work

1. Extend proc-macro parsing for explicit variant dimension overrides where needed.
2. Keep fallback inference for graph/weight types but write into typed fields.
3. Introduce registry version marker to support transitional readers.

### Exit Criteria

1. All existing `#[reduction]` impls compile without behavior regressions.
2. Inventory registration tests pass with typed variant metadata.

---

## Phase 4 - Model Migration (Representative, then Bulk)

### Deliverables

1. Migrate representative models first:
- `MaximumIndependentSet`
- `MinimumVertexCover`
- `QUBO`
- `Satisfiability`
2. Bulk migration for remaining models.

### Work

1. Replace repeated numeric bounds with `ObjectiveValue`.
2. Replace `solution_size(&[usize])` internals with assignment-based evaluation.
3. Standardize constructor naming and accessor patterns where inconsistent.
4. Remove per-model ad hoc variant string construction in favor of shared helpers.

### Exit Criteria

1. Representative model migration merged with no external behavior change.
2. Full model set migrated and old trait impls available through adapters only.

---

## Phase 5 - Reduction and Solver API Migration

### Deliverables

1. New reduction interface wired through all rules.
2. Solver trait supports assignment/evaluation primitives natively.

### Work

1. Update `src/rules/traits.rs` and rule implementations to v1 concepts.
2. Add solver overloads:
- `find_best_assignment`
- compatibility wrappers for old methods.
3. Ensure extraction and closed-loop reduction tests run on assignment wrappers.

### Exit Criteria

1. All rule unit tests pass.
2. End-to-end examples still compile and produce equivalent outputs.

---

## Phase 6 - Documentation and Contributor UX Overhaul

### Deliverables

1. Rewrite architecture and contributor docs to match v1 exactly.
2. Add model/reduction templates with new interfaces.

### Work

1. Update:
- `docs/src/arch.md`
- `docs/src/claude.md`
- `.claude/rules/adding-models.md`
- `.claude/rules/adding-reductions.md`
2. Add “minimal contributor path” docs:
- Implementing a new model in under N files.
- Implementing a new reduction with template + checklist.
3. Add lint/check script to detect docs drift against trait signatures.

### Exit Criteria

1. New contributor workflow tested by implementing one toy model and one toy reduction from templates.
2. No stale references to removed APIs in docs.

---

## Phase 7 - Compatibility Window, Deprecation, and Removal

### Deliverables

1. Explicit compatibility window (one release cycle).
2. Removal plan and changelog for old traits/interfaces.

### Work

1. Mark old traits/types deprecated with actionable migration messages.
2. Provide migration guide with old-to-new mapping table.
3. Remove deprecated layers at cutoff release.

### Exit Criteria

1. Migration guide complete and validated against all examples.
2. Deprecated API removal branch passes full CI.

---

## 6. Data Flow (v1 Target)

1. Instance creation:
- User constructs `ProblemInstance`.
2. Introspection:
- `ProblemSpec` exposes name + typed variant dimensions.
3. Evaluation:
- Solver builds `Assignment`, invokes `evaluate`, reads `Evaluation`.
4. Reduction:
- `Reduction<S,T>` exposes target instance and solution projection.
5. Registry/graph:
- Typed variant dimensions flow into reduction graph applicability and exports.

This flow replaces ad hoc mixing of metadata strings, raw vectors, and separate graph marker systems.

---

## 7. Error Handling Strategy

1. Introduce v1 error categories:
- `InvalidAssignment`
- `VariantMismatch`
- `ReductionNotApplicable`
- `SchemaVersionMismatch`
2. Keep error payloads structured and contributor-friendly.
3. Ensure conversion from legacy errors to v1 errors during migration window.
4. Add explicit validation entry points so panics become typed errors in external-facing paths where feasible.

---

## 8. Testing Strategy

## 8.1 Contract Tests

1. `Assignment` length and flavor validation.
2. Objective sense ordering semantics.
3. Variant dimension stability and serialization.
4. Reduction projection round-trip invariants.

## 8.2 Migration Safety Tests

1. Golden tests comparing old vs new evaluation for representative problems.
2. Golden tests for reduction graph node/edge identity.
3. Macro expansion tests for registry metadata.

## 8.3 End-to-End Tests

1. Existing example runs remain green.
2. Exported graph/schema artifacts diff-only where versioned changes are intended.
3. Solver outputs and objective values unchanged for current fixtures.

---

## 9. Work Breakdown and Milestones

1. M1: Contract frozen + tests scaffolded.
2. M2: Core v1 modules merged.
3. M3: Typed variant + unified graph taxonomy merged.
4. M4: Macro/registry migrated.
5. M5: Models migrated.
6. M6: Rules and solvers migrated.
7. M7: Docs/templates fully aligned.
8. M8: Deprecation cutoff and cleanup complete.

Each milestone requires green `make test clippy doc` and targeted artifact checks (`export-graph`, `export-schemas`).

---

## 10. Risk Register and Mitigations

1. **Risk:** Graph taxonomy migration breaks reduction applicability.  
   **Mitigation:** Keep compatibility adapter and run old/new applicability parity tests.

2. **Risk:** Macro inference regressions for generic reductions.  
   **Mitigation:** Add fixture-based macro expansion tests across representative reduction signatures.

3. **Risk:** Contributor confusion during transition window.  
   **Mitigation:** Keep one canonical v1 docs path and clearly label legacy docs as deprecated.

4. **Risk:** Hidden reliance on string variant keys in downstream tooling.  
   **Mitigation:** Keep stable serialized shape with explicit schema version and compatibility serializer.

---

## 11. Implementation Order Recommendation (PR Slices)

1. PR1: v1 core types + contract tests (no model migration).
2. PR2: typed variant + graph taxonomy unification + reduction graph parity tests.
3. PR3: macro/registry migration.
4. PR4: representative model migrations.
5. PR5: bulk model + rule migrations.
6. PR6: solver migration + compatibility shims.
7. PR7: docs/templates overhaul.
8. PR8: deprecation removals.

This keeps review units coherent and allows rollback at milestone boundaries.

---

## 12. Success Criteria

1. New contributors can implement a new model and reduction by following one v1 guide without reading legacy traits.
2. Public v1 interfaces remain stable throughout migration after M1 freeze.
3. Duplicate graph concept systems are removed.
4. Boilerplate trait-bound duplication is materially reduced.
5. All existing test suites and exports are green at completion.

---

## 13. Immediate Next Step (No Code Refactor Yet)

Create and review `docs/architecture/v1-contract.md` as the normative contract artifact, then start PR1 with contract tests and additive core modules.

