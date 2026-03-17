---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Path to Consecutive Ones Submatrix"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Path'
canonical_target_name: 'Consecutive Ones Submatrix'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Hamiltonian Path
**Target:** Consecutive Ones Submatrix
**Motivation:** Establishes NP-completeness of the CONSECUTIVE ONES SUBMATRIX problem via polynomial-time reduction from HAMILTONIAN PATH. The consecutive ones property (C1P) is a fundamental concept in combinatorial matrix theory, with applications in DNA physical mapping, PQ-tree algorithms, and interval graph recognition. While testing whether a full matrix has the C1P can be done in polynomial time (Booth and Lueker, 1976), finding a maximum-column submatrix with this property is NP-hard. The reduction encodes the graph structure into a binary matrix such that selecting K columns with the C1P corresponds to choosing edges forming a Hamiltonian path.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, SR14, p.229

## GJ Source Entry

> [SR14] CONSECUTIVE ONES SUBMATRIX
> INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
> QUESTION: Is there an m x K submatrix B of A that has the "consecutive ones" property, i.e., such that the columns of B can be permuted so that in each row all the 1's occur consecutively?
> Reference: [Booth, 1975]. Transformation from HAMILTONIAN PATH.
> Comment: The variant in which we ask instead that B have the "circular ones" property, i.e., that the columns of B can be permuted so that in each row either all the 1's or all the 0's occur consecutively, is also NP-complete. Both problems can be solved in polynomial time if K = n (in which case we are asking if A has the desired property), e.g., see [Fulkerson and Gross, 1965], [Tucker, 1971], and [Booth and Lueker, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hamiltonian Path instance G = (V, E) with |V| = p vertices and |E| = q edges, construct a Consecutive Ones Submatrix instance as follows. The idea (from Booth, 1975) is to encode the graph's incidence structure into a binary matrix where selecting K = p - 1 columns (edges) that have the consecutive ones property corresponds to finding a Hamiltonian path.

1. **Matrix construction:** Create a binary matrix A with m = p rows (one per vertex) and n = q columns (one per edge). Set a_{i,j} = 1 if vertex v_i is an endpoint of edge e_j, and a_{i,j} = 0 otherwise. This is the vertex-edge incidence matrix of G.

2. **Bound K:** Set K = p - 1 (the number of edges in a Hamiltonian path).

3. **Correctness (forward):** If G has a Hamiltonian path v_{pi(1)} -> v_{pi(2)} -> ... -> v_{pi(p)}, then the p-1 edges of this path form a submatrix B of K = p-1 columns. Order these columns as e_{pi(1),pi(2)}, e_{pi(2),pi(3)}, ..., e_{pi(p-1),pi(p)}. In this column ordering, each row (vertex) v_{pi(k)} has 1's in columns corresponding to edges incident to it on the path, which are the (k-1)-th and k-th columns (or just one column for the endpoints). These 1's are consecutive in this ordering. Thus B has the consecutive ones property.

4. **Correctness (reverse):** If there exists a submatrix B of K = p-1 columns with the consecutive ones property, then B consists of p-1 edges. Under the column permutation that makes all 1's consecutive, each vertex has its incident edges appearing as a consecutive block. Since each edge contributes exactly two 1's (one per endpoint) and the column permutation orders them linearly, this defines a path structure visiting all p vertices -- a Hamiltonian path.

**Key invariant:** A Hamiltonian path on p vertices uses exactly p-1 edges. The incidence matrix of these edges, when columns are ordered by the path traversal, naturally has the consecutive ones property (each vertex's incident path-edges are contiguous). Conversely, p-1 columns with C1P from an incidence matrix define an interval graph structure that must be a path.

**Time complexity of reduction:** O(p * q) to construct the incidence matrix.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- p = `num_vertices` of source Hamiltonian Path instance (|V|)
- q = `num_edges` of source Hamiltonian Path instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_rows` | `num_vertices` |
| `num_cols` | `num_edges` |
| `bound_k` | `num_vertices - 1` |

**Derivation:** The incidence matrix has one row per vertex and one column per edge. The bound K equals p-1 (edges in a Hamiltonian path).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianPath instance to ConsecutiveOnesSubmatrix, solve target with BruteForce (enumerate all (q choose p-1) column subsets, check each for C1P by trying all column permutations), extract solution, verify on source
- Test with known YES instance: path graph P_6 has a trivial Hamiltonian path; verify the incidence matrix has a 5-column submatrix with C1P
- Test with known NO instance: K_4 plus two isolated vertices has no Hamiltonian path; verify no 5-column submatrix with C1P exists in its incidence matrix
- For small instances, verify that the polynomial-time C1P test (Booth-Lueker PQ-tree algorithm) correctly identifies C1P submatrices

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianPath):**
Graph G with 6 vertices {v_1, v_2, v_3, v_4, v_5, v_6} and 8 edges:
- e_1: {v_1,v_2}, e_2: {v_1,v_3}, e_3: {v_2,v_3}, e_4: {v_2,v_4}, e_5: {v_3,v_5}, e_6: {v_4,v_5}, e_7: {v_4,v_6}, e_8: {v_5,v_6}
- Hamiltonian path exists: v_1 -> v_2 -> v_4 -> v_6 -> v_5 -> v_3 (uses edges e_1, e_4, e_7, e_8, e_5)

**Constructed target instance (ConsecutiveOnesSubmatrix):**
Incidence matrix A (6 x 8, rows=vertices, cols=edges):

|       | e_1 | e_2 | e_3 | e_4 | e_5 | e_6 | e_7 | e_8 |
|-------|-----|-----|-----|-----|-----|-----|-----|-----|
| v_1   |  1  |  1  |  0  |  0  |  0  |  0  |  0  |  0  |
| v_2   |  1  |  0  |  1  |  1  |  0  |  0  |  0  |  0  |
| v_3   |  0  |  1  |  1  |  0  |  1  |  0  |  0  |  0  |
| v_4   |  0  |  0  |  0  |  1  |  0  |  1  |  1  |  0  |
| v_5   |  0  |  0  |  0  |  0  |  1  |  1  |  0  |  1  |
| v_6   |  0  |  0  |  0  |  0  |  0  |  0  |  1  |  1  |

Bound K = 5 (= 6 - 1).

**Solution mapping:**
Select the 5 columns corresponding to edges on the Hamiltonian path: e_1, e_4, e_7, e_8, e_5.

Submatrix B (6 x 5) with columns ordered as path traversal [e_1, e_4, e_7, e_8, e_5]:

|       | e_1 | e_4 | e_7 | e_8 | e_5 |
|-------|-----|-----|-----|-----|-----|
| v_1   |  1  |  0  |  0  |  0  |  0  |  (1's at position 1: consecutive)
| v_2   |  1  |  1  |  0  |  0  |  0  |  (1's at positions 1-2: consecutive)
| v_3   |  0  |  0  |  0  |  0  |  1  |  (1's at position 5: consecutive)
| v_4   |  0  |  1  |  1  |  0  |  0  |  (1's at positions 2-3: consecutive)
| v_5   |  0  |  0  |  0  |  1  |  1  |  (1's at positions 4-5: consecutive)
| v_6   |  0  |  0  |  1  |  1  |  0  |  (1's at positions 3-4: consecutive)

Each row has consecutive 1's under the column ordering e_1, e_4, e_7, e_8, e_5.

**Verification:**
- The submatrix B with column permutation [e_1, e_4, e_7, e_8, e_5] has the C1P.
- Reading the path from the interval structure: v_1 covers [1,1], v_2 covers [1,2], v_4 covers [2,3], v_6 covers [3,4], v_5 covers [4,5], v_3 covers [5,5].
- This gives the Hamiltonian path: v_1 -> v_2 -> v_4 -> v_6 -> v_5 -> v_3.


## References

- **[Booth, 1975]**: [`Booth1975`] K. S. Booth (1975). "PQ Tree Algorithms". Ph.D. Thesis, University of California, Berkeley. UCRL-51953.
- **[Fulkerson and Gross, 1965]**: [`Fulkerson1965`] D. R. Fulkerson and D. A. Gross (1965). "Incidence matrices and interval graphs". *Pacific Journal of Mathematics* 15, pp. 835-855.
- **[Tucker, 1971]**: [`Tucker1971`] A. Tucker (1971). "A structure theorem for the consecutive ones property". In: *Proceedings of the 2nd Annual ACM Symposium on Theory of Computing*.
- **[Booth and Lueker, 1976]**: [`Booth1976`] K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using PQ-tree algorithms". *Journal of Computer and System Sciences* 13, pp. 335-379.
