---
name: Problem
about: Propose a new problem type
title: "[Model] BiconnectivityAugmentation"
labels: model
assignees: ''
---

## Motivation

BICONNECTIVITY AUGMENTATION (P94) from Garey & Johnson, A2 ND18. An NP-complete graph augmentation problem studied by Eswaran and Tarjan (1976). Given an undirected graph with weighted potential edges, the goal is to add a minimum-weight set of edges so that the resulting graph is biconnected (2-vertex-connected: cannot be disconnected by removing any single vertex). The unweighted version is polynomial-time solvable, but the weighted version is NP-complete. This problem is fundamental to network design and fault-tolerant communication network construction.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R39: HAMILTONIAN CIRCUIT -> BICONNECTIVITY AUGMENTATION (ND18)

## Definition

**Name:** <!-- ⚠️ Unverified --> `BiconnectivityAugmentation`
**Canonical name:** Biconnectivity Augmentation (also: Weighted 2-Vertex-Connectivity Augmentation)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND18

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w({u,v}) in Z+ for each unordered pair {u,v} of vertices from V, positive integer B.
QUESTION: Is there a set E' of unordered pairs of vertices from V such that sum_{e in E'} w(e) <= B and such that the graph G' = (V, E union E') is biconnected, i.e., cannot be disconnected by removing a single vertex?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n*(n-1)/2 binary variables (one per potential edge, i.e., each unordered pair of vertices)
- **Per-variable domain:** binary {0, 1} -- whether edge {u,v} is added to E'
- **Meaning:** variable x_{u,v} = 1 if the edge {u,v} is added to E'. The configuration must satisfy: G' = (V, E union E') is biconnected and sum(w({u,v}) * x_{u,v}) <= B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `BiconnectivityAugmentation`
**Variants:** graph topology (graph type parameter G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The initial undirected graph G = (V, E) |
| `edge_weights` | `HashMap<(usize,usize), W>` | Weight w({u,v}) for each potential edge pair |
| `budget` | `W` | Maximum total weight B for added edges |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The optimization version minimizes sum(w(e)) for e in E' subject to biconnectivity of G union E'.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|).
- A graph is biconnected if it is connected and has no articulation point (cut vertex).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Eswaran and Tarjan, 1976; transformation from HAMILTONIAN CIRCUIT). Remains NP-complete when all weights are 1 or 2 and E is empty. Solvable in polynomial time when all weights are equal.
- **Best known exact algorithm:** ILP-based exact methods with cut enumeration. Exact algorithms have complexity exponential in O(min(n, alpha)), where alpha is related to the block-cut tree structure. For small instances (up to ~28 vertices), exact ILP solvers are practical.
- **Unweighted version:** Solvable in O(n + m) time by finding the block-cut tree and computing the minimum number of edges to add (Eswaran and Tarjan, 1976).
- **Approximation:** 2-approximation via primal-dual methods; (1.5+epsilon)-approximation for the related weighted connectivity augmentation problem (Traub and Zenklusen, 2023).
- **References:**
  - K.P. Eswaran, R.E. Tarjan (1976). "Augmentation Problems." *SIAM Journal on Computing*, 5(4):653-665.
  - G.N. Frederickson, J. Ja'Ja' (1981). "Approximation Algorithms for Several Graph Augmentation Problems." *SIAM Journal on Computing*, 10(2):270-283.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weight w({u,v}) in Z+ for each unordered pair {u,v} of vertices from V, positive integer B.
QUESTION: Is there a set E' of unordered pairs of vertices from V such that sum_{e in E'} w(e) <= B and such that the graph G' = (V,E union E') is biconnected, i.e., cannot be disconnected by removing a single vertex?

Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: The related problem in which G' must be bridge connected, i.e., cannot be disconnected by removing a single edge, is also NP-complete. Both problems remain NP-complete if all weights are either 1 or 2 and E is empty. Both can be solved in polynomial time if all weights are equal.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all subsets of potential edges, check if adding them achieves biconnectivity and total weight <= B.
- [x] It can be solved by reducing to integer programming. Binary variable per potential edge, minimize total weight subject to biconnectivity constraints (no cut vertex).
- [x] Other: For the unweighted case, linear-time algorithms exist based on the block-cut tree decomposition.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 5 edges (a tree plus one edge):**
- Existing edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}
- This is a path graph; every internal vertex is an articulation point, so G is NOT biconnected.
- Edge weights for all potential edges:
  - {0,2}: w=1, {0,3}: w=2, {0,4}: w=3, {0,5}: w=2
  - {1,3}: w=1, {1,4}: w=2, {1,5}: w=3
  - {2,4}: w=1, {2,5}: w=2
  - {3,5}: w=1
- Budget B = 4

**Solution: add E' = {{0,2}, {1,3}, {2,4}, {3,5}} with weights 1+1+1+1 = 4 <= B:**
- New graph G' has edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {0,2}, {1,3}, {2,4}, {3,5}
- Check biconnectivity: remove any single vertex:
  - Remove 0: remaining {1,2,3,4,5} connected via {1,2},{1,3},{2,3},{2,4},{3,4},{3,5},{4,5}
  - Remove 1: remaining {0,2,3,4,5} connected via {0,2},{2,3},{2,4},{3,4},{3,5},{4,5}
  - Remove 2: remaining {0,1,3,4,5} connected via {0,1},{1,3},{3,4},{3,5},{4,5}
  - Remove 3: remaining {0,1,2,4,5} connected via {0,1},{0,2},{1,2},{2,4},{4,5}
  - Remove 4: remaining {0,1,2,3,5} connected via {0,1},{0,2},{1,2},{1,3},{2,3},{3,5}
  - Remove 5: remaining {0,1,2,3,4} connected via {0,1},{0,2},{1,2},{1,3},{2,3},{2,4},{3,4}
- G' is biconnected. Answer: YES with B=4.
- With B=3: we can only add 3 unit-weight edges. Adding {0,2},{1,3},{2,4} leaves vertex 5 as a pendant -- removing vertex 4 disconnects 5. Answer: NO.
