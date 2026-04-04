# Reduction Rules Implementation Log

Branch: `jg/new-rules-batch1`
Started: 2026-04-03

## Rules to implement (High confidence, 16 open)

### Ready (both models exist)
1. [ ] #973 SubsetSum → Partition
2. [ ] #379 MinimumDominatingSet → MinMaxMulticenter
3. [ ] #380 MinimumDominatingSet → MinimumSumMulticenter
4. [ ] #359 HamiltonianPathBetweenTwoVertices → LongestPath

### Need target model first
5. [ ] #388 ExactCoverBy3Sets → SubsetProduct
6. [ ] #844 KColoring → PartitionIntoCliques
7. [ ] #382 NAESatisfiability → SetSplitting
8. [ ] #481 Partition → OpenShopScheduling
9. [ ] #488 Partition → ProductionPlanning
10. [ ] #471 Partition → SequencingToMinimizeTardyTaskWeight
11. [ ] #868 Satisfiability → NonTautology
12. [ ] #569 SubsetSum → IntegerExpressionMembership
13. [ ] #882 3SAT → Kernel
14. [ ] #554 3SAT → SimultaneousIncongruences
15. [ ] #860 ExactCoverBy3Sets → MinimumWeightSolutionToLinearEquations
16. [ ] #911 HamiltonianPath → DegreeConstrainedSpanningTree

## Unexpected Events

### 2026-04-03 - #359 HamiltonianPathBetweenTwoVertices → LongestPath
- **Issue:** Initially classified as "ready" (both models exist), but `HamiltonianPathBetweenTwoVertices` model does NOT exist in the codebase. Only `HamiltonianPath` exists (different problem - no fixed endpoints).
- **Impact:** Moved from "ready" to "needs source model first". Only 3 rules are truly ready (both models exist): #973, #379, #380.

### 2026-04-03 - #379 and #380 MinDomSet → Multicenter rules BLOCKED
- **Issue:** Both issues explicitly say the reduction is blocked due to optimization vs decision framing mismatch.
  - MinimumDominatingSet has `Value = Min<W::Sum>` (optimization) with no K parameter
  - #379: MinMaxMulticenter has `Value = Or` → Min→Or type mismatch
  - #380: MinimumSumMulticenter has `Value = Min<W::Sum>` → types match but k parameter needs to come from an unknown optimal dominating set size (circular dependency)
- **Impact:** Only 1 rule is truly ready to implement immediately: #973. All other 15 need either models or type resolution.
- **Resolution:** These require a decision-variant `DominatingSet(G, K)` model first. Proceeding with rules that need target models created.

### 2026-04-03 - Updated confidence file adds 3 new High-confidence rules
- **File:** `~/Downloads/reduction_derivations_low_tier_reinspected.typ`
- **New High entries (20 total, was 17):**
  - HamiltonianPath → Isomorphic Spanning Tree (#912) — promoted from Low
  - NAE-Satisfiability → Maximum Cut (#166) — promoted from Low
  - X3C → Algebraic Equations over GF(2) (#859) — promoted from Low
- **New Medium entries (16 total, was 14):**
  - 3SAT → Feasible Register Assignment (#905) — promoted from Low
  - 3SAT → Quadratic Congruences (#553) — promoted from Low
- **Impact:** 3 additional rules to implement in the High-confidence batch.

### 2026-04-03 - All 12 target model issues are CLOSED but models NOT implemented
- **Issue:** All target model issues (#834, #830, #506, #513, #867, #885, #537, #852, #896, #552, #496, #854) show status CLOSED:COMPLETED but none of the models actually exist in the codebase.
- **Impact:** Must create all 12 target models from scratch as part of each rule implementation.
- **Resolution:** Each codex invocation includes both add-model and add-rule steps.

## Progress

| # | Rule | Status | Commit |
|---|------|--------|--------|
| # | Rule | Status | Commit |
|---|------|--------|--------|
| #973 | SubsetSum → Partition | DONE | 6398de17 |
| #868 | Satisfiability → NonTautology | DONE | 1784af77 |
| #844 | KColoring → PartitionIntoCliques | DONE | 188f24b8 |
| #882 | 3SAT → Kernel | DONE | 7cfb16a1 |
| #911 | HamPath → DegreeConstrainedSpanningTree | DONE | 745f90a0 |
| #382 | NAE-SAT → SetSplitting | DONE | 5e7e982d |
| #388 | X3C → SubsetProduct | DONE | bdf8ef48 |
| #569 | SubsetSum → IntegerExprMembership | DONE | 692680fa |
| #860 | X3C → MinWeightSolnLinEq | DONE | 4473224e |
| #554 | 3SAT → SimultaneousIncongruences | DONE | 0cc816db |
| #471 | Partition → SeqMinTardyTaskWeight | DONE | bb6de6c7 |
| #481 | Partition → OpenShopScheduling | TODO | - |
| #488 | Partition → ProductionPlanning | TODO | - |
| #379 | MinDomSet → MinMaxMulticenter | BLOCKED | - |
| #380 | MinDomSet → MinSumMulticenter | BLOCKED | - |
| #359 | HamPathBetween2 → LongestPath | BLOCKED | - |
| #912 | HamPath → IsomorphicSpanningTree | TODO (new) | - |
| #166 | NAE-SAT → MaximumCut | TODO (new) | - |
| #859 | X3C → AlgEqOverGF2 | TODO (new) | - |

