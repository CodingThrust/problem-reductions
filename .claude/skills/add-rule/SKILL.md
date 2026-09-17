---
name: add-rule
description: Use when adding a new reduction rule to the codebase, either from an issue or interactively
---

# Add Rule

Step-by-step guide for adding a new reduction rule (A -> B) to the codebase. By default, every rule goes through mathematical verification (via `/verify-reduction`) before implementation. Pass `--no-verify` to skip verification for trivial reductions.

## Invocation

```
/add-rule                     # interactive, with verification (default)
/add-rule --no-verify         # interactive, skip verification
```

When called from `/issue-to-pr`, the `--no-verify` flag is passed through if present.

## Step 0: Gather Required Information

Before any implementation, collect all required information. If called from `issue-to-pr`, the issue should already provide these. If used standalone, brainstorm with the user to fill in every item below.

### Required Information Checklist

| # | Item | Description | Example |
|---|------|-------------|---------|
| 1 | **Source problem** | The problem being reduced FROM (must already exist) | `MinimumVertexCover<SimpleGraph, i64>` |
| 2 | **Target problem** | The problem being reduced TO (must already exist) | `MaximumIndependentSet<SimpleGraph, i64>` |
| 3 | **Reduction algorithm** | How to transform source instance to target | "Copy graph and weights; IS on same graph as VC" |
| 4 | **Solution extraction** | How to map target solution back to source | "Complement: `1 - x` for each variable" |
| 5 | **Correctness argument** | Why the reduction preserves optimality | "S is independent set iff V\S is vertex cover" |
| 6 | **Parameter transform** | How target size relates to source size | `num_vertices = "num_vertices", num_edges = "num_edges"` |
| 7 | **Concrete example** | A small worked-out instance (tutorial style, clear intuition) | "Triangle graph: VC={0,1} -> IS={2}" |
| 8 | **Solving strategy** | How to solve the target problem | "BruteForce, or existing ILP reduction" |
| 9 | **Reference** | Paper, textbook, or URL for the reduction | URL or citation |

If any item is missing, ask the user to provide it. Put a high standard on item 7 (concrete example): it must be in tutorial style with clear intuition and easy to understand. Do NOT proceed until the checklist is complete.

## Step 0.5: Mathematical and API Contract

Read [the canonical complete-result recovery contract](../../../docs/src/design.md#complete-result-recovery).
Resolve the source and target's concrete `Solution` and `Value` types from their
implementations and check the construction, extraction preconditions, and
objective relationship. Different optimization directions or numeric value
types do not by themselves invalidate a witness reduction. Use the existing
complete-result, proof-only, or Turing capability required by the actual operation.
Report a concrete mathematical or Rust implementation mismatch if one exists;
do not apply a wrapper-pair whitelist.

## Arithmetic and Validation

Follow [the canonical arithmetic and boundary policy](../../../docs/src/design.md#arithmetic)
and [validation evidence](../../../docs/src/design.md#validation-evidence).
Derive representation requirements from the source, target, and construction.
Ask for clarification only when the mathematical domain is ambiguous, not to
make the contributor choose Rust types.

Check the construction's actual size arithmetic, coefficients, and auxiliary
identifiers. Preserve target `ConstructionError` as `ReductionError::Construction`
and report reduction arithmetic through `ReductionError`; do not stringify or
silently handle failures. Reuse shared conversion and extraction APIs according
to their contracts. Backend transport limits and precision checks belong to the
adapter, not this rule's applicability domain or mandatory test template.

## Reference Implementations

Read these first to understand the patterns:
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs`
- **Reduction tests:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- **Paper entry:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet`
- **Traits:** `src/rules/traits.rs` (`ReduceTo<T>`, `ReductionResult`)

## Step 1: Mathematical Verification (default, skip with `--no-verify`)

**If `--no-verify` was passed, skip to Step 2.**

Invoke the `/verify-reduction` skill to mathematically verify the reduction before writing Rust code. This runs the full verification pipeline: Typst proof, constructor Python script, independent adversary checks, and cross-comparison with coverage justified by the construction.

All verification artifacts are ephemeral — they exist only in conversation context and temp files. Nothing is committed to the repository.

**Proceed to implementation only when verification reports VERIFIED. For FAILED or INCOMPLETE, report the concrete defect or missing evidence and resolve it before implementing.**

If verification passes, the checked construction, recovery mapping, and YES/NO instances carry forward in conversation context to inform Steps 2-5. Use them as the canonical spec for the Rust `reduce_to()` and `recover_result()` implementation.

## Step 2: Implement the reduction

Create `src/rules/<source>_<target>.rs` (all lowercase, no underscores between words within a problem name):

```rust
// Required structure:
// 1. ReductionResult struct (holds the target problem + mapping state)
// 2. ReductionResult trait impl (target_problem + mandatory recover_result)
// 3. #[reduction(transform = exact { ... })] on ReduceTo impl
// 4. ReduceTo trait impl (reduce_to method)
// 5. #[cfg(test)] #[path = "..."] mod tests;
```

Key elements:

**ReductionResult struct:**
```rust
#[derive(Debug, Clone)]
pub struct ReductionXToY {
    target: TargetType,
    // any additional mapping state needed for recovery
}
```

**ReductionResult trait impl:**

`recover_result` is mandatory; it has no default implementation. For a rule whose
proof establishes all three implications (target optimal -> source optimal,
target feasible -> source feasible, target infeasible -> source infeasible),
explicitly use the internal helper:

```rust
use crate::rules::traits::recover_preserving_status;
use crate::solvers::ProblemOutcome;

impl ReductionResult for ReductionXToY {
    type Source = SourceType;
    type Target = TargetType;
    fn target_problem(&self) -> &Self::Target { &self.target }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| {
            self.map_solution(solution)
        })
    }
}
```

Keep the mathematical mapping in a private `map_solution` method, or inline a
short mapping in the closure. Do not add a forwarding method solely to call the
helper. It preserves the declared status, maps the witness, evaluates the source
candidate once, and propagates mapping/evaluation errors. The old target value
is not reused as the source value. `Infeasible` does not invoke the mapping.

**Choose this helper only when the proof supports all three implications.** It
cannot prove the premise, optimality, or source infeasibility from an invalid
mapped candidate. When recovery needs an optimum threshold, rejects feasible
incumbents, or has another mathematical interpretation, write an explicit
`match target` in `recover_result` instead. Examples: MVC -> FeedbackArcSet
rejects merely feasible targets; ILP -> QUBO interprets the optimum penalty.
Insufficient witness quality returns `ExtractionError::InsufficientSolutionQuality`,
never `SolveOutcome::Infeasible` without a proof.

Follow the canonical [recovery contract](../../../docs/src/design.md#complete-result-recovery).
Document the instance domain, witness premises, source guarantee, and
infeasibility interpretation. Target validation belongs to the solver/transport
boundary; keep the existing source evaluation when constructing recovered
outcomes. Do not duplicate model constraint checks inside the mapping or add
fallbacks for inputs excluded by its premises. Do not introduce default recovery,
policy flags, macros, or a new public mapping trait to remove this explicit choice.

**ReduceTo with `#[reduction]` macro** (a parameter relation is **required**):
```rust
#[reduction(transform = exact {
    field_name = "source_field",
})]
impl ReduceTo<TargetType> for SourceType {
    type Result = ReductionXToY;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        // If Step 1 ran: translate the verified Python reduce() logic
    }
}
```

Each primitive reduction is determined by the exact source/target variant pair. Keep one primitive registration per endpoint pair and declare `transform = exact`, `upper_bound`, or `unavailable` according to the actual parameter relationship; follow `.claude/CLAUDE.md` for metadata requirements.

A construction that cannot recover complete source results must not be registered
as a complete-result edge. Use the existing proof-only or Turing capability when
it describes the actual reduction; do not invent an aggregate-only recovery API.

## Step 3: Register in mod.rs

Add to `src/rules/mod.rs`:
- `mod <source>_<target>;`
- Register native ILP rules normally; there is no ILP solver feature gate.

## Step 4: Write unit tests

Create `src/unit_tests/rules/<source>_<target>.rs`:

**Required: closed-loop test** (`test_<source>_to_<target>_closed_loop`):
```rust
// 1. Create source problem instance
// 2. Reduce: let reduction = ReduceTo::<Target>::reduce_to(&source).unwrap();
// 3. Solve target; wrap each proven optimum with SolveOutcome::optimal(target, solution)
// 4. Recover: reduction.recover_result(&source, target_outcome)
// 5. Verify: extracted solution is valid and optimal for source
```

If Step 1 ran, use the verified YES/NO instances from conversation context to construct test cases. Include feasible and infeasible cases when both exist; for always-feasible optimization models, check the objective relationship instead.

Additional recommended tests:
- Verify target problem structure (correct size, edges, constraints)
- Edge cases (empty graph, single vertex, etc.)
- Weight preservation (if applicable)

Test the mathematical mapping for witnesses satisfying its premises, including all tied optima on suitable small instances. Also exercise feasible incumbents and infeasibility when reachable, checking the rule's declared implications or rejection. Keep necessary parsing/type-conversion tests at the transport boundary.

Link via `#[cfg(test)] #[path = "..."] mod tests;` at the bottom of the rule file.

## Step 5: Add canonical example

Define `canonical_rule_example_specs()` in the rule module and include it from `src/rules/mod.rs::canonical_rule_example_specs()`. This enrolls the rule in shared example checks. Extraction correctness checks use witnesses satisfying the mapping contract; model evaluation retains its own domain checks.

## Step 6: Document in paper (MANDATORY — DO NOT SKIP)

**This step is NOT optional.** Every reduction rule MUST have a corresponding `reduction-rule` entry in the paper. Skipping documentation is a blocking error — the PR will be rejected in review. Do not proceed to Step 6 until the paper entry is written and `make paper` compiles.

Write a `reduction-rule` entry in `docs/paper/reductions.typ`. **Reference example:** search for `reduction-rule("KColoring", "QUBO"` to see the gold-standard entry — use it as a template. For a minimal example, see MinimumVertexCover -> MaximumIndependentSet.

If Step 1 ran, adapt the verified Typst proof into the paper's macros. Do not rewrite the proof from scratch — reformat it.

### 6a. Write theorem body (rule statement)

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [Description ($n = ...$, $|E| = ...$)],
)[
  This $O(...)$ reduction @citation constructs [target structure] ... ($n k$ variables indexed by ...).
]
```

Three parts: complexity with citation, construction summary, overhead hint.

### 6b. Write proof body

Use these subsections with italic labels:

```typst
][
  _Construction._ [Full mathematical construction — enough detail to reimplement]

  _Correctness._ ($arrow.r.double$) If ... ($arrow.l.double$) If ...

  _Variable mapping._ [Only if non-trivial mapping]

  _Solution extraction._ [How to convert target solution back to source]
]
```

Must be self-contained (all notation defined) and reproducible.

### 6c. Write worked example (extra block)

Step-by-step walkthrough with concrete numbers from JSON data. Required steps:
1. Show source instance (dimensions, structure, graph visualization if applicable)
2. Walk through construction with intermediate values
3. Verify a concrete solution end-to-end
4. Witness semantics: state that the fixture stores one canonical witness; if multiplicity matters mathematically, explain it from the construction rather than from `solutions.len()`

Use `graph-colors`, `g-node()`, `g-edge()` for graph visualization — see reference examples.

**Reproducibility:** The `extra:` block must start with a `pred-commands()` call showing the create/reduce/solve/evaluate pipeline. The source-side `pred create --example ...` spec must be derived from the loaded canonical example data via the helper pattern in `write-rule-in-paper`; do not hand-write a bare alias and assume the default variant matches.

### 6d. Build and verify

```bash
make paper     # Must compile without errors
```

Checklist: notation self-contained, complexity cited, overhead consistent, example uses JSON data (not hardcoded), solution verified end-to-end, witness semantics respected, paper compiles.

## Step 7: Regenerate exports and verify

```bash
cargo run --example export_graph    # Generate reduction_graph.json for docs/paper builds
cargo run --example export_schemas  # Generate problem schemas for docs/paper builds
cargo run --features "example-db" --example export_examples
make test clippy                    # Must pass
```

`export_examples` refreshes the gitignored `docs/paper/data/examples.json` used by the paper.

Structural and quality review is handled by the `review-pipeline` stage, not here. The run stage just needs to produce working code.

## Solver Rules

- If the target problem already has a solver, use it directly.
- If the solving strategy requires ILP, implement and register the ILP reduction rule alongside.
- A direct-to-ILP rule is a production reduction, not a stub. Match the completeness bar used by strong ILP reductions in this repo: correct parameter relationships, structure + closed-loop + extraction tests, weighted/infeasible cases and arithmetic regressions justified by the construction, and representative solver integration.
- When this rule is the companion to a `[Model]` issue that explicitly claims ILP solvability, it belongs in the same PR as the model.
- If a custom solver is needed, implement in `src/solvers/` and document.

## CLI Impact

Adding a witness-preserving reduction rule does NOT require CLI changes -- the reduction graph is auto-generated from `#[reduction]` macros and the CLI discovers paths dynamically. However, both source and target models must already be fully registered through their model files (`ProblemSchemaEntry` and `declare_variants!`), including any aliases and `pred create` construction contract (see `add-model` skill).

`ExtractionError` already propagates through `pred extract` and bundle `pred solve`; add a rule-specific CLI test only when the CLI surface changes.

## File Naming

- Rule file: `src/rules/<sourcelower>_<targetlower>.rs` -- no underscores within a problem name
  - e.g., `maximumindependentset_qubo.rs`, `minimumvertexcover_maximumindependentset.rs`
- Test file: `src/unit_tests/rules/<sourcelower>_<targetlower>.rs`
- Canonical example: `canonical_rule_example_specs()` in the rule module, included from `src/rules/mod.rs`

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Forgetting `#[reduction(...)]` macro | Required for compile-time registration in the reduction graph |
| Registering proof-only or Turing reductions as complete-result edges | Register the actual capability; `#[reduction]` requires complete source-result recovery |
| Wrong overhead expression | Must accurately reflect the size relationship |
| Adding extra reduction metadata or duplicate primitive endpoint registration | Keep one primitive registration per endpoint pair and use only the `overhead` form of `#[reduction]` |
| Missing recovery mapping state | Store any index maps needed in the ReductionResult struct |
| Using status-preserving recovery without its premises | Prove all three status implications, or interpret outcomes explicitly in `recover_result` |
| Not adding a canonical example | Add the rule-local spec and include it from `src/rules/mod.rs` |
| Not regenerating reduction graph | Run `cargo run --example export_graph` after adding a rule |
| Skipping Step 6 (paper documentation) | **Every rule MUST have a `reduction-rule` entry in the paper. This is mandatory, not optional. PRs without documentation will be rejected.** |
| Source/target model not fully registered | Both problems must already have `ProblemSchemaEntry`, `declare_variants!`, registry aliases as needed, and a construction contract -- use `add-model` skill first |
| Treating a direct-to-ILP rule as a toy stub | Direct ILP reductions need correct parameter relationships and strong semantic regression tests, just like other production ILP rules |
| Skipping verification for complex reductions | Verification is default for a reason — `--no-verify` is for trivial identity/complement reductions only |

## Reduction lifecycle responsibilities

Apply the canonical [executed lifecycle](../../../docs/src/design.md#executed-reduction-lifecycle).
State the rule's instance domain, qualifying-witness premise, source guarantee,
and infeasibility interpretation. Check every qualifying tied optimum in small
exhaustive cases where ties are relevant. A witness flag alone does not prove
complete solvability or that adjacent path premises compose.

Construct each executed result once and share the target and mapping state. Outcome interpretation uses the rule's mathematical relation;
ordinary extraction assumes its premises. Keep necessary dynamic/JSON conversion
and reachable representation failures, but no checked/unchecked extraction or
pure forwarding wrappers. Do not add `SolutionAggregate` bounds to models or
mathematical mappings; it belongs to brute-force witness selection.
