---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to CLIQUE"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'CLIQUE'
source_in_codebase: true
target_in_codebase: true
---

**Source:** VERTEX COVER
**Target:** CLIQUE
**Motivation:** Establishes the tight equivalence between VERTEX COVER and CLIQUE via complement graphs, showing that a minimum vertex cover in G directly corresponds to a maximum clique in the complement of G.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Lemma 3.1, p.54

## Reduction Algorithm

> Lemma 3.1 For any graph G = (V,E) and subset V' ⊆ V, the following statements are equivalent:
>
> (a) V' is a vertex cover for G.
> (b) V-V' is an independent set for G.
> (c) V-V' is a clique in the complement G^c of G, where G^c = (V,E^c) with E^c = {{u,v}: u,v E V and {u,v} not-E E}.
>
> Thus we see that, in a rather strong sense, these three problems might be regarded simply as "different versions" of one another. Furthermore, the relationships displayed in the lemma make it a trivial matter to transform any one of the problems to either of the others.
>
> For example, to transform VERTEX COVER to CLIQUE, let G = (V,E) and K <= |V| constitute any instance of VC. The corresponding instance of CLIQUE is provided simply by the graph G^c and the integer J = |V|-K.
>
> This implies that the NP-completeness of all three problems will follow as an immediate consequence of proving that any one of them is NP-complete. We choose to prove this for VERTEX COVER.

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G, k) where G = (V, E) and k is the cover size bound, construct a MaximumClique instance as follows:

1. **Complement graph construction:** Build G^c = (V, E^c) where E^c contains exactly those pairs {u, v} that are NOT edges in E (but u ≠ v). The vertex set is unchanged.
2. **Clique size parameter:** Set J = |V| - k. A vertex cover of size ≤ k in G exists if and only if a clique of size ≥ J in G^c exists.
3. **Solution extraction:** Given a maximum clique C in G^c of size J, the vertex cover in G is V \ C (the complement of C), which has size |V| - J = k.

**Key invariant:** V' is a vertex cover in G if and only if V \ V' is a clique in G^c (by Lemma 3.1). Therefore MinVC(G) + MaxClique(G^c) = |V|.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source G
- m = `num_edges` of source G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_vertices * (num_vertices - 1) / 2 - num_edges` |

**Derivation:** The complement graph has the same vertices (n). Each pair of vertices either has an edge in G or in G^c (not both). The complete graph K_n has n(n-1)/2 edges, so G^c has n(n-1)/2 - m edges.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce source MinimumVertexCover instance to MaximumClique, solve target with BruteForce, extract solution (V \ clique), verify it is a valid vertex cover on the original graph
- Check that max clique size J in G^c satisfies J = n - (minimum VC size in G)
- Compare with known results: path graph P_n has VC of size n-1 and G^c = K_n minus a matching, so clique in complement has size 2

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- (Two triangles sharing a path, plus an extra vertex)
- Minimum vertex cover has size k = 3: {1, 2, 3} covers all edges (0-1 by 1, 0-2 by 2, 1-2 by both, 1-3 by both, 2-4 by 2, 3-4 by 3, 3-5 by 3)

**Constructed target instance (MaximumClique in G^c):**
Complement graph G^c has the same 6 vertices and all edges NOT in G:
- Total possible edges in K_6: 6*5/2 = 15
- Edges in G: 7
- Edges in G^c: 15 - 7 = 8
- G^c edges: {0,3}, {0,4}, {0,5}, {1,4}, {1,5}, {2,3}, {2,5}, {4,5}
- Target clique size: J = 6 - 3 = 3

**Solution mapping:**
- Maximum clique in G^c: {0, 4, 5} — check: {0,4} ✓, {0,5} ✓, {4,5} ✓ — all edges present in G^c
- Extracted vertex cover in G: V \ {0, 4, 5} = {1, 2, 3}
- Verification: edges {0,1} covered by 1 ✓, {0,2} covered by 2 ✓, {1,2} covered by 1,2 ✓, {1,3} covered by 1,3 ✓, {2,4} covered by 2 ✓, {3,4} covered by 3 ✓, {3,5} covered by 3 ✓
- All 7 edges covered, VC size = 3 = k ✓
