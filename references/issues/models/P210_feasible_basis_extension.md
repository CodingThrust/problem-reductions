---
name: Problem
about: Propose a new problem type
title: "[Model] FeasibleBasisExtension"
labels: model
assignees: ''
---

## Motivation

FEASIBLE BASIS EXTENSION (P210) from Garey & Johnson, A6 MP4. A classical NP-complete problem arising in linear programming theory. The problem asks whether a linear system Ax = b, x >= 0 has a feasible basis containing a prescribed set of columns -- a fundamental question in simplex-method initialization and LP sensitivity analysis. Its NP-completeness (Murty 1972, via reduction from HAMILTONIAN CIRCUIT) shows that even basic structural questions about LP bases can be computationally hard.

**Associated rules:**
- R155: Hamiltonian Circuit -> Feasible Basis Extension (Murty 1972)

## Definition

**Name:** <!-- ⚠️ Unverified --> `FeasibleBasisExtension`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Feasible Basis Extension; also: Basis Extension Problem
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP4

**Mathematical definition:**

INSTANCE: An m x n integer matrix A, m < n, a column vector a-bar of length m, and a subset S of the columns of A with |S| < m.
QUESTION: Is there a feasible basis B for Ax-bar = a-bar, x-bar >= 0, i.e., a nonsingular m x m submatrix B of A such that B^{-1}a-bar >= 0, and such that B contains all the columns in S?

A "feasible basis" B is a set of m column indices such that the corresponding m x m submatrix A_B is nonsingular and the basic solution x_B = A_B^{-1} a-bar satisfies x_B >= 0 (with all non-basic variables set to zero). The extension requirement is that a prescribed subset S of columns must be included in B.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n - |S| binary variables (one per column of A not already in S), deciding which additional columns to include in the basis.
- **Per-variable domain:** binary {0, 1} -- whether column j (not in S) is selected for the basis.
- **Meaning:** The configuration selects m - |S| additional columns from A \ S to form a basis B = S union {selected columns}. The assignment is satisfying if (1) |B| = m, (2) A_B is nonsingular, and (3) A_B^{-1} a-bar >= 0.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `FeasibleBasisExtension`
**Variants:** none (integer matrix with no graph/weight parameterization)

| Field | Type | Description |
|-------|------|-------------|
| `matrix` | `Vec<Vec<i64>>` | The m x n integer matrix A (row-major) |
| `rhs` | `Vec<i64>` | The column vector a-bar of length m |
| `required_columns` | `Vec<usize>` | The subset S of column indices that must appear in the basis |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Key getter methods: `num_rows()` (= m), `num_columns()` (= n), `num_required()` (= |S|).
- The matrix entries are integers (matching the GJ definition).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Murty, 1972; transformation from HAMILTONIAN CIRCUIT).
- **Best known exact algorithm:** Brute-force enumeration of all C(n - |S|, m - |S|) subsets of columns to extend S, checking nonsingularity and nonnegativity of B^{-1}a-bar for each candidate basis. Time: O(C(n - |S|, m - |S|) * m^3) where m^3 is the cost of matrix inversion/solving. No sub-exponential algorithm is known.
- **Special cases:** When |S| = 0 (no required columns), the problem reduces to finding any feasible basis, which is equivalent to Phase I of the simplex method and can be solved in polynomial time. When |S| = m - 1, only one column needs to be chosen, and the problem is solvable in O(n * m^2) time.
- **References:**
  - K.G. Murty (1972). "A fundamental problem in linear inequalities with applications to the travelling salesman problem." *Mathematical Programming* 2(1), pp. 296-308. Original NP-completeness proof via reduction from Hamiltonian Circuit.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** Integer Linear Programming (the feasibility question can be encoded as an ILP).
- **Known special cases:** When |S| = 0, the problem is equivalent to LP Phase I (polynomial). When the matrix A has special structure (e.g., totally unimodular), the problem may be easier.
- **Related problems:** Linear Programming feasibility, Basis Enumeration.

## Extra Remark

**Full book text:**

INSTANCE: An m x n integer matrix A, m < n, a column vector a-bar of length m, and a subset S of the columns of A with |S| < m.
QUESTION: Is there a feasible basis B for Ax-bar = a-bar, x-bar >= 0, i.e., a nonsingular m x m submatrix B of A such that B^{-1}a-bar >= 0, and such that B contains all the columns in S?

Reference: [Murty, 1972]. Transformation from HAMILTONIAN CIRCUIT.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all C(n - |S|, m - |S|) ways to extend S to an m-column set B; for each, check that A_B is nonsingular and A_B^{-1} a-bar >= 0. Total time O(C(n,m) * m^3).
- [x] It can be solved by reducing to integer programming. Introduce binary variables y_j for each column j not in S; add constraint sum(y_j) = m - |S|; encode nonsingularity and nonnegativity of B^{-1}a-bar via big-M constraints (though the encoding is non-trivial).
- [ ] Other: (TBD)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance (satisfiable):**

Consider the system Ax = a-bar, x >= 0 with m = 3 rows and n = 6 columns:

```
A = | 1  0  0  1  1  0 |
    | 0  1  0  1  0  1 |
    | 0  0  1  0  1  1 |
```

a-bar = (2, 3, 1)^T

Required columns: S = {0} (column 0 must be in the basis).

We need to find a 3 x 3 nonsingular submatrix B containing column 0 such that B^{-1} a-bar >= 0.

**Analysis:**
- Try B = {0, 1, 2}: A_B = I_3 (identity). B^{-1} a-bar = (2, 3, 1)^T >= 0. This is a feasible basis containing column 0.
- Answer: YES.

**Instance (unsatisfiable):**

```
A = | 1  1  1 |
    | 2  2  3 |
```

m = 2, n = 3, a-bar = (1, 1)^T, S = {0, 1}.

Required columns S = {0, 1}, so B must be {0, 1} (since |B| = m = 2).
A_B = [[1, 1], [2, 2]], which is singular (columns are linearly dependent).
No feasible basis containing S exists.
Answer: NO.

**Non-trivial instance:**

```
A = | 1  0  2  1  0  3 |
    | 0  1  1  0  2  1 |
    | 1  1  0  2  1  0 |
```

m = 3, n = 6, a-bar = (4, 3, 5)^T, S = {0, 3} (columns 0 and 3 must be in the basis).

Need to pick one more column from {1, 2, 4, 5} to form a 3-column basis B.

- B = {0, 3, 1}: A_B = [[1,1,0],[0,0,1],[1,2,1]]. det = 1*0*1 + 1*1*1 + 0*0*1 - 0*0*1 - 1*1*1 - 1*0*1 = ... Nonsingular. Solve: B^{-1} a-bar. With A_B = [[1,1,0],[0,0,1],[1,2,1]], solving A_B x = (4,3,5)^T gives x_0 = 3, x_3 = 1, x_1 = 2 (by back substitution). Since 3 >= 0, 1 >= 0, 2 >= 0, this is a feasible basis.
- Answer: YES, B = {0, 1, 3} is a feasible basis extension of S.
