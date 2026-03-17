---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Rectilinear Picture Compression"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Rectilinear Picture Compression'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** Rectilinear Picture Compression
**Motivation:** Establishes NP-completeness of RECTILINEAR PICTURE COMPRESSION via polynomial-time reduction from 3SAT. This reduction connects Boolean satisfiability to a geometric covering problem: it shows that determining the minimum number of axis-aligned rectangles needed to exactly cover the 1-entries of a binary matrix is computationally intractable. The result has implications for image compression, DNA array synthesis, integrated circuit manufacture, and access control list minimization.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.232

## GJ Source Entry

> [SR25] RECTILINEAR PICTURE COMPRESSION
> INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K.
> QUESTION: Is there a collection of K or fewer rectangles that covers precisely those entries in M that are 1's, i.e., is there a sequence of quadruples (a_i, b_i, c_i, d_i), 1 <= i <= K, where a_i <= b_i, c_i <= d_i, 1 <= i <= K, such that for every pair (i,j), 1 <= i,j <= n, M_{ij} = 1 if and only if there exists a k, 1 <= k <= K, such that a_k <= i <= b_k and c_k <= j <= d_k?
> Reference: [Masek, 1978]. Transformation from 3SAT.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a binary matrix M and budget K as follows (based on the approach described in Masek's 1978 manuscript):

1. **Variable gadgets:** For each variable x_i, construct a rectangular region in M representing the two possible truth values. The region contains a pattern of 1-entries that can be covered by exactly 2 rectangles in two distinct ways: one way corresponds to setting x_i = TRUE, the other to x_i = FALSE. Each variable gadget occupies a separate row band of the matrix.

2. **Clause gadgets:** For each clause C_j, construct a region that contains 1-entries arranged so that it can be covered by a single rectangle only if at least one of the literal choices from the variable gadgets "aligns" with the clause. Specifically, the clause gadget has 1-entries that extend into the variable gadget regions corresponding to the three literals in C_j. If a variable assignment satisfies a literal in C_j, the corresponding variable gadget's rectangle choice will cover part of the clause gadget; otherwise, an additional rectangle is needed.

3. **Matrix assembly:** The overall matrix M is assembled by placing variable gadgets in distinct row bands and clause gadgets in distinct column bands, with connecting 1-entries that link clauses to their literals. The matrix dimensions are polynomial in n and m.

4. **Budget:** Set K = 2n + m. Each variable requires exactly 2 rectangles (regardless of truth assignment), and each satisfied clause contributes 0 extra rectangles (its 1-entries are already covered by the variable rectangles). An unsatisfied clause would require at least 1 additional rectangle.

5. **Correctness (forward):** If the 3SAT instance is satisfiable, choose rectangle placements in each variable gadget according to the satisfying assignment. Since every clause has at least one satisfied literal, the literal's variable rectangle extends to cover the clause gadget's connecting entries. Total rectangles = 2n + m (at most) since the clause connectors are already covered.

6. **Correctness (reverse):** If K or fewer rectangles cover M, then each variable gadget uses exactly 2 rectangles (which determines a truth assignment), and each clause gadget must be covered without additional rectangles beyond the budget, meaning each clause must be satisfied by at least one literal.

**Time complexity of reduction:** O(poly(n, m)) to construct the matrix M (polynomial in the number of variables and clauses).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_variables` of source 3SAT instance (number of Boolean variables)
- m = `num_clauses` of source 3SAT instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `matrix_rows` | O(`num_variables` * `num_clauses`) |
| `matrix_cols` | O(`num_variables` * `num_clauses`) |
| `budget` | 2 * `num_variables` + `num_clauses` |

**Derivation:** The matrix dimensions are polynomial in n and m; the exact constants depend on the gadget sizes. Each variable gadget contributes a constant-height row band and each clause gadget contributes a constant-width column band, but connecting regions require additional rows/columns proportional to the number of connections. The budget is 2n (two rectangles per variable gadget) plus at most m (one rectangle per clause gadget that can be "absorbed" if the clause is satisfied).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a KSatisfiability(k=3) instance to RectilinearPictureCompression, solve the target by brute-force enumeration of rectangle collections, extract solution, verify on source
- Test with a known satisfiable 3SAT instance and verify the constructed matrix can be covered with 2n + m rectangles
- Test with a known unsatisfiable 3SAT instance and verify 2n + m rectangles are insufficient
- Verify the matrix M has 1-entries only where expected (variable gadgets, clause gadgets, and connecting regions)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability k=3):**
Variables: x_1, x_2, x_3 (n = 3)
Clauses (m = 2):
- C_1: (x_1 v x_2 v ~x_3)
- C_2: (~x_1 v x_2 v x_3)

**Constructed target instance (RectilinearPictureCompression):**
We construct a binary matrix with variable gadgets for x_1, x_2, x_3 and clause gadgets for C_1, C_2.

Schematic layout (simplified):

```
Variable gadgets (row bands):
  x_1 band: rows 1-3    | TRUE choice: rectangles covering cols 1-4, 7-8
                         | FALSE choice: rectangles covering cols 1-2, 5-8
  x_2 band: rows 4-6    | TRUE choice: rectangles covering cols 1-4, 9-10
                         | FALSE choice: rectangles covering cols 1-2, 5-10
  x_3 band: rows 7-9    | TRUE choice: rectangles covering cols 3-6, 9-10
                         | FALSE choice: rectangles covering cols 3-4, 7-10

Clause connectors:
  C_1 connector region: cols 7-8 (x_1 TRUE), cols 9-10 (x_2 TRUE), cols 7-8 (x_3 FALSE)
  C_2 connector region: cols 5-6 (x_1 FALSE), cols 9-10 (x_2 TRUE), cols 9-10 (x_3 TRUE)
```

Budget K = 2(3) + 2 = 8

**Solution mapping:**
Consider the truth assignment: x_1 = TRUE, x_2 = TRUE, x_3 = TRUE.
- C_1: (T v T v F) = TRUE (satisfied by x_1 and x_2)
- C_2: (F v T v T) = TRUE (satisfied by x_2 and x_3)

In the matrix covering:
- x_1 TRUE choice uses 2 rectangles that extend to cover C_1's x_1-connector
- x_2 TRUE choice uses 2 rectangles that extend to cover both C_1's and C_2's x_2-connectors
- x_3 TRUE choice uses 2 rectangles that extend to cover C_2's x_3-connector
- Total: 6 variable rectangles + clause gadgets already covered = 6 + 2 = 8 = K

**Reverse mapping:**
The rectangle placement forces a unique truth assignment per variable gadget. If a clause gadget requires an extra rectangle, the budget is exceeded, proving the formula is unsatisfiable.


## References

- **[Masek, 1978]**: [`Masek1978`] William J. Masek (1978). "Some {NP}-complete set covering problems". Unpublished manuscript, MIT Laboratory for Computer Science.
