---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DOMINATING SET to MIN-MAX MULTICENTER"
labels: rule
assignees: ''
canonical_source_name: 'Minimum Dominating Set'
canonical_target_name: 'Min-Max Multicenter (p-Center)'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** DOMINATING SET
**Target:** MIN-MAX MULTICENTER
**Motivation:** Establishes NP-completeness of MIN-MAX MULTICENTER (the p-center problem) via polynomial-time reduction from DOMINATING SET. The reduction exploits the fundamental equivalence that on unweighted unit-length graphs, a k-center solution with radius 1 is exactly a dominating set of size k. This is a key result in facility location theory, showing that optimal worst-case service placement is computationally intractable.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND50, p.220

## GJ Source Entry

> [ND50] MIN-MAX MULTICENTER
> INSTANCE: Graph G=(V,E), weight w(v)∈Z_0^+ for each v∈V, length l(e)∈Z_0^+ for each e∈E, positive integer K≤|V|, positive rational number B.
> QUESTION: Is there a set P of K "points on G" (where a point on G can be either a vertex in V or a point on an edge e∈E, with e regarded as a line segment of length l(e)) such that if d(v) is the length of the shortest path from v to the closest point in P, then max{d(v)·w(v): v∈V}≤B?
> Reference: [Kariv and Hakimi, 1976a]. Transformation from DOMINATING SET.
> Comment: Also known as the "p-center" problem. Remains NP-complete if w(v)=1 for all v∈V and l(e)=1 for all e∈E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree [Kariv and Hakimi, 1976a]. Variant in which we must choose a subset P⊆V is also NP-complete but solvable for fixed K and for trees [Slater, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumDominatingSet instance (G = (V, E), K) where K is the dominating set size bound, construct a MinMaxMulticenter instance as follows:

1. **Graph preservation:** Use the same graph G = (V, E) as the target graph.
2. **Set unit weights:** Assign w(v) = 1 for every vertex v ∈ V.
3. **Set unit edge lengths:** Assign l(e) = 1 for every edge e ∈ E.
4. **Set center count:** Use the same K as the number of centers to place.
5. **Set distance bound:** Set B = 1.
6. **Restrict to vertex centers:** Centers are placed only at vertices (the vertex p-center variant, which is also NP-complete per GJ).

**Correctness argument:**
- (Forward) If D ⊆ V is a dominating set with |D| ≤ K, then placing centers at all vertices in D gives a valid p-center solution: for every v ∈ V, either v ∈ D (so d(v) = 0 ≤ 1) or v has a neighbor in D (so d(v) = 1 ≤ 1). Thus max{d(v)·w(v)} ≤ 1 = B.
- (Backward) If P is a set of K vertex-centers achieving max{d(v)·1} ≤ 1, then every vertex v is either a center (d(v) = 0) or at distance 1 from a center (adjacent to some center). Therefore P is a dominating set.

**Key insight:** On unit-weight, unit-length graphs, a vertex set is a dominating set of size K if and only if it is a K-center with bottleneck radius ≤ 1.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `num_centers` | `K` (same as dominating set size bound) |

**Derivation:** The reduction preserves the graph exactly -- same vertices, same edges. Only the problem formulation changes by adding unit weights, unit lengths, and the distance bound B = 1. The overhead is O(1) per element (just annotating with weights/lengths).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce source MinimumDominatingSet instance to MinMaxMulticenter (unit weights, unit lengths, B=1), solve target with BruteForce, extract solution (the set of center vertices), verify it is a valid dominating set on the original graph
- Compare with known results from literature: on a path graph P_5, minimum dominating set has size 2 (e.g., vertices {1, 3}); the corresponding 2-center with B=1 should select the same vertices
- Verify equivalence: check that the answer to the dominating set decision problem equals the answer to the p-center decision problem for every test instance

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumDominatingSet):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 8 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,6}, {0,6}, {1,4}
- Minimum dominating set size = 2: D = {1, 5}
  - N[1] = {0, 1, 2, 4} (closed neighborhood)
  - N[5] = {4, 5, 6}
  - N[1] ∪ N[5] = {0, 1, 2, 4, 5, 6} -- missing vertex 3!
  - Try D = {1, 4}: N[1] = {0,1,2,4}, N[4] = {1,3,4,5}. Union = {0,1,2,3,4,5} -- missing 6.
  - Try D = {2, 5}: N[2] = {1,2,3}, N[5] = {4,5,6}. Union = {1,2,3,4,5,6} -- missing 0.
  - Try D = {0, 3, 5}: N[0]={0,1,6}, N[3]={2,3,4}, N[5]={4,5,6}. Union = V ✓. Size = 3.
  - Try D = {1, 5}: N[1]={0,1,2,4}, N[5]={4,5,6}. Union = {0,1,2,4,5,6} -- missing 3.
  - Try D = {1, 6}: N[1]={0,1,2,4}, N[6]={0,5,6}. Union = {0,1,2,4,5,6} -- missing 3.
  - Try D = {2, 6}: N[2]={1,2,3}, N[6]={0,5,6}. Union = {0,1,2,3,5,6} -- missing 4.
  - Try D = {2, 5}: Union missing 0. Try D = {1, 3, 5}: N[1]={0,1,2,4}, N[3]={2,3,4}, N[5]={4,5,6}. Union = V ✓. Size = 3.

Corrected: use a simpler graph.

**Source instance (MinimumDominatingSet):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {3,4}, {3,5}, {4,5}
- K = 2

Check D = {0, 3}:
- N[0] = {0, 1, 2}
- N[3] = {1, 2, 3, 4, 5}
- N[0] ∪ N[3] = {0, 1, 2, 3, 4, 5} = V ✓
- Minimum dominating set of size 2 exists.

**Constructed target instance (MinMaxMulticenter):**
- Same graph G with 6 vertices and 7 edges
- w(v) = 1 for all v, l(e) = 1 for all e
- K = 2 centers, B = 1

**Solution mapping:**
- Place centers at P = {0, 3}
- d(0) = 0 (center) ≤ 1 ✓
- d(1) = min(dist(1,0), dist(1,3)) = min(1, 1) = 1 ≤ 1 ✓
- d(2) = min(dist(2,0), dist(2,3)) = min(1, 1) = 1 ≤ 1 ✓
- d(3) = 0 (center) ≤ 1 ✓
- d(4) = min(dist(4,0), dist(4,3)) = min(3, 1) = 1 ≤ 1 ✓
- d(5) = min(dist(5,0), dist(5,3)) = min(3, 1) = 1 ≤ 1 ✓
- max{d(v)·w(v)} = 1 ≤ B = 1 ✓

**Extraction:** The set of center vertices {0, 3} is returned as the dominating set solution. Verified: N[0] ∪ N[3] = V ✓.


## References

- **[Kariv and Hakimi, 1976a]**: [`Kariv1976a`] Oded Kariv and S. Louis Hakimi (1976). "An algorithmic approach to network location problems -- {Part I}: the p-centers".
- **[Slater, 1976]**: [`Slater1976`] Peter J. Slater (1976). "{$R$}-domination in graphs". *Journal of the Association for Computing Machinery* 23, pp. 446–450.
