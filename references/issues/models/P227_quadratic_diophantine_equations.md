---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticDiophantineEquations"
labels: model
assignees: ''
---

## Motivation

QUADRATIC DIOPHANTINE EQUATIONS (P227) from Garey & Johnson, A7 AN8. An NP-complete number-theoretic problem: given positive integers a, b, c, determine whether there exist positive integers x and y such that ax^2 + by = c. This result by Manders and Adleman (1978) is a landmark in computational complexity, showing that even simple Diophantine equations with just two unknowns and degree two are NP-complete. This contrasts with the polynomial-time solvability of purely linear Diophantine equations (sum of a_i*x_i = c) and pure power equations (a*x^k = c). The general Diophantine problem (arbitrary polynomial, arbitrary many variables) is undecidable (Matiyasevich's theorem, extending MRDP), but restricting to a single nonlinear variable keeps the problem in NP.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** R166 (QUADRATIC DIOPHANTINE EQUATIONS -> SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS)
- **As target:** R171 (3SAT -> QUADRATIC DIOPHANTINE EQUATIONS)

## Definition

**Name:** `QuadraticDiophantineEquations`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN8

**Mathematical definition:**

INSTANCE: Positive integers a, b, and c.
QUESTION: Are there positive integers x and y such that ax^2 + by = c?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 2 (the unknowns x and y)
- **Per-variable domain:**
  - x: {1, 2, ..., floor(sqrt(c/a))} -- since ax^2 <= c, x is bounded by sqrt(c/a)
  - y: {1, 2, ..., floor((c-a)/b)} -- since by <= c - a, y is bounded by (c-a)/b
- **Meaning:** x is the variable appearing quadratically in the equation; y is the linear variable. The pair (x, y) must satisfy ax^2 + by = c exactly.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `QuadraticDiophantineEquations`
**Variants:** none (operates on three positive integers)

| Field | Type | Description |
|-------|------|-------------|
| `a` | `u64` | Coefficient of x^2 |
| `b` | `u64` | Coefficient of y |
| `c` | `u64` | Right-hand side constant |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For each candidate x in {1, ..., floor(sqrt(c/a))}, check if (c - a*x^2) is positive and divisible by b, yielding y = (c - a*x^2)/b. This runs in O(sqrt(c/a) * polylog(c)) time, which is pseudo-polynomial. The problem is NP-complete because c (and hence sqrt(c/a)) can be exponential in the input bit-length. Purely linear Diophantine equations are solvable in polynomial time using the extended Euclidean algorithm. The NP-hardness comes from the interaction of the quadratic term with the linear term and the positivity constraints.

## Extra Remark

**Full book text:**

INSTANCE: Positive integers a, b, and c.
QUESTION: Are there positive integers x and y such that ax^2 + by = c?

Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
Comment: Diophantine equations of the forms ax^k = c and Sigma_{i=1}^{k} a_i*x_i = c are solvable in polynomial time for arbitrary values of k. The general Diophantine problem, "Given a polynomial with integer coefficients in k variables, does it have an integer solution?" is undecidable, even for k = 13 [Matijasevic and Robinson, 1975]. However, the given problem can be generalized considerably (to simultaneous equations in many variables) while remaining in NP, so long as only one variable enters into the equations in a non-linear way (see [Gurari and Ibarra, 1978]).

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate x from 1 to floor(sqrt(c/a)); for each x, check if (c - a*x^2) > 0 and (c - a*x^2) mod b == 0, yielding y = (c - a*x^2)/b.)
- [ ] It can be solved by reducing to integer programming. (ILP with integer variables x, y >= 1 and equality constraint a*x^2 + b*y = c; however, the quadratic term makes this a mixed-integer quadratic program.)
- [ ] Other: The problem can be reduced to Quadratic Congruences (P220) by observing that ax^2 + by = c iff ax^2 ≡ c (mod b) with x bounded by sqrt(c/a).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
a = 3, b = 5, c = 53

**Question:** Are there positive integers x, y with 3x^2 + 5y = 53?

**Solution search:**
- x ranges from 1 to floor(sqrt(53/3)) = floor(sqrt(17.67)) = floor(4.2) = 4.
- x = 1: 3*1 + 5y = 53 => 5y = 50 => y = 10. YES! (x=1, y=10)
- x = 2: 3*4 + 5y = 53 => 5y = 41 => y = 8.2. Not integer.
- x = 3: 3*9 + 5y = 53 => 5y = 26 => y = 5.2. Not integer.
- x = 4: 3*16 + 5y = 53 => 5y = 5 => y = 1. YES! (x=4, y=1)

**Answer:** YES. Two solutions: (x=1, y=10) and (x=4, y=1).

**Verification of (x=4, y=1):** 3*(4^2) + 5*1 = 48 + 5 = 53 = c.

**Negative example:**
a = 3, b = 5, c = 10
- x = 1: 3 + 5y = 10 => 5y = 7 => y = 1.4. Not integer.
- x ranges up to floor(sqrt(10/3)) = floor(1.83) = 1.
- No solution exists. Answer: NO.
