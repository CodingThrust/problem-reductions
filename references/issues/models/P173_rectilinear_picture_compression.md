---
name: Problem
about: Propose a new problem type
title: "[Model] RectilinearPictureCompression"
labels: model
assignees: ''
---

## Motivation

RECTILINEAR PICTURE COMPRESSION (P173) from Garey & Johnson, A4 SR25. A classical NP-complete problem that asks whether a binary matrix can be exactly covered by K or fewer axis-aligned rectangles. The problem arises naturally in image compression (compressing monochromatic bitmap images), DNA array synthesis, integrated circuit manufacturing, and access control list minimization. It connects Boolean satisfiability to geometric covering, serving as a bridge between logic-based and combinatorial optimization problems.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R119: 3SAT -> Rectilinear Picture Compression (this model is the target)

## Definition

**Name:** <!-- ⚠️ Unverified --> `RectilinearPictureCompression`
**Canonical name:** Rectilinear Picture Compression (also: Rectangle Cover, Minimum Rectangle Cover)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR25, p.232

**Mathematical definition:**

INSTANCE: An n x n matrix M of 0's and 1's, and a positive integer K.
QUESTION: Is there a collection of K or fewer rectangles that covers precisely those entries in M that are 1's, i.e., is there a sequence of quadruples (a_i, b_i, c_i, d_i), 1 <= i <= K, where a_i <= b_i, c_i <= d_i, 1 <= i <= K, such that for every pair (i,j), 1 <= i,j <= n, M_{ij} = 1 if and only if there exists a k, 1 <= k <= K, such that a_k <= i <= b_k and c_k <= j <= d_k?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** The number of possible rectangles in an n x n grid is O(n^4) (each rectangle defined by row range [a,b] and column range [c,d]). For the satisfaction formulation, the decision involves selecting which rectangles to include.
- **Per-variable domain:** binary {0, 1} -- whether each candidate rectangle is included in the cover.
- **Meaning:** Variable r_{a,b,c,d} = 1 if the rectangle covering rows a..b and columns c..d is selected. A valid solution requires that the union of selected rectangles equals exactly the set of 1-entries in M (no 0-entry is covered, every 1-entry is covered), and the number of selected rectangles is at most K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `RectilinearPictureCompression`
**Variants:** none (no graph or weight parameters)

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<bool>>` | The n x n binary matrix M; `true` = 1, `false` = 0 |
| `rows` | `usize` | Number of rows n (redundant with matrix.len() but explicit) |
| `cols` | `usize` | Number of columns n (redundant with matrix[0].len() but explicit) |
| `budget` | `usize` | The budget K: maximum number of rectangles allowed |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- The matrix need not be square in general (GJ specifies n x n, but the problem generalizes to m x n).
- Each rectangle is specified as (a, b, c, d) with a <= b and c <= d, covering rows a..b and columns c..d.
- The cover must be exact: no 0-entry may be covered by any rectangle.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Brute-force enumeration over all subsets of maximal all-1 rectangles. The number of maximal rectangles in an n x n matrix is at most O(n^2) (each maximal rectangle is determined by its boundary). The exact algorithm tries all combinations of up to K rectangles from the set of maximal rectangles: O(binom(R, K) * n^2) where R is the number of maximal rectangles. In the worst case, R = O(n^2), giving O(n^{2K} * n^2) time. No significantly better exact algorithm is known.
- **Approximation:** The best known polynomial-time approximation guarantee is O(sqrt(log k)) where k is the number of 1-entries (Applegate et al., 2007). Integer programming formulations provide practical exact solutions for moderate-size instances.
- **NP-completeness:** NP-complete [Masek, 1978], via transformation from 3SAT.
- **References:**
  - W. J. Masek (1978). "Some NP-complete set covering problems." Unpublished manuscript, MIT.
  - D. L. Applegate, G. Calinescu, D. S. Johnson, H. Karloff, K. Ligett, J. Wang (2007). "Compressing rectilinear pictures and minimizing access control lists." SODA 2007.

## Extra Remark

**Full book text:**

INSTANCE: An n x n matrix M of 0's and 1's, and a positive integer K.
QUESTION: Is there a collection of K or fewer rectangles that covers precisely those entries in M that are 1's, i.e., is there a sequence of quadruples (ai,bi,ci,di), 1 <= i <= K, where ai <= bi, ci <= di, 1 <= i <= K, such that for every pair (i,j), 1 <= i,j <= n, Mij = 1 if and only if there exists a k, 1 <= k <= K, such that ak <= i <= bk and ck <= j <= dk?
Reference: [Masek, 1978]. Transformation from 3SAT.

**Connection to Set Cover:** Rectilinear Picture Compression can be viewed as a special case of Set Cover where the universe is the set of 1-entries in M, and the collection consists of all possible axis-aligned rectangles of 1-entries. The constraint that no 0-entry is covered restricts the allowed rectangles to maximal all-1 sub-rectangles.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate subsets of maximal all-1 rectangles, check each subset of size <= K for exact coverage.
- [x] It can be solved by reducing to integer programming -- binary variable for each maximal rectangle; constraint that every 1-entry is covered by at least one rectangle; constraint that no 0-entry is covered; objective/constraint that total rectangles <= K.
- [x] Other: Can be formulated as a weighted set cover problem. Practical heuristics based on greedy rectangle selection and local search are effective for moderate instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has solution):**
Matrix M (6 x 6):
```
1 1 1 0 0 0
1 1 1 0 0 0
0 0 1 1 1 0
0 0 1 1 1 0
0 0 0 0 1 1
0 0 0 0 1 1
```

Budget K = 3

Candidate rectangle cover:
- R1 = (1, 2, 1, 3): rows 1-2, cols 1-3 (covers the top-left 2x3 block of 1's)
- R2 = (3, 4, 3, 5): rows 3-4, cols 3-5 (covers the middle 2x3 block of 1's)
- R3 = (5, 6, 5, 6): rows 5-6, cols 5-6 (covers the bottom-right 2x2 block of 1's)

Verification:
- R1 covers: (1,1),(1,2),(1,3),(2,1),(2,2),(2,3) -- all are 1's in M
- R2 covers: (3,3),(3,4),(3,5),(4,3),(4,4),(4,5) -- all are 1's in M
- R3 covers: (5,5),(5,6),(6,5),(6,6) -- all are 1's in M
- Union of covered entries = all 1-entries in M? Check: M has 1's at rows 1-2 cols 1-3 (6), rows 3-4 cols 3-5 (6), rows 5-6 cols 5-6 (4) = 16 entries. R1+R2+R3 covers 6+6+4 = 16 entries.
- No overlap between rectangles, no 0-entry covered.
- 3 rectangles <= K = 3.

Answer: YES, 3 rectangles suffice.

**Instance 2 (no solution with budget 2):**
Same matrix M as above, but Budget K = 2.

The three blocks of 1's are disjoint (no rectangle can simultaneously cover 1's from two different blocks without also covering 0-entries). Therefore at least 3 rectangles are needed.

Answer: NO, 2 rectangles are insufficient.
