# Design: Proposed Reduction Rules — Typst Verification Note

**Date:** 2026-03-31
**Goal:** Create a standalone Typst document with compiled PDF that formalizes 9 reduction rules from issue #770, resolving blockers for 7 incomplete issues and adding 2 high-leverage NP-hardness chain extensions.

## Scope

### 9 Reductions

**Group 1 — NP-hardness proof chain extensions:**

| Issue | Reduction | Impact |
|-------|-----------|--------|
| #973 | SubsetSum → Partition | Unlocks ~12 downstream problems |
| #198 | MinimumVertexCover → HamiltonianCircuit | Unlocks ~9 downstream problems |

**Group 2 — Tier 1a blocked issues (fix + formalize):**

| Issue | Reduction | Current blocker |
|-------|-----------|----------------|
| #379 | DominatingSet → MinMaxMulticenter | Decision vs optimization MDS model |
| #380 | DominatingSet → MinSumMulticenter | Same |
| #888 | OptimalLinearArrangement → RootedTreeArrangement | Witness extraction impossible for naive approach |
| #822 | ExactCoverBy3Sets → AcyclicPartition | Missing algorithm (unpublished reference) |

**Group 3 — Tier 1b blocked issues (fix + formalize):**

| Issue | Reduction | Current blocker |
|-------|-----------|----------------|
| #892 | VertexCover → HamiltonianPath | Depends on #198 (VC→HC) being resolved |
| #894 | VertexCover → PartialFeedbackEdgeSet | Missing Yannakakis 1978b paper |
| #890 | MaxCut → OptimalLinearArrangement | Placeholder algorithm, no actual construction |

## Deliverables

1. **`docs/paper/proposed-reductions.typ`** — standalone Typst document
2. **`docs/paper/proposed-reductions.pdf`** — compiled PDF checked into repo
3. **Updated GitHub issues** — #379, #380, #888, #822, #892, #894, #890 corrected with verified algorithms
4. **One PR** containing the note, PDF, and issue updates

## Document Structure

```
Title: Proposed Reduction Rules — Verification Notes
Abstract: Motivation (NP-hardness gaps, blocked issues, impact analysis)

§1 Notation & Conventions
  - Standard symbols (G, V, E, w, etc.)
  - Proof structure: Construction → Correctness (⟹/⟸) → Solution Extraction
  - Overhead notation

§2 NP-Hardness Chain Extensions
  §2.1 SubsetSum → Partition (#973)
  §2.2 MinimumVertexCover → HamiltonianCircuit (#198)
  §2.3 VertexCover → HamiltonianPath (#892)

§3 Graph Reductions
  §3.1 MaxCut → OptimalLinearArrangement (#890)
  §3.2 OptimalLinearArrangement → RootedTreeArrangement (#888)

§4 Set & Domination Reductions
  §4.1 DominatingSet → MinMaxMulticenter (#379)
  §4.2 DominatingSet → MinSumMulticenter (#380)
  §4.3 ExactCoverBy3Sets → AcyclicPartition (#822)

§5 Feedback Set Reductions
  §5.1 VertexCover → PartialFeedbackEdgeSet (#894)
```

## Per-Reduction Entry Format

Each reduction follows the `reductions.typ` convention:

1. **Theorem statement** — 1-3 sentence intuition, citation (e.g., `[GJ79, ND50]`)
2. **Proof** with three mandatory subsections:
   - _Construction._ Numbered algorithm steps, all symbols defined before use
   - _Correctness._ Bidirectional: (⟹) forward direction, (⟸) backward direction
   - _Solution extraction._ How to map target solution back to source
3. **Overhead table** — target size fields as functions of source size fields
4. **Worked example** — concrete small instance, full construction steps, solution verification

Mathematical notation uses Typst math mode: `$V$`, `$E$`, `$arrow.r.double$`, `$overline(x)$`, etc.

## Research Plan for Blocked Issues

For each blocked reduction:

1. **Search** for the original reference via the citation in Garey & Johnson
2. **Reconstruct** the correct algorithm from the paper or from first principles
3. **Verify** correctness with a hand-worked example in the note
4. **Resolve** the blocker:
   - #379/#380: Clarify that the reduction operates on the decision variant; note model alignment needed
   - #888: Research Gavril 1977a gadget construction for forcing path-tree solutions
   - #822: Research the acyclic partition reduction from G&J or construct from first principles
   - #892: Chain through #198 (VC→HC→HP); detail the HC→HP modification
   - #894: Search for Yannakakis 1978b or reconstruct the gadget
   - #890: Research the Garey-Johnson-Stockmeyer 1976 construction

If a reference is unavailable, construct a novel reduction and clearly mark it as such.

## Typst Setup

- Standalone document (not importing from `reductions.typ`)
- Uses: `ctheorems` for theorem/proof environments, `cetz` if diagrams needed
- Page: A4, New Computer Modern 10pt
- Theorem numbering: `Theorem 2.1`, `Theorem 2.2`, etc.
- No dependency on `examples.json` or `reduction_graph.json` (standalone)
- Compile command: `typst compile docs/paper/proposed-reductions.typ docs/paper/proposed-reductions.pdf`

## Quality Criteria

Each reduction must satisfy:
1. **Math equations correct** — all formulas verified against source paper or hand-derivation
2. **Provable correctness** — both directions of the proof are rigorous, no hand-waving
3. **Algorithm clear** — detailed enough that a developer can implement `reduce_to()` and `extract_solution()` directly from the proof
4. **From math to code verifiable** — overhead expressions match the construction, worked example can be used as a test case

## PR Structure

- Branch: `feat/proposed-reductions-note`
- Files:
  - `docs/paper/proposed-reductions.typ` (new)
  - `docs/paper/proposed-reductions.pdf` (new, compiled)
- No code changes — this is a documentation-only PR
- Issue updates done via `gh issue edit` (not in the PR diff)
