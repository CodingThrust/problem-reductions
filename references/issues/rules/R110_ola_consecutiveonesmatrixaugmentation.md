---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Optimal Linear Arrangement to Consecutive Ones Matrix Augmentation"
labels: rule
assignees: ''
canonical_source_name: 'Optimal Linear Arrangement'
canonical_target_name: 'Consecutive Ones Matrix Augmentation'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Optimal Linear Arrangement
**Target:** Consecutive Ones Matrix Augmentation
**Motivation:** Establishes NP-completeness of CONSECUTIVE ONES MATRIX AUGMENTATION via polynomial-time reduction from OPTIMAL LINEAR ARRANGEMENT (GT42). The reduction encodes a vertex ordering problem as a matrix augmentation problem: given the vertex-edge incidence matrix of the graph, an optimal linear arrangement with low total edge length corresponds to a small number of 0-to-1 flips needed to achieve the consecutive ones property.
<!-- Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.229

## GJ Source Entry

> [SR16] CONSECUTIVE ONES MATRIX AUGMENTATION
> INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
> QUESTION: Is there a matrix A', obtained from A by changing K or fewer 0 entries to 1's, such that A' has the consecutive ones property?
> Reference: [Booth, 1975], [Papadimitriou, 1976a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
> Comment: Variant in which we ask instead that A' have the circular ones property is also NP-complete.

## Reduction Algorithm

<!-- Unverified: AI-generated summary below -->

**Summary:**
Given an OPTIMAL LINEAR ARRANGEMENT instance (G = (V, E), K_OLA), construct a CONSECUTIVE ONES MATRIX AUGMENTATION instance as follows:

Let n = |V| and m = |E|. We build the edge-vertex incidence matrix of G.

1. **Matrix construction:** Construct the m x n binary matrix A where rows correspond to edges and columns correspond to vertices. For edge e_i = {u, v}, set A[i][u] = 1 and A[i][v] = 1, and all other entries in row i to 0. Each row has exactly two 1's.

2. **Bound:** Set K_C1P = K_OLA - m, where m = |E|.

3. **Intuition:** In any column permutation (= vertex ordering f), the two 1's in row i (for edge {u,v}) are at positions f(u) and f(v). To make this row have the consecutive ones property, we must fill in all the 0's between positions f(u) and f(v), requiring |f(u) - f(v)| - 1 flips. The total number of flips across all rows is sum_{{u,v} in E} (|f(u) - f(v)| - 1) = (sum |f(u) - f(v)|) - m. Thus, achieving C1P with at most K_C1P = K_OLA - m flips is equivalent to finding an arrangement with total edge length at most K_OLA.

4. **Correctness (forward):** If G has a linear arrangement f with sum_{{u,v} in E} |f(u) - f(v)| <= K_OLA, then using f as the column permutation and filling gaps within each row requires sum |f(u) - f(v)| - m <= K_OLA - m = K_C1P flips. The resulting matrix has the C1P.

5. **Correctness (reverse):** If matrix A can be augmented to have C1P with at most K_C1P flips, then the column permutation achieving C1P defines a vertex ordering f. For each edge row, the flips needed are |f(u) - f(v)| - 1, so the total edge length is (flips + m) <= K_C1P + m = K_OLA.

**Time complexity of reduction:** O(n * m) to construct the incidence matrix.

## Size Overhead

<!-- Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source OptimalLinearArrangement instance (|V|)
- m = `num_edges` of source OptimalLinearArrangement instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_rows` | `num_edges` |
| `num_cols` | `num_vertices` |
| `bound` | `bound - num_edges` |

**Derivation:** The matrix has one row per edge and one column per vertex. The augmentation bound is the OLA bound minus the number of edges (accounting for the baseline cost of 1 per edge in any arrangement).

## Validation Method

<!-- Unverified: AI-suggested validation -->

- Closed-loop test: reduce an OptimalLinearArrangement instance to ConsecutiveOnesMatrixAugmentation, solve target with BruteForce, extract solution (column permutation + flipped entries), verify on source by reconstructing the linear arrangement.
- Test with path graph (polynomial OLA case): path P_6 with identity arrangement has cost 5 (optimal). Incidence matrix has 5 rows and 6 columns. K_C1P = 5 - 5 = 0. The incidence matrix of a path already has C1P (1's are already consecutive).
- Test with complete graph K_4: 4 vertices, 6 edges. Optimal arrangement cost is known. Verify augmentation bound matches.

## Example

<!-- Unverified: AI-constructed example -->

**Source instance (OptimalLinearArrangement):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: e0={0,1}, e1={1,2}, e2={2,3}, e3={3,4}, e4={4,5}, e5={0,3}, e6={2,5}
- Bound K_OLA = 11

**Constructed target instance (ConsecutiveOnesMatrixAugmentation):**
Matrix A (7 x 6), edge-vertex incidence matrix:
```
       v0 v1 v2 v3 v4 v5
e0:  [  1, 1, 0, 0, 0, 0 ]   (edge {0,1})
e1:  [  0, 1, 1, 0, 0, 0 ]   (edge {1,2})
e2:  [  0, 0, 1, 1, 0, 0 ]   (edge {2,3})
e3:  [  0, 0, 0, 1, 1, 0 ]   (edge {3,4})
e4:  [  0, 0, 0, 0, 1, 1 ]   (edge {4,5})
e5:  [  1, 0, 0, 1, 0, 0 ]   (edge {0,3})
e6:  [  0, 0, 1, 0, 0, 1 ]   (edge {2,5})
```
Bound K_C1P = 11 - 7 = 4

**Solution mapping:**
- Column permutation (arrangement): f(0)=1, f(1)=2, f(2)=3, f(3)=4, f(4)=5, f(5)=6
  (identity ordering: v0, v1, v2, v3, v4, v5)
- With identity ordering, rows e0-e4 already have consecutive 1's (adjacent vertices).
- Row e5 (edge {0,3}): 1's at columns 0 and 3. Need to fill positions 1 and 2. Flips: 2.
- Row e6 (edge {2,5}): 1's at columns 2 and 5. Need to fill positions 3 and 4. Flips: 2.
- Total flips: 0+0+0+0+0+2+2 = 4 = K_C1P. YES.

**Verification:**
- Total edge length: |1-2|+|2-3|+|3-4|+|4-5|+|5-6|+|1-4|+|3-6| = 1+1+1+1+1+3+3 = 11 = K_OLA.
- Total flips = 11 - 7 = 4 = K_C1P. Consistent.


## References

- **[Booth, 1975]**: [`Booth1975`] K. S. Booth (1975). "{PQ} Tree Algorithms". University of California, Berkeley.
- **[Papadimitriou, 1976a]**: [`Papadimitriou1976a`] Christos H. Papadimitriou (1976). "The {NP}-completeness of the bandwidth minimization problem". *Computing* 16, pp. 263-270.
- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237-267.
