---
name: Problem
about: Propose a new problem type
title: "[Model] LongestPath"
labels: model
assignees: ''
---

## Motivation

LONGEST PATH (P105) from Garey & Johnson, A2 ND29. A classical NP-complete problem useful for reductions. Asks whether a graph contains a simple s-t path whose total edge length meets or exceeds a bound K. NP-complete even with unit edge lengths, where it generalizes the Hamiltonian path problem. Solvable in polynomial time on DAGs.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in the current rule set.
- **As target:** R50: HAMILTONIAN PATH BETWEEN TWO VERTICES -> LONGEST PATH

## Definition

**Name:** `LongestPath`
<!-- ⚠️ Unverified -->
**Canonical name:** LONGEST PATH (also: Longest Simple Path, Maximum Weight Path)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND29

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K, specified vertices s,t ∈ V.
QUESTION: Is there a simple path in G from s to t of length K or more, i.e., whose edge lengths sum to at least K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |E| binary variables (one per edge), indicating whether the edge is included in the path.
- **Per-variable domain:** {0, 1} -- edge is excluded or included in the s-t path.
- **Meaning:** The variable assignment encodes a subset of edges. A satisfying assignment is a subset S of E such that the subgraph induced by S forms a simple path from s to t and the sum of l(e) for e in S is at least K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `LongestPath`
**Variants:** graph type (G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `G` | The undirected graph G = (V, E) |
| `lengths` | `Vec<W>` | Edge length l(e) for each edge (indexed by edge index) |
| `source` | `usize` | Index of source vertex s |
| `target` | `usize` | Index of target vertex t |
| `bound` | `W` | The length bound K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- For the optimization variant, one would maximize the path length (removing the bound K), making it an `OptimizationProblem` with `Direction::Maximize`.
- The unit-weight special case is equivalent to finding the longest simple path by hop count.
- Polynomial-time solvable on DAGs (directed acyclic graphs) via topological sort + DP.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(2^n) via inclusion-exclusion / algebraic methods. The color-coding technique of Alon, Yuster, and Zwick (1995) solves the k-path problem in O*(2^k) time (FPT in path length k). For general longest path, exhaustive search over all simple paths dominates.
- **Classic algorithm:** O(n! / (n-k)!) brute force over all simple paths of length k; or O(n * 2^n) dynamic programming over vertex subsets (similar to Held-Karp).
- **NP-completeness:** NP-complete by transformation from HAMILTONIAN PATH BETWEEN TWO VERTICES (Garey & Johnson, ND29). Remains NP-complete with unit edge lengths.
- **Special cases:** Polynomial-time on DAGs, trees, interval graphs, and some other restricted graph classes.
- **References:**
  - N. Alon, R. Yuster, U. Zwick (1995). "Color-coding." *Journal of the ACM*, 42(4):844-856.
  - R. Williams (2009). "Finding paths of length k in O*(2^k) time." *Information Processing Letters*, 109(6):315-318.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K, specified vertices s,t ∈ V.
QUESTION: Is there a simple path in G from s to t of length K or more, i.e., whose edge lengths sum to at least K?
Reference: Transformation from HAMILTONIAN PATH BETWEEN TWO VERTICES.
Comment: Remains NP-complete if l(e) = 1 for all e ∈ E, as does the corresponding problem for directed paths in directed graphs. The general problem can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a]. The analogous directed and undirected "shortest path" problems can be solved for arbitrary graphs in polynomial time (e.g., see [Lawler, 1976a]), but are NP-complete if negative lengths are allowed.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all simple s-t paths and check if total length >= K.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Held-Karp-style DP in O(n * 2^n); color-coding in O*(2^k) for k-length paths.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES -- path of length >= K exists):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 10 edges:
- Edges with lengths: {0,1}:3, {0,2}:2, {1,3}:4, {2,3}:1, {2,4}:5, {3,5}:2, {4,5}:3, {4,6}:2, {5,6}:4, {1,6}:1
- s = 0, t = 6, K = 16
- Simple path: 0 -> 2 -> 4 -> 5 -> 3 -> 1 -> 6
  - Length: 2 + 5 + 3 + 2 + 4 + 1 = 17 >= K = 16
- Answer: YES

**Instance 2 (NO -- no path of length >= K):**
Same graph, s = 0, t = 6, K = 20.
- The longest simple path from 0 to 6 visits all vertices.
- Path 0->1->3->5->4->2->... cannot reach 6 without revisiting.
- Best path: 0->2->4->5->3->1->6 has length 17.
- No simple s-t path has length >= 20.
- Answer: NO
