---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to ISOMORPHIC SPANNING TREE"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Path'
canonical_target_name: 'Isomorphic Spanning Tree'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN PATH
**Target:** ISOMORPHIC SPANNING TREE
**Motivation:** Establishes NP-completeness of ISOMORPHIC SPANNING TREE via polynomial-time reduction from HAMILTONIAN PATH. The reduction is essentially trivial: a Hamiltonian path IS a spanning tree isomorphic to a path graph P_n. This observation shows that even the special case where T is a simple path is already NP-complete. The problem remains NP-complete for other tree types including full binary trees (Papadimitriou and Yannakakis, 1978) and 3-stars.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND8, p.207

## GJ Source Entry

> [ND8] ISOMORPHIC SPANNING TREE
> INSTANCE: Graph G=(V,E), tree T=(V_T,E_T).
> QUESTION: Does G contain a spanning tree isomorphic to T?
> Reference: Transformation from HAMILTONIAN PATH.
> Comment: Remains NP-complete even if (a) T is a path, (b) T is a full binary tree [Papadimitriou and Yannakakis, 1978], or if (c) T is a 3-star (that is, V_T={v_0} union {u_i,v_i,w_i: 1<=i<=n}, E_T={{v_0,u_i},{u_i,v_i},{v_i,w_i}: 1<=i<=n}) [Garey and Johnson, ----]. Solvable in polynomial time by graph matching if G is a 2-star. For a classification of the complexity of this problem for other types of trees, see [Papadimitriou and Yannakakis, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HAMILTONIAN PATH instance G = (V, E) with n = |V| vertices, construct an ISOMORPHIC SPANNING TREE instance (G, T) as follows:

1. **Graph preservation:** Keep the graph G = (V, E) unchanged as the host graph.

2. **Tree construction:** Set T = P_n, the path graph on n vertices. Explicitly: T = (V_T, E_T) where V_T = {t_0, t_1, ..., t_{n-1}} and E_T = {{t_i, t_{i+1}} : 0 <= i <= n-2}. This is a tree with n vertices, maximum degree 2, and exactly n-1 edges.

3. **Parameter:** The target tree T has |V_T| = |V| = n (required for T to be a spanning tree of G).

4. **Solution extraction:** A spanning tree of G isomorphic to P_n is exactly a Hamiltonian path in G. Given a spanning subgraph S of G that is isomorphic to T = P_n, the vertex ordering given by the isomorphism defines a Hamiltonian path in G.

**Correctness:**
- (Forward) If G has a Hamiltonian path v_{pi(0)}, v_{pi(1)}, ..., v_{pi(n-1)}, then the edges {{v_{pi(i)}, v_{pi(i+1)}} : 0 <= i <= n-2} form a spanning tree of G isomorphic to P_n (the isomorphism maps v_{pi(i)} to t_i).
- (Backward) If G has a spanning tree isomorphic to P_n, that tree visits all n vertices in a path, which is by definition a Hamiltonian path in G.

This is a direct (identity on G) reduction: the graph is unchanged, and only the tree T is constructed.

**Source:** Garey & Johnson (1979), *Computers and Intractability*, p. 207, ND8.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` (host graph) | `num_vertices` (unchanged) |
| `num_edges` (host graph) | `num_edges` (unchanged) |
| `tree_vertices` | `num_vertices` |
| `tree_edges` | `num_vertices - 1` |

**Derivation:** The host graph is unchanged. The target tree T = P_n has exactly n vertices and n-1 edges. Total input size is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a graph G, reduce to (G, P_n), solve the Isomorphic Spanning Tree problem with BruteForce (enumerate spanning trees and check isomorphism to P_n), extract the Hamiltonian path from the isomorphism, verify it visits all vertices exactly once using only edges of G.
- Forward test: construct a graph with a known Hamiltonian path, verify G has a spanning tree isomorphic to P_n.
- Backward test: construct a graph with no Hamiltonian path (e.g., Petersen graph), verify no spanning tree isomorphic to P_n exists.
- Identity check: verify the host graph in the target instance is identical to the source graph.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Hamiltonian Path):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 10 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}, {4,6}, {5,6}, {0,6}
- This graph has a Hamiltonian path: 0 -- 1 -- 3 -- 5 -- 6 -- 4 -- 2

**Constructed target instance (Isomorphic Spanning Tree):**
- Host graph: G (unchanged, same 7 vertices and 10 edges)
- Target tree: T = P_7 (path on 7 vertices)
  - T vertices: {t_0, t_1, t_2, t_3, t_4, t_5, t_6}
  - T edges: {t_0,t_1}, {t_1,t_2}, {t_2,t_3}, {t_3,t_4}, {t_4,t_5}, {t_5,t_6}

**Solution mapping:**
- Spanning tree of G isomorphic to P_7: edges {0,1}, {1,3}, {3,5}, {5,6}, {6,4}, {4,2}
  - This corresponds to the path 0 -- 1 -- 3 -- 5 -- 6 -- 4 -- 2
  - Isomorphism: 0 -> t_0, 1 -> t_1, 3 -> t_2, 5 -> t_3, 6 -> t_4, 4 -> t_5, 2 -> t_6
  - Verify edges: {0,1} in G? YES. {1,3}? YES. {3,5}? YES. {5,6}? YES. {6,4}? YES ({4,6}). {4,2}? YES ({2,4}).
  - Spanning? All 7 vertices visited exactly once. YES.
  - Isomorphic to P_7? Degree sequence: 0 has degree 1, 2 has degree 1, all others have degree 2 in the tree. Matches P_7.

**Greedy trap:** A greedy approach starting from vertex 0 might try 0--1--2--4--3--5--6. Check: {0,1} YES, {1,2} YES, {2,4} YES, {4,3} YES, {3,5} YES, {5,6} YES. This also works! But starting 0--2--4--6--... then 6 connects to 0 (already visited) and 5. Continue 6--5--3--1. Check: {5,3} YES, {3,1} YES. Path: 0--2--4--6--5--3--1. All edges present, all vertices visited. Another valid Hamiltonian path.

A graph WITHOUT a Hamiltonian path: take the same vertices but remove edges to create a disconnected or high-degree-centralized structure (e.g., a star K_{1,6}). The star has no spanning path since the center must be visited multiple times.


## References

- **[Papadimitriou and Yannakakis, 1978]**: [`Papadimitriou1978f`] Christos H. Papadimitriou and M. Yannakakis (1978). "On the complexity of minimum spanning tree problems".
- **[Garey and Johnson, ----]**: *(not found in bibliography)*
