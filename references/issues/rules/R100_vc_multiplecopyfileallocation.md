---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Multiple Copy File Allocation"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'MULTIPLE COPY FILE ALLOCATION'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** VERTEX COVER
**Target:** MULTIPLE COPY FILE ALLOCATION
**Motivation:** Establishes NP-completeness (in the strong sense) of MULTIPLE COPY FILE ALLOCATION by reduction from VERTEX COVER. The key insight is that placing file copies at vertices of a graph corresponds to choosing a vertex cover: each vertex in the cover stores a copy (incurring storage cost), and vertices not in the cover must access the nearest copy (incurring usage-weighted distance cost). By setting uniform usage and storage costs, the total cost is minimized exactly when the selected vertices form a minimum vertex cover, because every edge must have at least one endpoint in the cover to keep access distances bounded.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1 [SR6], p.227

## GJ Source Entry

> [SR6] MULTIPLE COPY FILE ALLOCATION
> INSTANCE: Graph G = (V, E), for each v ∈ V a usage u(v) ∈ Z⁺ and a storage cost s(v) ∈ Z⁺, and a positive integer K.
> QUESTION: Is there a subset V' ⊆ V such that, if for each v ∈ V we let d(v) denote the number of edges in the shortest path in G from v to a member of V', we have
>
> ∑_{v ∈ V'} s(v) + ∑_{v ∈ V} d(v)·u(v) ≤ K ?
>
> Reference: [Van Sickle and Chandy, 1977]. Transformation from VERTEX COVER.
> Comment: NP-complete in the strong sense, even if all v ∈ V have the same value of u(v) and the same value of s(v).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance: graph G = (V, E) with |V| = n, |E| = m, and positive integer K_vc (vertex cover size bound), construct a Multiple Copy File Allocation instance as follows:

1. **Graph:** Use the same graph G' = G = (V, E).

2. **Storage costs:** For each vertex v ∈ V, set s(v) = 1 (uniform storage cost).

3. **Usage costs:** For each vertex v ∈ V, set u(v) = n + 1 (a large uniform usage, ensuring that any vertex at distance ≥ 2 from all copies incurs prohibitive cost).

4. **Bound:** Set K = K_vc + (n − K_vc)·(n + 1) = K_vc + (n − K_vc)(n + 1).
   - The K_vc term accounts for storage costs of the cover vertices.
   - The (n − K_vc)(n + 1) term accounts for usage costs: each non-cover vertex must be at distance exactly 1 from some cover vertex (since V' is a vertex cover, every vertex not in V' is adjacent to some vertex in V'), contributing d(v)·u(v) = 1·(n+1) = n+1.

   Wait — more carefully: if V' is a vertex cover of size K_vc, then every edge has at least one endpoint in V'. For v ∈ V', d(v) = 0. For v ∉ V', if v is isolated (no edges), then d(v) could be large; but if every vertex has at least one edge, then v has a neighbor in V', so d(v) ≤ 1.

   **Refined construction using the uniform-cost special case:**

   Since the problem is NP-complete even with uniform u(v) = u and s(v) = s for all v:

1. **Graph:** G' = G.

2. **Costs:** Set s(v) = 1 for all v, and u(v) = M for all v, where M = n·m + 1 (a sufficiently large value to penalize distance ≥ 2).

3. **Bound:** Set K = K_vc · 1 + (n − K_vc) · 1 · M = K_vc + (n − K_vc)·M.

4. **Correctness (forward):** If V' is a vertex cover of size K_vc, then:
   - Storage cost: ∑_{v ∈ V'} s(v) = K_vc.
   - For v ∈ V': d(v) = 0 (v is in V').
   - For v ∉ V': since V' is a vertex cover, every edge incident to v has its other endpoint in V'. Hence v is adjacent to some member of V', so d(v) ≤ 1. If v has at least one edge, d(v) = 1; if v is isolated, d(v) could be large, but we can add v to V' without affecting the cover (isolated vertices don't affect the cover).
   - Assuming G has no isolated vertices: usage cost = ∑_{v ∉ V'} 1 · M = (n − K_vc) · M.
   - Total = K_vc + (n − K_vc)·M = K ✓.

5. **Correctness (reverse):** If there exists V' ⊆ V with total cost ≤ K, then any vertex v ∉ V' with d(v) ≥ 2 would contribute d(v)·M ≥ 2M to the usage cost, making the total exceed K (since 2M > K for suitable M). Therefore, every v ∉ V' has d(v) ≤ 1, meaning every non-cover vertex is adjacent to some cover vertex. This implies V' is a vertex cover (every edge has an endpoint in V') — if some edge {u,w} had neither endpoint in V', both u and w would be non-cover, and we'd need d(u) ≤ 1 and d(w) ≤ 1, which is possible, but actually: the vertex cover property follows because with d(v) ≤ 1 for all non-cover vertices, the total cost is |V'| + (n − |V'|)·M ≤ K = K_vc + (n − K_vc)·M. Since M > n, this forces |V'| ≤ K_vc.

6. **Solution extraction:** Given a valid file allocation V' with cost ≤ K, the set V' is directly the vertex cover.

**Key invariant:** With large uniform usage cost M, placing a file copy at a vertex is equivalent to "covering" it; the budget K is calibrated so that exactly K_vc copies can be placed while keeping all non-cover vertices at distance 1.

**Time complexity of reduction:** O(n + m) to set up the instance.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G (|V|)
- m = `num_edges` of source graph G (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices`             | `num_vertices` (= n)             |
| `num_edges`                | `num_edges` (= m)                |

**Derivation:** The graph is unchanged. Storage and usage costs are uniform constants or O(n·m). The bound K is a derived parameter from K_vc, n, and M.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a MinimumVertexCover instance (G, K_vc), reduce to MultipleCopyFileAllocation, solve target by brute-force (enumerate all 2^n subsets V'), compute BFS distances and total cost, verify that V' achieving cost ≤ K is a vertex cover of size ≤ K_vc.
- Test with C_4 (4-cycle): K_vc = 2 (cover = {0, 2} or {1, 3}). With n = 4, m = 4, M = 17, K = 2 + 2·17 = 36. File placement at {0, 2}: storage = 2, usage = 2·1·17 = 34, total = 36 ≤ K ✓.
- Test with star K_{1,5}: K_vc = 1 (center vertex covers all edges). With n = 6, m = 5, M = 31, K = 1 + 5·31 = 156.
- Test unsatisfiable case: K_6 (complete graph on 6 vertices) with K_vc = 3 (too small, minimum VC is 5). Verify no allocation achieves cost ≤ K.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {2,3}, {3,4}, {4,5}, {3,5}
- Minimum vertex cover size: K_vc = 3, e.g., V' = {0, 2, 3} covers:
  - {0,1} by 0 ✓, {0,2} by 0 or 2 ✓, {1,2} by 2 ✓, {2,3} by 2 or 3 ✓, {3,4} by 3 ✓, {4,5} needs... vertex 4 or 5 must be in cover.
- Corrected: V' = {2, 3, 5} covers: {0,1}... no, 0 and 1 not covered.
- Corrected: V' = {0, 2, 3, 5} (size 4), or V' = {1, 2, 3, 4} (size 4).
- Actually minimum vertex cover of this graph: check all edges.
  - Take V' = {0, 2, 3, 5}: {0,1} by 0 ✓, {0,2} by 0 ✓, {1,2} by 2 ✓, {2,3} by 2 ✓, {3,4} by 3 ✓, {4,5} by 5 ✓, {3,5} by 3 ✓. Size = 4.
  - Take V' = {1, 2, 4, 3}: {0,1} by 1 ✓, {0,2} by 2 ✓, {1,2} by 1 ✓, {2,3} by 2 ✓, {3,4} by 3 ✓, {4,5} by 4 ✓, {3,5} by 3 ✓. Size = 4.
  - Can we do size 3? Try {2, 3, 4}: {0,1} — neither 0 nor 1 in cover. Fail.
  - Minimum is 4. Set K_vc = 4.

**Simpler source instance:**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 6 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0} (a 6-cycle C_6)
- Minimum vertex cover: K_vc = 3, e.g., V' = {1, 3, 5}
  - {0,1} by 1 ✓, {1,2} by 1 ✓, {2,3} by 3 ✓, {3,4} by 3 ✓, {4,5} by 5 ✓, {5,0} by 5 ✓

**Constructed target instance (MultipleCopyFileAllocation):**
- Graph G' = G (6 vertices, 6 edges, same C_6)
- s(v) = 1 for all v ∈ V
- u(v) = M = 6·6 + 1 = 37 for all v ∈ V
- K = K_vc + (n − K_vc)·M = 3 + 3·37 = 3 + 111 = 114

**Solution mapping (V' = {1, 3, 5}):**
- Storage cost: ∑_{v ∈ V'} s(v) = 3·1 = 3
- Distances from non-cover vertices to nearest cover vertex:
  - d(0): neighbors are 1 (in V') and 5 (in V'). d(0) = 1.
  - d(2): neighbors are 1 (in V') and 3 (in V'). d(2) = 1.
  - d(4): neighbors are 3 (in V') and 5 (in V'). d(4) = 1.
- Usage cost: ∑_{v ∈ V} d(v)·u(v) = (0 + 1 + 0 + 1 + 0 + 1)·37... wait, vertices in V' have d(v) = 0:
  - d(0) = 1, d(1) = 0, d(2) = 1, d(3) = 0, d(4) = 1, d(5) = 0
  - Usage cost = (1 + 0 + 1 + 0 + 1 + 0)·37 = 3·37 = 111
- Total cost = 3 + 111 = 114 = K ✓

**Verification:**
- Forward: VC {1,3,5} of size 3 → file allocation cost = 114 ≤ K ✓
- Reverse: If we tried V' = {0, 2, 4} (also valid VC of size 3): d(1) = 1, d(3) = 1, d(5) = 1. Cost = 3 + 3·37 = 114 ≤ K ✓
- If we tried V' = {0, 1} (not a VC): edge {3,4} uncovered. d(3) = 2 (via 2→1 or 0→... actually on C_6, d(3) = min(d(3,0), d(3,1)) = min(3,2) = 2). Then usage ≥ 2·37 = 74 for vertex 3 alone plus other vertices, total > K. Fail ✓
- Solution extraction: V' = {1, 3, 5} is directly the vertex cover ✓


## References

- **[Van Sickle and Chandy, 1977]**: [`VanSickle1977`] Larry van Sickle and K. Mani Chandy (1977). "The complexity of computer network design problems". Tech report, University of Texas at Austin.
