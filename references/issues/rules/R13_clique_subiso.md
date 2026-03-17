---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to SUBGRAPH ISOMORPHISM"
labels: rule
assignees: ''
canonical_source_name: 'CLIQUE'
canonical_target_name: 'SUBGRAPH ISOMORPHISM'
source_in_codebase: true
target_in_codebase: false
---

**Source:** CLIQUE
**Target:** SUBGRAPH ISOMORPHISM
**Motivation:** Establishes NP-completeness of SUBGRAPH ISOMORPHISM via polynomial-time reduction from CLIQUE, by observing that detecting a clique of size k is equivalent to detecting a copy of the complete graph K_k as a subgraph.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Reduction Algorithm

> (3) SUBGRAPH ISOMORPHISM
> INSTANCE: Two graphs, G = (V1,E1) and H = (V2,E2).
> QUESTION: Does G contain a subgraph isomorphic to H, that is, a subset V ⊆ V1 and a subset E ⊆ E1 such that |V|=|V2|, |E|=|E2|, and there exists a one-to-one function f: V2->V satisfying {u,v} E E2 if and only if {f(u),f(v)} E E?
>
> Proof: Restrict to CLIQUE by allowing only instances for which H is a complete graph, that is, E2 contains all possible edges joining two members of V2.

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MaximumClique instance (G, k) where G = (V₁, E₁) and k is the clique size, construct a SubgraphIsomorphism instance as follows:

1. **Host graph:** G₁ = G (the original graph, passed through unchanged).
2. **Pattern graph construction:** Build G₂ = K_k, the complete graph on k vertices {0, 1, ..., k-1}, with all k(k-1)/2 edges. This is the graph whose subgraph embedding we seek.
3. **Query:** Ask whether G₁ contains a subgraph isomorphic to G₂.
4. **Solution extraction:** Given a subgraph isomorphism f: V₂ → V₁, the image f(V₂) ⊆ V₁ is a clique of size k in G. Conversely, any k-clique {v₁, ..., v_k} in G yields an isomorphism by mapping the i-th pattern vertex to vᵢ.

**Key invariant:** G contains a clique of size k if and only if G contains a subgraph isomorphic to K_k, because K_k is the unique graph on k vertices with all k(k-1)/2 edges.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G (= |V₁|)
- m = `num_edges` of source graph G (= |E₁|)
- k = clique size parameter from source instance

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices_host` | `num_vertices` |
| `num_edges_host` | `num_edges` |
| `num_vertices_pattern` | k (the clique size parameter) |
| `num_edges_pattern` | k * (k - 1) / 2 |

**Derivation:**
- Host graph G₁ = G is passed through unchanged: |V₁| = n, |E₁| = m
- Pattern graph G₂ = K_k: |V₂| = k vertices, |E₂| = k(k-1)/2 edges (complete graph)
- Note: k ≤ n since a clique cannot exceed the number of vertices

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MaximumClique instance to SubgraphIsomorphism, solve the SubgraphIsomorphism with BruteForce (enumerate all injective mappings f: V₂ → V₁ and check edge preservation), extract the clique from the image of f, verify the clique on the original graph
- Test with both a graph containing a k-clique and one without to verify bidirectional correctness
- Verify the pattern graph is always complete: |E₂| = k(k-1)/2
- Include a non-trivial case where k ≥ 4 and the host graph is dense enough to have multiple potential cliques but only one of size k

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MaximumClique):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 13 edges:
- Edges: {0,1}, {0,2}, {0,3}, {1,2}, {1,3}, {2,3}, {1,4}, {2,4}, {4,5}, {4,6}, {5,6}, {0,5}, {3,6}
- (Vertices {0,1,2,3} form a 4-clique K_4; vertices {4,5,6} form a triangle)
- Target clique size: k = 4

**Constructed target instance (SubgraphIsomorphism):**

Host graph G₁ = G (7 vertices, 13 edges, same as above)

Pattern graph G₂ = K_4 (complete graph on 4 vertices {a, b, c, d}):
- Vertices: {a, b, c, d} — 4 vertices
- Edges: {a,b}, {a,c}, {a,d}, {b,c}, {b,d}, {c,d} — 6 edges = 4*3/2
- |V₂| = 4, |E₂| = 6

**Solution mapping:**
- Subgraph isomorphism found: f(a)=0, f(b)=1, f(c)=2, f(d)=3
- Verify edge preservation:
  - {a,b} → {f(a),f(b)} = {0,1} ∈ E₁ ✓
  - {a,c} → {f(a),f(c)} = {0,2} ∈ E₁ ✓
  - {a,d} → {f(a),f(d)} = {0,3} ∈ E₁ ✓
  - {b,c} → {f(b),f(c)} = {1,2} ∈ E₁ ✓
  - {b,d} → {f(b),f(d)} = {1,3} ∈ E₁ ✓
  - {c,d} → {f(c),f(d)} = {2,3} ∈ E₁ ✓
- All 6 pattern edges are preserved ✓
- Extracted clique in G: {f(a), f(b), f(c), f(d)} = {0, 1, 2, 3} — 4-clique ✓

**Non-trivial structure:**
- The triangle {4,5,6} is a 3-clique but NOT a 4-clique (vertex 4 is not adjacent to vertex 0, 2, or 3 except via 1 and 2, so it cannot extend the 4-clique).
- The subgraph isomorphism from K_4 must map to {0,1,2,3} specifically; the mapping f(a)=4, f(b)=5, f(c)=6, f(d)=? fails since K_4 requires a 4th vertex adjacent to all others.
