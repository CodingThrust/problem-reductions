---
name: how-to-review
description: Use when reviewing a pull request or a finished, uncommitted diff in this repo — as a fresh-context reviewer dispatched after implementation, or when a human asks to "review PR N".
---

# How to Review

You are a read-only reviewer: evaluate and report, never edit, commit, push, or merge. Run from a
checkout of the PR head (`gh pr checkout N`); the scope helper diffs `HEAD` against
`merge-base origin/main`. Architecture and conventions are in `.claude/CLAUDE.md`; judge against
them, and against the current code rather than memory.

## Gather scope once

```bash
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text
python3 scripts/pipeline_pr.py context --repo "$REPO" --pr "$PR" --format text  # comments, CI, linked issue
gh pr diff "$PR"
```

The review-implementation packet gives review type (model/rule/generic), subject, whitelist and
completeness checks, and changed files. Treat a `fail` there as a lead, not a verdict: confirm it
against the actual PR file list. Pass `--kind/--name/--source/--target` when auto-detection is wrong.
Read every changed file in full.

## (a) Structural and semantic

**Hard fails, whatever the PR type**
- Generated exports in the diff: `docs/src/reductions/reduction_graph.json`,
  `docs/src/reductions/problem_schemas.json`, `docs/paper/data/examples.json`. They are gitignored
  build outputs of `make doc` / `make paper`.
- Unrelated edits, deleted models/rules/tests, or core-trait/macro changes the issue does not need.

**Model** (`src/models/<category>/<file>.rs`)
- `Problem` impl with `type Value`; `declare_variants!` with one `default` and a complexity string
  whose base matches the cited best-known algorithm (polynomial problems must not be exponential).
- `ProblemSchemaEntry` with an explicit category; aliases only in `ProblemSchemaEntry.aliases` /
  `declare_variants!`, and only standard literature abbreviations that do not collide.
- Brute force via `register_brute_force!`; `BruteForceProblem::dimensions()` is the real
  configuration space.
- Infeasible configurations evaluate to `Max(None)` / `Min(None)` / `Extremum(None)`, or `Or(false)`
  for feasibility problems.
- `pred create <Name> --help` flags come from the schema fields or a model-local `CreateSpec` —
  no model-name branches in `problemreductions-cli/` or MCP code, and no leftover manual dispatch
  arms in `dispatch.rs` / `problem_name.rs` after merging main.
- `#[cfg(feature = "example-db")] canonical_model_example_specs()` in the model file, chained from
  the category `mod.rs` (`src/example_db/model_builders.rs` only aggregates categories — grepping it
  for the model name proves nothing).
- Test file `src/unit_tests/models/<category>/<file>.rs` linked via `#[path]`, with at least 3 tests.

**Rule** (`src/rules/<source>_<target>.rs`)
- `#[reduction(transform = exact|upper_bound|unavailable { ... })]` covering every target parameter
  exactly once (there is no `overhead =` form any more); registered in `src/rules/mod.rs`.
- `canonical_rule_example_specs()` in the rule file, collected in `src/rules/mod.rs`.
- Every direct `extract_solution()` calls `validate_target_solution()` once, then decodes only the
  mathematical mapping; malformed input yields `ExtractionError` — no panic, clamp, truncation,
  invented default, or recovery branch — and each rejection path is tested.
- Closed-loop test `test_<source>_to_<target>_closed_loop` in `src/unit_tests/rules/`.

**Both**
- Error contracts: construction returns `ConstructionError`, `evaluate()` returns `EvaluationError`,
  reductions return `ReductionError` (target construction failures kept as
  `ReductionError::Construction`); no public `Result<_, String>` in new code.
- Numeric contract per `docs/src/design.md#numeric-types-and-arithmetic`: `u64` parameters, `TryFrom`
  at range boundaries, checked arithmetic for derived sizes, same range in Rust/serde/CLI/MCP.
- Paper: `display-name` + `problem-def`, or `reduction-rule` per directed edge. Example values must be
  loaded (`load-model-example` / `load-example`) from `examples.json`, never hand-written; see
  how-to-write-manual.

**Semantic checks — do these by hand, do not trust green CI**
- Rule: pick a small source instance, trace `reduce_to()` line by line, and confirm the target
  encodes the same question and `extract_solution` inverts it. Count every target size by hand and
  compare with the `transform` block; `pred path <S> <T> instance.json` measures the constructed
  sizes on a real instance. Check the paper proof is sound, not just present.
- Model: `evaluate()` implements exactly the stated definition (watch "at least" vs "all"), including
  empty/zero/infeasible edge cases.

**Issue compliance** (linked issue from the context packet)
- Issue comments override the body; read all of them first.
- Definition, framing (objective vs feasibility), configuration space, complexity, reduction
  algorithm, extraction, and transform formulas match the issue.
- Round trip: the issue's example instance appears in a test and in the canonical example, with the
  issue's stated optimum confirmed by brute force rather than a hardcoded assertion.

## (b) Quality — project-specific only

- Trivial instances (single edge, 2 vertices) hide bugs; expect at least one instance with 5+ vertices
  (or equivalent size).
- Closed-loop tests must check the extracted source solution is optimal against brute force on the
  source, with the target also solved by brute force (see `src/rules/test_helpers.rs`
  `assert_*_round_trip_*`).
- Tests that recompute the implementation's formula prove nothing; tests that only check `is_some()`
  or shapes are too weak; missing infeasible/boundary cases is Important.
- Duplicated logic that an existing helper already covers.
- HCI only if `problemreductions-cli/` changed: actionable error messages, `--help` examples,
  consistency with sibling commands, no silent data loss.

## (c) Feature test as a user

Install the PR's CLI with `make cli` (`cargo install` of `problemreductions-cli`), then exercise the new item from a scratch directory, as a user would:

```bash
pred list <Name>            # or: pred list --rules <Source>
pred show <Name>
pred create --example <Name> -o inst.json         # rule: --example <S> --to <T>
pred inspect inst.json
pred solve inst.json                               # also --solver brute-force
pred path <S> <T> -o paths.json && jq '.paths[0]' paths.json > route.json
pred reduce inst.json --via route.json -o bundle.json && pred solve bundle.json
```

Check outputs against the issue's example. Reproduce every problem before reporting it. Do not fix.

## Report

Classify findings as **Critical** (wrong results, broken contract, blacklisted file), **Important**
(missing tests/components, weak verification, issue deviation), **Minor** (style, docs). Give
`file:line` and a concrete suggested fix for each. Post one comment:

```bash
python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "$PR" --body-file review.md
```

The body starts with `## Agentic Review Report`, then sections for structural, quality, and feature
test, then the severity-sorted findings. For an uncommitted diff with no PR, return the same report
to the caller instead of posting.
