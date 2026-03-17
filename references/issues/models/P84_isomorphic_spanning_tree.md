---
name: Problem
about: Propose a new problem type
title: "[Model] IsomorphicSpanningTree"
labels: model
assignees: ''
---

## Motivation

ISOMORPHIC SPANNING TREE (P84) from Garey & Johnson, A2 ND8. A classical NP-complete problem that generalizes the Hamiltonian path problem: finding a Hamiltonian path is equivalent to finding a spanning tree isomorphic to the path graph P_n. The problem remains NP-complete even for restricted tree types (paths, full binary trees, 3-stars).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R29 (HAMILTONIAN PATH -> ISOMORPHIC SPANNING TREE) -- the NP-completeness proof referenced in G&J.
- **As target:** R289 (HAMILTONIAN PATH -> ISOMORPHIC SPANNING TREE) -- duplicate entry in rule set referencing the same reduction from A2.1 ND8.
- **As source:** None found in the current rule set.

## Definition

**Name:** `IsomorphicSpanningTree`
<!-- ⚠️ Unverified -->
**Canonical name:** ISOMORPHIC SPANNING TREE (also: Spanning Tree Isomorphism, Spanning Subgraph Isomorphic to Tree)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND8

**Mathematical definition:**

INSTANCE: Graph G = (V,E), tree T = (VT,ET).
QUESTION: Does G contain a spanning tree isomorphic to T?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| = |VT| variables, encoding a bijection (permutation) from VT to V. Alternatively, n*(n-1)/2 binary variables, one per potential edge in the spanning tree.
- **Per-variable domain:** For a permutation encoding: each variable maps a tree vertex to a graph vertex, domain = {0, 1, ..., n-1}. For an edge-selection encoding: {0, 1} indicating whether each edge of G is included in the spanning tree.
- **Meaning:** The variable assignment defines a spanning subgraph of G that must (a) be a tree (connected, n-1 edges) and (b) be isomorphic to T as a graph. Equivalently, it defines a bijection pi: VT -> V such that for every edge {u,v} in ET, {pi(u), pi(v)} is an edge in E.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `IsomorphicSpanningTree`
**Variants:** graph type parameter G

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The host graph G in which a spanning tree is sought |
| `tree` | `SimpleGraph` | The target tree T that the spanning tree must be isomorphic to |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Precondition: |V(G)| = |V(T)| and T must be a tree (connected, |VT|-1 edges).
- No weight type is needed (the question is purely structural).
- The `tree` field stores T as a graph; a runtime check or type-level guarantee ensures it is indeed a tree.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The general case reduces to subgraph isomorphism, which can be solved in O*(2^n) time by dynamic programming. For the special case where T is a path (Hamiltonian Path), the best algorithm is Bjorklund's randomized O*(1.657^n) time. For general trees, backtracking with constraint propagation is typically used; no improvement over O*(2^n) is known for arbitrary trees.
- **Special cases:**
  - T = path: equivalent to Hamiltonian Path, O*(1.657^n) randomized (Bjorklund, 2010/2014).
  - T = full binary tree: NP-complete (Papadimitriou and Yannakakis, 1978).
  - T = 3-star: NP-complete (Garey and Johnson).
  - T = 2-star: polynomial time via graph matching.
- **NP-completeness:** NP-complete (Garey and Johnson, 1979, ND8). Transformation from HAMILTONIAN PATH.
- **References:**
  - Garey, M.R. and Johnson, D.S. (1979). *Computers and Intractability*. W.H. Freeman.
  - Papadimitriou, C.H. and Yannakakis, M. (1978). "On the complexity of minimum spanning tree problems".
  - Bjorklund, A. (2014). "Determinant Sums for Undirected Hamiltonicity". *SIAM J. Computing* 43(1):280-299.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), tree T = (VT,ET).
QUESTION: Does G contain a spanning tree isomorphic to T?

Reference: Transformation from HAMILTONIAN PATH.
Comment: Remains NP-complete even if (a) T is a path, (b) T is a full binary tree [Papadimitriou and Yannakakis, 1978], or if (c) T is a 3-star (that is, VT = {v0} union {ui,vi,wi: 1 <= i <= n}, ET = {{v0,ui},{ui,vi},{vi,wi}: 1 <= i <= n}) [Garey and Johnson, ----]. Solvable in polynomial time by graph matching if G is a 2-star. For a classification of the complexity of this problem for other types of trees, see [Papadimitriou and Yannakakis, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all (n-1)-edge subsets of E that form a tree, check isomorphism to T.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Enumerate all permutations pi: VT -> V and check if the induced edge set is a subset of E. For T = path, use Held-Karp DP in O(n^2 * 2^n). For general T, use backtracking with constraint propagation (prune based on degree sequence compatibility).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, tree = caterpillar):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 12 edges:
- Edges: {0,1}, {0,2}, {0,3}, {1,2}, {1,4}, {2,3}, {2,5}, {3,6}, {4,5}, {4,6}, {5,6}, {1,3}
- Tree T (caterpillar): vertices {a, b, c, d, e, f, g}, edges {a,b}, {b,c}, {c,d}, {d,e}, {b,f}, {c,g}
  - Degree sequence of T: a:1, b:3, c:3, d:2, e:1, f:1, g:1
- Solution: Map b->2, c->1, d->4, a->3, e->5, f->0, g->3... Let's be careful.
  - Need bijection pi: {a,b,c,d,e,f,g} -> {0,1,2,3,4,5,6} such that all tree edges map to graph edges.
  - Try pi: a->0, b->1, c->2, d->3, e->6, f->4, g->5.
    - {a,b} -> {0,1} YES. {b,c} -> {1,2} YES. {c,d} -> {2,3} YES. {d,e} -> {3,6} YES. {b,f} -> {1,4} YES. {c,g} -> {2,5} YES.
    - All 6 tree edges map to graph edges. Spanning (all 7 vertices used). Valid.
- Answer: YES.

**Instance 2 (NO, tree = star K_{1,6}):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,6}, {0,6}, {0,3}, {3,6}
- Tree T = K_{1,6} (star with center and 6 leaves): center c, leaves l1..l6. Edges: {c,l1}, ..., {c,l6}.
  - T requires a vertex of degree 6. In G, maximum degree is: 0 has degree 3 ({0,1},{0,6},{0,3}), 3 has degree 4 ({2,3},{3,4},{0,3},{3,6}). No vertex has degree >= 6.
- Answer: NO (no vertex has sufficient degree to serve as the star center).
