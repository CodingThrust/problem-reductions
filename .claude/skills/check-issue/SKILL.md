---
name: check-issue
description: Use when reviewing a [Rule] or [Model] GitHub issue for quality before implementation — checks usefulness, non-triviality, correctness of literature claims, and writing quality
---

# Check Issue

Quality gate for `[Rule]` and `[Model]` GitHub issues. Runs 4 checks, posts a structured report as a GitHub comment, and adds labels for failures. Does NOT close or fix issues.

## Invocation

```bash
/check-issue <issue-number>                # check a specific issue
/check-issue <issue-number> --force        # re-check even if already checked
/check-issue model                         # pick next [Model] from Backlog
/check-issue rule                          # pick next [Rule] from Backlog
```

## Prerequisites

```bash
pred --version 2>/dev/null || make cli
```

## Steps

### 0. Resolve Issue

If argument is `model` or `rule` (not a number), pick from Backlog:

```bash
uv run --project scripts scripts/pipeline_board.py backlog <model|rule> --format json
```

Pick the first item. If empty, STOP: `No [Model]/[Rule] issues in Backlog.`

Then fetch the issue:

```bash
gh issue view <NUMBER> --json title,body,labels,comments
```

- Detect type from title: `[Rule]` or `[Model]`. If neither, STOP: "This skill only checks [Rule] and [Model] issues."
- Scan comments for existing `## Issue Quality Check`. If found and no `--force`: STOP with "Already checked (comment from YYYY-MM-DD). Use `--force` to re-check."
- **`[Rule]` prerequisite:** Parse Source and Target from the body, then run `pred show <name> --json` for both. If either problem does **not** exist in the codebase → STOP: "Cannot check: `<name>` is not implemented yet. Add the model first." Do not proceed with any checks — overhead metrics, size_fields, and example verification all depend on both problems being implemented.

### 1. Run 4 Checks

#### Check 1: Usefulness (fail label: `Useless`)

**`[Rule]` issues** (source and target already resolved in Step 0)**:**

1. Check existing path: `pred path <source> <target> --json`
   - No path → **Pass**
   - Path exists → run `/topology-sanity-check redundancy <source> <target>`. Use its verdict: Not Redundant → **Pass**, Redundant → **Fail** (include dominating path), Inconclusive → **Warn**
3. Motivation field empty or vague ("enables X" without explaining why) → **Warn**

**`[Model]` issues:**

1. Parse problem name. Check if already exists: `pred show <name> --json`. If exists → **Fail**.
2. Issue must mention at least one concrete reduction to/from an existing problem. None mentioned → **Fail** ("orphan node"). Vague mention → **Warn**.
3. Motivation empty → **Warn**. No solver path in "How to solve" → **Warn**.

#### Check 2: Non-trivial (fail label: `Trivial`)

**`[Rule]` issues — Fail if:**

- Variable substitution only (1-to-1 relabeling, no new constraints)
- Subtype coercion (cast to more general type, no structural change)
- Same-problem identity (e.g., `MIS<SimpleGraph, One>` → `MIS<SimpleGraph, i32>`)
- Insufficient detail (hand-wave, not step-by-step)
- Exception: trivial reduction that connects disconnected problems → **Pass**

**`[Model]` issues — Fail if:**

- **Trivial re-skin:** mathematically identical to an existing problem, just different domain language (e.g., a physics-flavored wrapper around Partition). The NP-hardness comes entirely from the embedded known problem, not from the domain-specific structure.
- **Isomorphic:** same feasibility constraints and objective as an existing problem under a different name.
- **Trivial variant:** could be handled by adding a graph/weight type to an existing model.

Check against existing problems: `pred list --json`. Pass if genuinely different feasibility constraints or objective.

#### Check 3: Correctness (fail label: `Wrong`)

**3a: Extract references** — Parse all citations from the issue body (paper titles, authors, years, DOIs, arxiv IDs, textbook references).

**3b: Project knowledge base** — Two-step process:

1. Read `references.md` (in this skill's directory) for quick fact-checking — contains known complexity bounds, key results, and established reductions. If the issue's claims contradict known facts → **Fail** immediately.
2. Read `docs/paper/references.bib` for the full bibliography. If a cited paper is already there, verify the issue's claim matches what the paper actually says. Misquoted result → **Fail**.

**3c: External verification** — For each reference NOT in the bibliography, use the fallback chain: arxiv MCP → Semantic Scholar MCP → WebSearch + WebFetch. For each reference verify all three:

- Paper exists with matching title, authors, and year
- The specific theorem/result cited actually appears in the paper
- Cross-check the claim against at least one other source (survey, textbook, or independent reference)

Unverifiable claim (paper not found, claim not in paper) → **Fail** with specifics.

**3d: Better algorithm discovery** — If a more recent paper, lower overhead construction, or better bounds are found → **Recommendation** (not a failure).

**Additional checks for `[Rule]` issues:**

- **Source/target definition match:** the issue's description of source and target problems must match their implemented schemas and semantics (cross-check via `pred show --json`). Mismatch → **Fail**.
- **Overhead expression correctness:** verify overhead formulas against the reduction algorithm — count the actual constructed objects. Test with the issue's example: run the reduction, compare target sizes against the formula. Mismatch → **Fail**.

**Additional checks for `[Model]` issues:**

- **Definition correctness:** formal definition is well-formed, feasibility and objective clearly separated, variable domain matches problem semantics (binary for selection, k-ary for coloring, etc.)
- **Representation feasibility:** proposed Schema data types can represent the stated domain. If types are too narrow (e.g., fixed-width int for arbitrary-size field) → **Fail**. Scope-restricted acknowledgment (e.g., "targets small fields only") → **Pass** with note.
- **Complexity verification:** The cited algorithm exists, the time bound matches the paper, polynomial problems are indeed polynomial (not NP-hard), exponential base is correct (e.g., 1.1996^n for MIS, not 2^n).

#### Check 4: Well-written (fail label: `PoorWritten`)

**4a: Required sections** — All must be present and substantive (not placeholder):

| Section | Rule | Model |
| --- | --- | --- |
| Source / Target | required | — |
| Name | — | required |
| Motivation | required | required |
| Reference | required | required |
| Definition | — | required (input, feasibility, objective) |
| Variables | — | required (count, domain, meaning) |
| Schema | — | required (type, variants, fields) |
| Dims | — | required (config space per variable, e.g. `[2; n]` for binary) |
| Size fields | — | required (getter names + meanings, e.g. `num_vertices`, `num_edges`) |
| Complexity | — | required (algorithm + concrete expression, e.g. `2^{0.8765n}`) |
| How to solve | — | required (at least one solver method) |
| Reduction Algorithm | required (complete, step-by-step) | — |
| Size Overhead | required (code metric names + formulas) | — |
| Validation Method | required | — |
| Example | required | required |
| Expected Outcome | — | required (sat: valid solution + why; opt: optimal solution + value) |

Missing or placeholder → **Fail**.

**4b: Algorithm / Definition completeness**

- `[Rule]`: Every step numbered and unambiguous, all intermediate values defined, no gaps ("similarly for remaining variables"), solution extraction clear. High-level sketch → **Fail**.
- `[Model]`: Input structure specified, feasibility as math conditions, all quantifiers explicit ("for all edges (u,v) in E" not "adjacent vertices don't share colors"), objective stated. Naming: optimization problems must use `Maximum`/`Minimum` prefix.

**4c: Symbol and notation consistency**

- All symbols defined before first use (e.g., algorithm references `n` → must have "let n = |V|" first)
- Consistent across sections (algorithm defines `G = (V, E)` but overhead uses `N` without defining → **Fail**)
- `[Rule]`: Cross-check against `pred show --json` output for **both** source and target — overhead metric names must match target's `size_fields`, overhead expression variables must map to source's `size_fields`, and the issue's description of source/target problems must match their implemented schemas. Any mismatch → **Fail**.
- `[Model]`: Schema field descriptions must match symbols in Definition

**4d: Example quality (subagent)**

Dispatch a subagent to evaluate the issue's example. The subagent should:

1. Check the example **size** — the total search space (product of all variable domains) should be roughly 128–10,000 configurations. Fewer than 4 is trivial; more than ~10,000 makes brute-force slow. `[Model]` examples must exercise the defining features (e.g., "MultivariateQuadratic" must have quadratic terms) and provide expected outcome (sat: valid solution + why; opt: optimal solution + value).
2. Check the example is **fully worked** — for `[Rule]` issues this means the example must explicitly enumerate every constructed object: list all target variables by name, write out every constraint, and state the objective expression. Merely summarizing dimensions (e.g., "15 variables, 19 constraints") is **not** fully worked — the actual variables and constraints must appear. For `[Model]` issues, the example must show the concrete instance data and the expected evaluation. If the construction is only summarized → **Fail** on Well-written.
3. **Reproduce with a Python script** (write to `/tmp/`) — construct the instance from the issue's data, brute-force enumerate all configurations via `itertools.product` when feasible (prefer enumeration over solver libraries; fall back to a solver only for continuous/unbounded domains like ILP), verify the claimed solution is optimal/satisfying, and for `[Rule]` issues check round-trip: source optimum maps to valid target assignment, target optimum maps back to source optimum.

The example should have at least 2 suboptimal feasible solutions — a too-simple instance can pass even with a buggy reduction. If verification fails, flag as **Fail** on Correctness with the specific mismatch.

### 2. Post Report and Labels

Post a single GitHub comment:

````markdown
## Issue Quality Check — [Rule|Model]

| Check | Result | Details |
| --- | --- | --- |
| Usefulness | [Pass/Fail/Warn] | [one-line summary] |
| Non-trivial | [Pass/Fail/Warn] | [one-line summary] |
| Correctness | [Pass/Fail/Warn] | [one-line summary] |
| Well-written | [Pass/Fail/Warn] | [one-line summary] |

**Overall: X passed, Y failed, Z warnings**

---

### Usefulness
[Detailed explanation]

### Non-trivial
[Detailed explanation]

### Correctness
[Per-reference verification results, better algorithms found]

### Well-written
[Specific items to fix]

#### Recommendations
- [Suggestions for improvement]
````

**Labels** — add for failed checks only (not warnings):

```bash
gh issue edit <NUMBER> --add-label "Useless"     # Check 1 failed
gh issue edit <NUMBER> --add-label "Trivial"      # Check 2 failed
gh issue edit <NUMBER> --add-label "Wrong"         # Check 3 failed
gh issue edit <NUMBER> --add-label "PoorWritten"   # Check 4 failed
```

"Good" label requires: zero failures AND zero warnings on Usefulness or Correctness. Warnings on Non-trivial or Well-written alone do not block "Good".

If re-checking: remove stale failure labels before adding new ones.

**Never close the issue.** Labels and comments only.

## Common Mistakes

| Mistake | Fix |
| --- | --- |
| Failing on warnings | Only add labels for definitive failures |
| Closing issues | Labels and comments only |
| Hallucinating paper content | If not found, say "not found" |
| Hallucinating issue references | Only reference issues verified with `gh issue view` |
| Wrong template | `[Rule]` and `[Model]` have different required sections |
| Offering to fix | That's `/fix-issue`'s job, not this skill's |
