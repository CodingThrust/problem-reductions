---
name: review-quality
description: Generic code quality review — evaluates DRY, KISS, cohesion/coupling, test quality, and HCI. Read-only, no code changes.
---

# Quality Review

Generic code quality review that applies to any code change. Evaluates design principles, test quality, and (if applicable) CLI/HCI quality.

**This skill is read-only.** It evaluates and reports — it does NOT fix, commit, or push anything.

## Invocation

- `/review-quality` -- auto-detect from git diff

Called by `review-pipeline` as one of three parallel sub-reviews.

## Step 1: Get Context

**If the caller (e.g., `review-pipeline`) already provided a pre-generated review-implementation report in the prompt, use that directly and skip the generation command below.**

Otherwise, generate the context yourself:

```bash
REPORT=$(python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text)
printf '%s\n' "$REPORT"
```

Extract from the report:
- `Review Range`: base SHA and head SHA
- `Changed Files` and `Diff Stat`
- `Linked Issue Context`

## Step 2: Read the Diff

```bash
git diff --stat {BASE_SHA}..{HEAD_SHA}
git diff {BASE_SHA}..{HEAD_SHA}
```

Then read all changed files in full.

## Step 3: Evaluate Design Principles

### DRY (Don't Repeat Yourself)
Is there duplicated logic that should be extracted into a shared helper? Check for copy-pasted code blocks across files (similar graph construction, weight handling, or solution extraction patterns).

### KISS (Keep It Simple, Stupid)
Is the implementation unnecessarily complex? Look for: over-engineered abstractions, convoluted control flow, premature generalization, layers of indirection that add no value.

### High Cohesion, Low Coupling (HC/LC)
Does each module/function/struct have a single, well-defined responsibility?
- **Low cohesion**: Function doing unrelated things
- **High coupling**: Modules depending on each other's internals
- **Mixed concerns**: A single file containing both problem logic and CLI/serialization logic
- **God functions**: Functions longer than ~50 lines doing multiple conceptually distinct things

## Step 4: Evaluate HCI (if CLI/MCP files changed)

Only check these if the diff touches `problemreductions-cli/`:

- **Error messages** — Are they actionable? Bad: `"invalid parameter"`. Good: `"KColoring requires --k <value> (e.g., --k 3)"`.
- **Discoverability** — Missing `--help` examples? Undocumented flags? Silent failures that should suggest alternatives?
- **Consistency** — Similar operations expressed similarly? Parameter names, output formats, error styles uniform?
- **Least surprise** — Output matches expectations? No contradictory output or silent data loss?
- **Feedback** — Tool confirms what it did? Echoes interpreted parameters for ambiguous operations?

## Step 5: Evaluate Test Quality

Flag tests that:
- **Only check types/shapes, not values**: e.g., `assert!(result.is_some())` without checking the solution is correct
- **Mirror the implementation**: Tests recomputing the same formula as the code prove nothing
- **Lack adversarial cases**: Only happy path. Tests must include infeasible configs and boundary cases
- **Use trivial instances only**: Single-edge or 2-node tests may pass with bugs. Need 5+ vertex instances
- **Closed-loop without verification**: Must verify extracted solution is **optimal** (compare brute-force on both source and target)
- **Assert count too low**: 1-2 asserts for non-trivial code is insufficient

## Output Format

```
## Quality Review

### Design Principles
- DRY: OK / ISSUE — [description with file:line]
- KISS: OK / ISSUE — [description with file:line]
- HC/LC: OK / ISSUE — [description with file:line]

### HCI (if CLI/MCP changed)
- Error messages: OK / ISSUE — [description]
- Discoverability: OK / ISSUE — [description]
- Consistency: OK / ISSUE — [description]
- Least surprise: OK / ISSUE — [description]
- Feedback: OK / ISSUE — [description]

### Test Quality
- Naive test detection: OK / ISSUE
  - [specific tests flagged with reason and file:line]

### Issues

#### Critical (Must Fix)
[Bugs, correctness issues, data loss risks]

#### Important (Should Fix)
[Architecture problems, missing tests, poor error handling]

#### Minor (Nice to Have)
[Code style, optimization opportunities]

### Summary
- [list of all ISSUE items as bullet points with severity]
```
