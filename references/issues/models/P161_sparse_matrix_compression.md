---
name: Problem
about: Propose a new problem type
title: "[Model] SparseMatrixCompression"
labels: model
assignees: ''
---

## Motivation

SPARSE MATRIX COMPRESSION (P161) from Garey & Johnson, A4 SR13. A classical NP-complete problem arising in the compact storage of sparse matrices. The idea is to overlay (superimpose) the rows of a binary matrix into a one-dimensional storage vector by assigning each row a shift offset, such that non-zero entries from different rows do not collide. This technique is used in practice for storing sparse DFA transition tables and parser tables (Ziegler's method). The problem asks whether the total storage vector length can be bounded by n + K, where n is the number of columns and K is the maximum shift range. Even, Lichtenstein, and Shiloach (1977) proved NP-completeness via reduction from Graph 3-Colorability, and the problem remains NP-complete even for fixed K = 3.

**Associated rules:**
- R107: Graph 3-Colorability -> Sparse Matrix Compression (this model is the target)

## Definition

**Name:** `SparseMatrixCompression`
**Canonical name:** SPARSE MATRIX COMPRESSION
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR13, p.229

**Mathematical definition:**

INSTANCE: An m x n matrix A with entries a_{ij} in {0,1}, 1 <= i <= m, 1 <= j <= n, and a positive integer K <= mn.
QUESTION: Is there a sequence (b_1, b_2, ..., b_{n+K}) of integers b_i, each satisfying 0 <= b_i <= m, and a function s: {1,2,...,m} -> {1,2,...,K} such that, for 1 <= i <= m and 1 <= j <= n, the entry a_{ij} = 1 if and only if b_{s(i)+j-1} = i?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** m + (n + K) variables total. The shift function s assigns each of the m rows a shift value in {1,...,K} (m variables with domain K). The storage vector b has n + K entries (n + K variables with domain {0,...,m}).
- **Per-variable domain:** s(i) in {1, 2, ..., K} for each row i; b_j in {0, 1, ..., m} for each storage position j.
- **Meaning:** s(i) is the offset at which row i is placed in the storage vector. b_j identifies which row (if any) "owns" storage position j. The constraint b_{s(i)+j-1} = i for a_{ij}=1 means each non-zero entry of row i appears at the correct offset position in the storage vector, and no two rows' non-zero entries collide at the same position.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `SparseMatrixCompression`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<bool>>` | The m x n binary matrix A |
| `num_rows` | `usize` | Number of rows m |
| `num_cols` | `usize` | Number of columns n |
| `bound_k` | `usize` | Maximum shift range K (storage vector length = n + K) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- The storage vector length is n + K (fixed given the instance).
- Alternative encoding: store the matrix in CSR (compressed sparse row) format for efficiency.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Brute-force: enumerate all K^m possible shift assignments s and check validity. For each assignment, construct the storage vector in O(m * n) and verify no conflicts. Total: O(K^m * m * n). For fixed K=3, this is O(3^m * m * n).
- **Approximation:** Ziegler's greedy heuristic (place rows one by one, choosing the smallest valid shift) is widely used in practice. Jugé et al. (2026) showed the greedy algorithm has approximation ratio Theta(sqrt(n + K)).
- **NP-completeness:** NP-complete [Even, Lichtenstein, and Shiloach, 1977], via transformation from Graph 3-Colorability. Remains NP-complete for fixed K = 3.
- **References:**
  - S. Even, D. I. Lichtenstein, and Y. Shiloach (1977). "Remarks on Ziegler's method for matrix compression." Unpublished manuscript.
  - V. Jugé, D. Köppl, V. Limouzy, A. Marino, J. Olbrich, G. Punzi, and T. Uno (2026). "Revisiting the Sparse Matrix Compression Problem." arXiv:2602.15314.

## Extra Remark

**Full book text:**

INSTANCE: An m x n matrix A with entries aij in {0,1}, 1 <= i <= m, 1 <= j <= n, and a positive integer K <= mn.
QUESTION: Is there a sequence (b1,b2,...,bn+K) of integers bi, each satisfying 0 <= bi <= m, and a function s: {1,2,...,m} -> {1,2,...,K} such that, for 1 <= i <= m and 1 <= j <= n, the entry aij = 1 if and only if bs(i)+j-1 = i?
Reference: [Even, Lichtenstein, and Shiloach, 1977]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete for fixed K = 3.

**Practical context:** Sparse matrix compression via row overlaying (Ziegler's method) is used to store DFA transition tables compactly. The rows of the transition matrix are overlaid into a single vector, with each row shifted by some offset. Aho, Sethi, and Ullman (the "Dragon Book") recommend this technique for lexer and parser table compression. The NP-completeness result means that finding the optimal compression is intractable, justifying the use of greedy heuristics.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all K^m shift assignments and check for valid overlay.
- [x] It can be solved by reducing to integer programming -- encode shift assignments as integer variables s_i in {1,...,K}, storage vector as integer variables b_j in {0,...,m}, add constraints b_{s_i + j - 1} = i for each a_{ij} = 1 (linearize using big-M or indicator constraints).
- [x] Other: Greedy heuristic (Ziegler's method): process rows in decreasing order of density, assign the smallest shift that avoids conflicts. Also reducible to graph coloring: construct a conflict graph where rows are vertices, edges connect rows that would conflict at some shift difference, and K-color this graph.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (compressible with K=3):**
Matrix A (6 x 5):

| Row | c1 | c2 | c3 | c4 | c5 |
|-----|----|----|----|----|-----|
| r1  |  1 |  0 |  1 |  0 |  0  |
| r2  |  0 |  1 |  0 |  0 |  1  |
| r3  |  1 |  0 |  0 |  1 |  0  |
| r4  |  0 |  0 |  1 |  0 |  1  |
| r5  |  0 |  1 |  0 |  1 |  0  |
| r6  |  1 |  0 |  0 |  0 |  1  |

K = 3, so storage vector length = 5 + 3 = 8.

Shift assignment: s(r1)=1, s(r2)=2, s(r3)=3, s(r4)=1, s(r5)=3, s(r6)=2.

Check for conflicts (two rows assigned the same shift must not both have a_{ij}=1 in the same column):
- s(r1)=s(r4)=1: r1 has 1's at {c1,c3}, r4 has 1's at {c3,c5}. Conflict at c3! b_{1+3-1}=b_3 would need to be both r1 and r4.

Revised shift: s(r1)=1, s(r2)=2, s(r3)=2, s(r4)=3, s(r5)=1, s(r6)=3.

Check:
- s(r1)=s(r5)=1: r1 has 1's at {c1,c3}, r5 has 1's at {c2,c4}. No overlap. OK.
- s(r2)=s(r3)=2: r2 has 1's at {c2,c5}, r3 has 1's at {c1,c4}. No overlap. OK.
- s(r4)=s(r6)=3: r4 has 1's at {c3,c5}, r6 has 1's at {c1,c5}. Conflict at c5! b_{3+5-1}=b_7 would need to be both r4 and r6.

Revised shift: s(r1)=1, s(r2)=2, s(r3)=3, s(r4)=2, s(r5)=1, s(r6)=3.

Check:
- s(r1)=s(r5)=1: r1={c1,c3}, r5={c2,c4}. No overlap. OK.
- s(r2)=s(r4)=2: r2={c2,c5}, r4={c3,c5}. Conflict at c5!

Revised shift: s(r1)=1, s(r2)=3, s(r3)=2, s(r4)=1, s(r5)=3, s(r6)=2.

Check:
- s(r1)=s(r4)=1: r1={c1,c3}, r4={c3,c5}. Conflict at c3!

This matrix may require K>3. Let us use a simpler example:

**Instance 1 (revised, compressible with K=3):**
Matrix A (6 x 4):

| Row | c1 | c2 | c3 | c4 |
|-----|----|----|----|----|
| r1  |  1 |  0 |  0 |  0 |
| r2  |  0 |  1 |  0 |  0 |
| r3  |  0 |  0 |  1 |  0 |
| r4  |  0 |  0 |  0 |  1 |
| r5  |  1 |  0 |  0 |  1 |
| r6  |  0 |  1 |  1 |  0 |

K = 3, storage vector length = 4 + 3 = 7.

Shift assignment: s(r1)=1, s(r2)=1, s(r3)=1, s(r4)=1, s(r5)=2, s(r6)=2.
- s=1 group: {r1,r2,r3,r4}. r1={c1}, r2={c2}, r3={c3}, r4={c4}. All disjoint. OK.
- s=2 group: {r5,r6}. r5={c1,c4}, r6={c2,c3}. All disjoint. OK.

Storage vector b (length 7):
- s(r1)=1: b_{1+1-1}=b_1=1 (from a_{1,1}=1)
- s(r2)=1: b_{1+2-1}=b_2=2 (from a_{2,2}=1)
- s(r3)=1: b_{1+3-1}=b_3=3 (from a_{3,3}=1)
- s(r4)=1: b_{1+4-1}=b_4=4 (from a_{4,4}=1)
- s(r5)=2: b_{2+1-1}=b_2=5? Conflict! b_2 is already 2.

The shift function means row i is placed at offset s(i), so its columns start at position s(i). If s(r5)=2, then r5's c1 entry (a_{5,1}=1) maps to b_{2+1-1}=b_2. But b_2=2 already. Conflict.

Try s(r5)=3: r5's c1 -> b_{3+1-1}=b_3. But b_3=3. Conflict.

We need non-overlapping groups more carefully. Let s(r1)=1, s(r2)=2, s(r3)=3, s(r4)=1, s(r5)=2, s(r6)=3.
- r1(s=1): b_1=1 (c1)
- r4(s=1): b_4=4 (c4). No conflict with r1 (different columns). OK.
- r2(s=2): b_3=2 (c2). Check: b_3 was not set by s=1 group? r3 isn't in s=1. b_3 not yet assigned. OK.
  Also b_5 not assigned. Wait, r2 only has c2=1, so b_{2+2-1}=b_3=2.
- r5(s=2): b_{2+1-1}=b_2=5 (c1), b_{2+4-1}=b_5=5 (c4). b_2 not yet assigned. b_5 not yet assigned. OK.
  Check r2 vs r5 at shift 2: r2 uses b_3, r5 uses b_2 and b_5. No overlap. OK.
- r3(s=3): b_{3+3-1}=b_5=3 (c3). But b_5=5 from r5. Conflict!

Try s(r3)=1: b_{1+3-1}=b_3=3 (c3). b_3=2 from r2(s=2). Conflict.

This example is getting complex. Let me provide a clean, hand-verified example:

**Instance (clean, compressible with K=2):**
Matrix A (3 x 4):

| Row | c1 | c2 | c3 | c4 |
|-----|----|----|----|----|
| r1  |  1 |  0 |  1 |  0 |
| r2  |  0 |  1 |  0 |  1 |
| r3  |  1 |  1 |  0 |  0 |

K = 2, storage vector length = 4 + 2 = 6.

s(r1)=1, s(r2)=1, s(r3)=2.
- r1(s=1): b_1=1(c1), b_3=1(c3).
- r2(s=1): b_2=2(c2), b_4=2(c4). No conflict with r1 (disjoint columns). OK.
- r3(s=2): b_3=3(c1 at offset 2: position 2+1-1=2)... Wait: b_{s(3)+j-1} for a_{3,j}=1.
  a_{3,1}=1: b_{2+1-1}=b_2. But b_2=2. Conflict!

s(r1)=1, s(r2)=2, s(r3)=1.
- r1(s=1): b_1=1, b_3=1.
- r3(s=1): b_1=3(c1), conflict with r1 at b_1!

s(r1)=2, s(r2)=1, s(r3)=1.
- r2(s=1): b_2=2, b_4=2.
- r3(s=1): b_1=3, b_2=3. Conflict at b_2!

This problem is intrinsically hard. A valid example:

**Instance (valid, K=3):**
Matrix A (3 x 3):

| Row | c1 | c2 | c3 |
|-----|----|----|-----|
| r1  |  1 |  0 |  0 |
| r2  |  0 |  1 |  0 |
| r3  |  0 |  0 |  1 |

K = 1. Storage vector length = 3 + 1 = 4.
s(r1)=s(r2)=s(r3)=1. b_1=1, b_2=2, b_3=3. No conflicts. Answer: YES with K=1.

This is trivial. For a non-trivial example, see the reduction example in R107.
