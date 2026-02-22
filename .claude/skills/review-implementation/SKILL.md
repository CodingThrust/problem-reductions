---
name: review-implementation
description: Use after implementing a model or rule to verify completeness and correctness before committing
---

# Review Implementation

Automated review checklist for verifying that a new model or rule implementation is complete. Run this after finishing `add-model` or `add-rule`, before committing.

## Invocation

Auto-detects the implementation type from changed files. Can also be invoked with an explicit argument:
- `/review-implementation` -- auto-detect from `git diff`
- `/review-implementation model MaximumClique` -- review a specific model
- `/review-implementation rule mis_qubo` -- review a specific rule

## Step 1: Detect What Changed

Use `git diff --name-only` (against main branch or last commit) to identify:
- Files in `src/models/` -> model review
- Files in `src/rules/` (not `mod.rs`, `traits.rs`, `cost.rs`, `graph.rs`, `registry.rs`) -> rule review
- Both -> run both reviews

Extract the problem name(s) and rule source/target from the file paths.

## Step 2: Run Structural Checks

For each detected change, run the appropriate checklist below. Report results as a table with pass/fail per item.

### Model Checklist

Given: problem name `P`, category `C`, file stem `F` (snake_case).

| # | Check | Verification method |
|---|-------|-------------------|
| 1 | Model file exists | `Glob("src/models/{C}/{F}.rs")` |
| 2 | `inventory::submit!` present | `Grep("inventory::submit", file)` |
| 3 | `#[derive(...Serialize, Deserialize)]` on struct | `Grep("Serialize.*Deserialize", file)` |
| 4 | `Problem` trait impl | `Grep("impl.*Problem for.*{P}", file)` |
| 5 | `OptimizationProblem` or `SatisfactionProblem` impl | `Grep("(OptimizationProblem\|SatisfactionProblem).*for.*{P}", file)` |
| 6 | `#[cfg(test)]` + `#[path = "..."]` test link | `Grep("#\\[path =", file)` |
| 7 | Test file exists | `Glob("src/unit_tests/models/{C}/{F}.rs")` |
| 8 | Test has creation test | `Grep("fn test_.*creation\|fn test_{F}.*basic", test_file)` |
| 9 | Test has evaluation test | `Grep("fn test_.*evaluat", test_file)` |
| 10 | Registered in `{C}/mod.rs` | `Grep("mod {F}", "src/models/{C}/mod.rs")` |
| 11 | Re-exported in `models/mod.rs` | `Grep("{P}", "src/models/mod.rs")` |
| 12 | CLI `load_problem` arm | `Grep('"{P}"', "problemreductions-cli/src/dispatch.rs")` |
| 13 | CLI `serialize_any_problem` arm | `Grep('"{P}".*try_ser', "problemreductions-cli/src/dispatch.rs")` |
| 14 | CLI `resolve_alias` entry | `Grep("{P}", "problemreductions-cli/src/problem_name.rs")` |
| 15 | Paper `display-name` entry | `Grep('"{P}"', "docs/paper/reductions.typ")` |
| 16 | Paper `problem-def` block | `Grep('problem-def.*"{P}"', "docs/paper/reductions.typ")` |

### Rule Checklist

Given: source `S`, target `T`, rule file stem `R` = `{s}_{t}` (lowercase), example stem `E` = `reduction_{s}_to_{t}`.

| # | Check | Verification method |
|---|-------|-------------------|
| 1 | Rule file exists | `Glob("src/rules/{R}.rs")` |
| 2 | `#[reduction(...)]` macro present | `Grep("#\\[reduction", file)` |
| 3 | `ReductionResult` impl present | `Grep("impl.*ReductionResult", file)` |
| 4 | `ReduceTo` impl present | `Grep("impl.*ReduceTo", file)` |
| 5 | `#[cfg(test)]` + `#[path = "..."]` test link | `Grep("#\\[path =", file)` |
| 6 | Test file exists | `Glob("src/unit_tests/rules/{R}.rs")` |
| 7 | Closed-loop test present | `Grep("fn test_.*closed_loop\|fn test_.*to_.*basic", test_file)` |
| 8 | Registered in `rules/mod.rs` | `Grep("mod {R}", "src/rules/mod.rs")` |
| 9 | Example file exists | `Glob("examples/{E}.rs")` |
| 10 | Example has `pub fn run()` | `Grep("pub fn run", example_file)` |
| 11 | Example has `fn main()` | `Grep("fn main", example_file)` |
| 12 | `example_test!` registered | `Grep("example_test!\\({E}\\)", "tests/suites/examples.rs")` |
| 13 | `example_fn!` registered | `Grep("example_fn!.*{E}", "tests/suites/examples.rs")` |
| 14 | Paper `reduction-rule` entry | `Grep('reduction-rule.*"{S}".*"{T}"', "docs/paper/reductions.typ")` |

## Step 3: Run Build Checks

After structural checks, run:

```bash
make test clippy
```

Report pass/fail. If tests fail, identify which tests and suggest fixes.

## Step 4: Semantic Review (AI Judgment)

Read the implementation files and assess:

### For Models:
1. **`evaluate()` correctness** -- Does it check feasibility before computing the objective? Does it return `SolutionSize::Invalid` / `false` for infeasible configs?
2. **`dims()` correctness** -- Does it return the actual configuration space? (e.g., `vec![2; n]` for binary)
3. **`problem_size_names`/`problem_size_values` consistency** -- Do the names match what `ReductionOverhead` uses?
4. **Weight handling** -- Are weights managed via inherent methods, not traits?

### For Rules:
1. **`extract_solution` correctness** -- Does it correctly invert the reduction? Does the returned solution have the right length (source dimensions)?
2. **Overhead accuracy** -- Does `poly!(...)` reflect the actual size relationship?
3. **Example quality** -- Is it tutorial-style? Does it use the instance from the issue? Does the JSON export include both source and target data?
4. **Paper quality** -- Is the reduction-rule statement precise? Is the proof sketch sound? Is the example figure clear?

### Code Quality Principles (applies to both Models and Rules):
1. **DRY (Don't Repeat Yourself)** -- Is there duplicated logic that should be extracted into a shared helper, utility function, or common module? Check for copy-pasted code blocks across files (e.g., similar graph construction, weight handling, or solution extraction patterns). If duplication is found, suggest extracting shared logic.
2. **KISS (Keep It Simple, Stupid)** -- Is the implementation unnecessarily complex? Look for: over-engineered abstractions, convoluted control flow, premature generalization, or layers of indirection that add no value. The implementation should be as simple as possible while remaining correct and maintainable.

## Output Format

Present results as:

```
## Review: [Model/Rule] [Name]

### Structural Completeness
| # | Check | Status |
|---|-------|--------|
| 1 | Model file exists | PASS |
| 2 | inventory::submit! | PASS |
| ... | ... | ... |
| N | Paper entry | FAIL -- missing display-name |

### Build Status
- `make test`: PASS
- `make clippy`: PASS

### Semantic Review
- evaluate() correctness: OK
- dims() correctness: OK
- DRY compliance: OK / [duplicated logic found in ...]
- KISS compliance: OK / [unnecessary complexity found in ...]
- [any other issues found]

### Summary
- X/Y structural checks passed
- [list of action items for any failures]
```

## Integration with Other Skills

This skill is called automatically at the end of:
- `add-model` (after Step 7: Verify)
- `add-rule` (after Step 6: Verify)

It can also be invoked standalone via `/review-implementation`.
