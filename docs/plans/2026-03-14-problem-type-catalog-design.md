# Problem Type Catalog Design

## Goal

Make adding a new model or reduction rule closer to a local change, while preserving the repo's current mathematical explicitness and runtime guarantees.

The design should reduce duplicated metadata across CLI naming, variant resolution, canonical examples, and documentation-facing export, without weakening the existing type-level model implementations or reduction registry.

## Current Pain Points

Today the same conceptual object is represented in several different places:

- The Rust model type implements `Problem` and declares `NAME` plus `variant()`.
- CLI naming and aliases are maintained separately in `problemreductions-cli/src/problem_name.rs`.
- Canonical examples are maintained centrally in `src/example_db/model_builders.rs` and `src/example_db/rule_builders.rs`.
- Default variant selection is derived from the reduction graph.
- Exported identities use `export::ProblemRef`, which is just `{ name, variant }` with no schema validation.

This is rigorous in the sense that the repo is explicit, but not minimal in the contributor workflow. Adding a new problem or rule usually requires touching several parallel metadata surfaces.

## Non-Goals

This design does not try to:

- remove or replace the existing generic Rust problem structs such as `MaximumIndependentSet<G, W>`
- replace the reduction inventory mechanism
- generate theorem prose or paper text automatically
- eliminate explicit examples or explicit defaults

The goal is to concentrate metadata ownership, not to hide semantics behind macros or code generation.

## Recommended Direction

Introduce a canonical problem type catalog that owns:

- canonical type identity
- aliases
- declared variant dimensions and defaults
- validation for runtime references
- references to canonical examples

Keep the current typed model implementations and the current reduction graph. The catalog sits beside them and becomes the single metadata layer used by CLI parsing, export lookup, example lookup, and future docs tooling.

## Decisions Locked In

This design assumes the following decisions:

- the catalog is the source of truth for variant schema
- the reduction graph is the source of truth for variant reachability
- example registration starts with explicit per-module collection, not inventory
- exact `(source_ref, target_ref)` endpoint pairs are the primitive rule identity
- docs and paper metadata remain outside the catalog
- `Problem::NAME` is kept only as a migration bridge, then removed in the final cleanup step

These are treated as design constraints below, not open questions.

## Core Concepts

### 1. ProblemType

`ProblemType` is the canonical named family currently informally represented by `Problem::NAME` plus alias tables plus default-variant logic.

Example:

```rust
pub struct ProblemType {
    pub canonical_name: &'static str,
    pub display_name: &'static str,
    pub aliases: &'static [&'static str],
    pub dimensions: &'static [VariantDimension],
}
```

This is not the concrete Rust implementation type. It is the runtime/catalog identity for a mathematical problem family such as Maximum Independent Set.

### 2. VariantDimension

Each problem type declares its allowed dimensions in schema form.

```rust
pub struct VariantDimension {
    pub key: &'static str,
    pub default_value: &'static str,
    pub allowed_values: &'static [&'static str],
}
```

For `MaximumIndependentSet`, that would mean something like:

- `graph`: default `SimpleGraph`
- `weight`: default `One`

This removes the need for CLI code to guess defaults by looking at graph ordering.

### 2a. Schema Validity vs Graph Reachability

The design treats these as different concepts.

- Schema-valid means a variant is allowed by the problem type's declared dimensions.
- Graph-reachable means a concrete variant currently exists as a node in the reduction graph.

Example:

- `MaximumIndependentSet` may declare `graph in {SimpleGraph, UnitDiskGraph, PlanarGraph}`
- and `weight in {One, i32}`
- then `MaximumIndependentSet/PlanarGraph/i32` is schema-valid
- but it is graph-reachable only if a concrete node for that variant is currently registered in the reduction graph

This separation is important because different subsystems need different notions of validity:

- CLI parsing and typed reference construction should validate against schema
- reduction queries, path search, and graph visualization should validate against reachability

The catalog answers "is this a well-formed variant of this problem type?"

The reduction graph answers "does this concrete variant currently participate in the reduction system?"

### 3. Typed ProblemRef

The current exported `ProblemRef` is just strings:

```rust
pub struct ProblemRef {
    pub name: String,
    pub variant: BTreeMap<String, String>,
}
```

Internally, that should become a validated type:

```rust
pub struct ProblemRef<'a> {
    pub problem_type: &'a ProblemType,
    pub variant: VariantSpec,
}
```

`VariantSpec` remains a map-like representation, but it is created only through validation against the owning `ProblemType`.

Properties:

- all keys are known dimensions for that problem type
- all values are allowed for that dimension
- omitted dimensions are filled from declared defaults
- equality is canonicalized

The current JSON/export `ProblemRef` can remain as an external DTO. The typed `ProblemRef` becomes the internal runtime representation.

### 4. Declarative Example Specs

Examples should be declared close to the owning model or rule, then assembled centrally.

Instead of keeping a giant hand-maintained `build_model_examples()` list and `build_rule_examples()` list, use declarative registrations such as:

```rust
pub struct ModelExampleSpec {
    pub id: &'static str,
    pub problem: ProblemRefLiteral,
    pub build: fn() -> ModelExample,
}

pub struct RuleExampleSpec {
    pub id: &'static str,
    pub source: ProblemRefLiteral,
    pub target: ProblemRefLiteral,
    pub build: fn() -> RuleExample,
}
```

The actual example payloads stay explicit. The change is only in where they are declared and how they are indexed.

The first implementation should use explicit per-module collection rather than `inventory` for examples. That keeps the migration conservative and debuggable.

## Ownership Boundaries

The design is intentionally split by responsibility:

- `Problem` trait and generic Rust model types: implementation-level semantics
- `ProblemType` catalog: naming, defaults, variant schema, alias resolution
- reduction graph: reachability, variant nodes, path analysis
- example DB: canonical witness data indexed by typed refs
- export layer: JSON DTOs

This is the main simplification. Right now these concerns leak into one another.

## How Contributor Workflow Changes

### Adding a New Model

Current shape:

- define the model type
- declare variants
- add aliases in CLI code
- add canonical example in the central builder list
- sometimes update docs/paper metadata manually

Target shape:

- define the model type
- declare one local `ProblemType` registration
- optionally declare one local canonical model example

Everything else should be assembled or validated from those declarations.

### Adding a New Rule

Current shape:

- implement the reduction
- ensure the reduction registry sees it
- add a canonical rule example in a central list
- often maintain theorem/docs metadata separately

Target shape:

- implement the reduction
- declare one local exact `(source_ref, target_ref)` reduction registration
- optionally declare one local canonical rule example

This is still explicit, but it becomes much closer to a local edit.

## Rule Identity

The current system already traverses the graph by exact source and target variants. This design makes that the explicit identity model for primitive reductions.

The invariant is:

```rust
there is at most one primitive reduction registration for each exact
(source_problem_ref, target_problem_ref) endpoint pair
```

Why:

- graph traversal and overhead lookup already operate on exact endpoints
- shared implementation code can still be reused behind multiple wrapper impls
- contributors do not need to maintain a second rule-identity namespace

If the repo ever wants multiple primitive constructions with the same exact endpoints, this design would need to be revisited. For now, the simpler invariant is preferred.

## Migration Strategy

### Phase 1: Catalog Without Behavioral Change

- add `ProblemType`, `VariantDimension`, and typed internal `ProblemRef`
- populate catalog entries for existing problems
- keep existing `Problem::NAME`, `variant()`, and reduction graph behavior
- require `Problem::NAME` to match the catalog canonical name during the migration
- make CLI alias/default resolution read from the catalog instead of local tables

This phase should not change reduction execution.

### Phase 2: Typed Example Indexing

- convert example DB lookup to use typed refs internally
- keep existing JSON format externally
- replace central variant matching heuristics with catalog validation

This removes a large class of stringly-typed ambiguity.

### Phase 3: Declarative Example Registration

- move model example declarations near their owning models
- move rule example declarations near their owning rules
- have `example_db` assemble the final database from explicit per-module registrations

This is the step that materially reduces extension friction.

### Phase 4: Remove `Problem::NAME`

- move remaining internal call sites from `Problem::NAME` to catalog-backed type identity
- add a direct bridge from implementation types to their `ProblemType`
- delete `Problem::NAME` once export, CLI, example DB, and registry call sites no longer depend on it

This is the final cleanup step. It is intentionally delayed so the architectural migration stays reviewable and behavior-preserving until the end.

## Invariants To Enforce

The catalog layer should validate the following:

- canonical problem names are unique
- aliases are globally unique
- every dimension key is unique within a problem type
- every default value is contained in its dimension's allowed values
- every example references a valid problem type and valid variant
- every rule example references a declared exact `(source_ref, target_ref)` pair
- exported DTOs round-trip through typed refs without loss

These checks should run in normal CI, not behind an infrequently used feature gate.

## Main Benefits

- localizes the metadata needed to add a new problem or rule
- removes duplicated alias/default logic from CLI code
- makes runtime references mathematically cleaner and less stringly-typed
- preserves explicit examples and explicit defaults
- creates a stable basis for future docs/export tooling

## Main Costs

- introduces a second layer beside the `Problem` trait, which must be kept conceptually clear
- requires migration effort across CLI and example DB code
- may expose mismatches between declared type-level variants and currently reachable graph variants

## Remaining Design Risks

These are implementation risks, not unresolved product decisions:

- the catalog schema can drift from the type-level variant declarations unless CI checks both representations against each other
- the repo may currently rely on reduction-graph node existence in places that should really accept any schema-valid `ProblemRef`
- some CLI flows, especially `pred create`, may need a mixed strategy because construction support is narrower than schema validity
- removing `Problem::NAME` in the last step will touch many files at once, so that final cleanup should happen only after the catalog bridge is already stable

## Recommended First Slice

If this design is accepted, the first implementation slice should be:

1. Add `ProblemType` catalog definitions for existing problems.
2. Move alias and default-variant parsing in CLI to the catalog.
3. Introduce a typed internal `ProblemRef` plus conversion to and from export DTOs.
4. Leave example declaration migration for a second pass.

That gets most of the rigor benefit without immediately forcing a large example-system rewrite.
