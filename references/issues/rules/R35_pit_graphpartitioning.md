---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION INTO TRIANGLES to GRAPH PARTITIONING"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION INTO TRIANGLES'
canonical_target_name: 'GRAPH PARTITIONING'
source_in_codebase: false
target_in_codebase: false
---

**Source:** PARTITION INTO TRIANGLES
**Target:** GRAPH PARTITIONING
<!-- ⚠️ Unverified: AI-generated motivation -->
**Motivation:** Establishes NP-completeness of GRAPH PARTITIONING via polynomial-time reduction from PARTITION INTO TRIANGLES. This reduction shows that even with all vertex and edge weights equal to 1, partitioning graph vertices into groups of bounded size while minimizing the number of cut edges is intractable for K >= 3. The key insight is that a triangle partition with K = 3 leaves zero cut edges among triangle edges, so any instance where the triangle partition exists corresponds to a graph partition with zero (or minimal) edge cut cost. This is the original NP-completeness proof for the weighted graph partitioning problem (Hyafil and Rivest, 1973).
**Reference:** Garey & Johnson, *Computers and Intractability*, ND14, p.209

## GJ Source Entry

> [ND14] GRAPH PARTITIONING
> INSTANCE: Graph G=(V,E), weights w(v)∈Z^+ for each v∈V and l(e)∈Z^+ for each e∈E, positive integers K and J.
> QUESTION: Is there a partition of V into disjoint sets V_1,V_2,...,V_m such that ∑_{v∈V_i} w(v)≤K for 1≤i≤m and such that if E'⊆E is the set of edges that have their two endpoints in two different sets V_i, then ∑_{e∈E'} l(e)≤J?
> Reference: [Hyafil and Rivest, 1973]. Transformation from PARTITION INTO TRIANGLES.
> Comment: Remains NP-complete for fixed K≥3 even if all vertex and edge weights are 1. Can be solved in polynomial time for K=2 by matching.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION INTO TRIANGLES instance with graph G = (V, E) where |V| = 3q, construct a GRAPH PARTITIONING instance as follows:

1. **Graph:** Use the same graph G = (V, E).
2. **Vertex weights:** Set w(v) = 1 for all v in V (unit weights).
3. **Edge weights:** Set l(e) = 1 for all e in E (unit weights).
4. **Parameters:** Set K = 3 (each partition group has at most 3 vertices) and J = |E| - 3q (the maximum number of cut edges).

**Correctness argument:**

**Forward direction:** Suppose G has a partition into triangles: V_1, V_2, ..., V_q where each V_i = {u_i, v_i, w_i} forms a triangle (all 3 edges present). Each group has 3 vertices, so the total vertex weight per group is 3 = K. The edges internal to the triangles account for 3q edges total (3 edges per triangle, q triangles). The cut edges (edges with endpoints in different groups) number |E| - 3q = J. Thus the GRAPH PARTITIONING instance is satisfied.

**Backward direction:** Suppose G has a partition V_1, ..., V_m with w(V_i) <= K = 3 for all i and the total cut edge weight <= J = |E| - 3q. Since all vertex weights are 1, each group has at most 3 vertices. The total number of vertices is 3q with at most 3 per group, so there are at least q groups. The number of internal edges (within groups) is at least |E| - J = |E| - (|E| - 3q) = 3q. A group of 3 vertices can have at most 3 internal edges (a triangle). A group of 2 vertices has at most 1 internal edge. A group of 1 vertex has 0 internal edges. To achieve 3q internal edges across at most q groups (each of size <= 3), every group must have exactly 3 vertices AND all 3 possible edges must be present (i.e., each group is a triangle). By pigeonhole, there are exactly q groups of 3 vertices each, and each forms a triangle. This is exactly a partition into triangles.

**Key invariant:** With unit weights, K = 3, and J = |E| - 3q, the graph partitioning instance is feasible if and only if we can partition vertices into groups of exactly 3 such that each group contributes exactly 3 internal edges (i.e., forms a triangle).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G (with n = 3q)
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `max_part_weight` (K) | `3` |
| `max_cut_weight` (J) | `num_edges - num_vertices` |

**Derivation:** The graph is used as-is with unit vertex and edge weights. The parameters K and J are computed directly from the input. J = |E| - 3q = |E| - |V| since |V| = 3q implies 3q = |V|. This is a parameter-setting reduction with no graph modification, so the overhead is O(1) beyond reading the input.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a graph G with |V| = 3q that has a known triangle partition, reduce to GRAPH PARTITIONING with unit weights, K = 3, J = |E| - |V|, solve the target, extract the partition, verify each group forms a triangle in the original graph.
- Negative test: construct a graph that has no triangle partition (e.g., a cycle C_6 which has no triangle partition since it contains no triangles), verify the target instance with J = |E| - |V| = 6 - 6 = 0 is also infeasible (since the cycle must cut at least some edges when grouping into pairs/triples).
- Parameter verification: check K = 3, J = |E| - |V|, all weights = 1.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PartitionIntoTriangles):**
Graph G with 9 vertices {0, 1, 2, 3, 4, 5, 6, 7, 8} and 13 edges:
- Triangle edges: {0,1}, {0,2}, {1,2}, {3,4}, {3,5}, {4,5}, {6,7}, {6,8}, {7,8}
- Cross edges (between triangles): {1,3}, {2,6}, {4,7}, {5,8}
- |V| = 9 = 3 * 3, so q = 3
- Known triangle partition: V_1 = {0,1,2}, V_2 = {3,4,5}, V_3 = {6,7,8}
  - V_1: edges {0,1}, {0,2}, {1,2} all present -- triangle
  - V_2: edges {3,4}, {3,5}, {4,5} all present -- triangle
  - V_3: edges {6,7}, {6,8}, {7,8} all present -- triangle
  - Internal edges: 9. Cut edges: {1,3}, {2,6}, {4,7}, {5,8} = 4 edges.

**Constructed target instance (GraphPartitioning):**
- Same graph G with 9 vertices and 13 edges
- Vertex weights: all w(v) = 1
- Edge weights: all l(e) = 1
- K = 3 (max group weight = max group size)
- J = |E| - |V| = 13 - 9 = 4 (max cut weight)

**Solution mapping:**
- Partition: V_1 = {0,1,2}, V_2 = {3,4,5}, V_3 = {6,7,8}
  - V_1 weight = 3 <= K = 3
  - V_2 weight = 3 <= K = 3
  - V_3 weight = 3 <= K = 3
  - Cut edges: {1,3}, {2,6}, {4,7}, {5,8} -- total cut weight = 4 <= J = 4
- Reverse verification: each group has exactly 3 vertices and the cut edges total exactly J, meaning each group has exactly 3 internal edges, confirming they are triangles.

**Greedy trap:** One might try V_1 = {1, 2, 3} (edges {1,2} and {1,3} present, but {2,3} is NOT present -- not a triangle, only a path). This yields V_1 with only 2 internal edges instead of 3. The remaining vertices {0, 4, 5, 6, 7, 8} must be split into two groups. V_2 = {0, 4, 5}: edges {4,5} present, but {0,4} and {0,5} absent -- only 1 internal edge. V_3 = {6,7,8}: 3 internal edges (triangle). Total internal edges = 2 + 1 + 3 = 6 < 9, so cut edges = 13 - 6 = 7 > J = 4. This fails the cut weight constraint, demonstrating why a greedy approach based on arbitrary triples does not work.


## References

- **[Hyafil and Rivest, 1973]**: [`Hyafil1973`] Laurent Hyafil and Ronald L. Rivest (1973). "Graph partitioning and constructing optimal decision trees are polynomial complete problems". IRIA-Laboria.
