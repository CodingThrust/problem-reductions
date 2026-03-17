---
name: Problem
about: Propose a new problem type
title: "[Model] MinSumMulticenter"
labels: model
assignees: ''
---

## Motivation

MIN-SUM MULTICENTER (P127) from Garey & Johnson, A2 ND51. A classical NP-complete facility location problem, also known as the p-median problem. Given a graph with vertex weights and edge lengths, the goal is to place K service centers so as to minimize the total weighted distance from all vertices to their nearest centers. Arises in optimal placement of warehouses, hospitals, schools, and other service facilities. Unlike the min-max variant (p-center), which minimizes the worst-case distance, the p-median minimizes average/total service cost.

**Associated reduction rules:**
- As target: R71 (DOMINATING SET -> MIN-SUM MULTICENTER)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinSumMulticenter`
**Canonical name:** <!-- ⚠️ Unverified: web search --> p-Median Problem; also: Min-Sum Multicenter, Uncapacitated Facility Location (variant)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND51

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" such that if d(v) is the length of the shortest path from v to the closest point in P, then Sigma_{v in V} d(v)·w(v) ≤ B?

The optimization version asks: find K points on G minimizing the total weighted distance Sigma_{v in V} d(v)·w(v).

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| binary variables (one per vertex). As noted by GJ, there is no loss of generality in restricting centers to vertices.
- **Per-variable domain:** binary {0, 1} — whether vertex v is selected as a center
- **Meaning:** variable x_v = 1 if vertex v is chosen as a center location. Exactly K variables must be set to 1. The configuration is valid if Sigma_{v in V} d(v)·w(v) ≤ B, where d(v) is the shortest weighted-path distance from v to the nearest selected center.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MinSumMulticenter`
**Variants:** graph topology, weight type

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The underlying graph G = (V, E) |
| `vertex_weights` | `Vec<W>` | Non-negative weight w(v) for each vertex v |
| `edge_lengths` | `Vec<W>` | Non-negative length l(e) for each edge e |
| `k` | `usize` | Number of centers to place (K) |

**Notes:**
- This is a satisfaction (decision) problem in the GJ formulation: `Metric = bool`, implementing `SatisfactionProblem`.
- Alternatively, the optimization version minimizes total weighted distance for a given K — then `Metric = SolutionSize<W>`, implementing `OptimizationProblem` with `Direction::Minimize`.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_centers()` (= K).
- Per GJ comment: restricting P to a subset of V (vertex-restricted variant) does not lose generality for p-median.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Kariv and Hakimi, 1979; transformation from DOMINATING SET). Remains NP-complete even with unit weights and unit edge lengths.
- **Best known exact algorithm:** The p-median problem is typically solved via integer linear programming with branch-and-bound. For the unweighted unit-length variant, the problem is closely related to dominating set. Exact exponential-time algorithms are based on LP relaxation and branch-and-bound, with worst-case time O*(2^n). No faster general exact exponential algorithm is known specifically for p-median beyond ILP approaches.
- **Polynomial cases:** Solvable in polynomial time for fixed K and for arbitrary K on trees (Kariv and Hakimi, 1979).
- **Approximation:** (1 + 3/e)-approximation by Charikar et al. (1999); LP-rounding approaches give better constants.
- **References:**
  - O. Kariv, S. L. Hakimi (1979). "An Algorithmic Approach to Network Location Problems. II: The p-Medians." *SIAM J. Appl. Math.*, 37(3):539-560.
  - M. Charikar, S. Guha, E. Tardos, D. B. Shmoys (1999). "A Constant-Factor Approximation Algorithm for the k-Median Problem." *Proc. 31st ACM STOC*, pp. 1-10.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** General facility location / median location problems
- **Known special cases:**
  - Vertex p-median: centers restricted to vertices (no loss of generality per GJ)
  - Unweighted unit-length variant: w(v)=1, l(e)=1 (still NP-complete)
  - Tree graphs: polynomial-time solvable (Kariv and Hakimi, 1979)
  - Fixed K: polynomial-time solvable
- **Related problems:** MIN-MAX MULTICENTER (p-center, ND50) uses bottleneck objective instead of sum

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" such that if d(v) is the length of the shortest path from v to the closest point in P, then Sigma_{v in V} d(v)·w(v) ≤ B?
Reference: [Kariv and Hakimi, 1976b]. Transformation from DOMINATING SET.
Comment: Also known as the "p-median" problem. It can be shown that there is no loss of generality in restricting P to being a subset of V. Remains NP-complete if w(v) = 1 for all v ∈ V and l(e) = 1 for all e ∈ E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all C(n, K) subsets of K vertices; for each subset, compute all-pairs shortest paths and evaluate Sigma_{v in V} d(v)·w(v). Check feasibility against B.
- [x] It can be solved by reducing to integer programming. Binary variable x_v for each vertex (center selection) and assignment variables y_{v,u} for each vertex-center pair; minimize Sigma_{v,u} w(v)·d(v,u)·y_{v,u} subject to: each vertex assigned to exactly one center, assignment only to selected centers, exactly K centers.
- [x] Other: Lagrangian relaxation with subgradient optimization; column generation / branch-and-price.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 8 edges, unit weights and lengths:**
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,6}, {0,6}, {2,5}
- K = 2, B = 5

**All-pairs shortest distances (unit edge lengths):**
All w(v) = 1, l(e) = 1.

**Optimal 2-median:** Place centers at vertices {2, 5}.
- d(0) = dist(0, {2,5}) = min(dist(0,2), dist(0,5)) = min(2, 2) = 2
- d(1) = dist(1, {2,5}) = min(1, 3) = 1
- d(2) = 0 (center)
- d(3) = dist(3, {2,5}) = min(1, 2) = 1
- d(4) = dist(4, {2,5}) = min(2, 1) = 1
- d(5) = 0 (center)
- d(6) = dist(6, {2,5}) = min(3, 1) = 1

Total weighted distance = 2 + 1 + 0 + 1 + 1 + 0 + 1 = 6.

Wait -- that gives 6 > B=5. Let us try centers at {1, 5}:
- d(0) = 1, d(1) = 0, d(2) = 1, d(3) = dist(3,{1,5}) = min(2,2) = 2, d(4) = min(3,1) = 1, d(5) = 0, d(6) = min(3,1) = 1
- Total = 1 + 0 + 1 + 2 + 1 + 0 + 1 = 6. Same.

Try B = 6: answer is YES with P = {2, 5} or {1, 5}.
Try B = 5: check {0, 3}: d(0)=0, d(1)=1, d(2)=2, d(3)=0, d(4)=1, d(5)=2, d(6)=1. Total = 0+1+2+0+1+2+1 = 7. No.
Check {0, 4}: d(0)=0, d(1)=1, d(2)=2, d(3)=1, d(4)=0, d(5)=1, d(6)=1. Total = 6. No.

**Corrected example: B = 6, K = 2.**
- Centers {2, 5}: total = 6 ≤ 6 ✓. Answer: YES.
- For K = 2, B = 5: no placement of 2 centers achieves total ≤ 5. Answer: NO.
