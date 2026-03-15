# Paper Redesign Spec

**Date:** 2026-03-14
**Title:** Bridging NP-Hard Problems: Scaling Software Beyond Human Capacity with Agentic Coding

## Core Thesis

**Bridge problems** are software projects too large for humans at scale. Agents can build them because systematic verification constrains agent output to match contributor-specified ground truth. NP-hard reductions are the first convincing example.

## Bridge Problem Definition

A software project where subtasks are homogeneous and formally verifiable, but three structural barriers make human-only execution infeasible at scale:

1. **Convention drift** — humans can't maintain uniform conventions across hundreds of contributions; agents read CLAUDE.md every time
2. **Effort exhaustion** — humans can't sustain the energy to verify 100+ problems and continuously test without a user community
3. **Knowledge discontinuity** — humans graduate, newcomers can't absorb implicit knowledge; skills make onboarding executable

The correctness concern: contributor-specified ground truth (definitions, examples, expected behavior) flows through the verification stack (type system → round-trip tests → overhead validation → agentic tests), constraining what agents can produce. Agent output ⊆ contributor intent.

## Section Structure

### Section 1: Introduction (~1.5 pages)
- Open with familiar examples (airlines, chips, logistics → NP-hard problems → need reductions)
- The reduction graph idea (connect problems to solvers)
- **The claim:** This is a *bridge problem* — software too large for humans, made possible by agents constrained through systematic verification
- Three barriers preview
- **Fig 1: Scaling Wall**
- Contributions bullet list

### Section 2: Bridge Problems (~2 pages)
- Define bridge problems formally
- Three barriers, each with concrete evidence from this project:
  - Convention drift: agents never deviated from file naming, trait implementation, test patterns
  - Effort exhaustion: 45 rules in 9 weeks vs Julia predecessor 20 types in 4 years; agents never tire of running the same verification loop
  - Knowledge discontinuity: skills encode workflow as executable documents; new maintainers invoke same skills that produced the codebase
- The correctness concern and how verification addresses it
- **Fig 2: Verification Funnel** — agent generates candidates → type system rejects invalid structure → round-trip tests reject wrong semantics → agentic tests reject poor UX → only correct code survives
- Other candidate domains (algorithm libraries, compiler optimization passes, HDL, numerical linear algebra)

### Section 3: Case Study — The Reduction Graph (~2 pages)
- What is a reduction (brief definition)
- Graph structure: 27 problems, 45 rules, 56 edges
- **Fig 3: Reduction Graph** with color-coded solver-reachability arrows:
  - Blue = paths reaching ILP (Gurobi/CPLEX)
  - Red = paths reaching QUBO (D-Wave)
  - Green = paths reaching UD-MIS (Rydberg atoms)
  - Nodes colored by which solvers they can reach (multi-color = multiple solvers)
  - Reader traces arrows forward (problem → solver) or backward (solver → problems)
- Emergent compositionality highlighted as multi-hop colored paths (e.g., Factoring → CircuitSAT → SAT → ILP)
- Round-trip testing (brief)

### Section 4: Methodology — Skills + Verification (~2 pages)
- Skills: persistent, versioned workflow scripts that encode convention
- **Fig 4: Pipeline** (existing 6-stage board, orange=human, blue=agent)
- The 14 skills and what they do (table or compact list)
- Verification stack in practice (type system, unit tests, closed-loop, overhead validation, agentic tests)

### Section 5: Evidence (~2 pages)
- **Fig 5: Development Timeline** — cumulative plot of problem types + rules over 9 weeks, phase annotations (manual → basic-skills → full-pipeline), Julia predecessor 4-year trajectory overlaid
- Development metrics (58 PRs, 15:1 agent-to-human message ratio)
- Quality gate: 75% rejection rate on 322 batch-submitted proposals
- Barrier-by-barrier evidence:
  - Convention: zero convention violations in agent-authored code
  - Effort: acceleration curve across phases
  - Continuity: skills as executable onboarding (dev-setup, add-rule, etc.)

### Section 6: Discussion (~1.5 pages)
- Limitations (N=1, skill engineering cost, confounding factors)
- Why human experts remain essential (LLM reasoning limits, citing existing research)
- Future work (100+ problems, formal verification with Lean/Coq, cost-aware path selection, automated discovery via AlphaEvolve)

### Appendices
- A1: System Architecture (trait hierarchy, ReduceTo, macros) — existing figure
- A2: Verification Stack Details (7-layer pyramid) — existing figure
- A3: Topology Issues (orphan nodes, redundant rules, NP-hardness gaps) — moved from main text

## Figure Specifications

### Fig 1: Scaling Wall (NEW)
- **Type:** Line chart with annotations
- **X-axis:** Number of problem types (0 → 200)
- **Y-axis:** Software quality (convention compliance, test coverage, documentation completeness)
- **Lines:**
  - Human team trajectory: rises then plateaus/declines as it hits 3 barrier walls
  - Agent + verification trajectory: breaks through all 3 walls
- **Annotations:** Three vertical dashed lines marking the barriers (convention drift, effort exhaustion, knowledge discontinuity)
- **Data points:** Julia predecessor at 20 (4 years), this work at 27 (9 weeks), vision at 100+
- **Format:** Typst/CeTZ or TikZ

### Fig 2: Verification Funnel (NEW)
- **Type:** Funnel/filter diagram
- **Flow:** Wide at top (agent generates many candidate implementations) → narrowing through filters → narrow at bottom (correct code)
- **Layers (top to bottom):**
  1. Agent output (wide) — "many plausible implementations"
  2. Type system filter — "rejects structural errors (wrong trait impl, type mismatch)"
  3. Round-trip tests filter — "rejects semantic errors (wrong transformation, broken inverse)"
  4. Overhead validation — "rejects incorrect complexity claims"
  5. Agentic feature tests — "rejects UX/documentation issues"
  6. Correct code (narrow) — "matches contributor ground truth"
- **Side annotation:** "Contributor-specified ground truth" arrow pointing into each filter level
- **Format:** Typst/CeTZ or TikZ

### Fig 3: Reduction Graph (REDESIGN)
- **Base:** Existing 27-node directed graph
- **Enhancement:** Color-coded edges by solver reachability
  - Blue edges/paths → ILP (Gurobi/CPLEX)
  - Red edges/paths → QUBO (D-Wave quantum annealer)
  - Green edges/paths → UD-MIS (Rydberg atom arrays)
- **Node coloring:** Nodes tinted by which solvers they can reach (multi-color for multiple solvers)
- **Solver hubs:** Prominent labels at bottom: "Gurobi/CPLEX", "D-Wave", "Rydberg"
- **Key insight:** Same graph answers both "what can this solver solve?" and "what solvers can this problem reach?"
- **Format:** Typst/CeTZ (redesign existing reduction-graph.typ)

### Fig 4: Pipeline (EXISTING — keep as-is)
- 6-stage Kanban board
- Orange = human judgment points, Blue = agent-automated steps

### Fig 5: Development Timeline (NEW)
- **Type:** Cumulative line plot
- **X-axis:** Time (weeks 1-9, with dates)
- **Y-axis (left):** Cumulative count (problem types, reduction rules)
- **Lines:** Two lines — problem types (solid) and reduction rules (dashed)
- **Phase bands:** Background shading for manual / basic-skills / full-pipeline phases
- **Overlay:** Julia predecessor trajectory (4 years to 20 types) shown as a faint reference line, dramatically slower
- **Data source:** git-mining-results.json
- **Format:** Typst/CeTZ-plot or matplotlib-generated PDF

## Key Changes from Current Paper
1. "Bridge problem" concept elevated from Discussion to Section 2
2. Reduction graph becomes case study illustrating the concept, not the main event
3. 4 impossibilities merged to 3 barriers (effort exhaustion + testing frequency combined)
4. Framing: "agents break through barriers, verification ensures correctness" (not "humans + agents must combine")
5. Reduction graph figure redesigned with solver-reachability coloring
6. Three Roles figure cut (pipeline is sufficient)
7. Topology Issues figure moved to appendix
8. Problem tree figure absorbed into reduction graph or cut
