---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveOnesSubmatrix"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE ONES SUBMATRIX (P162) from Garey & Johnson, A4 SR14. A classical NP-complete problem in combinatorial matrix theory. The consecutive ones property (C1P) -- that the columns of a binary matrix can be permuted so that all 1's in each row are contiguous -- is fundamental in computational biology (DNA physical mapping), interval graph recognition, and PQ-tree algorithms. While testing whether a full matrix has the C1P is polynomial (Booth and Lueker, 1976), finding the largest column subset with this property is NP-hard. The problem connects to interval graph theory: a binary matrix has the C1P if and only if the corresponding hypergraph is an interval hypergraph.

**Associated rules:**
- R108: Hamiltonian Path -> Consecutive Ones Submatrix (this model is the target)

## Definition

**Name:** `ConsecutiveOnesSubmatrix`
**Canonical name:** CONSECUTIVE ONES SUBMATRIX
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR14, p.229

**Mathematical definition:**

INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K <= n.
QUESTION: Is there an m x K submatrix B of A (formed by selecting K columns) that has the "consecutive ones" property, i.e., such that the columns of B can be permuted so that in each row all the 1's occur consecutively?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n binary variables (one per column, indicating whether that column is selected) plus a permutation of the selected K columns.
- **Per-variable domain:** Column selection: {0, 1} for each of the n columns. Column ordering: a permutation of the K selected columns.
- **Meaning:** The binary variable c_j = 1 means column j is included in the submatrix B. The permutation pi defines the column ordering of B. The constraint is that exactly K columns are selected and, under permutation pi, every row's 1-entries are contiguous.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ConsecutiveOnesSubmatrix`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<bool>>` | The m x n binary matrix A |
| `num_rows` | `usize` | Number of rows m |
| `num_cols` | `usize` | Number of columns n |
| `bound_k` | `usize` | Required number of columns K for the submatrix |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- The optimization variant (maximize K) is also NP-hard.
- When K = n, the problem reduces to testing whether the full matrix has the C1P, which is solvable in O(m + n + sum of 1-entries) time using PQ-trees (Booth and Lueker, 1976).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Brute-force: enumerate all (n choose K) column subsets, test each for C1P using Booth-Lueker's linear-time PQ-tree algorithm. Total: O(C(n,K) * (m + n)). For general instances, this is exponential.
- **Fixed-parameter tractability:** The problem is FPT when parameterized by the number of columns to delete (n - K). Dom et al. (2014) gave FPT algorithms for several consecutive ones submatrix variants.
- **Approximation:** The problem admits a 2-approximation for the maximization variant on sparse matrices (Tan and Zhang, 2007).
- **NP-completeness:** NP-complete [Booth, 1975], via transformation from HAMILTONIAN PATH. Remains NP-hard for (2,3)-matrices and (3,2)-matrices.
- **Polynomial special case:** K = n: solvable in O(m + n + f) time where f is the number of 1-entries, using PQ-trees [Booth and Lueker, 1976].
- **References:**
  - K. S. Booth (1975). "PQ Tree Algorithms." Ph.D. Thesis, UC Berkeley.
  - K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using PQ-tree algorithms." *JCSS* 13, pp. 335-379.
  - M. Dom, J. Guo, and R. Niedermeier (2014). "Approximation and fixed-parameter algorithms for consecutive ones submatrix problems." *JCSS* 76(4), pp. 291-305.

## Extra Remark

**Full book text:**

INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there an m x K submatrix B of A that has the "consecutive ones" property, i.e., such that the columns of B can be permuted so that in each row all the 1's occur consecutively?
Reference: [Booth, 1975]. Transformation from HAMILTONIAN PATH.
Comment: The variant in which we ask instead that B have the "circular ones" property, i.e., that the columns of B can be permuted so that in each row either all the 1's or all the 0's occur consecutively, is also NP-complete. Both problems can be solved in polynomial time if K = n (in which case we are asking if A has the desired property), e.g., see [Fulkerson and Gross, 1965], [Tucker, 1971], and [Booth and Lueker, 1976].

**Related variants:**
- Circular ones property: same as C1P but allowing wrap-around (also NP-complete for the submatrix selection problem).
- Consecutive ones editing: minimum number of 0->1 or 1->0 flips to make a matrix have the C1P (NP-hard).
- Row deletion variant: minimum number of rows to delete to achieve C1P (NP-hard).

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all (n choose K) column subsets, test each for C1P.
- [x] It can be solved by reducing to integer programming -- binary variable for each column selection, add constraints encoding the C1P (using auxiliary variables for column ordering and interval endpoints).
- [x] Other: FPT algorithms parameterized by n-K (Dom et al., 2014). Heuristic approaches using PQ-trees with branch-and-bound.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has C1P submatrix with K=5):**
Matrix A (6 x 8):

|       | c1 | c2 | c3 | c4 | c5 | c6 | c7 | c8 |
|-------|----|----|----|----|----|----|----|----|
| r1    |  1 |  0 |  0 |  0 |  0 |  1 |  0 |  0 |
| r2    |  1 |  1 |  0 |  0 |  0 |  0 |  0 |  1 |
| r3    |  0 |  0 |  1 |  0 |  0 |  0 |  1 |  0 |
| r4    |  0 |  1 |  1 |  0 |  0 |  0 |  0 |  0 |
| r5    |  0 |  0 |  0 |  1 |  1 |  0 |  0 |  0 |
| r6    |  0 |  0 |  0 |  0 |  1 |  1 |  0 |  0 |

K = 5.

Select columns {c1, c2, c3, c5, c6}. Submatrix B:

|       | c1 | c2 | c3 | c5 | c6 |
|-------|----|----|----|----|-----|
| r1    |  1 |  0 |  0 |  0 |  1 |
| r2    |  1 |  1 |  0 |  0 |  0 |
| r3    |  0 |  0 |  1 |  0 |  0 |
| r4    |  0 |  1 |  1 |  0 |  0 |
| r5    |  0 |  0 |  0 |  1 |  0 |
| r6    |  0 |  0 |  0 |  1 |  1 |

Column permutation: [c6, c1, c2, c3, c5]:

|       | c6 | c1 | c2 | c3 | c5 |
|-------|----|----|----|----|-----|
| r1    |  1 |  1 |  0 |  0 |  0 | 1's at [1,2]: consecutive
| r2    |  0 |  1 |  1 |  0 |  0 | 1's at [2,3]: consecutive
| r3    |  0 |  0 |  0 |  1 |  0 | 1's at [4]: consecutive
| r4    |  0 |  0 |  1 |  1 |  0 | 1's at [3,4]: consecutive
| r5    |  0 |  0 |  0 |  0 |  1 | 1's at [5]: consecutive
| r6    |  1 |  0 |  0 |  0 |  1 | 1's at [1] and [5]: NOT consecutive!

The permutation [c6, c1, c2, c3, c5] does not work for r6. Let's try another selection.

Select columns {c1, c2, c3, c5, c6}. Try permutation [c5, c6, c1, c2, c3]:

|       | c5 | c6 | c1 | c2 | c3 |
|-------|----|----|----|----|-----|
| r1    |  0 |  1 |  1 |  0 |  0 | 1's at [2,3]: consecutive
| r2    |  0 |  0 |  1 |  1 |  0 | 1's at [3,4]: consecutive
| r3    |  0 |  0 |  0 |  0 |  1 | 1's at [5]: consecutive
| r4    |  0 |  0 |  0 |  1 |  1 | 1's at [4,5]: consecutive
| r5    |  1 |  0 |  0 |  0 |  0 | 1's at [1]: consecutive
| r6    |  1 |  1 |  0 |  0 |  0 | 1's at [1,2]: consecutive

All rows have consecutive 1's. Answer: YES.

**Instance 2 (no C1P submatrix with K=4):**
Matrix A (3 x 4), the "Tucker matrix" pattern:

|       | c1 | c2 | c3 | c4 |
|-------|----|----|----|----|
| r1    |  1 |  1 |  0 |  1 |
| r2    |  1 |  0 |  1 |  1 |
| r3    |  0 |  1 |  1 |  0 |

K = 4 (i.e., the full matrix). No column permutation makes all 1's consecutive in every row simultaneously (this is an asteroidal triple / Tucker obstruction). For any permutation of {c1,c2,c3,c4}, at least one row will have non-consecutive 1's. Answer: NO for K=4.

But for K=3, selecting {c1,c2,c3} with order [c2,c1,c3]:

|       | c2 | c1 | c3 |
|-------|----|----|-----|
| r1    |  1 |  1 |  0 | consecutive
| r2    |  0 |  1 |  1 | consecutive
| r3    |  1 |  0 |  1 | NOT consecutive

Try [c1,c3,c2]:

|       | c1 | c3 | c2 |
|-------|----|----|-----|
| r1    |  1 |  0 |  1 | NOT consecutive

Try selecting {c1,c2,c4} with order [c2,c1,c4]:

|       | c2 | c1 | c4 |
|-------|----|----|-----|
| r1    |  1 |  1 |  1 | consecutive
| r2    |  0 |  1 |  1 | consecutive
| r3    |  1 |  0 |  0 | consecutive

Answer: YES for K=3 with columns {c1,c2,c4} under permutation [c2,c1,c4].
