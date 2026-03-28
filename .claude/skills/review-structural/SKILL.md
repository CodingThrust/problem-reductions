---
name: review-structural
description: Project-specific structural completeness check for a PR — verifies model/rule checklists, build, semantic correctness, issue compliance. Read-only, no code changes.
---

# Structural Review

Project-specific structural completeness check. Verifies that a model or rule implementation has all required components, passes build checks, and matches the linked issue specification.

**This skill is read-only.** It evaluates and reports — it does NOT fix, commit, or push anything.

## Invocation

- `/review-structural` -- auto-detect from git diff
- `/review-structural model MaximumClique` -- review a specific model
- `/review-structural rule mis_qubo` -- review a specific rule

Called by `review-pipeline` as one of three parallel sub-reviews.

## Step 1: Get Context

**If the caller (e.g., `review-pipeline`) already provided a pre-generated review-implementation report in the prompt, use that directly and skip the generation command below.**

Otherwise, generate the context yourself:

```bash
set -- python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text

# Explicit subject overrides:
# set -- "$@" --kind model --name MaximumClique
# set -- "$@" --kind rule --name mis_qubo --source MaximumIndependentSet --target QUBO

REPORT=$("$@")
printf '%s\n' "$REPORT"
```

Extract from the report:
- `Scope`: review type (model/rule/generic), problem name, category, file stem
- `Deterministic Checks`: whitelist + completeness status
- `Linked Issue Context`: issue requirements to check against
- `Changed Files` and `Diff Stat`

If review type is `generic` (no new model/rule detected), report "No structural review needed for generic changes" and stop.

## Step 2: Run Structural Checklist

### Model Checklist

Only run if review type includes "model". Given: problem name `P`, category `C`, file stem `F`.

| # | Check | How to verify |
|---|-------|--------------|
| 1 | Model file exists | `Glob("src/models/{C}/{F}.rs")` |
| 2 | `inventory::submit!` present | `Grep("inventory::submit", file)` |
| 3 | `#[derive(...Serialize, Deserialize)]` on struct | `Grep("Serialize.*Deserialize", file)` |
| 4 | `Problem` trait impl | `Grep("impl.*Problem for.*{P}", file)` |
| 5 | Aggregate value is present | `Grep("type Value =", file)` |
| 6 | `#[cfg(test)]` + `#[path = "..."]` test link | `Grep("#\\[path =", file)` |
| 7 | Test file exists | `Glob("src/unit_tests/models/{C}/{F}.rs")` |
| 8 | Test file has >= 3 test functions | `Grep("fn test_", test_file)` — count matches, FAIL if < 3 |
| 9 | Registered in `{C}/mod.rs` | `Grep("mod {F}", "src/models/{C}/mod.rs")` |
| 10 | Re-exported in `models/mod.rs` | `Grep("{P}", "src/models/mod.rs")` |
| 11 | Variant registration exists | `Grep("declare_variants!|VariantEntry", file)` |
| 12 | CLI `resolve_alias` entry | `Grep("{P}", "problemreductions-cli/src/problem_name.rs")` |
| 13 | CLI `create` support | `Grep('"{P}"', "problemreductions-cli/src/commands/create.rs")` |
| 14 | Canonical model example registered | `Grep("{P}", "src/example_db/model_builders.rs")` |
| 15 | Paper `display-name` entry | `Grep('"{P}"', "docs/paper/reductions.typ")` |
| 16 | Paper `problem-def` block | `Grep('problem-def.*"{P}"', "docs/paper/reductions.typ")` |

### Rule Checklist

Only run if review type includes "rule". Given: source `S`, target `T`, rule file stem `R`, example stem `E`.

| # | Check | How to verify |
|---|-------|--------------|
| 1 | Rule file exists | `Glob("src/rules/{R}.rs")` |
| 2 | `#[reduction(...)]` macro present | `Grep("#\\[reduction", file)` |
| 3 | `ReductionResult` impl present | `Grep("impl.*ReductionResult", file)` |
| 4 | `ReduceTo` impl present | `Grep("impl.*ReduceTo", file)` |
| 5 | `#[cfg(test)]` + `#[path = "..."]` test link | `Grep("#\\[path =", file)` |
| 6 | Test file exists | `Glob("src/unit_tests/rules/{R}.rs")` |
| 7 | Closed-loop test present | `Grep("fn test_.*closed_loop\|fn test_.*to_.*basic", test_file)` |
| 8 | Registered in `rules/mod.rs` | `Grep("mod {R}", "src/rules/mod.rs")` |
| 9 | Canonical rule example registered | `Grep("{S}|{T}|{R}", "src/example_db/rule_builders.rs")` |
| 10 | Example-db lookup tests exist | `Grep("find_rule_example|build_rule_db", "src/unit_tests/example_db.rs")` |
| 11 | Paper `reduction-rule` entry | `Grep('reduction-rule.*"{S}".*"{T}"', "docs/paper/reductions.typ")` |

## Step 2b: Blacklisted File Check

Scan the PR's changed files for auto-generated files that must never be committed:
- `docs/src/reductions/reduction_graph.json`
- `docs/src/reductions/problem_schemas.json`
- `src/example_db/fixtures/examples.json` (legacy path, deleted on main)
- `docs/paper/data/examples.json` (current output path, gitignored)

If any of these files appear in the diff, report **FAIL — blacklisted auto-generated file committed**. These files are rebuilt by CI/`make doc`/`make paper` and must not be in PRs.

## Step 3: Build Check

Run:
```bash
make test clippy
```

Report pass/fail. If tests fail, identify which tests. **Do NOT fix anything** — just report.

## Step 4: Semantic Review

### For Models:
1. **`evaluate()` correctness** — Does it check feasibility before computing the objective when the model has invalid configurations? Objective models should return `Max/Min/Extremum(None)` for infeasible configs, witness problems should return `false`, and aggregate-only models should return the per-configuration contribution that matches the intended fold semantics.
2. **`dims()` correctness** — Does it return the actual configuration space? (e.g., `vec![2; n]` for binary)
3. **Size getter consistency** — Do inherent getter methods (e.g., `num_vertices()`, `num_edges()`) match names used in overhead expressions?
4. **Weight handling** — Are weights managed via inherent methods, not traits?

### For Rules:
1. **`extract_solution` correctness** — Does it correctly invert the reduction? Does the returned solution have the right length (source dimensions)?
2. **Overhead accuracy** — Does `overhead = { field = "expr" }` reflect the actual size relationship?
3. **Example quality** — Is it tutorial-style? Does the JSON export include both source and target data?
4. **Paper quality** — Is the reduction-rule statement precise? Is the proof sketch sound?

## Step 5: Issue Compliance Review

Only if a linked issue was provided.

### For Models (check against issue):
| # | Check |
|---|-------|
| 1 | Problem name matches issue |
| 2 | Mathematical definition matches |
| 3 | Problem framing (objective / witness / aggregate-only) matches |
| 4 | Type parameters match |
| 5 | Configuration space matches |
| 6 | Feasibility check matches |
| 7 | Objective function matches |
| 8 | Complexity matches |

### For Rules (check against issue):
| # | Check |
|---|-------|
| 1 | Source/target match issue |
| 2 | Reduction algorithm matches |
| 3 | Solution extraction matches |
| 4 | Correctness preserved |
| 5 | Overhead expressions match |
| 6 | Example matches |

Flag any deviation as ISSUE.

## Output Format

```
## Structural Review: [model/rule] [Name]

### Structural Completeness
| # | Check | Status |
|---|-------|--------|
| 1 | ... | PASS / FAIL — reason |

### Build Status
- `make test`: PASS / FAIL
- `make clippy`: PASS / FAIL

### Semantic Review
- [check]: OK / ISSUE — description

### Issue Compliance (if linked issue found)
| # | Check | Status |
|---|-------|--------|
| 1 | ... | OK / ISSUE — deviation description |

### Summary
- X/Y structural checks passed
- X/Y issue compliance checks passed (if applicable)
- [list of all FAIL/ISSUE items as bullet points]
```
