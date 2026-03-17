---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoPathsOfLength2"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO PATHS OF LENGTH 2 (P10) from Garey & Johnson, Chapter 3, Section 3.3, p.76. A classical NP-complete problem useful for reductions. This problem partitions graph vertices into triples, each inducing a path of length 2 (a P3 subgraph). It serves as a building block for proving NP-completeness of graph partitioning problems.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** R31b (PARTITION INTO PATHS OF LENGTH 2 -> BOUNDED COMPONENT SPANNING FOREST) via Hadlock 1974
- **As target:** Reduced from 3-DIMENSIONAL MATCHING (3DM) in GJ Chapter 3

## Definition

<!-- ⚠️ Unverified -->
**Name:** `PartitionIntoPathsOfLength2`
**Canonical name:** PARTITION INTO PATHS OF LENGTH 2
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.3, p.76

**Mathematical definition:**

INSTANCE: Graph G = (V,E), with |V| = 3q for a positive integer q.
QUESTION: Is there a partition of V into q disjoint sets V_1, V_2, ..., V_q of three vertices each so that, for each V_t = {v_{t[1]}, v_{t[2]}, v_{t[3]}}, at least two of the three edges {v_{t[1]}, v_{t[2]}}, {v_{t[1]}, v_{t[3]}}, and {v_{t[2]}, v_{t[3]}} belong to E?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| variables (one per vertex), encoding which partition group each vertex belongs to
- **Per-variable domain:** {0, 1, ..., q-1} where q = |V|/3, indicating the index of the partition set the vertex is assigned to
- **Meaning:** Variable i = j means vertex i is assigned to partition set V_j. A valid assignment must place exactly 3 vertices in each group, and each group must induce at least 2 edges (i.e., a path of length 2 or a triangle).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `PartitionIntoPathsOfLength2`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) with |V| = 3q |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed (the question is purely structural).
- The constraint |V| = 3q (divisible by 3) is a precondition on the input.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** The general problem is NP-complete. No known sub-exponential algorithm for general graphs. A brute-force approach enumerates all partitions of n vertices into groups of 3, giving O(n! / (3!^q * q!)) configurations to check, which is bounded by O(3^n) using the standard set-partition DP approach.
- **NP-completeness:** Proved by reduction from 3-DIMENSIONAL MATCHING (GJ, Theorem 3.5, p.76).
- **Special cases:** The problem of partitioning into paths of length k in bipartite graphs of maximum degree 3 is NP-complete for all k >= 2.
- **References:**
  - M. R. Garey and D. S. Johnson (1979). *Computers and Intractability*, Chapter 3, p.76.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), with |V| = 3q for a positive integer q.
QUESTION: Is there a partition of V into q disjoint sets V_1, V_2, ..., V_q of three vertices each so that, for each V_t = {v_{t[1]}, v_{t[2]}, v_{t[3]}}, at least two of the three edges {v_{t[1]}, v_{t[2]}}, {v_{t[1]}, v_{t[3]}}, and {v_{t[2]}, v_{t[3]}} belong to E?

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all partitions of V into triples and check the path condition.
- [x] It can be solved by reducing to integer programming — assign binary variables x_{v,j} for vertex v in group j, with constraints ensuring each group has exactly 3 vertices and at least 2 internal edges.
- [x] Other: Dynamic programming over subsets in O(3^n * poly(n)) time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — partition exists):**
Graph G with 9 vertices {0, 1, 2, 3, 4, 5, 6, 7, 8} and 10 edges:
- Edges: {0,1}, {1,2}, {3,4}, {4,5}, {6,7}, {7,8}, {0,3}, {2,5}, {3,6}, {5,8}
- q = 3, so we need 3 groups of 3 vertices
- Partition: V_1 = {0, 1, 2}, V_2 = {3, 4, 5}, V_3 = {6, 7, 8}
  - V_1: edges {0,1} and {1,2} present (path 0-1-2) -- 2 edges present
  - V_2: edges {3,4} and {4,5} present (path 3-4-5) -- 2 edges present
  - V_3: edges {6,7} and {7,8} present (path 6-7-8) -- 2 edges present
- Answer: YES

**Instance 2 (NO — no valid partition):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 4 edges:
- Edges: {0,1}, {2,3}, {0,4}, {1,5}
- q = 2, so we need 2 groups of 3 vertices
- Note: vertex 4 and vertex 5 have degree 1 each, vertex 2 and vertex 3 have degree 1 each
- Any group containing both vertices 4 and 5 has at most 1 edge involving them ({0,4} or {1,5}, not both unless 0 or 1 is in the group, but then we need 2 edges in the triple)
- Exhaustive check: no partition of 6 vertices into two triples yields at least 2 edges per triple
- Answer: NO
