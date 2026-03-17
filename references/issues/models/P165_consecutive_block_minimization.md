---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveBlockMinimization"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE BLOCK MINIMIZATION (P165) from Garey & Johnson, A4 SR17. An NP-complete problem from the domain of storage and retrieval. Given a binary matrix, find a column permutation that minimizes the total number of maximal blocks of consecutive 1's across all rows. This problem arises in information retrieval (consecutive file organization), scheduling, production planning, the glass cutting industry, and data compression.

**Associated rules:**
- R111: Hamiltonian Path -> Consecutive Block Minimization (as target)

## Definition

**Name:** `ConsecutiveBlockMinimization`
**Canonical name:** CONSECUTIVE BLOCK MINIMIZATION
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR17

**Mathematical definition:**

INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a permutation of the columns of A that results in a matrix B having at most K blocks of consecutive 1's? (A block of consecutive 1's in row i is a maximal run of consecutive 1-entries b_{i,j}, b_{i,j+1}, ..., b_{i,j+l} = 1.)

## Variables

<!-- Unverified: AI-inferred variable mapping -->
- **Count:** n variables, one per column, representing the column's position in the permutation.
- **Per-variable domain:** Each column variable takes a value in {1, 2, ..., n}, forming a permutation.
- **Meaning:** The assignment encodes a column permutation pi. A satisfying assignment is a permutation pi such that the resulting matrix B (columns reordered by pi) has at most K maximal blocks of consecutive 1's in total across all rows.

## Schema (data type)

<!-- Unverified: AI-designed schema -->
**Type name:** `ConsecutiveBlockMinimization`
**Variants:** None

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<u8>>` | The m x n binary matrix A (row-major, entries 0 or 1) |
| `bound` | `usize` | The positive integer K (upper bound on total number of 1-blocks) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- A "1-block" is a maximal contiguous run of 1's in a row. The total count is summed over all rows.
- When K equals the number of non-all-zero rows, the problem reduces to testing the consecutive ones property, which is polynomial-time solvable via PQ-trees.

## Complexity

<!-- Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O(n! * m * n) brute-force over all column permutations. For practical instances, ILP formulations and metaheuristics (iterated local search, exponential neighborhood search) are used.
- **NP-completeness:** NP-complete [Kou, 1977]. Transformation from HAMILTONIAN PATH.
- **Approximation:** 1.5-approximation algorithm exists [Dom, Guo, Hueffner, Niedermeier, 2008]. Polynomial-time local-improvement algorithms also known.
- **Circular variant:** The variant where we count blocks allowing wrap-around (i.e., the first and last column are adjacent) is also NP-complete [Booth, 1975].
- **References:**
  - L. T. Kou (1977). "Polynomial complete consecutive information retrieval problems." *SIAM Journal on Computing*, 6(1):67-75.
  - M. Dom, J. Guo, F. Huffner, R. Niedermeier (2008). "Consecutive block minimization is 1.5-approximable." *Information Processing Letters*, 108(3):161-163.
  - K. S. Booth (1975). "PQ Tree Algorithms." Ph.D. thesis, University of California, Berkeley.

## Extra Remark

**Full book text:**

INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a permutation of the columns of A that results in a matrix B having at most K blocks of consecutive 1's, i.e., having at most K entries bij such that bij = 1 and either bi,j+1 = 0 or j = n?
Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
Comment: Remains NP-complete if "j = n" is replaced by "j = n and bi1 = 0" [Booth, 1975]. If K equals the number of rows of A that are not all 0, then these problems are equivalent to testing A for the consecutive ones property or the circular ones property, respectively, and can be solved in polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all n! column permutations and count blocks of consecutive 1's in each row.
- [x] It can be solved by reducing to integer programming -- ILP with binary variables x_{c,p} for column c at position p, and constraints counting 1-blocks.
- [x] Other: Metaheuristics (iterated local search, simulated annealing); 1.5-approximation; polynomial-time local-improvement.

## Example Instance

<!-- Unverified: AI-constructed example -->

**Instance 1 (YES instance):**
Matrix A (3 x 6):
```
Row 0: [1, 0, 1, 0, 1, 0]
Row 1: [0, 1, 0, 1, 0, 1]
Row 2: [1, 1, 0, 0, 1, 1]
```
K = 3

With identity column order, Row 0 has three 1-blocks ({0},{2},{4}), Row 1 has three 1-blocks ({1},{3},{5}), Row 2 has two 1-blocks ({0,1},{4,5}). Total = 3+3+2 = 8 > 3.

Column permutation pi = (0, 2, 4, 1, 3, 5) (reorder columns: even indices first, then odd):
```
Row 0: [1, 1, 1, 0, 0, 0]  -> 1 block
Row 1: [0, 0, 0, 1, 1, 1]  -> 1 block
Row 2: [1, 0, 1, 1, 0, 1]  -> 3 blocks
```
Total = 1+1+3 = 5 > 3.

Try pi = (0, 4, 2, 1, 5, 3):
```
Row 0: [1, 1, 1, 0, 0, 0]  -> 1 block
Row 1: [0, 0, 0, 1, 1, 1]  -> 1 block
Row 2: [1, 1, 0, 1, 1, 0]  -> 2 blocks
```
Total = 1+1+2 = 4 > 3.

Answer: NO (minimum achievable is 4 for this matrix)

**Instance 2 (YES instance with Hamiltonian path connection):**
Matrix A (adjacency matrix of a path graph P_6), 6 x 6:
```
     v0 v1 v2 v3 v4 v5
v0: [ 0, 1, 0, 0, 0, 0]
v1: [ 1, 0, 1, 0, 0, 0]
v2: [ 0, 1, 0, 1, 0, 0]
v3: [ 0, 0, 1, 0, 1, 0]
v4: [ 0, 0, 0, 1, 0, 1]
v5: [ 0, 0, 0, 0, 1, 0]
```
K = 6 (one block per non-zero row)

Identity permutation: each row already has 1's in consecutive positions. Blocks: 1+1+1+1+1+1 = 6 = K.
Answer: YES (the identity permutation achieves consecutive ones, since the adjacency matrix of a path is a band matrix)
