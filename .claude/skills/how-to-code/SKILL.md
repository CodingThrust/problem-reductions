---
name: how-to-code
description: Use when implementing or modifying a problem model (src/models/) or a reduction rule (src/rules/) in this repo, including Decision<P> variants, aggregate reductions, and direct <Model> -> ILP rules.
---

# How to Code a Model or Rule

`.claude/CLAUDE.md` is the architecture reference (traits, macros, registry, numeric contract).
This guide adds the checklists and the traps that are easy to miss.

## Principles

- Copy the closest existing model/rule, not a template from memory. APIs here change often.
- Implement exactly the issue's mathematics. Missing definitions, domains, or constraints are an
  issue problem (how-to-triage-issue), not something to invent.
- Rules: verify the math first with how-to-verify, then translate its verified `reduce()` /
  `extract_solution()` into Rust. Skip only for trivially mechanical rules (identity, complement,
  variant cast).
- One item per PR. A rule needs both models already on `main`. Exception: a `[Model]` issue that
  explicitly claims direct ILP solvability ships its `<Model> -> ILP` rule in the same PR, held to
  the full production bar.

## Required inputs (from the issue)

Model: name (with `Maximum`/`Minimum` prefix only when there is a direction), formal definition
with input domains, objective or feasibility condition, best-known exact algorithm with citation
(concrete numbers only, e.g. `1.1996^num_vertices`), a small example with its expected outcome
(optimal solution + value, or a satisfying witness + why). The issue's expected outcome is the
source of truth for tests, example-db, and paper.

Rule: source/target variants, construction, extraction, correctness argument, parameter transform,
worked example, reference.

A model with no existing or planned rule becomes an orphan in the reduction graph — say so in the
PR and link or file a companion rule issue.

## Model checklist

Reference: `src/models/graph/maximum_independent_set.rs` + `src/unit_tests/models/graph/maximum_independent_set.rs`.
Decision variant reference: `src/models/graph/minimum_vertex_cover.rs` (`DecisionProblemMeta` impl or
`decision_problem_meta!`, inherent getters on `Decision<P>`, `register_decision_variant!`,
`decision_canonical_model_example_specs()`). Never hand-write a decision model.

1. `src/models/<category>/<name>.rs`:
   - `inventory::submit! { ProblemSchemaEntry { .. } }` with `display_name`, well-established
     `aliases` only (never invent), `dimensions`, explicit `category`, `fields`.
   - `impl Problem`: `NAME`, `type Solution` (mathematical witness, e.g. `Vec<bool>`),
     `type Value` (`Max`/`Min`/`Extremum` for objectives, `Or` for feasibility, value-only
     aggregates like `Sum`/`And` when no witness exists), `crate::problem_parameters![(..)]`
     (names used by complexity strings and rule transforms), `variant()` via `crate::variant_params!`,
     `evaluate() -> Result<Value, EvaluationError>` (infeasible = `Max(None)`/`Or(false)`; malformed
     input and overflow = `Err`).
   - Fallible `try_new` shared by `new`, serde `Deserialize`, and `TryFrom<CreateSpec>`.
   - `crate::declare_variants!` — one `default`, `create <Spec>` when construction differs from
     persisted JSON (model-local `#[derive(Deserialize, crate::CreateSpec)]` DTO, `FIELDS` used in
     the schema entry), `random` only where a meaningful generator exists (`impl_random_generate!`).
   - Brute force (when a finite enumeration exists): `impl BruteForceProblem { fn dimensions() }`
     plus `crate::register_brute_force! { Model<..> decode |_, indices| .. }` per variant.
   - `#[cfg(feature = "example-db")] canonical_model_example_specs()` in the model file.
   - `#[cfg(test)] #[path = "../../unit_tests/models/<category>/<name>.rs"] mod tests;`
2. `src/models/<category>/mod.rs`: `pub(crate) mod`, `pub use`, and
   `specs.extend(<name>::canonical_model_example_specs())`. Re-export in `src/models/mod.rs`.
3. Tests: ≥3 functions (creation, evaluate valid/invalid/malformed, brute-force solve, serde
   round-trip, `test_<name>_paper_example` reproducing the issue/paper example exactly).
4. Paper `problem-def` + `display-name` entry — how-to-write-manual.

Never add model-name branches, aliases, or parsers in CLI (`problemreductions-cli/`) or MCP code;
construction, aliases, random, and solving are all discovered from the registry.

## Rule checklist

Reference: `src/rules/minimumvertexcover_maximumindependentset.rs` + `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`.
Traits and helpers: `src/rules/traits.rs`, `src/rules/test_helpers.rs`, `src/rules/ilp_helpers.rs`.

1. `src/rules/<source>_<target>.rs` (lowercase, no underscores inside a name):
   - Result struct holding the target + any index maps. `extract_solution(&self, &Target::Solution)
     -> ExtractionResult<Source::Solution>` starts with exactly one
     `crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?`
     (composed extractors delegate to the first direct decoder).
   - `#[reduction(transform = exact { .. })]`, `upper_bound { .. }`, or `unavailable { .. }`, with an
     auxiliary `unavailable = { param = "reason" }` block for unrepresentable target parameters.
     Every target parameter appears exactly once. There is no `overhead =` form.
   - `reduce_to(&self) -> Result<Self::Result, ReductionError>`; wrap target construction failures
     with `Self::target_construction(e)`, never stringify.
   - Aggregate value mapping: `#[crate::aggregate_reduction] impl AggregateReductionResult` on the
     same result type (`extract_value`); generic results need `crate::register_aggregate_reduction!`
     per concrete instance (see `src/rules/kcoloring_casts.rs`).
   - `#[cfg(feature = "example-db")] canonical_rule_example_specs()` in the rule file.
2. `src/rules/mod.rs`: `pub(crate) mod` + `specs.extend(<rule>::canonical_rule_example_specs())`.
   ILP rules are not feature-gated (the `ilp-solver`/`ilp-highs` features no longer exist).
3. Exactly one primitive registration per exact (source variant, target variant) pair; share helpers,
   don't duplicate endpoints.
4. Tests in `src/unit_tests/rules/<source>_<target>.rs`: `test_<source>_to_<target>_closed_loop`
   (use `assert_*_round_trip_*` from `test_helpers.rs`), an infeasible instance, target structure
   and parameter counts vs the transform, and one test per malformed representation the decoder
   rejects (zero/multiple one-hot bits, duplicate permutation entries, ...). Aggregate edges: test
   `extract_value` against `BruteForce::solve` on both sides.
5. Paper `reduction-rule` entry — how-to-write-manual (adapt the how-to-verify proof, don't rewrite).

## Traps

- Type gate before coding: resolve concrete `Value` types (see how-to-verify). `Min<i64>` and
  `Min<usize>` are different; `Max`->`Min`, `Min`->`Or`, and anything targeting `Sum`/`And` cannot
  be a witness `ReduceTo`.
- Numeric contract (`docs/src/design.md#numeric-types-and-arithmetic`): `usize` only for indices,
  lengths, brute-force dimensions; `u64` parameters; `i64` signed values; finite `f64`. `TryFrom`
  at every range/sign boundary (`ReduceTo::exact_i64` for counts), checked arithmetic for derived
  totals, `i64_to_exact_f64` for i64->f64. No `as` casts that change range or sign.
- Error phases stay typed: `ConstructionError` / `EvaluationError` / `ReductionError` /
  `ExtractionError`. No `Result<_, String>`, no panics on user-reachable paths.
- Extraction decodes only the defined mapping: never truncate, clamp, default, or "repair".
- Solver API: `BruteForce::new().solve(&p) -> Result<Option<Solution>, SolveError>` and
  `find_all_witnesses`. There is no `Solver` trait, `find_witness`, or `dims()`.
- Complexity strings: best-known algorithm with citation; polynomial problems must not get
  exponential bounds; variable names must be declared parameters.
- Direct ILP rules are production rules: exact transform where possible, closed-loop plus
  infeasible/weighted/pathological tests, `assert_bf_vs_ilp` where cheap.

## Gates

- `make check` (fmt-check + clippy `-D warnings` + full test suite) and `make paper` pass.
- New code coverage >95% (`make coverage` locally; codecov on the PR per how-to-ship).
- Every test <5 s — shrink instances rather than slow the suite.
- Never commit generated JSON: `*.json` is gitignored (`docs/src/reductions/reduction_graph.json`,
  `problem_schemas.json`, `docs/paper/data/`); don't `git add -f` them.

## Helper commands

```bash
pred show <Problem> [--json]                  # schema, variants, reductions
pred path <S/variant> <T/variant> --limit all # existing paths and parameter transforms
pred create --example <spec> | pred solve -   # smoke-test the canonical example
cargo run --example export_graph              # refresh reduction_graph.json (also run by make paper)
```
