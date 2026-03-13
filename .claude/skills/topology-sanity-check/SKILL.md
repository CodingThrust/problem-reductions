---
name: topology-sanity-check
description: Run sanity checks on the reduction graph topology — detect orphan (isolated) problems, NP-hardness proof gaps, and redundant reduction rules
---

# Topology Sanity Check

Runs structural health checks on the reduction graph. Detects orphan problems, verifies NP-hardness proof chains from 3-SAT, and identifies redundant reduction rules dominated by composite paths.

## Invocation

```
/topology-sanity-check                          # Run ALL checks
/topology-sanity-check orphans                  # Orphan detection only
/topology-sanity-check np-hardness              # 3-SAT reachability only
/topology-sanity-check redundancy               # Rule redundancy only
/topology-sanity-check redundancy <source> <target>  # Check a specific rule
```

Examples:
```
/topology-sanity-check
/topology-sanity-check orphans
/topology-sanity-check np-hardness
/topology-sanity-check redundancy
/topology-sanity-check redundancy MIS ILP
```

---

## Check 1: Orphan Detection (`orphans`)

Finds problem types that have no reduction rules connecting them to the main graph — they are registered but unreachable.

### Run

```bash
cargo run --example detect_isolated_problems 2>&1
```

### Report

Parse the output and produce:

```markdown
## Orphan Detection Report

### Graph Summary
- Total problem types: N
- Connected components: N

### Isolated Problems (N)

| # | Problem | Variants | Category |
|---|---------|----------|----------|
| 1 | ProblemName | 2 (f64, i32) | algebraic |

### Existing Issues

For each isolated problem, search GitHub for open issues that would connect it:

    gh issue list --repo CodingThrust/problem-reductions --state open --search "<problem name>" --json number,title

Report matches or "No issues filed".

### Verdict

- **All connected**: Every problem type is reachable from the main component.
- **N orphans found**: List them and reference issue #610 (meta-issue tracking connectivity).
```

---

## Check 2: NP-Hardness Proof Chains (`np-hardness`)

Verifies that every NP-hard problem has a directed reduction path from 3-SAT, constituting a proof of NP-hardness. Problems without such a path are classified as: in P (correctly unreachable), intermediate complexity, orphans, or NP-hard with a missing proof chain.

### Run

```bash
cargo run --example detect_unreachable_from_3sat 2>&1
```

### Report

Parse the output and produce:

```markdown
## NP-Hardness Proof Chain Report

### Summary
- Total problem types: N
- Reachable from 3-SAT: N
- Not reachable: N

### Reachable from 3-SAT (N)

| # | Problem | Hops |
|---|---------|------|
| 1 | KSatisfiability | 0 |
| 2 | Satisfiability | 1 |

### NP-hard but missing proof chain (N) — needs new reductions

| # | Problem | Outgoing | Incoming |
|---|---------|----------|----------|
| 1 | MaximumClique | 1 | 0 |

### Correctly unreachable

**In P:** MaximumMatching, KSatisfiability(K2), KColoring(K2)
**Intermediate complexity:** Factoring

### Orphans (no edges at all)
[list]

### Verdict

- **PASS**: All NP-hard problems have proof chains from 3-SAT
- **WARN**: N NP-hard problems missing proof chains (list them)
```

The script automatically classifies unreachable problems. Problems with 0 incoming AND 0 outgoing reductions are orphans. Known P-time problems (MaximumMatching, 2-SAT, 2-Coloring) and intermediate-complexity problems (Factoring) are flagged as correctly unreachable. Everything else is reported as a missing proof chain.

---

## Check 3: Rule Redundancy (`redundancy`)

Determines whether reduction rules are redundant (dominated by composite paths through the reduction graph). Can check all primitive rules or a single source-target pair.

### Mode A: Check All Rules (no arguments after `redundancy`)

Run the codebase's `find_dominated_rules` analysis test:

```bash
cargo test test_find_dominated_rules_returns_known_set -- --nocapture 2>&1
```

This runs the analysis from `src/rules/analysis.rs` which:
1. Enumerates every primitive reduction rule (direct edge) in the graph
2. For each, finds all alternative composite paths
3. Uses polynomial normalization and monomial-dominance to compare overheads
4. Reports dominated rules and unknown comparisons

Always report rules with full variant-qualified endpoints, not just base names.
Use the same display style as `ReductionStep`, e.g.
`MaximumIndependentSet {graph: "SimpleGraph", weight: "One"} -> MaximumIndependentSet {graph: "KingsSubgraph", weight: "i32"}`.
Base-name-only summaries are ambiguous and can hide cast-only paths.

Parse the test output and report:

```markdown
## Rule Redundancy Report

### Dominated Rules (N)

| # | Rule | Dominating Path |
|---|------|-----------------|
| 1 | Source {variant...} -> Target {variant...} | A -> B -> C |

### Unknown Comparisons (N)

| # | Rule | Reason |
|---|------|--------|
| 1 | Source {variant...} -> Target {variant...} | expression comparison returned Unknown |

### Allowed (acknowledged) dominated rules

List the entries from the `allowed` set in `test_find_dominated_rules_returns_known_set`
(file: `src/unit_tests/rules/analysis.rs`), and note when that allow-list is keyed only by base names while the reported dominated rule is variant-specific.

### Verdict

- If test passes: all dominated rules are acknowledged in the allow-list.
- If test fails: report the unexpected dominated rule or stale allow-list entry.
```

### Mode B: Check Single Rule (source target arguments)

#### Step 1: Resolve Problem Names

Use MCP tools (`show_problem`) to validate and resolve aliases (MIS = MaximumIndependentSet, MVC = MinimumVertexCover, SAT = Satisfiability, etc.).

#### Step 2: Check if Rule Already Exists

Use `show_problem` on the source and check its `reduces_to` array for a direct edge to the target.

- **Direct edge exists**: Report "Direct rule `<source> -> <target>` already exists" and proceed to redundancy analysis (Step 3).
- **No direct edge**: Report "No direct rule from `<source> -> <target>` exists yet." Then check if any path exists:
  - Use `find_path` MCP tool.
  - **Path exists**: Report the cheapest existing path and its overhead. This is the baseline the proposed new rule must beat to be non-redundant.
  - **No path exists**: Report "No path exists — a new rule would be novel (not redundant)." Stop here.

#### Step 3: Find All Paths

Use `find_path` with `all: true` to get all paths between source and target.

#### Step 4: Compare Overheads

For each composite path (length > 1 step):

1. Extract the **overall overhead** from the path result
2. Extract the **direct rule's overhead** from the single-step path
3. Compare field by field:
   - For polynomial expressions: compare degree — lower degree means the composite is better
   - For equal-degree polynomials: compare leading coefficients
   - For non-polynomial (exp, log): report as "Unknown — manual review needed"

**Dominance definition:** A composite path **dominates** the direct rule if, on every common overhead field, the composite's expression has equal or smaller asymptotic growth.

#### Step 5: Report Results

```markdown
## Redundancy Check: <Source> -> <Target>

### Direct Rule
- Rule: `Source {variant...} -> Target {variant...}`
- Overhead: [field = expr, ...]

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
> The direct rule `Source {variant...} -> Target {variant...}` is dominated by the composite path `[path]`.
> Consider removing it unless it provides value for:
> - Simpler solution extraction (fewer intermediate steps)
> - Educational/documentation clarity
> - Better numerical behavior in practice

If not redundant:
> The direct rule `Source {variant...} -> Target {variant...}` is not dominated by any composite path.
> It provides overhead that cannot be achieved through existing reductions.
```

---

## Combined Report (no arguments)

When invoked with no arguments, run all three checks and produce a combined report. Run Check 1 and Check 2 in parallel (both are `cargo run --example`), then Check 3 sequentially:

```markdown
# Topology Sanity Check

## 1. Orphan Detection
[orphan report]

## 2. NP-Hardness Proof Chains
[np-hardness report]

## 3. Rule Redundancy
[redundancy report]

## Summary
- Orphans: N isolated problems
- NP-hard without proof chain: N
- Dominated rules: N (M acknowledged)
- Unknown comparisons: N
- Overall: PASS / WARN / FAIL
```

**Overall verdict:**
- **PASS**: No orphans, all NP-hard problems have proof chains, no unexpected dominated rules
- **WARN**: Has orphans (tracked in #610), missing proof chains, or unknown comparisons, but no unexpected dominated rules
- **FAIL**: Unexpected dominated rules found (test failure)

## Notes

- "Equal overhead" does not necessarily mean a rule should be removed — direct rules have practical advantages (simpler extraction, fewer steps)
- The analysis uses asymptotic comparison (big-O), so constant factors are ignored
- This means the check can produce false alarms, especially when overhead metadata keeps only leading terms or when a long composite path is asymptotically comparable but practically much worse
- Treat "dominated" as "potentially redundant, requires manual review" unless the composite path is also clearly preferable structurally
- When overhead expressions involve variables from different problems (e.g., `num_vertices` vs `num_clauses`), comparison may not be meaningful — report as Unknown
- The ground truth for what the codebase considers dominated is `src/rules/analysis.rs` (`find_dominated_rules`) with the allow-list in `src/unit_tests/rules/analysis.rs` (`test_find_dominated_rules_returns_known_set`)
