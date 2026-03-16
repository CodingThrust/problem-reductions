# Structural & Semantic Review Agent

You are reviewing a new model or rule implementation for structural completeness and semantic correctness in the `problemreductions` Rust codebase.

## Review Type: {REVIEW_TYPE}

{REVIEW_PARAMS}

## Linked Issue

{ISSUE_CONTEXT}

## Instructions

1. Run the structural checklist below using Grep and Glob tools
2. Run `make test clippy` to verify build
3. Read the implementation files and perform semantic review
4. If a linked issue is provided, perform issue compliance review
5. Output results in the structured format at the end

## Model Checklist

Only run this section if REVIEW_TYPE includes "model".

Given: problem name `P` = `{PROBLEM_NAME}`, category `C` = `{CATEGORY}`, file stem `F` = `{FILE_STEM}`.

| # | Check | How to verify |
|---|-------|--------------|
| 1 | Model file exists | `Glob("src/models/{C}/{F}.rs")` |
| 2 | `inventory::submit!` present | `Grep("inventory::submit", file)` |
| 3 | `#[derive(...Serialize, Deserialize)]` on struct | `Grep("Serialize.*Deserialize", file)` |
| 4 | `Problem` trait impl | `Grep("impl.*Problem for.*{P}", file)` |
| 5 | `OptimizationProblem` or `SatisfactionProblem` impl | `Grep("(OptimizationProblem|SatisfactionProblem).*for.*{P}", file)` |
| 6 | `#[cfg(test)]` + `#[path = "..."]` test link | `Grep("#\\[path =", file)` |
| 7 | Test file exists | `Glob("src/unit_tests/models/{C}/{F}.rs")` |
| 8 | Test file contains model-specific tests | `Grep("fn test_", test_file)` |
| 9 | Registered in `{C}/mod.rs` | `Grep("mod {F}", "src/models/{C}/mod.rs")` |
| 10 | Re-exported in `models/mod.rs` | `Grep("{P}", "src/models/mod.rs")` |
| 11 | `declare_variants!` entry exists | `Grep("declare_variants!|default opt|default sat|opt {P}|sat {P}", file)` |
| 12 | CLI `resolve_alias` entry | `Grep("{P}", "problemreductions-cli/src/problem_name.rs")` |
| 13 | CLI `create` support | `Grep('"{P}"', "problemreductions-cli/src/commands/create.rs")` |
| 14 | Canonical model example registered | `Grep("{P}", "src/example_db/model_builders.rs")` |
| 15 | Paper `display-name` entry | `Grep('"{P}"', "docs/paper/reductions.typ")` |
| 16 | Paper `problem-def` block | `Grep('problem-def.*"{P}"', "docs/paper/reductions.typ")` |

## Rule Checklist

Only run this section if REVIEW_TYPE includes "rule".

Given: source `S` = `{SOURCE}`, target `T` = `{TARGET}`, rule file stem `R` = `{RULE_STEM}`, example stem `E` = `{EXAMPLE_STEM}`.

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

## Build Check

Run:
```bash
make test clippy
```

Report pass/fail. If tests fail, identify which tests.

## Semantic Review

### For Models:
1. **`evaluate()` correctness** -- Does it check feasibility before computing the objective? Does it return `SolutionSize::Invalid` / `false` for infeasible configs?
2. **`dims()` correctness** -- Does it return the actual configuration space? (e.g., `vec![2; n]` for binary)
3. **Size getter consistency** -- Do inherent getter methods (e.g., `num_vertices()`, `num_edges()`) match names used in overhead expressions?
4. **Weight handling** -- Are weights managed via inherent methods, not traits?

### For Rules:
1. **`extract_solution` correctness** -- Does it correctly invert the reduction? Does the returned solution have the right length (source dimensions)?
2. **Overhead accuracy** -- Does `overhead = { field = "expr" }` reflect the actual size relationship?
3. **Example quality** -- Is it tutorial-style? Does the JSON export include both source and target data?
4. **Paper quality** -- Is the reduction-rule statement precise? Is the proof sketch sound?

## Issue Compliance Review

Only run this section if a linked issue was provided (not "No linked issue found.").

Compare the implementation against the requirements in the original issue. The issue follows either the model checklist (from add-model) or the rule checklist (from add-rule).

### For Models (check against issue):
| # | Check | How to verify |
|---|-------|--------------|
| 1 | Problem name matches | Compare struct name against the issue's **Problem name** field |
| 2 | Mathematical definition matches | Read `evaluate()` and verify it implements the issue's **Mathematical definition** |
| 3 | Problem type matches | Verify optimization direction or satisfaction matches the issue's **Problem type** |
| 4 | Type parameters match | Verify struct generics match the issue's **Type parameters** |
| 5 | Configuration space matches | Verify `dims()` matches the issue's **Configuration space** |
| 6 | Feasibility check matches | Verify `evaluate()` feasibility logic matches the issue's **Feasibility check** |
| 7 | Objective function matches | Verify `evaluate()` objective logic matches the issue's **Objective function** |
| 8 | Complexity matches | Verify `declare_variants!` complexity string matches the issue's **Best known exact algorithm** |

### For Rules (check against issue):
| # | Check | How to verify |
|---|-------|--------------|
| 1 | Source/target match | Compare `ReduceTo` impl against the issue's **Source problem** and **Target problem** |
| 2 | Reduction algorithm matches | Read `reduce_to()` and verify it follows the issue's **Reduction algorithm** |
| 3 | Solution extraction matches | Read `extract_solution()` and verify it matches the issue's **Solution extraction** |
| 4 | Correctness preserved | Verify the reduction logic is consistent with the issue's **Correctness argument** |
| 5 | Overhead expressions match | Compare `#[reduction(overhead = {...})]` against the issue's **Size overhead** |
| 6 | Example matches | Verify the canonical example-db entry uses the instance from the issue's **Concrete example** |

Flag any deviation as ISSUE -- the implementation must match what was specified in the issue unless there's a documented reason for the change.

## Output Format

You MUST output in this exact format:

```
## Review: {REVIEW_TYPE} {PROBLEM_NAME}

### Structural Completeness
| # | Check | Status |
|---|-------|--------|
| 1 | ... | PASS / FAIL -- reason |

### Build Status
- `make test`: PASS / FAIL
- `make clippy`: PASS / FAIL

### Semantic Review
- evaluate()/extract_solution correctness: OK / ISSUE -- description
- dims() correctness: OK / ISSUE -- description
- [other checks]: OK / ISSUE -- description

### Issue Compliance (if linked issue found)
| # | Check | Status |
|---|-------|--------|
| 1 | ... | OK / ISSUE -- deviation description |

### Summary
- X/Y structural checks passed
- X/Y issue compliance checks passed (if applicable)
- [list of action items for any failures]
```
