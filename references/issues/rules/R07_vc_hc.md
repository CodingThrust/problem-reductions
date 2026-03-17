---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to HAMILTONIAN CIRCUIT"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'HAMILTONIAN CIRCUIT'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** HAMILTONIAN CIRCUIT
**Motivation:** Establishes NP-completeness of HAMILTONIAN CIRCUIT by a gadget-based polynomial-time reduction from VERTEX COVER, enabling downstream reductions to HAMILTONIAN PATH, TSP, and other tour-finding problems.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.4, p.56-60

## Reduction Algorithm

> Theorem 3.4 HAMILTONIAN CIRCUIT is NP-complete
> Proof: It is easy to see that HC E NP, because a nondeterministic algorithm need only guess an ordering of the vertices and check in polynomial time that all the required edges belong to the edge set of the given graph.
>
> We transform VERTEX COVER to HC. Let an arbitrary instance of VC be given by the graph G = (V,E) and the positive integer K <= |V|. We must construct a graph G' = (V',E') such that G' has a Hamiltonian circuit if and only if G has a vertex cover of size K or less.
>
> Once more our construction can be viewed in terms of components connected together by communication links. First, the graph G' has K "selector" vertices a1,a2, . . . , aK, which will be used to select K vertices from the vertex set V for G. Second, for each edge in E, G' contains a "cover-testing" component that will be used to ensure that at least one endpoint of that edge is among the selected K vertices. The component for e = {u,v} E E is illustrated in Figure 3.4. It has 12 vertices,
>
> V'_e = {(u,e,i),(v,e,i): 1 <= i <= 6}
>
> and 14 edges,
>
> E'_e = {{(u,e,i),(u,e,i+1)},{(v,e,i),(v,e,i+1)}: 1 <= i <= 5}
>      U {{(u,e,3),(v,e,1)},{(v,e,3),(u,e,1)}}
>      U {{(u,e,6),(v,e,4)},{(v,e,6),(u,e,4)}}
>
> In the completed construction, the only vertices from this cover-testing component that will be involved in any additional edges are (u,e,1), (v,e,1), (u,e,6), and (v,e,6). This will imply, as the reader may readily verify, that any Hamiltonian circuit of G' will have to meet the edges in E'_e in exactly one of the three configurations shown in Figure 3.5. Thus, for example, if the circuit "enters" this component at (u,e,1), it will have to "exit" at (u,e,6) and visit either all 12 vertices in the component or just the 6 vertices (u,e,i), 1 <= i <= 6.
>
> Additional edges in our overall construction will serve to join pairs of cover-testing components or to join a cover-testing component to a selector vertex. For each vertex v E V, let the edges incident on v be ordered (arbitrarily) as e_{v[1]}, e_{v[2]}, . . . , e_{v[deg(v)]}, where deg(v) denotes the degree of v in G, that is, the number of edges incident on v. All the cover-testing components corresponding to these edges (having v as endpoint) are joined together by the following connecting edges:
>
> E'_v = {{(v,e_{v[i]},6),(v,e_{v[i+1]},1)}: 1 <= i < deg(v)}
>
> As shown in Figure 3.6, this creates a single path in G' that includes exactly those vertices (x,y,z) having x = v.
>
> The final connecting edges in G' join the first and last vertices from each of these paths to every one of the selector vertices a1,a2, . . . , aK. These edges are specified as follows:
>
> E'' = {{a_i,(v,e_{v[1]},1)},{a_i,(v,e_{v[deg(v)]},6)}: 1 <= i <= K, v E V}
>
> The completed graph G' = (V',E') has
>
> V' = {a_i: 1 <= i <= K} U (U_{e E E} V'_e)
>
> and
>
> E' = (U_{e E E} E'_e) U (U_{v E V} E'_v) U E''
>
> It is not hard to see that G' can be constructed from G and K in polynomial time.
>
> We claim that G' has a Hamiltonian circuit if and only if G has a vertex cover of size K or less. Suppose <v1,v2, . . . , vn>, where n = |V'|, is a Hamiltonian circuit for G'. Consider any portion of this circuit that begins at a vertex in the set {a1,a2, . . . , aK}, ends at a vertex in {a1,a2, . . . , aK}, and that encounters no such vertex internally. Because of the previously mentioned restrictions on the way in which a Hamiltonian circuit can pass through a cover-testing component, this portion of the circuit must pass through a set of cover-testing components corresponding to exactly those edges from E that are incident on some one particular vertex v E V. Each of the cover-testing components is traversed in one of the modes (a), (b), or (c) of Figure 3.5, and no vertex from any other cover-testing component is encountered. Thus the K vertices from {a1,a2, . . . , aK} divide the Hamiltonian circuit into K paths, each path corresponding to a distinct vertex v E V. Since the Hamiltonian circuit must include all vertices from every one of the cover-testing components, and since vertices from the cover-testing component for edge e E E can be traversed only by a path corresponding to an endpoint of e, every edge in E must have at least one endpoint among those K selected vertices. Therefore, this set of K vertices forms the desired vertex cover for G.
>
> Conversely, suppose V* ⊆ V is a vertex cover for G with |V*| <= K. We can assume that |V*| = K since additional vertices from V can always be added and we will still have a vertex cover. Let the elements of V* be labeled as v1,v2, . . . , vK. The following edges are chosen to be "in" the Hamiltonian circuit for G'. From the cover-testing component representing each edge e = {u,v} E E, choose the edges specified in Figure 3.5(a), (b), or (c) depending on whether {u,v} ∩ V* equals, respectively, {u}, {u,v}, or {v}. One of these three possibilities must hold since V* is a vertex cover for G. Next, choose all the edges in E'_{v_i} for 1 <= i <= K. Finally, choose the edges
>
> {a_i,(v_i,e_{v_i[1]},1)}, 1 <= i <= K
>
> {a_{i+1},(v_i,e_{v_i[deg(v_i)]},6)}, 1 <= i < K
>
> and
>
> {a_1,(v_K,e_{v_K[deg(v_K)]},6)}
>
> We leave to the reader the task of verifying that this set of edges actually corresponds to a Hamiltonian circuit for G'.

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G, K) where G = (V, E) with n = |V| vertices, m = |E| edges, and K the cover size bound, construct a HamiltonianCircuit instance G' = (V', E') as follows:

1. **Selector vertices:** Add K vertices a_1, ..., a_K representing "slots" for the K cover vertices.

2. **Cover-testing gadgets:** For each edge e = {u, v} ∈ E, add 12 vertices — (u,e,1)...(u,e,6) and (v,e,1)...(v,e,6) — connected by 14 internal edges forming two 6-chains with two cross-links at positions 3→1 and 6→4 (in both directions). Each gadget has exactly three valid traversal modes: pass through all 12 vertices (both u and v "selected"), pass through only the u-side (only u selected), or pass through only the v-side (only v selected).

3. **Vertex path edges:** For each vertex v ∈ V, chain together its incident gadgets in arbitrary order using edges {(v, e_{v[i]}, 6), (v, e_{v[i+1]}, 1)} for consecutive incident edges. This forms a single path through all gadget-vertices labelled with v.

4. **Selector connection edges:** For each selector a_i and each vertex v ∈ V, add edges {a_i, (v, e_{v[1]}, 1)} and {a_i, (v, e_{v[deg(v)]}, 6)} connecting the selector to both endpoints of v's vertex path. Total: 2kn edges.

5. **Solution extraction:** A Hamiltonian circuit in G' is divided by the K selector vertices into K sub-paths. Each sub-path traverses exactly the gadgets corresponding to edges incident on some vertex v; the K such vertices form the vertex cover. Conversely, given a vertex cover {v_1, ..., v_K}, assign a_i to v_i, traverse each gadget in mode (a/b/c) depending on cover membership, and connect using the selector edges to form the circuit.

**Vertex count:** 12m (gadgets) + k (selectors) = 12m + k
**Edge count:** 14m (gadget internal) + (2m − n) (vertex path chains) + 2kn (selector connections) = 16m − n + 2kn

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source MinimumVertexCover instance (|V|)
- m = `num_edges` of source MinimumVertexCover instance (|E|)
- k = cover size bound parameter (K)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `12 * num_edges + k` |
| `num_edges` | `16 * num_edges - num_vertices + 2 * k * num_vertices` |

**Derivation:**
- Vertices: each of the m edge gadgets has 12 vertices, plus k selector vertices → 12m + k
- Edges:
  - 14 per gadget (5+5 chain edges + 4 cross-links) × m gadgets = 14m
  - Vertex path edges: for each vertex v, deg(v)−1 chain edges; total = ∑_v (deg(v)−1) = 2m − n
  - Selector connections: k selectors × n vertices × 2 endpoints = 2kn
  - Total = 14m + (2m − n) + 2kn = 16m − n + 2kn

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small MinimumVertexCover instance (G, K) to HamiltonianCircuit G', solve G' with BruteForce, then verify that if a Hamiltonian circuit exists, the corresponding K vertices form a valid vertex cover of G, and vice versa.
- Test with a graph that has a known minimum vertex cover (e.g., a path graph P_n has minimum VC of size n−1) and verify the HC instance has a Hamiltonian circuit iff the cover size K ≥ minimum.
- Test with K < minimum VC size to confirm no Hamiltonian circuit is found.
- Verify vertex and edge counts in G' match the formulas: |V'| = 12m + k, |E'| = 16m − n + 2kn.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 4 vertices {0, 1, 2, 3} and 6 edges (K_4):
- Edges (indexed): e_0={0,1}, e_1={0,2}, e_2={0,3}, e_3={1,2}, e_4={1,3}, e_5={2,3}
- n = 4, m = 6, K = 3
- Minimum vertex cover of size 3: {0, 1, 2} covers all edges

**Constructed target instance (HamiltonianCircuit):**
- Vertex count: 12 × 6 + 3 = 75 vertices
- Edge count: 16 × 6 − 4 + 2 × 3 × 4 = 96 − 4 + 24 = 116 edges

Gadget for e_0 = {0,1}: vertices (0,e_0,1)...(0,e_0,6) and (1,e_0,1)...(1,e_0,6) with internal edges:
- Chains: {(0,e_0,i),(0,e_0,i+1)} and {(1,e_0,i),(1,e_0,i+1)} for i=1..5 (10 edges)
- Cross-links: {(0,e_0,3),(1,e_0,1)}, {(1,e_0,3),(0,e_0,1)}, {(0,e_0,6),(1,e_0,4)}, {(1,e_0,6),(0,e_0,4)} (4 edges)

Vertex path for vertex 0 (incident edges: e_0, e_1, e_2):
- Chain edges: {(0,e_0,6),(0,e_1,1)}, {(0,e_1,6),(0,e_2,1)} (2 edges)

Selector connections for a_1 and vertex 0:
- {a_1, (0,e_0,1)}, {a_1, (0,e_2,6)} (entry/exit of vertex 0's path)

**Solution mapping (vertex cover {0,1,2} with K=3, assigning a_1↔0, a_2↔1, a_3↔2):**
- For e_0={0,1}: both in cover → mode (b): traverse all 12 vertices of gadget e_0
- For e_3={1,2}: both in cover → mode (b): traverse all 12 vertices of gadget e_3
- For e_1={0,2}: both in cover → mode (b): traverse all 12 vertices of gadget e_1
- For e_2={0,3}: only 0 in cover → mode (a): traverse only the 0-side (6 vertices)
- For e_4={1,3}: only 1 in cover → mode (a): traverse only the 1-side (6 vertices)
- For e_5={2,3}: only 2 in cover → mode (a): traverse only the 2-side (6 vertices)
- Circuit: a_1 → [gadgets for vertex 0] → a_2 → [gadgets for vertex 1] → a_3 → [gadgets for vertex 2] → a_1
- All 75 vertices are visited exactly once ✓
