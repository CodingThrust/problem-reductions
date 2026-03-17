---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to FEEDBACK ARC SET"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'FEEDBACK ARC SET'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** FEEDBACK ARC SET
**Motivation:** Establishes NP-completeness of FEEDBACK ARC SET via polynomial-time reduction from VERTEX COVER by splitting each vertex into an in-node and out-node connected by an internal arc, so that selecting a vertex cover corresponds to selecting the internal arcs that break every directed cycle.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT8

## GJ Source Entry

> [GT8]  FEEDBACK ARC SET
> INSTANCE:  Directed graph G = (V,A), positive integer K ≤ |A|.
> QUESTION:  Is there a subset A' ⊆ A with |A'| ≤ K such that A' contains at least one arc from every directed cycle in G?
>
> Reference:  [Karp, 1972]. Transformation from VERTEX COVER.
> Comment:  Remains NP-complete for digraphs in which no vertex has total indegree and out-degree more than 3, and for edge digraphs [Gavril, 1977a]. Solvable in polynomial time for planar digraphs [Luchesi, 1976]. The corresponding problem for undirected graphs is trivially solvable in polynomial time.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G=(V,E), k) where G is an undirected graph with n=|V| and m=|E|, construct a MinimumFeedbackArcSet instance (G'=(V',A'), k') as follows:

1. **Vertex splitting:** For each vertex v ∈ V, create two nodes: v^in and v^out. So V' = {v^in, v^out : v ∈ V}, giving |V'| = 2n.
2. **Internal arcs:** For each vertex v ∈ V, add arc (v^in → v^out) to A'. These n arcs represent the vertices of G. Including this arc in the FAS corresponds to "selecting" vertex v in the vertex cover.
3. **Crossing arcs:** For each undirected edge {u,v} ∈ E, add two arcs: (u^out → v^in) and (v^out → u^in). These 2m arcs encode the adjacency of G.
4. **Budget parameter:** Set k' = k.
5. **Key invariant:** Every directed cycle in G' must traverse at least one internal arc. A directed cycle necessarily follows the pattern: ... → u^in → u^out → v^in → v^out → u^in → ... (passing through two internal arcs and two crossing arcs). Removing internal arcs (v^in → v^out) for some set S ⊆ V breaks all cycles if and only if S is a vertex cover of G (since every cycle uses crossing arcs corresponding to an edge {u,v}, which is broken by taking u or v into S).
6. **Solution extraction:** Given a FAS A' of size ≤ k containing only internal arcs, extract S = {v ∈ V : (v^in → v^out) ∈ A'}. This S is a vertex cover of G.

**Correctness:**
- (⇒) If S is a vertex cover for G of size ≤ k, then A' = {(v^in → v^out) : v ∈ S} is a FAS of size ≤ k. For any directed cycle C in G', C must use some crossing arc (u^out → v^in) for edge {u,v} ∈ E, and the cycle continues through (v^in → v^out) or (u^in → u^out). Since S covers {u,v}, at least one of these internal arcs is in A', so C is broken.
- (⇐) If A' is a FAS of size ≤ k, we can assume WLOG that A' contains only internal arcs (crossing arcs that are in A' can be replaced by the corresponding internal arc without increasing the size). Then S = {v ∈ V : (v^in → v^out) ∈ A'} is a vertex cover: for each edge {u,v} ∈ E, the cycle u^in → u^out → v^in → v^out → u^in must be broken, so at least one of (u^in → u^out) or (v^in → v^out) is in A', meaning at least one of u, v is in S.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `2 * num_vertices` |
| `num_arcs` | `num_vertices + 2 * num_edges` |

**Derivation:**
- Vertices: each original vertex v split into v^in and v^out → |V'| = 2n
- Arcs: n internal arcs (one per vertex) + 2m crossing arcs (two per edge) → |A'| = n + 2m

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to MinimumFeedbackArcSet, solve target with BruteForce, extract vertex cover (vertices whose internal arcs are in the FAS), verify it is a valid vertex cover on the original graph
- Check that the minimum FAS size in G' equals the minimum VC size in G
- Verify that the constructed digraph has exactly n + 2m arcs and 2n vertices
- Test on a star graph K_{1,r}: minimum VC = 1 (center vertex); constructed digraph should have minimum FAS = 1 (only the center's internal arc needs to be removed)
- Verify that no crossing arc alone can break all cycles (a crossing arc (u^out → v^in) only disrupts cycles using that specific arc, while the internal arc covers all cycles through vertex u or v)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- (Two triangles sharing vertices 1,2,3, with leaf vertex 5 attached to 3)
- Minimum vertex cover: k = 3, e.g. {1, 2, 3}:
  - {0,1} by 1 ✓, {0,2} by 2 ✓, {1,2} by 1,2 ✓, {1,3} by 1,3 ✓
  - {2,4} by 2 ✓, {3,4} by 3 ✓, {3,5} by 3 ✓

**Constructed target instance (MinimumFeedbackArcSet):**
Directed graph G' with 12 vertices and 13 arcs:

Vertices: {0^in, 0^out, 1^in, 1^out, 2^in, 2^out, 3^in, 3^out, 4^in, 4^out, 5^in, 5^out}

Internal arcs (6 total, one per original vertex):
- (0^in → 0^out), (1^in → 1^out), (2^in → 2^out)
- (3^in → 3^out), (4^in → 4^out), (5^in → 5^out)

Crossing arcs (14 total, two per original edge):
- Edge {0,1}: (0^out → 1^in) and (1^out → 0^in)
- Edge {0,2}: (0^out → 2^in) and (2^out → 0^in)
- Edge {1,2}: (1^out → 2^in) and (2^out → 1^in)
- Edge {1,3}: (1^out → 3^in) and (3^out → 1^in)
- Edge {2,4}: (2^out → 4^in) and (4^out → 2^in)
- Edge {3,4}: (3^out → 4^in) and (4^out → 3^in)
- Edge {3,5}: (3^out → 5^in) and (5^out → 3^in)

Total arcs: 6 + 14 = 20 arcs. (n + 2m = 6 + 14 = 20 ✓)

Target budget: K' = k = 3

Example directed cycles in G':
- C_01: 0^in → 0^out → 1^in → 1^out → 0^in (length-4 cycle via edge {0,1})
- C_02: 0^in → 0^out → 2^in → 2^out → 0^in (length-4 cycle via edge {0,2})
- C_13: 1^in → 1^out → 3^in → 3^out → 1^in (length-4 cycle via edge {1,3})

**Solution mapping:**
- Selected internal arcs: A' = {(1^in → 1^out), (2^in → 2^out), (3^in → 3^out)} (size 3)
- Cycle C_01 broken: (1^in → 1^out) ∈ A' ✓
- Cycle C_02 broken: (2^in → 2^out) ∈ A' ✓
- Cycle C_13 broken: (1^in → 1^out) ∈ A' ✓
- All 7 corresponding cycles are broken by at least one internal arc in A' ✓
- Extracted vertex cover: S = {1, 2, 3} (vertices whose internal arcs are in A')
- Verification on G: all 7 edges covered ✓
- FAS size = 3 = k ✓

**Greedy trap:** Vertex 4 appears in 2 edges ({2,4} and {3,4}). A greedy approach might select vertex 4's internal arc to break two cycles. However, vertex 4 can only break cycles involving edges {2,4} and {3,4}, while selecting vertex 2 (or 3) breaks more cycles. This shows that a greedy strategy based on local arc counts can miss the optimal global solution.


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.
- **[Luchesi, 1976]**: [`Luchesi1976`] Claudio L. Luchesi (1976). "A Minimax Equality for Directed Graphs". University of Waterloo.
