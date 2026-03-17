---
name: Problem
about: Propose a new problem type
title: "[Model] BoundedComponentSpanningForest"
labels: model
assignees: ''
---

## Motivation

BOUNDED COMPONENT SPANNING FOREST (P86) from Garey & Johnson, A2 ND10. An NP-complete graph partitioning problem that asks whether the vertices of a weighted graph can be grouped into at most K connected components, each with total weight at most B. This problem generalizes both Hamiltonian circuit (when K=1 and the spanning subgraph must be a cycle) and balanced graph partitioning problems. It has direct applications to political redistricting (partitioning precincts into contiguous districts of bounded population).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R31a (HAMILTONIAN CIRCUIT -> BOUNDED COMPONENT SPANNING FOREST) via Garey and Johnson, 1979
- **As target:** R31b (PARTITION INTO PATHS OF LENGTH 2 -> BOUNDED COMPONENT SPANNING FOREST) via Hadlock, 1974

## Definition

<!-- ⚠️ Unverified -->
**Name:** `BoundedComponentSpanningForest`
**Canonical name:** BOUNDED COMPONENT SPANNING FOREST
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND10

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(v) in Z_0^+ for each v in V, positive integers K <= |V| and B.
QUESTION: Can the vertices in V be partitioned into k <= K disjoint sets V_1, V_2, ..., V_k such that, for 1 <= i <= k, the subgraph of G induced by V_i is connected and the sum of the weights of the vertices in V_i does not exceed B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| variables (one per vertex), encoding the component assignment
- **Per-variable domain:** {0, 1, ..., K-1}, indicating which partition set the vertex is assigned to
- **Meaning:** Variable i = j means vertex i is assigned to component V_j. A valid assignment must ensure: (1) each non-empty component induces a connected subgraph, (2) the total weight in each component is at most B, and (3) at most K components are used.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `BoundedComponentSpanningForest`
**Variants:** graph topology (graph type parameter G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `weights` | `Vec<W>` | Non-negative integer weight w(v) for each vertex v |
| `max_components` | `usize` | Upper bound K on the number of components |
| `max_weight` | `W` | Upper bound B on the total weight of each component |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Weight type W should support non-negative integers (i32 or u32).
- Special case: when all weights equal 1, B controls the maximum component size.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** General case requires exponential time. A brute-force approach enumerates all partitions and checks connectivity + weight constraints, giving O(K^n * poly(n)) time. Subset DP approaches yield O(3^n * poly(n)) for the partition step.
- **NP-completeness:** NP-complete (Hadlock, 1974). Transformation from PARTITION INTO PATHS OF LENGTH 2. Also NP-complete via reduction from HAMILTONIAN CIRCUIT when K = |V| - 1 (spanning trees).
- **Special cases:**
  - Polynomial time if G is a tree (Hadlock, 1974).
  - Polynomial time if all weights equal 1 and B = 2 (Hadlock, 1974) — reduces to finding a perfect matching.
  - Remains NP-complete if all weights equal 1 and B is any fixed integer > 2 (Garey and Johnson).
- **References:**
  - F. O. Hadlock (1974). "Minimum spanning forests of bounded trees." *Proceedings of the 5th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 449--460.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weight w(v) in Z_0^+ for each v in V, positive integers K <= |V| and B.
QUESTION: Can the vertices in V be partitioned into k <= K disjoint sets V_1, V_2, ..., V_k such that, for 1 <= i <= k, the subgraph of G induced by V_i is connected and the sum of the weights of the vertices in V_i does not exceed B?

Reference: [Hadlock, 1974]. Transformation from PARTITION INTO PATHS OF LENGTH 2.
Comment: Remains NP-complete even if all weights equal 1 and B is any fixed integer larger than 2 [Garey and Johnson, --]. Can be solved in polynomial time if G is a tree or if all weights equal 1 and B = 2 [Hadlock, 1974].

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all partitions of V into at most K groups, check connectivity and weight constraints.
- [x] It can be solved by reducing to integer programming — binary variables x_{v,j} for vertex v in component j, with connectivity and weight constraints.
- [x] Other: Subset DP over connected subsets; tree decomposition-based algorithms for bounded treewidth graphs.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — valid partition exists):**
Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7}, edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,6}, {6,7}, {0,7}, {1,5}, {2,6}
- Weights: w(0)=2, w(1)=3, w(2)=1, w(3)=2, w(4)=3, w(5)=1, w(6)=2, w(7)=1
- K = 3, B = 6
- Total weight = 15
- Partition: V_1 = {0, 1, 7} (connected via edges {0,1}, {0,7}; weight = 2+3+1 = 6 <= B)
             V_2 = {2, 3, 4} (connected via edges {2,3}, {3,4}; weight = 1+2+3 = 6 <= B)
             V_3 = {5, 6} (connected via edge {5,6}; weight = 1+2 = 3 <= B)
- 3 components <= K=3, all connected, all weights <= B=6
- Answer: YES

**Instance 2 (NO — no valid partition):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5}, edges: {0,1}, {1,2}, {3,4}, {4,5}
- Weights: all w(v) = 1
- K = 2, B = 3
- Note: graph has two connected components {0,1,2} and {3,4,5}, each of size 3 = B
- But to partition into K=2 components with weight <= 3 and connectivity, the only possibility is {0,1,2} and {3,4,5}, which works with weight 3 each. So this IS feasible.
- Revised: same graph but K = 2, B = 2
- Now each component can have at most 2 vertices. Component {0,1,2} must be split, but {0,2} is not connected (no edge). Must use {0,1} and {1,2} — but vertex 1 can only be in one. So {0,1} is connected (weight 2) and {2} alone (weight 1). Similarly {3,4} (weight 2) and {5} alone (weight 1). That gives 4 components > K=2.
- Answer: NO
