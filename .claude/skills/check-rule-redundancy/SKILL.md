---
name: check-rule-redundancy
description: Use when checking if a reduction rule (source-target pair) is redundant — i.e., dominated by a composite path through other rules in the reduction graph
---

# Check Rule Redundancy

Determines whether reduction rules are redundant (dominated by composite paths through the reduction graph). Can check a single source-target pair or all primitive rules at once.

## Invocation

```
/check-rule-redundancy                  # Check ALL primitive rules
/check-rule-redundancy <source> <target> # Check a specific rule
```

Examples:
```
/check-rule-redundancy
/check-rule-redundancy MIS ILP
/check-rule-redundancy MaximumIndependentSet QUBO
```

## Mode 1: Check All Rules (no arguments)

When invoked without arguments, run the codebase's `find_dominated_rules` analysis test directly:

```bash
cargo test test_find_dominated_rules_returns_known_set -- --nocapture 2>&1
```

This runs the analysis from `src/rules/analysis.rs` which:
1. Enumerates every primitive reduction rule (direct edge) in the graph
2. For each, finds all alternative composite paths
3. Uses polynomial normalization and monomial-dominance to compare overheads
4. Reports dominated rules and unknown comparisons

Parse the test output and report a summary:

```markdown
## All Primitive Rules — Redundancy Report

### Dominated Rules (N)

| # | Rule | Dominating Path |
|---|------|-----------------|
| 1 | Source -> Target | A -> B -> C |

### Unknown Comparisons (N)

| # | Rule | Reason |
|---|------|--------|
| 1 | Source -> Target | expression comparison returned Unknown |

### Allowed (acknowledged) dominated rules

List the entries from the `allowed` set in `test_find_dominated_rules_returns_known_set`
(file: `src/unit_tests/rules/analysis.rs`).

### Verdict

- If test passes: all dominated rules are acknowledged in the allow-list.
- If test fails: report the unexpected dominated rule or stale allow-list entry.
```

## Mode 2: Check Single Rule (source target arguments)

### Step 1: Resolve Problem Names

Use MCP tools (`show_problem`) to validate and resolve aliases (MIS = MaximumIndependentSet, MVC = MinimumVertexCover, SAT = Satisfiability, etc.).

### Step 2: Check if Rule Already Exists

Use `show_problem` on the source and check its `reduces_to` array for a direct edge to the target.

- **Direct edge exists**: Report "Direct rule `<source> -> <target>` already exists" and proceed to redundancy analysis (Step 3).
- **No direct edge**: Report "No direct rule from `<source> -> <target>` exists yet." Then check if any path exists:
  - Use `find_path` MCP tool.
  - **Path exists**: Report the cheapest existing path and its overhead. This is the baseline the proposed new rule must beat to be non-redundant.
  - **No path exists**: Report "No path exists — a new rule would be novel (not redundant)." Stop here.

### Step 3: Find All Paths

Use `find_path` with `all: true` to get all paths between source and target.

### Step 4: Compare Overheads

For each composite path (length > 1 step):

1. Extract the **overall overhead** from the path result
2. Extract the **direct rule's overhead** from the single-step path
3. Compare field by field:
   - For polynomial expressions: compare degree — lower degree means the composite is better
   - For equal-degree polynomials: compare leading coefficients
   - For non-polynomial (exp, log): report as "Unknown — manual review needed"

**Dominance definition:** A composite path **dominates** the direct rule if, on every common overhead field, the composite's expression has equal or smaller asymptotic growth.

### Step 5: Report Results

Output a structured report:

```markdown
## Redundancy Check: <Source> -> <Target>

### Direct Rule
- Overhead: [field = expr, ...]
- Variants: [source variant] -> [target variant]

### Composite Paths Found: N

| # | Path | Steps | Overhead | Comparison |
|---|------|-------|----------|------------|
| 1 | A -> B -> C | 2 | field = expr | Dominates / Worse / Unknown |

### Verdict

- **Redundant**: At least one composite path dominates the direct rule
- **Not Redundant**: No composite path dominates the direct rule
- **Inconclusive**: Some paths have Unknown comparison (non-polynomial overhead)

### Recommendation

If redundant:
> The direct rule `<source> -> <target>` is dominated by the composite path `[path]`.
> Consider removing it unless it provides value for:
> - Simpler solution extraction (fewer intermediate steps)
> - Educational/documentation clarity
> - Better numerical behavior in practice

If not redundant:
> The direct rule `<source> -> <target>` is not dominated by any composite path.
> It provides overhead that cannot be achieved through existing reductions.
```

## Notes

- "Equal overhead" does not necessarily mean the rule should be removed — direct rules have practical advantages (simpler extraction, fewer steps)
- The analysis uses asymptotic comparison (big-O), so constant factors are ignored
- This means the check can produce false alarms, especially when overhead metadata keeps only leading terms or when a long composite path is asymptotically comparable but practically much worse
- Treat "dominated" as "potentially redundant, requires manual review" unless the composite path is also clearly preferable structurally
- When overhead expressions involve variables from different problems (e.g., `num_vertices` vs `num_clauses`), comparison may not be meaningful — report as Unknown
- The ground truth for what the codebase considers dominated is `src/rules/analysis.rs` (`find_dominated_rules`) with the allow-list in `src/unit_tests/rules/analysis.rs` (`test_find_dominated_rules_returns_known_set`)
