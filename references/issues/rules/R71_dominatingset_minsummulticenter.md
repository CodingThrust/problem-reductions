---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DOMINATING SET to MIN-SUM MULTICENTER"
labels: rule
assignees: ''
canonical_source_name: 'Minimum Dominating Set'
canonical_target_name: 'Min-Sum Multicenter (p-Median)'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** DOMINATING SET
**Target:** MIN-SUM MULTICENTER
**Motivation:** Establishes NP-completeness of MIN-SUM MULTICENTER (the p-median problem) via polynomial-time reduction from DOMINATING SET. The reduction shows that finding optimal median facility locations that minimize total service cost is computationally intractable, even on unweighted unit-length graphs. This is a foundational result in operations research and facility location theory.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND51, p.220

## GJ Source Entry

> [ND51] MIN-SUM MULTICENTER
> INSTANCE: Graph G=(V,E), weight w(v)∈Z_0^+ for each v∈V, length l(e)∈Z_0^+ for each e∈E, positive integer K≤|V|, positive rational number B.
> QUESTION: Is there a set P of K "points on G" such that if d(v) is the length of the shortest path from v to the closest point in P, then Σ_{v∈V} d(v)·w(v)≤B?
> Reference: [Kariv and Hakimi, 1976b]. Transformation from DOMINATING SET.
> Comment: Also known as the "p-median" problem. It can be shown that there is no loss of generality in restricting P to being a subset of V. Remains NP-complete if w(v)=1 for all v∈V and l(e)=1 for all e∈E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumDominatingSet instance (G = (V, E), K) where K is the dominating set size bound, construct a MinSumMulticenter instance as follows:

1. **Graph modification:** Construct a new graph G' from G by adding a large number of pendant (degree-1) vertices to each original vertex. Specifically, for each vertex v ∈ V, add M new pendant vertices connected only to v, where M is a sufficiently large value (e.g., M = n^2 where n = |V|).
2. **Set unit weights:** Assign w(v) = 1 for every vertex in G' (both original and pendant vertices).
3. **Set unit edge lengths:** Assign l(e) = 1 for every edge in G'.
4. **Set center count:** Use the same K as the number of centers to place.
5. **Set distance bound:** Set B appropriately based on M and K to ensure equivalence.

**Alternative (simpler) reduction for unit weights/lengths:**
Since GJ states the problem remains NP-complete with unit weights and unit lengths, a simpler reduction works:

1. **Preserve graph:** Use the same graph G = (V, E).
2. **Set unit weights and lengths:** w(v) = 1 for all v, l(e) = 1 for all e.
3. **Set center count:** K centers.
4. **Set distance bound:** B = |V| - K. If a dominating set of size K exists, every non-center vertex is at distance at most 1 from a center, so the total distance is at most |V| - K.

**Correctness argument (for the simpler variant):**
- (Forward) If D is a dominating set with |D| = K, placing centers at D gives: for each v ∈ D, d(v) = 0; for each v ∉ D, d(v) ≤ 1 (since v has a neighbor in D). Total = Σ d(v) ≤ 0·K + 1·(n-K) = n - K = B.
- (Backward) If centers P achieve Σ d(v) ≤ n - K with K centers, then the n - K non-center vertices each contribute d(v) ≥ 1 to the sum (each must reach some center). For the sum to be at most n - K, each non-center vertex must have d(v) = 1, meaning every non-center vertex is adjacent to some center. Thus P is a dominating set.

**Key insight:** With unit weights and unit lengths, a K-center placement achieves total distance exactly n - K if and only if every non-center vertex is adjacent to a center, which is precisely the dominating set condition.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

For the simpler unit-weight, unit-length reduction:

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `num_centers` | `K` (same as dominating set size bound) |

**Derivation:** The simple reduction preserves the graph exactly. The graph structure is unchanged; only the problem formulation adds unit weights, unit lengths, and sets B = n - K. For the pendant-vertex reduction variant, num_vertices = n + n·M = n(1+M) and num_edges = m + n·M.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce source MinimumDominatingSet instance to MinSumMulticenter (unit weights, unit lengths, B = n - K), solve target with BruteForce, extract solution (the set of center vertices), verify it is a valid dominating set on the original graph
- Verify that for the extracted solution, each non-center vertex is at distance exactly 1 from a center (confirming dominating set property)
- Compare with known results from literature: on a star graph K_{1,n-1}, the single center vertex is a dominating set of size 1, and should yield total distance n - 1

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumDominatingSet):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {3,4}, {3,5}, {4,5}
- K = 2

Dominating set D = {0, 3}:
- N[0] = {0, 1, 2}
- N[3] = {1, 2, 3, 4, 5}
- N[0] ∪ N[3] = V ✓

**Constructed target instance (MinSumMulticenter):**
- Same graph G with 6 vertices and 7 edges
- w(v) = 1 for all v, l(e) = 1 for all e
- K = 2 centers, B = 6 - 2 = 4

**Solution mapping:**
- Place centers at P = {0, 3}
- d(0) = 0 (center)
- d(1) = min(dist(1,0), dist(1,3)) = min(1, 1) = 1
- d(2) = min(dist(2,0), dist(2,3)) = min(1, 1) = 1
- d(3) = 0 (center)
- d(4) = min(dist(4,0), dist(4,3)) = min(3, 1) = 1
- d(5) = min(dist(5,0), dist(5,3)) = min(3, 1) = 1
- Σ d(v)·w(v) = 0 + 1 + 1 + 0 + 1 + 1 = 4 ≤ B = 4 ✓

**Extraction:** The set of center vertices {0, 3} is returned as the dominating set solution. Verified: N[0] ∪ N[3] = V ✓.

**Checking that K = 1 is infeasible:**
For any single center v, at least one vertex is at distance ≥ 2 (e.g., center at 3: d(0) = 2, total ≥ 2 + 0 + ... > n - 1 = 5 is not necessarily enough, but more importantly no single vertex dominates all of V). Center at 0: N[0] = {0,1,2}, misses {3,4,5}. Total distance = 0 + 1 + 1 + 2 + 3 + 3 = 10 > B = 5. Not feasible for K=1.


## References

- **[Kariv and Hakimi, 1976b]**: [`Kariv1976b`] Oded Kariv and S. Louis Hakimi (1976). "An algorithmic approach to network location problems -- {Part 2}: the p-medians".
