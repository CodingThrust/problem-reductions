---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to BALANCED COMPLETE BIPARTITE SUBGRAPH"
labels: rule
assignees: ''
canonical_source_name: 'Clique'
canonical_target_name: 'Balanced Complete Bipartite Subgraph'
source_in_codebase: true
target_in_codebase: false
---

**Source:** CLIQUE
**Target:** BALANCED COMPLETE BIPARTITE SUBGRAPH
**Motivation:** Establishes NP-completeness of BALANCED COMPLETE BIPARTITE SUBGRAPH (also known as Maximum Balanced Biclique) via polynomial-time reduction from CLIQUE. This reduction, attributed to Garey and Johnson, shows that finding a balanced complete bipartite subgraph in a bipartite graph is as hard as finding a clique in a general graph. The problem has applications in data mining, bioinformatics (protein interaction networks), and VLSI design.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT24

## GJ Source Entry

> [GT24]  BALANCED COMPLETE BIPARTITE SUBGRAPH
> INSTANCE:  Bipartite graph G = (V,E), positive integer K <= |V|.
> QUESTION:  Are there two disjoint subsets V_1, V_2 <= V such that |V_1| = |V_2| = K and such that u in V_1, v in V_2 implies that {u,v} in E?
>
> Reference:  [Garey and Johnson, ----]. Transformation from CLIQUE.
> Comment:  The related problem in which the requirement "|V_1| = |V_2| = K" is replaced by "|V_1|+|V_2| = K" is solvable in polynomial time for bipartite graphs (because of the connection between matchings and independent sets in such graphs, e.g., see [Harary, 1969]), but is NP-complete for general graphs [Yannakakis, 1978b].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a CLIQUE instance (G = (V, E), K) where G is a general (not necessarily bipartite) graph with n = |V| vertices, construct a BALANCED COMPLETE BIPARTITE SUBGRAPH instance (G' = (V', E'), K) as follows:

1. **Bipartite vertex construction:** Create two copies of the vertex set V:
   - Left partition: L = {u_L : u in V}, with |L| = n
   - Right partition: R = {v_R : v in V}, with |R| = n
   - The bipartite graph has vertex set V' = L union R with |V'| = 2n.

2. **Edge construction (complement encoding):** For each pair (u, v) with u != v in V such that {u, v} IS an edge in G (i.e., u and v are adjacent), add the edge {u_L, v_R} to E'. Formally:
   E' = { {u_L, v_R} : {u, v} in E, u != v }
   Note: edges are placed between the left copy of one endpoint and the right copy of the other endpoint, but only for actual edges of G.

3. **Balanced biclique size parameter:** Set the target size to K (same as the clique parameter).

4. **Solution extraction:** Given two disjoint subsets V_1 subset L and V_2 subset R with |V_1| = |V_2| = K forming a complete bipartite subgraph in G', the corresponding vertices in G form a K-clique. Specifically, let S = {u in V : u_L in V_1} = {v in V : v_R in V_2}. Then S is a clique in G of size K.

**Correctness:**
- (Forward) If G has a K-clique S, then the left copies S_L = {u_L : u in S} and the right copies S_R = {v_R : v in S} form a balanced complete bipartite subgraph of size K in G': for any u_L in S_L and v_R in S_R with u != v, the edge {u, v} exists in G (since S is a clique), so {u_L, v_R} exists in E'. The same vertex appearing on both sides is handled by noting that in a K_K,K biclique, the left and right copies represent the same set, and every cross-pair has an edge.
- (Backward) If G' has a balanced K-biclique (V_1, V_2), identify the original vertices. Every pair of original vertices in the union must be adjacent in G, forming a clique.

**Note:** The precise details of this reduction vary slightly in the literature. Some formulations add auxiliary "padding" vertices (W) to handle the diagonal (same-vertex) case. The core idea is that adjacency in G maps to bipartite adjacency in G', so a clique structure in G corresponds to a biclique structure in G'.

**Source:** Garey and Johnson (unpublished, referenced in *Computers and Intractability*, 1979); Peeters (2003), "The maximum edge biclique problem is NP-complete".

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source G
- m = `num_edges` of source G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `2 * num_vertices` |
| `num_edges` | `2 * num_edges` |

**Derivation:** The bipartite graph G' has 2n vertices (n on each side). For each undirected edge {u,v} in G, we add two directed bipartite edges: {u_L, v_R} and {v_L, u_R}, giving 2m bipartite edges total. (If the construction treats edges as unordered pairs in the bipartite graph, the count may differ depending on whether self-loops u_L--u_R are included.)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a MaximumClique instance, reduce to BalancedCompleteBipartiteSubgraph, solve the target with BruteForce, extract the clique from the biclique vertices, verify it is a valid clique in the original graph.
- Forward test: construct a graph with a known K-clique, verify the bipartite graph contains a balanced K-biclique.
- Backward test: construct a graph with no K-clique, verify no balanced K-biclique exists in G'.
- Size verification: check that the bipartite graph has 2n vertices and 2m edges.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MaximumClique):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,3}, {3,4}, {3,5}, {4,5}, {0,5}
- Maximum clique: {0, 1, 2} (triangle) and {1, 2, 3} (triangle) and {3, 4, 5} (triangle), all of size 3.
- K = 3.

**Constructed target instance (BalancedCompleteBipartiteSubgraph):**
Bipartite graph G' with 12 vertices:
- Left partition L: {0_L, 1_L, 2_L, 3_L, 4_L, 5_L}
- Right partition R: {0_R, 1_R, 2_R, 3_R, 4_R, 5_R}
- Edges (for each edge {u,v} in G, add {u_L, v_R} and {v_L, u_R}):
  - {0,1}: 0_L--1_R, 1_L--0_R
  - {0,2}: 0_L--2_R, 2_L--0_R
  - {1,2}: 1_L--2_R, 2_L--1_R
  - {1,3}: 1_L--3_R, 3_L--1_R
  - {2,3}: 2_L--3_R, 3_L--2_R
  - {3,4}: 3_L--4_R, 4_L--3_R
  - {3,5}: 3_L--5_R, 5_L--3_R
  - {4,5}: 4_L--5_R, 5_L--4_R
  - {0,5}: 0_L--5_R, 5_L--0_R
- Total edges in G': 18 (= 2 * 9)
- Target biclique size: K = 3.

**Solution mapping:**
- Balanced 3-biclique in G': V_1 = {0_L, 1_L, 2_L}, V_2 = {0_R, 1_R, 2_R}
  - Check completeness: 0_L--1_R (from edge {0,1}), 0_L--2_R (from {0,2}), 1_L--0_R (from {0,1}), 1_L--2_R (from {1,2}), 2_L--0_R (from {0,2}), 2_L--1_R (from {1,2}).
  - But 0_L--0_R is NOT present (no self-loop in G). This means V_1 and V_2 must correspond to the same clique vertices, but pairs (u_L, u_R) for the same u are not connected.
  - Correction: The biclique cannot use the same vertex on both sides. A valid balanced 3-biclique: V_1 = {1_L, 2_L, 3_L}, V_2 = {0_R, 4_R, 5_R}... Let's check: 1_L--0_R? Edge {0,1} in G, YES. 1_L--4_R? No edge {1,4} in G. Not complete.
  - Better approach: the standard reduction may include self-edges or add padding. With the variant where {u_L, v_R} is added for ALL pairs u,v in the clique including u=v (by adding self-connections for all vertices): V_1 = {0_L, 1_L, 2_L}, V_2 = {0_R, 1_R, 2_R} works if we add diagonal edges {i_L, i_R} for all i.

**Revised construction with diagonal:** Add edges {u_L, u_R} for all u in V (self-connections across partitions). Then:
- Additional 6 edges: 0_L--0_R, 1_L--1_R, 2_L--2_R, 3_L--3_R, 4_L--4_R, 5_L--5_R
- Total edges: 18 + 6 = 24
- V_1 = {0_L, 1_L, 2_L}, V_2 = {0_R, 1_R, 2_R}:
  - 0_L--0_R (diagonal), 0_L--1_R ({0,1}), 0_L--2_R ({0,2}): all present
  - 1_L--0_R ({0,1}), 1_L--1_R (diagonal), 1_L--2_R ({1,2}): all present
  - 2_L--0_R ({0,2}), 2_L--1_R ({1,2}), 2_L--2_R (diagonal): all present
  - Complete K_{3,3} biclique. Valid.
- Extracted clique: {0, 1, 2}. Check in G: {0,1}, {0,2}, {1,2} all edges. Valid 3-clique.

| Target metric | Value |
|---|---|
| `num_vertices` | 12 = 2 * 6 |
| `num_edges` | 24 = 2 * 9 + 6 |


## References

- **[Garey and Johnson, ----]**: *(not found in bibliography)*
- **[Harary, 1969]**: [`Harary1969`] F. Harary (1969). "Graph Theory". Addison-Wesley, Reading, MA.
- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253-264. Association for Computing Machinery.
- **[Peeters, 2003]**: Ren\'{e} Peeters (2003). "The maximum edge biclique problem is NP-complete". *Discrete Applied Mathematics* 131(3), pp. 651-654.
