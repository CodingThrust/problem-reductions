---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION INTO PATHS OF LENGTH 2 to BOUNDED COMPONENT SPANNING FOREST"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION INTO PATHS OF LENGTH 2'
canonical_target_name: 'BOUNDED COMPONENT SPANNING FOREST'
source_in_codebase: false
target_in_codebase: false
---

**Source:** PARTITION INTO PATHS OF LENGTH 2
**Target:** BOUNDED COMPONENT SPANNING FOREST
<!-- ⚠️ Unverified: AI-generated motivation -->
**Motivation:** Establishes NP-completeness of BOUNDED COMPONENT SPANNING FOREST via polynomial-time reduction from PARTITION INTO PATHS OF LENGTH 2. This is the original reduction by Hadlock (1974) showing that partitioning a graph into small connected components of bounded weight is intractable. The key insight is that a path of length 2 (P3) on 3 vertices is a connected subgraph, so a P3-partition of vertices is exactly a partition into connected components of size 3 with unit weights.
**Reference:** Garey & Johnson, *Computers and Intractability*, ND10, p.208

## GJ Source Entry

> [ND10] BOUNDED COMPONENT SPANNING FOREST
> INSTANCE: Graph G=(V,E), weight w(v)∈Z_0^+ for each v∈V, positive integers K≤|V| and B.
> QUESTION: Can the vertices in V be partitioned into k≤K disjoint sets V_1,V_2,...,V_k such that, for 1≤i≤k, the subgraph of G induced by V_i is connected and the sum of the weights of the vertices in V_i does not exceed B?
> Reference: [Hadlock, 1974]. Transformation from PARTITION INTO PATHS OF LENGTH 2.
> Comment: Remains NP-complete even if all weights equal 1 and B is any fixed integer larger than 2 [Garey and Johnson, ——]. Can be solved in polynomial time if G is a tree or if all weights equal 1 and B=2 [Hadlock, 1974].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION INTO PATHS OF LENGTH 2 instance with graph G = (V, E) where |V| = 3q, construct a BOUNDED COMPONENT SPANNING FOREST instance as follows:

1. **Graph:** Use the same graph G = (V, E).
2. **Vertex weights:** Set w(v) = 1 for all v in V (unit weights).
3. **Parameters:** Set K = q = |V|/3 (exactly q components) and B = 3 (each component has at most 3 vertices).

**Correctness argument:**

**Forward direction:** Suppose G has a partition into paths of length 2: V_1, V_2, ..., V_q where each V_t = {v_{t[1]}, v_{t[2]}, v_{t[3]}} has at least 2 of the 3 possible edges. With at least 2 edges among 3 vertices, the induced subgraph is connected (a path of length 2, i.e., P3, or a triangle). Each component has 3 vertices with unit weights, so the total weight is 3 = B. There are q = K components. Thus the BOUNDED COMPONENT SPANNING FOREST instance is satisfied.

**Backward direction:** Suppose G has a partition into k <= K = q connected components, each with total weight at most B = 3. Since all weights are 1, each component has at most 3 vertices. The total number of vertices is 3q and there are at most q components, so each component has exactly 3 vertices (by pigeonhole). A connected graph on 3 vertices must have at least 2 edges (a path P3 or a triangle K3). For P3: the path itself provides 2 of the 3 possible edges. For K3: all 3 edges are present. Either way, the partition condition for PARTITION INTO PATHS OF LENGTH 2 is satisfied.

**Key invariant:** A connected subgraph on exactly 3 vertices has at least 2 edges, which is precisely the PARTITION INTO PATHS OF LENGTH 2 requirement.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G (must satisfy n = 3q)
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `max_components` (K) | `num_vertices / 3` |
| `max_weight` (B) | `3` |

**Derivation:** The graph is used as-is with unit vertex weights. The parameters K and B are computed directly from n. This is a parameter-setting reduction with no graph modification, so the overhead is O(1) beyond reading the input.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a graph G with |V| = 3q that has a known P3-partition, reduce to BOUNDED COMPONENT SPANNING FOREST with unit weights, K = q, B = 3, solve the target with BruteForce, extract the partition, verify each component is connected with exactly 3 vertices and at least 2 internal edges.
- Negative test: construct a graph with no valid P3-partition (e.g., a star graph K_{1,3q-1} where the center has too high degree to be covered by paths), verify the target instance also has no solution.
- Check parameter values: K = |V|/3, B = 3, all weights = 1.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PartitionIntoPathsOfLength2):**
Graph G with 9 vertices {0, 1, 2, 3, 4, 5, 6, 7, 8} and 11 edges:
- Edges: {0,1}, {1,2}, {0,2}, {3,4}, {4,5}, {6,7}, {7,8}, {1,3}, {2,6}, {5,8}, {0,5}
- |V| = 9 = 3 * 3, so q = 3
- Known P3-partition: V_1 = {0, 1, 2}, V_2 = {3, 4, 5}, V_3 = {6, 7, 8}
  - V_1: edges {0,1}, {1,2}, {0,2} -- 3 edges (triangle), >= 2 required
  - V_2: edges {3,4}, {4,5} -- 2 edges (path 3-4-5), >= 2 required
  - V_3: edges {6,7}, {7,8} -- 2 edges (path 6-7-8), >= 2 required
- Note: edges {1,3}, {2,6}, {5,8}, {0,5} cross between groups (these are "greedy traps" -- choosing a group like {1,3,4} might seem appealing due to edges {1,3} and {3,4}, but then vertex 0 and 2 would need to form groups with remaining vertices)

**Constructed target instance (BoundedComponentSpanningForest):**
- Same graph G with 9 vertices and 11 edges
- Vertex weights: all w(v) = 1
- K = 3 (at most 3 components)
- B = 3 (each component weight at most 3)

**Solution mapping:**
- Partition: V_1 = {0, 1, 2}, V_2 = {3, 4, 5}, V_3 = {6, 7, 8}
  - V_1: connected (triangle {0,1,2}), weight = 1+1+1 = 3 <= B
  - V_2: connected (path 3-4-5), weight = 1+1+1 = 3 <= B
  - V_3: connected (path 6-7-8), weight = 1+1+1 = 3 <= B
  - 3 components <= K = 3
- Reverse verification: each component has exactly 3 vertices and is connected, so each triple has >= 2 internal edges, confirming a valid P3-partition.

**Greedy trap:** Starting with the cross-edge {1,3}, one might try V_1 = {1, 3, 4} (edges {1,3}, {3,4} -- valid path). But then remaining vertices {0, 2, 5, 6, 7, 8} must partition into two connected triples. V_2 = {0, 2, 6}: edges {0,2} and {2,6} -- path, OK. V_3 = {5, 7, 8}: edge {7,8} present, need edge {5,7} or {5,8} -- {5,8} present! Path 5-8-7, OK. So this is also a valid partition, showing the instance has multiple solutions.


## References

- **[Hadlock, 1974]**: [`Hadlock1974`] F. O. Hadlock (1974). "Minimum spanning forests of bounded trees". In: *Proceedings of the 5th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 449–460. Utilitas Mathematica Publishing.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*
