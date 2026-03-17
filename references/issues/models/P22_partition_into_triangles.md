---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoTriangles"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO TRIANGLES (P22) from Garey & Johnson, A1.1 GT11. A classical NP-complete problem central to proving hardness of graph partitioning and covering problems. Each part of the partition must form a complete triangle (K3), making this strictly harder than PARTITION INTO PATHS OF LENGTH 2 (which only requires 2 of 3 edges).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** R35b (PARTITION INTO TRIANGLES -> GRAPH PARTITIONING) via Hyafil and Rivest, 1973
- **As target:** R19 (EXACT COVER BY 3-SETS -> PARTITION INTO TRIANGLES) via Schaefer, 1974 (GJ Theorem 3.7)

## Definition

<!-- ⚠️ Unverified -->
**Name:** `PartitionIntoTriangles`
**Canonical name:** PARTITION INTO TRIANGLES
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT11

**Mathematical definition:**

INSTANCE: Graph G = (V,E), with |V| = 3q for some integer q.
QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q, each containing exactly 3 vertices, such that for each V_i = {u_i, v_i, w_i}, 1 <= i <= q, all three of the edges {u_i,v_i}, {u_i,w_i}, and {v_i,w_i} belong to E?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| variables (one per vertex), encoding the triangle group assignment
- **Per-variable domain:** {0, 1, ..., q-1} where q = |V|/3, indicating which triangle the vertex belongs to
- **Meaning:** Variable i = j means vertex i is in triangle group V_j. A valid assignment places exactly 3 vertices per group, and the 3 vertices must form a triangle (all 3 edges present in E).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `PartitionIntoTriangles`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) with |V| = 3q |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed (purely structural question).
- The constraint |V| divisible by 3 is a precondition on the input.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:**
  - General graphs: O(2^n * poly(n)) via inclusion-exclusion / subset DP.
  - Bounded degree 4: O(1.0222^n) time, linear space (van Rooij, van Kooten Niekerk, Bodlaender, 2011).
  - Bounded degree 3: polynomial time (linear-time algorithm exists).
- **NP-completeness:** NP-complete (Schaefer, 1974). Transformation from EXACT COVER BY 3-SETS (GJ Theorem 3.7). Remains NP-complete on graphs of maximum degree 4.
- **ETH lower bound:** No subexponential-time algorithm on degree-4 graphs unless the Exponential-Time Hypothesis fails.
- **References:**
  - T. J. Schaefer (1974). Cited in Garey & Johnson.
  - J. M. M. van Rooij, M. van Kooten Niekerk, H. L. Bodlaender (2011). "Partition Into Triangles on Bounded Degree Graphs." *Theory of Computing Systems* 51, pp. 687--718.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), with |V| = 3q for some integer q.
QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q, each containing exactly 3 vertices, such that for each V_i = {u_i, v_i, w_i}, 1 <= i <= q, all three of the edges {u_i,v_i}, {u_i,w_i}, and {v_i,w_i} belong to E?
Reference: [Schaefer, 1974]. Transformation from 3DM (see Chapter 3).
Comment: See next problem for a generalization.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all partitions of V into triples and check the triangle condition.
- [x] It can be solved by reducing to integer programming — binary variables x_{v,t} for vertex v in triangle t, constraints enforcing exactly 3 vertices per triangle and all 3 edges present.
- [x] Other: Subset DP in O(3^n * poly(n)); for degree-4 graphs, O(1.0222^n) exact algorithm.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — triangle partition exists):**
Graph G with 9 vertices {0, 1, 2, 3, 4, 5, 6, 7, 8} and 12 edges:
- Edges: {0,1}, {0,2}, {1,2}, {3,4}, {3,5}, {4,5}, {6,7}, {6,8}, {7,8}, {0,3}, {2,6}, {4,7}
- q = 3, so we need 3 triangles covering all vertices
- Partition: V_1 = {0, 1, 2}, V_2 = {3, 4, 5}, V_3 = {6, 7, 8}
  - V_1: {0,1}, {0,2}, {1,2} all present -- triangle
  - V_2: {3,4}, {3,5}, {4,5} all present -- triangle
  - V_3: {6,7}, {6,8}, {7,8} all present -- triangle
- Answer: YES

**Instance 2 (NO — no triangle partition exists):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {3,4}, {3,5}, {4,5}, {1,3}
- q = 2, so we need 2 triangles covering all 6 vertices
- The only triangles in G are: {0,1,2} and {3,4,5}
- Partition {0,1,2}, {3,4,5} works! Wait -- let's make it harder.
- Revised: 6 vertices, 6 edges: {0,1}, {0,2}, {1,2}, {2,3}, {3,4}, {4,5}
- Triangles present: only {0,1,2}
- Remaining vertices {3,4,5}: edges {2,3}, {3,4}, {4,5} -- but {3,5} is missing, so {3,4,5} is not a triangle
- No valid triangle partition exists
- Answer: NO
