---
name: Problem
about: Propose a new problem type
title: "[Model] SimultaneousDivisibilityOfLinearPolynomials"
labels: model
assignees: ''
---

## Motivation

SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS (P222) from Garey & Johnson, A7 AN3. A remarkable problem at the boundary of decidability: given vectors of coefficients defining pairs of linear polynomials in multiple variables, determine whether there exist positive integer values for the variables such that each "a-polynomial" divides the corresponding "b-polynomial". The problem is not known to be in NP in general (solutions may be doubly exponential in size), but is NP-complete for any fixed number of divisibility constraints n >= 5. The general problem becomes undecidable over rings of integers in real quadratic extensions of the rationals. This result by Lipshitz (1977, 1978) is fundamental to understanding the boundary between decidability and undecidability in number theory.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** (none known in GJ appendix)
- **As target:** R166 (QUADRATIC DIOPHANTINE EQUATIONS -> SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS)

## Definition

**Name:** `SimultaneousDivisibilityOfLinearPolynomials`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN3

**Mathematical definition:**

INSTANCE: Vectors a_i = (a_i[0], . . . , a_i[m]) and b_i = (b_i[0], . . . , b_i[m]), 1 <= i <= n, with positive integer entries.
QUESTION: Do there exist positive integers x_1, x_2, . . . , x_m such that, for 1 <= i <= n, a_i[0] + Sigma_{j=1}^{m} (a_i[j]*x_j) divides b_i[0] + Sigma_{j=1}^{m} (b_i[j]*x_j)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** m (the unknowns x_1, ..., x_m)
- **Per-variable domain:** positive integers {1, 2, 3, ...} -- unbounded in general, though for fixed n the solutions (if they exist) are bounded
- **Meaning:** Each x_j is a positive integer. The m-tuple (x_1, ..., x_m) must simultaneously satisfy n divisibility conditions: for each i, the linear form a_i[0] + sum(a_i[j]*x_j) must divide the linear form b_i[0] + sum(b_i[j]*x_j).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SimultaneousDivisibilityOfLinearPolynomials`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `a_vectors` | `Vec<Vec<u64>>` | Coefficient vectors a_i = (a_i[0], ..., a_i[m]) for the divisor polynomials, 1 <= i <= n |
| `b_vectors` | `Vec<Vec<u64>>` | Coefficient vectors b_i = (b_i[0], ..., b_i[m]) for the dividend polynomials, 1 <= i <= n |

Note: All vectors must have the same length (m+1), where m is the number of unknowns. The number of constraint pairs n = len(a_vectors) = len(b_vectors).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For fixed n (number of constraints), the problem is decidable and in NP (Lipshitz 1978). Solutions, when they exist, can be bounded, and exhaustive search is possible. For fixed n >= 5, the problem is NP-complete. For general n, the problem is decidable (over the integers) but not known to be in NP -- solutions may be doubly exponential in the input size. The decidability proof uses the local-to-global principle: checking solutions in p-adic integers for finitely many primes p. No sub-exponential algorithms are known for the general case.

## Extra Remark

**Full book text:**

INSTANCE: Vectors a_i = (a_i[0], . . . , a_i[m]) and b_i = (b_i[0], . . . , b_i[m]), 1 <= i <= n, with positive integer entries.
QUESTION: Do there exist positive integers x_1, x_2, . . . , x_m such that, for 1 <= i <= n, a_i[0] + Sigma_{j=1}^{m} (a_i[j]*x_j) divides b_i[0] + Sigma_{j=1}^{m} (b_i[j]*x_j)?

Reference: [Lipshitz, 1977], [Lipshitz, 1978]. Transformation from QUADRATIC DIOPHANTINE EQUATIONS.
Comment: Not known to be in NP, but belongs to NP for any fixed n. NP-complete for any fixed n >= 5. General problem is undecidable if the vector entries and the x_j are allowed to range over the ring of "integers" in a real quadratic extension of the rationals. See reference for related decidability and undecidability results.

## How to solve

- [x] It can be solved by (existing) bruteforce. (For fixed n and bounded coefficients, enumerate positive integer tuples (x_1, ..., x_m) up to a computable bound and check all divisibility conditions.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: For fixed n, decidability via local-to-global principle (p-adic methods).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
n = 2 constraints, m = 2 variables (x_1, x_2):

Constraint 1: (1 + x_1) | (6 + 2*x_1 + x_2)
- a_1 = (1, 1, 0), b_1 = (6, 2, 1)

Constraint 2: (2 + x_2) | (10 + x_1 + 3*x_2)
- a_2 = (2, 0, 1), b_2 = (10, 1, 3)

**Question:** Do there exist positive integers x_1, x_2 satisfying both divisibility conditions?

**Solution search:**
- x_1 = 1, x_2 = 1:
  - C1: (1+1) | (6+2+1) => 2 | 9? No.
- x_1 = 2, x_2 = 2:
  - C1: (1+2) | (6+4+2) => 3 | 12? Yes (12/3=4).
  - C2: (2+2) | (10+2+6) => 4 | 18? No.
- x_1 = 2, x_2 = 4:
  - C1: (1+2) | (6+4+4) => 3 | 14? No.
- x_1 = 3, x_2 = 3:
  - C1: (1+3) | (6+6+3) => 4 | 15? No.
- x_1 = 5, x_2 = 2:
  - C1: (1+5) | (6+10+2) => 6 | 18? Yes (18/6=3).
  - C2: (2+2) | (10+5+6) => 4 | 21? No.
- x_1 = 5, x_2 = 6:
  - C1: (1+5) | (6+10+6) => 6 | 22? No.
- x_1 = 2, x_2 = 6:
  - C1: (1+2) | (6+4+6) => 3 | 16? No.
- x_1 = 5, x_2 = 10:
  - C1: (1+5) | (6+10+10) => 6 | 26? No.
- x_1 = 11, x_2 = 2:
  - C1: (1+11) | (6+22+2) => 12 | 30? No.
- x_1 = 11, x_2 = 6:
  - C1: (1+11) | (6+22+6) => 12 | 34? No.
- x_1 = 5, x_2 = 3:
  - C1: (1+5) | (6+10+3) => 6 | 19? No.
- x_1 = 2, x_2 = 8:
  - C1: (1+2) | (6+4+8) => 3 | 18? Yes (18/3=6).
  - C2: (2+8) | (10+2+24) => 10 | 36? No.
- x_1 = 8, x_2 = 2:
  - C1: (1+8) | (6+16+2) => 9 | 24? No.
- x_1 = 2, x_2 = 14:
  - C1: (1+2) | (6+4+14) => 3 | 24? Yes (24/3=8).
  - C2: (2+14) | (10+2+42) => 16 | 54? No.
- x_1 = 8, x_2 = 8:
  - C1: (1+8) | (6+16+8) => 9 | 30? No.
- x_1 = 5, x_2 = 12:
  - C1: (1+5) | (6+10+12) => 6 | 28? No.

Let me try a simpler instance:

**Simpler input:**
n = 2 constraints, m = 1 variable (x_1):

Constraint 1: (1 + x_1) | (5 + 3*x_1)
- a_1 = (1, 1), b_1 = (5, 3)

Constraint 2: (2 + x_1) | (8 + 3*x_1)
- a_2 = (2, 1), b_2 = (8, 3)

**Solution search:**
- Note: (5 + 3*x_1) = 3*(1 + x_1) + 2, so (1 + x_1) | (5 + 3*x_1) iff (1 + x_1) | 2.
  - So 1 + x_1 in {1, 2}, meaning x_1 in {0, 1}. Since x_1 must be positive: x_1 = 1.
- Check C2 for x_1 = 1: (2+1) | (8+3) => 3 | 11? No. FAIL.

Try: a_1 = (1,1), b_1 = (4,2), a_2 = (1,1), b_2 = (6,3):
- C1: (1+x_1) | (4+2*x_1) = 2*(1+x_1) + 2, so need (1+x_1) | 2. x_1 = 1.
- C2: (1+1) | (6+3) => 2 | 9? No.

**Working example:**
n = 2, m = 1:
- C1: (1 + x_1) | (6 + 3*x_1). Note: 6+3*x_1 = 3*(1+x_1) + 3 = 3*(2+x_1). Need (1+x_1) | 3*(2+x_1).
  Since 3*(2+x_1) = 3*(1+x_1) + 3, need (1+x_1) | 3. So 1+x_1 in {1,3}, x_1 in {0,2}. x_1=2.
- C2: (1 + 2*x_1) | (7 + 4*x_1). For x_1=2: (1+4)|(7+8) => 5|15? Yes!

a_1 = (1,1), b_1 = (6,3), a_2 = (1,2), b_2 = (7,4).

**Answer:** YES, x_1 = 2.
**Verification:**
- C1: (1+2) | (6+6) => 3 | 12 = 4. YES.
- C2: (1+4) | (7+8) => 5 | 15 = 3. YES.
