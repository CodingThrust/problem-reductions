---
name: Problem
about: Propose a new problem type
title: "[Model] MinMaxMulticenter"
labels: model
assignees: ''
---

## Motivation

MIN-MAX MULTICENTER (P126) from Garey & Johnson, A2 ND50. A classical NP-complete facility location problem, also known as the p-center problem. Given a graph with vertex weights and edge lengths, the goal is to place K service centers (points on the graph) so as to minimize the maximum weighted distance from any vertex to its nearest center. Arises in emergency facility location, network design, and service coverage optimization. Closely related to the dominating set problem: on unweighted unit-length graphs, a set of K vertices is a dominating set if and only if it is a K-center solution with radius 1.

**Associated reduction rules:**
- As target: R70 (DOMINATING SET -> MIN-MAX MULTICENTER)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinMaxMulticenter`
**Canonical name:** <!-- ⚠️ Unverified: web search --> p-Center Problem; also: Min-Max Multicenter, Vertex k-Center Problem
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND50

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" (where a point on G can be either a vertex in V or a point on an edge e ∈ E, with e regarded as a line segment of length l(e)) such that if d(v) is the length of the shortest path from v to the closest point in P, then max{d(v)·w(v): v ∈ V} ≤ B?

The optimization version asks: find K points on G minimizing the maximum weighted distance max{d(v)·w(v): v ∈ V}.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| binary variables (one per vertex), representing candidate center locations. In the full "absolute" version, centers can also lie on edge interiors, but the vertex-restricted variant (which is also NP-complete) uses vertex-only placement.
- **Per-variable domain:** binary {0, 1} — whether vertex v is selected as a center
- **Meaning:** variable x_v = 1 if vertex v is chosen as a center location. Exactly K variables must be set to 1. The configuration is valid if max{d(v)·w(v): v ∈ V} ≤ B, where d(v) is the shortest weighted-path distance from v to the nearest selected center.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MinMaxMulticenter`
**Variants:** graph topology, weight type

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The underlying graph G = (V, E) |
| `vertex_weights` | `Vec<W>` | Non-negative weight w(v) for each vertex v |
| `edge_lengths` | `Vec<W>` | Non-negative length l(e) for each edge e |
| `k` | `usize` | Number of centers to place (K) |

**Notes:**
- This is a satisfaction (decision) problem in the GJ formulation: `Metric = bool`, implementing `SatisfactionProblem`.
- Alternatively, the optimization version minimizes the bottleneck radius B for a given K — then `Metric = SolutionSize<W>`, implementing `OptimizationProblem` with `Direction::Minimize`.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_centers()` (= K).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Kariv and Hakimi, 1979; transformation from DOMINATING SET). Remains NP-complete even with unit weights and unit edge lengths.
- **Best known exact algorithm:** The vertex p-center problem on general graphs inherits hardness from dominating set. For the unweighted vertex-restricted variant, solving p-center reduces to O(n^2) calls to a dominating set decision oracle (binary search on distance thresholds). Using the best dominating set algorithm: O*(1.4969^n) per threshold check (van Rooij and Bodlaender, 2011), giving overall O*(1.4969^n) with polynomial overhead.
- **Polynomial cases:** Solvable in polynomial time for fixed K (O(n^{2K+1}) by Drezner, 1984), and for arbitrary K on trees (Kariv and Hakimi, 1979).
- **Approximation:** 2-approximation is optimal unless P=NP; no (2-epsilon)-approximation exists for any epsilon > 0 (Hsu and Nemhauser, 1979).
- **References:**
  - O. Kariv, S. L. Hakimi (1979). "An Algorithmic Approach to Network Location Problems. I: The p-Centers." *SIAM J. Appl. Math.*, 37(3):513-538.
  - W. L. Hsu, G. L. Nemhauser (1979). "Easy and hard bottleneck location problems." *Discrete Appl. Math.*, 1(3):209-215.
  - J. M. W. van Rooij, H. L. Bodlaender (2011). "Exact algorithms for dominating set." *Discrete Appl. Math.*, 159(17):2147-2164.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** General facility location / bottleneck covering problems
- **Known special cases:**
  - Vertex k-center: centers restricted to vertices (also NP-complete)
  - Unweighted unit-length variant: w(v)=1, l(e)=1 (NP-complete, equivalent to minimum dominating set)
  - Tree graphs: polynomial-time solvable (Kariv and Hakimi, 1979)
  - Fixed K: polynomial-time solvable

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" (where a point on G can be either a vertex in V or a point on an edge e ∈ E, with e regarded as a line segment of length l(e)) such that if d(v) is the length of the shortest path from v to the closest point in P, then max{d(v)·w(v): v ∈ V} ≤ B?
Reference: [Kariv and Hakimi, 1976a]. Transformation from DOMINATING SET.
Comment: Also known as the "p-center" problem. Remains NP-complete if w(v) = 1 for all v ∈ V and l(e) = 1 for all e ∈ E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree [Kariv and Hakimi, 1976a]. Variant in which we must choose a subset P ⊆ V is also NP-complete but solvable for fixed K and for trees [Slater, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all C(n, K) subsets of K vertices; for each subset, compute all-pairs shortest paths and evaluate max{d(v)·w(v)}. Check feasibility against B.
- [x] It can be solved by reducing to integer programming. Binary variable x_v for each vertex; minimize B subject to: for each vertex v, sum constraints ensuring at least one center is within distance B/w(v); exactly K centers selected (sum x_v = K).
- [x] Other: Reduce to iterated dominating set checks at different distance thresholds (Minieka's approach); branch-and-cut algorithms.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges, unit weights and lengths:**
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {0,5}, {1,4}
- K = 2, B = 1

**Shortest path distances (unit edge lengths):**
All edge lengths l(e) = 1, all vertex weights w(v) = 1.

**Optimal 2-center:** Place centers at vertices {1, 4}.
- d(0) = dist(0, {1,4}) = dist(0,1) = 1. w(0)·d(0) = 1 ≤ 1 ✓
- d(1) = 0 (center). ✓
- d(2) = dist(2, {1,4}) = min(dist(2,1), dist(2,4)) = min(1, 2) = 1. ✓
- d(3) = dist(3, {1,4}) = min(dist(3,1), dist(3,4)) = min(2, 1) = 1. ✓
- d(4) = 0 (center). ✓
- d(5) = dist(5, {1,4}) = min(dist(5,1), dist(5,4)) = min(2, 1) = 1. ✓
- max{d(v)·w(v)} = 1 ≤ B = 1 ✓

**Note:** This is equivalent to asking whether {1, 4} is a dominating set of G (since unit weights and lengths). Indeed, N[1] = {0,1,2,4} and N[4] = {1,3,4,5}, so N[1] ∪ N[4] = V. ✓
