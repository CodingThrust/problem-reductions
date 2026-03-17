---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumFeedbackArcSet"
labels: model
assignees: ''
---

## Motivation

FEEDBACK ARC SET (P19) from Garey & Johnson, A1.1 GT8. A classical NP-complete problem (Karp, 1972) that asks for the minimum number of arcs to remove from a directed graph to make it acyclic. It appears in ranking aggregation, sports tournament scheduling, deadlock avoidance, and causal inference. The problem is the "arc" analogue of the Feedback Vertex Set problem; unlike undirected feedback arc set (which is trivially solvable since every cycle must have an edge and spanning trees are cycle-free), the directed version is NP-complete.

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinimumFeedbackArcSet`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Directed Feedback Arc Set (DFAS); also: Minimum Feedback Arc Set, Minimum Acyclic Subgraph (complement formulation)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT8

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that A' contains at least one arc from every directed cycle in G?

The optimization version (which is the natural Rust model) asks: find the minimum-size subset A' ⊆ A such that G − A' is a directed acyclic graph (equivalently, find the maximum acyclic subgraph).

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** m = |A| binary variables (one per arc)
- **Per-variable domain:** binary {0, 1} — whether arc a ∈ A is included in the feedback arc set A'
- **Meaning:** variable x_a = 1 if arc a is selected into the FAS. The configuration (x_0, ..., x_{m-1}) encodes a candidate subset A' ⊆ A. The assignment is valid (a valid FAS) if for every directed cycle C in G, at least one arc of C has x_a = 1.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MinimumFeedbackArcSet`
**Variants:** graph topology (directed graph type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `DirectedGraph` | The directed graph G = (V, A) on which a feedback arc set is sought |

**Notes:**
- This is an optimization (minimization) problem: `Metric = SolutionSize<i32>`, implementing `OptimizationProblem`.
- The objective is to minimize |A'|, the number of selected arcs.
- Variables are indexed over arcs (edges), not vertices — the configuration space has dimension m = |A|.
- Key getter methods needed: `num_vertices()` (= |V|), `num_arcs()` (= |A|).
- The complement formulation (maximum acyclic subgraph) keeps m − |A'| arcs and is equivalent.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Karp, 1972; transformation from VERTEX COVER). Item GT8 on Karp's original list of 21 NP-complete problems.
- **Best known exact algorithm:** O*(2^n) time via dynamic programming over subsets of vertices (Bodlaender et al.; the Gurevich-Shelah / Held-Karp style DP over vertex orderings). This is the best known worst-case bound for the general directed FAS problem; improving this to sub-2^n is an explicit open problem.
- **Alternative exact approach:** O*(2^m) brute force over arc subsets, where m = |A| (enumerate all 2^m subsets A' ⊆ A and check acyclicity). Practical exact methods use branch-and-bound with ILP or cycle enumeration (Baharev, Schichl, Neumaier, Achterberg, 2021).
- **Polynomial-time solvable cases:** Planar digraphs (Luchesi, 1976; via Luchesi-Younger theorem relating minimum FAS to maximum cycle packing). Also solvable for DAGs (trivially: empty FAS).
- **FPT (parameterized by k):** Fixed-parameter tractable: O(n^{O(1)} · 4^k · k!) algorithm via iterative compression (not yet matching the vertex-deletion FPT bound); whether FAS has a polynomial kernel remains open.
- **References:**
  - R.M. Karp (1972). "Reducibility Among Combinatorial Problems." *Complexity of Computer Computations*, pp. 85–103. Plenum Press. Original NP-completeness proof.
  - C.L. Luchesi and S.L. Younger (1978). "A minimax theorem for directed graphs." *Journal of the London Mathematical Society*, 17(3):369–374. Polynomial time for planar digraphs.
  - A. Baharev, H. Schichl, A. Neumaier, T. Achterberg (2021). "An Exact Method for the Minimum Feedback Arc Set Problem." *ACM Journal of Experimental Algorithmics*, 26. Practical exact solver.
  - H.L. Bodlaender, E. Fomin, D. Lokshtanov, E. Penninkx, S. Saurabh, D.M. Thilikos (2009). Kernelization survey. Exact O*(2^n) DP baseline.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** (none — FAS is a fundamental problem)
- **Known special cases:** FAS on tournament graphs (every pair of vertices has exactly one arc; solvable in O*(1.6181^n) and has a 3-approximation), FAS on planar digraphs (polynomial time)
- **Restriction:** The undirected analogue (minimum feedback edge set in an undirected graph) is trivially polynomial — it equals m − (n − c) where c is the number of connected components (i.e., the number of non-tree edges in a spanning forest). The directed version is fundamentally harder due to directed cycle structure.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that A' contains at least one arc from every directed cycle in G?
Reference: [Karp, 1972]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for digraphs in which no vertex has total indegree and out-degree more than 3, and for edge digraphs [Gavril, 1977a]. Solvable in polynomial time for planar digraphs [Luchesi, 1976]. The corresponding problem for undirected graphs is trivially solvable in polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all 2^m subsets A' ⊆ A; for each candidate, check if G − A' is a DAG (using a DFS-based cycle detection in O(n + m)). Total time O(2^m · (n + m)).
- [x] It can be solved by reducing to integer programming. Introduce binary variable x_a for each arc a; minimize ∑x_a subject to: for each directed cycle C in G, ∑_{a ∈ C} x_a ≥ 1. (Exponentially many constraints, handled via iterative constraint generation / branch-and-cut.)
- [x] Other: Dynamic programming over vertex subsets in O*(2^n) time (Held-Karp style); exact branch-and-bound solvers (Baharev et al. 2021); FPT algorithms parameterized by k.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Directed graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 arcs:**
- Arc list (indexed 0–8):
  - a_0: (0→1), a_1: (1→2), a_2: (2→0)  — forms directed cycle C_1: 0→1→2→0
  - a_3: (1→3), a_4: (3→4), a_5: (4→1)  — forms directed cycle C_2: 1→3→4→1
  - a_6: (2→5), a_7: (5→3), a_8: (3→0)  — forms directed cycle C_3: 0→1→3→0 (via a_0, a_3, a_8) and C_4: 2→5→3→0→1→2 (long cycle)

Directed cycles:
- C_1: 0→1→2→0 (arcs a_0, a_1, a_2)
- C_2: 1→3→4→1 (arcs a_3, a_4, a_5)
- C_3: 0→1→3→0 (arcs a_0, a_3, a_8)
- C_4: 2→5→3→0→1→2 (arcs a_6, a_7, a_8, a_0, a_1) — wait: a_8 is (3→0), so: 2→5→3→0→1→2

**Budget:** K = 2

**Minimum FAS analysis:**
- Can we break all cycles with 2 arcs?
  - Select A' = {a_2, a_5} = {(2→0), (4→1)}:
    - C_1 (0→1→2→0): arc a_2 = (2→0) ∈ A' ✓
    - C_2 (1→3→4→1): arc a_5 = (4→1) ∈ A' ✓
    - C_3 (0→1→3→0): neither a_2 nor a_5 appears. C_3 uses arcs a_0, a_3, a_8. NOT broken ✗
  - Select A' = {a_2, a_8} = {(2→0), (3→0)}:
    - C_1: a_2 ∈ A' ✓
    - C_2: neither arc in C_2 — NOT broken ✗
  - Select A' = {a_0, a_4} = {(0→1), (3→4)}:
    - C_1 (a_0, a_1, a_2): a_0 ∈ A' ✓
    - C_2 (a_3, a_4, a_5): a_4 ∈ A' ✓
    - C_3 (a_0, a_3, a_8): a_0 ∈ A' ✓
    - C_4 (a_6, a_7, a_8, a_0, a_1): a_0 ∈ A' ✓
    - All cycles broken! ✓
  - **Minimum FAS = {a_0, a_4} = {(0→1), (3→4)}, size = 2.**

**Verification:**
After removing arcs (0→1) and (3→4), remaining arcs:
- (1→2), (2→0), (1→3), (4→1), (2→5), (5→3), (3→0)
- Check for cycles: Try to find a cycle not using removed arcs. Starting from 0: 0 has no outgoing arcs left. Starting from 1: 1→2→0 (0 has no outgoing arcs → dead end). 1→3→0 (dead end). No cycle reachable. DAG ✓

**Greedy trap:** Greedily removing the arc appearing in the most cycles might choose arc a_0 (appears in C_1, C_3, C_4 — 3 cycles). After removing a_0, cycle C_2 still remains (1→3→4→1), requiring a second removal. The optimal solution {a_0, a_4} achieves this, but a greedy strategy could instead pick a_2 first (appears in 1 cycle), then need 2 more arcs for C_2 and C_3. The greedy order matters significantly.
