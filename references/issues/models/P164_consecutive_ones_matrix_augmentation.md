---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveOnesMatrixAugmentation"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE ONES MATRIX AUGMENTATION (P164) from Garey & Johnson, A4 SR16. An NP-complete problem from the domain of storage and retrieval, asking whether a binary matrix can be made to have the consecutive ones property (C1P) by changing at most K zeros to ones. It arises in information retrieval, physical mapping of DNA, and sparse matrix compression. The C1P is testable in polynomial time using PQ-trees, but augmenting a matrix to achieve it is NP-complete.

**Associated rules:**
- R110: Optimal Linear Arrangement -> Consecutive Ones Matrix Augmentation (as target)

## Definition

**Name:** `ConsecutiveOnesMatrixAugmentation`
**Canonical name:** CONSECUTIVE ONES MATRIX AUGMENTATION
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR16

**Mathematical definition:**

INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a matrix A', obtained from A by changing K or fewer 0 entries to 1's, such that A' has the consecutive ones property for columns? (That is, there exists a permutation of the columns of A' such that in each row the 1's appear consecutively.)

## Variables

<!-- Unverified: AI-inferred variable mapping -->
- **Count:** The decision variables are: (1) a column permutation (n! possibilities), and (2) which zero entries to flip to one (at most K of the m*n - nnz(A) zero entries).
- **Per-variable domain:** For the column permutation: each column maps to a position in {1, ..., n}. For the augmentation: each zero entry is either flipped (1) or not (0).
- **Meaning:** A satisfying assignment is a set S of zero-entries of A with |S| <= K, and a column permutation pi, such that after flipping S to ones and applying pi, every row has its ones in a contiguous block.

## Schema (data type)

<!-- Unverified: AI-designed schema -->
**Type name:** `ConsecutiveOnesMatrixAugmentation`
**Variants:** None

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<u8>>` | The m x n binary matrix A (row-major, entries 0 or 1) |
| `bound` | `usize` | The positive integer K (max number of 0->1 flips allowed) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The consecutive ones property (C1P) for columns means there exists a column permutation such that in every row the 1-entries are contiguous.
- The variant asking for the circular ones property (1's wrap around) is also NP-complete per GJ.

## Complexity

<!-- Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(2^n) by trying all column permutations (or more precisely, testing C1P for each subset of augmentations). For fixed K, the problem may admit FPT algorithms parameterized by K.
- **NP-completeness:** NP-complete [Booth, 1975], [Papadimitriou, 1976a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
- **Related polynomial results:** Testing whether a matrix already has the C1P (K=0) is solvable in linear time O(m + n + number of ones) using PQ-trees [Booth and Lueker, 1976].
- **Approximation:** Negative results are known: a large class of simple augmentation algorithms cannot find a near-optimum solution [Hochbaum and Levin, 1985].
- **References:**
  - K. S. Booth (1975). "PQ Tree Algorithms." Ph.D. thesis, University of California, Berkeley.
  - K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using PQ-tree algorithms." *J. Computer and System Sciences*, 13:335-379.
  - C. H. Papadimitriou (1976). "The NP-completeness of the bandwidth minimization problem." *Computing*, 16:263-270.

## Extra Remark

**Full book text:**

INSTANCE: An m x n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a matrix A', obtained from A by changing K or fewer 0 entries to 1's, such that A' has the consecutive ones property?
Reference: [Booth, 1975], [Papadimitriou, 1976a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
Comment: Variant in which we ask instead that A' have the circular ones property is also NP-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all column permutations and for each, count the minimum number of 0->1 flips needed to make each row consecutive; check if total <= K.
- [x] It can be solved by reducing to integer programming -- binary variables for column ordering and augmentation decisions, with C1P constraints linearized.
- [x] Other: PQ-tree based approaches can enumerate valid orderings efficiently when the matrix is close to having C1P; branch-and-bound with C1P feasibility tests.

## Example Instance

<!-- Unverified: AI-constructed example -->

**Instance 1 (YES instance):**
Matrix A (4 x 6):
```
Row 0: [1, 0, 1, 0, 0, 0]
Row 1: [0, 1, 0, 1, 0, 0]
Row 2: [0, 0, 1, 0, 1, 0]
Row 3: [0, 0, 0, 1, 0, 1]
```
K = 4

Column permutation: identity (columns already in order 0,1,2,3,4,5).
- Row 0 has 1's at columns 0 and 2 (gap at column 1). Flip A[0][1] = 0 -> 1. Cost: 1.
- Row 1 has 1's at columns 1 and 3 (gap at column 2). Flip A[1][2] = 0 -> 1. Cost: 1.
- Row 2 has 1's at columns 2 and 4 (gap at column 3). Flip A[2][3] = 0 -> 1. Cost: 1.
- Row 3 has 1's at columns 3 and 5 (gap at column 4). Flip A[3][4] = 0 -> 1. Cost: 1.
- Total flips: 4 <= K = 4.

After augmentation:
```
Row 0: [1, 1, 1, 0, 0, 0]
Row 1: [0, 1, 1, 1, 0, 0]
Row 2: [0, 0, 1, 1, 1, 0]
Row 3: [0, 0, 0, 1, 1, 1]
```
This has the consecutive ones property (each row's 1's are contiguous).
Answer: YES

**Instance 2 (NO instance):**
Matrix A (3 x 6):
```
Row 0: [1, 0, 0, 0, 0, 1]
Row 1: [0, 1, 0, 0, 1, 0]
Row 2: [0, 0, 1, 1, 0, 0]
```
K = 0

The matrix does not already have C1P (in any column permutation, rows 0 and 1 cannot both be made consecutive simultaneously with no flips). Since K = 0, no augmentation is allowed.
Answer: NO
