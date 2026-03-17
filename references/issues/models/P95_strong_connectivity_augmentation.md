---
name: Problem
about: Propose a new problem type
title: "[Model] StrongConnectivityAugmentation"
labels: model
assignees: ''
---

## Motivation

STRONG CONNECTIVITY AUGMENTATION (P95) from Garey & Johnson, A2 ND19. An NP-complete directed graph augmentation problem studied by Eswaran and Tarjan (1976). Given a directed graph with weighted potential arcs, the goal is to add a minimum-weight set of arcs so that the resulting digraph is strongly connected (every vertex is reachable from every other vertex). The unweighted version is polynomial-time solvable in linear time, but the weighted version is NP-complete. Fundamental to robust communication network design.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R40: HAMILTONIAN CIRCUIT -> STRONG CONNECTIVITY AUGMENTATION (ND19)

## Definition

**Name:** <!-- ⚠️ Unverified --> `StrongConnectivityAugmentation`
**Canonical name:** Strong Connectivity Augmentation (also: Weighted Strong Connectivity Augmentation)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND19

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), weight w(u,v) in Z+ for each ordered pair (u,v) in V x V, positive integer B.
QUESTION: Is there a set A' of ordered pairs of vertices from V such that sum_{a in A'} w(a) <= B and such that the graph G' = (V, A union A') is strongly connected?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n*(n-1) binary variables (one per potential directed arc, i.e., each ordered pair of distinct vertices)
- **Per-variable domain:** binary {0, 1} -- whether arc (u,v) is added to A'
- **Meaning:** variable x_{u,v} = 1 if the arc (u,v) is added to A'. The configuration must satisfy: G' = (V, A union A') is strongly connected and sum(w(u,v) * x_{u,v}) <= B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `StrongConnectivityAugmentation`
**Variants:** graph topology (directed graph type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `DirectedGraph` | The initial directed graph G = (V, A) |
| `arc_weights` | `HashMap<(usize,usize), W>` | Weight w(u,v) for each potential arc |
| `budget` | `W` | Maximum total weight B for added arcs |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The optimization version minimizes sum(w(a)) for a in A' subject to strong connectivity of G union A'.
- Key getter methods: `num_vertices()` (= |V|), `num_arcs()` (= |A|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Eswaran and Tarjan, 1976; transformation from HAMILTONIAN CIRCUIT). Remains NP-complete when all weights are 1 or 2 and A is empty. Solvable in polynomial time when all weights are equal.
- **Best known exact algorithm:** ILP-based exact methods with efficient cut enumeration. Exact algorithms have exponential complexity in O(min(n, alpha)). For the unweighted version, a linear-time algorithm exists based on the condensation DAG of strongly connected components (Eswaran and Tarjan, 1976).
- **Approximation:** 2-approximation via minimum cost branching/arborescence. The (1.5+epsilon)-approximation of Traub and Zenklusen (2023) applies to the related undirected connectivity augmentation; the directed weighted case remains harder to approximate.
- **References:**
  - K.P. Eswaran, R.E. Tarjan (1976). "Augmentation Problems." *SIAM Journal on Computing*, 5(4):653-665.
  - S. Raghavan (2005). "A Note on Eswaran and Tarjan's Algorithm for the Strong Connectivity Augmentation Problem." *The Next Wave in Computing, Optimization, and Decision Technologies*, pp. 19-26.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A), weight w(u,v) in Z+ for each ordered pair (u,v) in V x V, positive integer B.
QUESTION: Is there a set A' of ordered pairs of vertices from V such that sum_{a in A'} w(a) <= B and such that the graph G' = (V,A union A') is strongly connected?

Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete if all weights are either 1 or 2 and A is empty. Can be solved in polynomial time if all weights are equal.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all subsets of potential arcs, check if adding them achieves strong connectivity and total weight <= B.
- [x] It can be solved by reducing to integer programming. Binary variable per potential arc, minimize total weight subject to strong connectivity constraints (flow-based formulation).
- [x] Other: For the unweighted case, linear-time algorithm based on SCC decomposition. The minimum number of arcs to add is max(s, p), where s and p are the number of sources and sinks in the condensation DAG.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Directed graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 5 arcs (a path):**
- Existing arcs: (0->1), (1->2), (2->3), (3->4), (4->5)
- This is a directed path; NOT strongly connected (cannot reach 0 from 5).
- Arc weights for all potential arcs (ordered pairs):
  - (5->0): w=1, (5->1): w=2, (5->2): w=3
  - (4->0): w=2, (4->1): w=3, (3->0): w=2
  - (2->0): w=1, (1->0): w=1
  - (3->1): w=2, (4->2): w=2, (5->3): w=1
  - All other pairs: w=4
- Budget B = 2

**Solution: add A' = {(5->0)} with weight 1 + ... wait, need to check:**
- Adding just (5->0) with w=1: G' has arcs 0->1->2->3->4->5->0, forming a Hamiltonian cycle.
- G' is strongly connected. Total cost = 1 <= B=2. Answer: YES.

**With budget B = 0:**
- Cannot add any arcs. G is not strongly connected. Answer: NO.

**Alternative instance (not a simple path):**
- Existing arcs: (0->1), (1->2), (2->0), (3->4), (4->5), (5->3)
- Two SCCs: {0,1,2} and {3,4,5} with no arcs between them.
- Need at least one arc each direction to connect the two SCCs.
- Adding (2->3) with w=1 and (5->0) with w=1: total cost 2 <= B=2.
- G' is strongly connected. Answer: YES.
