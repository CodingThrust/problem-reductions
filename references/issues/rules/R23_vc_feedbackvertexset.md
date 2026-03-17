---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to FEEDBACK VERTEX SET"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'FEEDBACK VERTEX SET'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** FEEDBACK VERTEX SET
**Motivation:** Establishes NP-completeness of FEEDBACK VERTEX SET via polynomial-time reduction from VERTEX COVER, showing that every undirected edge can be converted into a directed 2-cycle so that a vertex cover in the original graph corresponds exactly to a feedback vertex set in the constructed digraph.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT7

## GJ Source Entry

> [GT7]  FEEDBACK VERTEX SET
> INSTANCE:  Directed graph G = (V,A), positive integer K ≤ |V|.
> QUESTION:  Is there a subset V' ⊆ V with |V'| ≤ K such that V' contains at least one vertex from every directed cycle in G?
>
> Reference:  [Karp, 1972]. Transformation from VERTEX COVER.
> Comment:  Remains NP-complete for digraphs having no in- or out-degree exceeding 2, for planar digraphs with no in- or out-degree exceeding 3 [Garey and Johnson, ——], and for edge digraphs [Gavril, 1977a], but can be solved in polynomial time for reducible graphs [Shamir, 1977]. The corresponding problem for undirected graphs is also NP-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G=(V,E), k) where G is an undirected graph, construct a MinimumFeedbackVertexSet instance (G'=(V',A'), k') as follows:

1. **Vertex set:** V' = V (one vertex in G' for each vertex in G; no new vertices are added).
2. **Arc set:** For each undirected edge {u,v} ∈ E, add both directed arcs (u→v) and (v→u) to A'. Each undirected edge becomes a directed 2-cycle of length 2: u→v→u.
3. **Budget parameter:** Set k' = k.
4. **Key invariant:** Every directed cycle in G' is one of these 2-cycles (the graph has no other cycles because every arc came from a symmetric pair). A feedback vertex set V' ⊆ V must include at least one of {u, v} for every 2-cycle u→v→u — which is precisely the vertex cover condition on G.
5. **Solution extraction:** Any FVS of size ≤ k in G' is a vertex cover of size ≤ k in G, and vice versa.

**Correctness:**
- (⇒) If S is a vertex cover for G of size ≤ k, then for every directed 2-cycle u→v→u in G', at least one of u, v is in S (since {u,v} ∈ E). Hence S is a FVS for G' of size ≤ k.
- (⇐) If S is a FVS for G' of size ≤ k, then for every arc (u,v) ∈ A', the 2-cycle u→v→u is broken, so at least one of u, v is in S. Since every edge {u,v} ∈ E generated arcs in both directions, S covers every edge of G.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_arcs` | `2 * num_edges` |

**Derivation:**
- Vertices: same vertex set, no additions → |V'| = n
- Arcs: each undirected edge {u,v} yields two directed arcs (u→v) and (v→u) → |A'| = 2m

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to MinimumFeedbackVertexSet, solve target with BruteForce, extract solution (same vertex set), verify it is a valid vertex cover on the original undirected graph
- Check that the minimum FVS size in G' equals the minimum VC size in G
- Test with a path graph P_n (minimum VC = n−1): the constructed digraph has 2(n−1) arcs forming n−1 independent 2-cycles; minimum FVS = n−1 ✓
- Test with a complete graph K_n (minimum VC = n−1): verify minimum FVS = n−1 in the all-pairs bidirected digraph

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- (Two triangles sharing vertex 1–3 path, with extra leaf vertex 5)
- Minimum vertex cover: k = 3, for example {1, 2, 3}:
  - {0,1} covered by 1 ✓, {0,2} covered by 2 ✓, {1,2} covered by 1,2 ✓
  - {1,3} covered by 1,3 ✓, {2,4} covered by 2 ✓, {3,4} covered by 3 ✓, {3,5} covered by 3 ✓

**Constructed target instance (MinimumFeedbackVertexSet):**
Directed graph G' with 6 vertices {0, 1, 2, 3, 4, 5} and 14 arcs:
- From edge {0,1}: arcs (0→1) and (1→0)
- From edge {0,2}: arcs (0→2) and (2→0)
- From edge {1,2}: arcs (1→2) and (2→1)
- From edge {1,3}: arcs (1→3) and (3→1)
- From edge {2,4}: arcs (2→4) and (4→2)
- From edge {3,4}: arcs (3→4) and (4→3)
- From edge {3,5}: arcs (3→5) and (5→3)

Directed cycles in G': seven 2-cycles: {0↔1}, {0↔2}, {1↔2}, {1↔3}, {2↔4}, {3↔4}, {3↔5}

Target budget: K' = k = 3

**Solution mapping:**
- Minimum FVS in G': {1, 2, 3}
  - Breaks 2-cycle {0↔1}: vertex 1 ∈ FVS ✓
  - Breaks 2-cycle {0↔2}: vertex 2 ∈ FVS ✓
  - Breaks 2-cycle {1↔2}: both 1,2 ∈ FVS ✓
  - Breaks 2-cycle {1↔3}: both 1,3 ∈ FVS ✓
  - Breaks 2-cycle {2↔4}: vertex 2 ∈ FVS ✓
  - Breaks 2-cycle {3↔4}: vertex 3 ∈ FVS ✓
  - Breaks 2-cycle {3↔5}: vertex 3 ∈ FVS ✓
- Extracted vertex cover in G: {1, 2, 3} — same set
- Verification: all 7 edges of G are covered ✓
- FVS size = 3 = k ✓

**Greedy trap:** Vertex 4 appears in two 2-cycles ({2↔4} and {3↔4}), but selecting vertex 4 wastes budget compared to selecting vertex 2 or 3, which each cover more cycles. This demonstrates that a greedy "highest degree" strategy for FVS can fail.


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.
- **[Shamir, 1977]**: [`Shamir1977`] Adi Shamir (1977). "Finding minimum cutsets in reducible graphs". Laboratory for Computer Science, Massachusetts Institute of Technology.
