---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Path (for cubic graphs) to Consecutive Ones Matrix Partition"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Path'
canonical_target_name: 'Consecutive Ones Matrix Partition'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Hamiltonian Path (for cubic graphs)
**Target:** Consecutive Ones Matrix Partition
**Motivation:** Establishes NP-completeness of CONSECUTIVE ONES MATRIX PARTITION via polynomial-time reduction from HAMILTONIAN PATH restricted to cubic (3-regular) graphs. This result shows that even the problem of partitioning the rows of a binary matrix into just two groups, each having the consecutive ones property, is NP-hard. The reduction exploits the regularity of cubic graphs: the adjacency-plus-identity matrix A+I of a cubic graph has exactly 4 ones per row, and a Hamiltonian path decomposes the edges into two sets (path edges and non-path edges) that each induce a C1P structure when the columns are appropriately permuted.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, SR15, p.229

## GJ Source Entry

> [SR15] CONSECUTIVE ONES MATRIX PARTITION
> INSTANCE: An m x n matrix A of 0's and 1's.
> QUESTION: Can the rows of A be partitioned into two groups such that the resulting m_1 x n and m_2 x n matrices (m_1 + m_2 = m) each have the consecutive ones property?
> Reference: [Lipsky, 1978]. Transformation from HAMILTONIAN PATH for cubic graphs.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hamiltonian Path instance on a cubic (3-regular) graph G = (V, E) with |V| = p vertices and |E| = 3p/2 edges, construct a Consecutive Ones Matrix Partition instance as follows (following Lipsky, 1978).

1. **Key observation for cubic graphs:** In a cubic graph, every vertex has degree 3. A Hamiltonian path uses p-1 edges and visits all p vertices. Each internal vertex on the path has exactly 2 path-edges and 1 non-path-edge incident to it; each endpoint has 1 path-edge and 2 non-path-edges. The total non-path edges are 3p/2 - (p-1) = p/2 + 1.

2. **Matrix construction:** Construct the vertex-edge incidence matrix A of G. This is a p x (3p/2) binary matrix where a_{i,j} = 1 if vertex v_i is an endpoint of edge e_j. Each row has exactly 3 ones (since G is cubic).

3. **Row partition goal:** We need to partition the rows into two groups such that each group's submatrix has the C1P. The idea is that one group corresponds to path-edge incidences and the other to non-path-edge incidences.

4. **Augmented construction:** The actual Lipsky reduction constructs a matrix from the graph structure such that each row corresponds to a vertex and encodes its adjacency pattern. The columns are ordered and the row partition reflects the decomposition of the cubic graph's edge set into path edges and non-path edges.

5. **Correctness (forward):** If G has a Hamiltonian path, the path edges define a path graph on all p vertices. The incidence matrix restricted to path-edge columns, with columns ordered by path traversal, has the C1P (each vertex's incident path-edges are consecutive). The remaining non-path edges form a matching plus possibly some extra edges on the two endpoints, and their incidence structure also has the C1P under an appropriate column ordering. Partitioning rows based on which edge-type dominates (or using an auxiliary encoding) yields two groups each with C1P.

6. **Correctness (reverse):** If the rows can be partitioned into two groups each with C1P, the interval structure induced by each group constrains the graph structure. For a cubic graph, this constraint forces one group to encode a Hamiltonian path and the other to encode the complementary edge set.

**Key invariant:** The 3-regularity of the source graph is essential -- it ensures each vertex has a fixed, small number of incident edges, which constrains the row partition to reflect a Hamiltonian path decomposition.

**Time complexity of reduction:** O(p^2) to construct the matrix from the cubic graph.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- p = `num_vertices` of source cubic graph (|V|)
- q = `num_edges` = 3p/2 (since the graph is cubic)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_rows` | `num_vertices` |
| `num_cols` | `num_edges` (= 3 * num_vertices / 2) |

**Derivation:** The incidence matrix has one row per vertex and one column per edge. For a cubic graph, q = 3p/2, so both dimensions are linear in p. The actual Lipsky construction may introduce auxiliary rows/columns, but the overhead remains polynomial.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianPath instance (restricted to cubic graph) to ConsecutiveOnesMatrixPartition, solve target with BruteForce (enumerate all 2^m row partitions, check each pair of submatrices for C1P), extract solution, verify on source
- Test with known YES instance: the Petersen graph minus an edge yields a cubic-like structure; alternatively, use the prism graph (3-regular, 6 vertices) which has a Hamiltonian path
- Test with known NO instance: construct a cubic graph known to have no Hamiltonian path and verify no valid row partition exists
- For small cubic graphs (6-10 vertices), verify that Hamiltonicity agrees with partitionability

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianPath on a cubic graph):**
Prism graph (triangular prism) with 6 vertices {v_1, ..., v_6} and 9 edges:
- Triangle 1: e_1:{v_1,v_2}, e_2:{v_2,v_3}, e_3:{v_1,v_3}
- Triangle 2: e_4:{v_4,v_5}, e_5:{v_5,v_6}, e_6:{v_4,v_6}
- Connecting: e_7:{v_1,v_4}, e_8:{v_2,v_5}, e_9:{v_3,v_6}
- Each vertex has degree 3 (cubic graph).
- Hamiltonian path: v_1 -> v_2 -> v_5 -> v_4 -> v_6 -> v_3 (uses edges e_1, e_8, e_4, e_6, e_9)

**Constructed target instance (ConsecutiveOnesMatrixPartition):**
Incidence matrix A (6 x 9, rows=vertices, cols=edges):

|       | e_1 | e_2 | e_3 | e_4 | e_5 | e_6 | e_7 | e_8 | e_9 |
|-------|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| v_1   |  1  |  0  |  1  |  0  |  0  |  0  |  1  |  0  |  0  |
| v_2   |  1  |  1  |  0  |  0  |  0  |  0  |  0  |  1  |  0  |
| v_3   |  0  |  1  |  1  |  0  |  0  |  0  |  0  |  0  |  1  |
| v_4   |  0  |  0  |  0  |  1  |  0  |  1  |  1  |  0  |  0  |
| v_5   |  0  |  0  |  0  |  1  |  1  |  0  |  0  |  1  |  0  |
| v_6   |  0  |  0  |  0  |  0  |  1  |  1  |  0  |  0  |  1  |

**Solution mapping:**
Hamiltonian path: v_1 -> v_2 -> v_5 -> v_4 -> v_6 -> v_3
Path edges: {e_1, e_8, e_4, e_6, e_9} (5 edges)
Non-path edges: {e_2, e_3, e_5, e_7} (4 edges)

Group 1 (path-edge columns ordered by traversal: e_1, e_8, e_4, e_6, e_9):

|       | e_1 | e_8 | e_4 | e_6 | e_9 |
|-------|-----|-----|-----|-----|-----|
| v_1   |  1  |  0  |  0  |  0  |  0  | 1's at [1]: consecutive
| v_2   |  1  |  1  |  0  |  0  |  0  | 1's at [1,2]: consecutive
| v_5   |  0  |  1  |  1  |  0  |  0  | 1's at [2,3]: consecutive
| v_4   |  0  |  0  |  1  |  1  |  0  | 1's at [3,4]: consecutive
| v_6   |  0  |  0  |  0  |  1  |  1  | 1's at [4,5]: consecutive
| v_3   |  0  |  0  |  0  |  0  |  1  | 1's at [5]: consecutive

This group has the C1P under the path-order column permutation.

Group 2 (non-path-edge columns: e_2, e_3, e_5, e_7):

|       | e_7 | e_3 | e_2 | e_5 |
|-------|-----|-----|-----|-----|
| v_1   |  1  |  1  |  0  |  0  | 1's at [1,2]: consecutive
| v_2   |  0  |  0  |  1  |  0  | 1's at [3]: consecutive
| v_3   |  0  |  1  |  1  |  0  | 1's at [2,3]: consecutive
| v_4   |  1  |  0  |  0  |  0  | 1's at [1]: consecutive
| v_5   |  0  |  0  |  0  |  1  | 1's at [4]: consecutive
| v_6   |  0  |  0  |  0  |  1  | 1's at [4]: consecutive

With column ordering [e_7, e_3, e_2, e_5], this group also has the C1P.

**Note:** The above partition is by columns (edges). The actual Lipsky reduction partitions rows (vertices) into two groups, not columns. The example above demonstrates the structural idea; the precise reduction gadgetry may differ from this incidence-matrix sketch.

**Verification:**
- The Hamiltonian path v_1 -> v_2 -> v_5 -> v_4 -> v_6 -> v_3 is valid (all edges exist, all vertices visited once).
- The path-edge incidence submatrix has the C1P.
- The non-path-edge incidence submatrix has the C1P.


## References

- **[Lipsky, 1978]**: [`Lipsky1978`] W. Lipsky, Jr. (1978). "On the structure of some problems related to the consecutive ones property and graph connectivity". Unpublished manuscript / technical report.
