---
name: Problem
about: Propose a new problem type
title: "[Model] GraphPartitioning"
labels: model
assignees: ''
---

## Motivation

GRAPH PARTITIONING (P90) from Garey & Johnson, A2 ND14. A fundamental NP-complete graph optimization problem asking whether the vertices of a weighted graph can be partitioned into groups of bounded total vertex weight such that the total weight of edges crossing between groups is bounded. This captures the classic min-cut balanced partitioning problem that arises in VLSI circuit layout, parallel computing load balancing, and sparse matrix reordering.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R35b (PARTITION INTO TRIANGLES -> GRAPH PARTITIONING) via Hyafil and Rivest, 1973
- **As target:** R35 (MAX 2SAT -> GRAPH PARTITIONING) via Garey, Johnson, and Stockmeyer, 1976

## Definition

<!-- ⚠️ Unverified -->
**Name:** `GraphPartitioning`
**Canonical name:** GRAPH PARTITIONING
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND14

**Mathematical definition:**

INSTANCE: Graph G = (V, E), weights w(v) in Z^+ for each v in V and l(e) in Z^+ for each e in E, positive integers K and J.
QUESTION: Is there a partition of V into disjoint sets V_1, V_2, ..., V_m such that sum_{v in V_i} w(v) <= K for 1 <= i <= m and such that if E' is the set of edges that have their two endpoints in two different sets V_i, then sum_{e in E'} l(e) <= J?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| variables (one per vertex), encoding the partition group assignment
- **Per-variable domain:** {0, 1, ..., m_max - 1} where m_max is the maximum number of groups (bounded by n in the worst case)
- **Meaning:** Variable i = j means vertex i is assigned to partition set V_j. A valid assignment must ensure: (1) the total vertex weight in each group is at most K, and (2) the total weight of edges crossing between different groups is at most J.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `GraphPartitioning`
**Variants:** graph topology (graph type parameter G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `vertex_weights` | `Vec<W>` | Positive integer weight w(v) for each vertex v |
| `edge_weights` | `Vec<W>` | Positive integer weight l(e) for each edge e |
| `max_part_weight` | `W` | Upper bound K on total vertex weight per partition |
| `max_cut_weight` | `W` | Upper bound J on total weight of cross-partition edges |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The number of parts m is not fixed; it is determined by the partition itself (m can range from 1 to |V|).
- When all vertex and edge weights are 1, K bounds the maximum group size and J bounds the number of cut edges.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** General case requires exponential time. A brute-force approach enumerates all partitions of n vertices, checking weight constraints. The Bell number B(n) bounds the number of partitions. Subset DP approaches yield O(3^n * poly(n)) time.
- **NP-completeness:** NP-complete (Hyafil and Rivest, 1973). Transformation from PARTITION INTO TRIANGLES. Remains NP-complete for fixed K >= 3 even if all vertex and edge weights are 1.
- **Polynomial special cases:**
  - K = 2 (bisection): solvable in polynomial time by matching.
  - No polynomial-time approximation within any finite factor for the (k,1)-balanced partition variant unless P = NP.
- **Heuristics:** Kernighan-Lin algorithm (1970), spectral methods, multilevel approaches (METIS, etc.).
- **References:**
  - L. Hyafil and R. L. Rivest (1973). "Graph partitioning and constructing optimal decision trees are polynomial complete problems." IRIA-Laboria Report.
  - M. R. Garey, D. S. Johnson, and L. Stockmeyer (1976). "Some simplified NP-complete graph problems." *Theoretical Computer Science* 1, pp. 237--267.
  - B. W. Kernighan and S. Lin (1970). "An efficient heuristic procedure for partitioning graphs." *Bell System Technical Journal* 49(2), pp. 291--307.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weights w(v) in Z^+ for each v in V and l(e) in Z^+ for each e in E, positive integers K and J.
QUESTION: Is there a partition of V into disjoint sets V_1, V_2, ..., V_m such that sum_{v in V_i} w(v) <= K for 1 <= i <= m and such that if E' is the set of edges that have their two endpoints in two different sets V_i, then sum_{e in E'} l(e) <= J?

Reference: [Hyafil and Rivest, 1973]. Transformation from PARTITION INTO TRIANGLES.
Comment: Remains NP-complete for fixed K >= 3 even if all vertex and edge weights are 1. Can be solved in polynomial time for K = 2 by matching.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all vertex partitions and check both weight constraints.
- [x] It can be solved by reducing to integer programming — binary variables x_{v,j} for vertex v in group j, with vertex weight sum constraints per group and edge cut weight constraint.
- [x] Other: Kernighan-Lin heuristic; spectral partitioning; multilevel methods (METIS).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — valid partition exists):**
Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 10 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,0}, {4,5}, {5,6}, {6,7}, {7,4}, {0,4}, {2,6}
- Vertex weights: all w(v) = 1
- Edge weights: all l(e) = 1
- K = 4 (max group weight), J = 2 (max cut weight)
- Partition: V_1 = {0, 1, 2, 3}, V_2 = {4, 5, 6, 7}
  - V_1 weight = 4 <= K=4, V_2 weight = 4 <= K=4
  - Cut edges: {0,4} and {2,6}, total cut weight = 2 <= J=2
- Answer: YES

**Instance 2 (NO — no valid partition):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5}, complete graph K_6 (15 edges):
- Vertex weights: all w(v) = 1
- Edge weights: all l(e) = 1
- K = 3, J = 2
- Any partition into groups of size <= 3 requires at least 2 groups. With 2 groups of 3 in K_6: cut edges = 3 * 3 = 9 >> J=2. With 3 groups of 2: each pair of groups contributes 2*2=4 cut edges, total = 12 >> J=2.
- In K_6, any non-trivial partition cuts many edges; J=2 is far too small.
- Answer: NO
