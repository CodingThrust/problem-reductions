---
name: Problem
about: Propose a new problem type
title: "[Model] NonTrivialGreatestCommonDivisor"
labels: model
assignees: ''
---

## Motivation

NON-TRIVIAL GREATEST COMMON DIVISOR (P226) from Garey & Johnson, A7 AN7. An NP-hard problem asking whether a set of sparse polynomials has a GCD of degree greater than zero. Due to Plaisted (1977), this problem is not known to be in NP or co-NP but is solvable in pseudo-polynomial time. Remains NP-hard even when restricted to {-1, +1}-coefficient polynomials or when only 2 polynomials are given.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R170: 3SAT -> Non-Trivial Greatest Common Divisor (source: Plaisted, 1977a)

## Definition

**Name:** `NonTrivialGreatestCommonDivisor`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN7

**Mathematical definition:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0.
QUESTION: Does the greatest common divisor of the polynomials sum_{j=1}^{k} a_i[j] * z^{b_i[j]}, 1 <= i <= m, have degree greater than zero?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 0 (this is a pure decision problem; the answer is determined by the input polynomials)
- **Per-variable domain:** N/A
- **Meaning:** The problem asks whether a set of sparse polynomials shares a common polynomial factor of positive degree. There are no configuration variables. A witness for a YES answer would be a common root (a complex number z_0 such that all polynomials vanish at z_0), but verifying this over the complex numbers is non-trivial, which is why the problem is not known to be in NP.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `NonTrivialGreatestCommonDivisor`
**Variants:** none (no type parameters)

| Field | Type | Description |
|-------|------|-------------|
| `polynomials` | `Vec<Vec<(i64, u64)>>` | Sequences A_1, ..., A_m; each A_i is a list of (coefficient, exponent) pairs representing a sparse polynomial |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Solvable in pseudo-polynomial time using standard polynomial GCD algorithms (e.g., Euclidean algorithm for polynomials). The key challenge is that the polynomials are given in sparse representation, so the degree D = max(b_i[j]) can be exponential in the input size. The dense GCD algorithm takes O(D^2) operations, which is pseudo-polynomial. For two polynomials, the subresultant-based GCD algorithm runs in O(D * k^2) where k is the number of nonzero terms. The problem is NP-hard but not known to be in NP or co-NP. The NP-hardness holds even when m = 2 (just two polynomials) or when all coefficients are restricted to {-1, +1}. [Plaisted, JCSS 14:210-221, 1977; Plaisted, FOCS 1976; Plaisted, FOCS 1977]

## Extra Remark

**Full book text:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0.
QUESTION: Does the greatest common divisor of the polynomials sum_{j=1}^{k} a_i[j] * z^{b_i[j]}, 1 <= i <= m, have degree greater than zero?

Reference: [Plaisted, 1977a]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1 [Plaisted, 1976] or if m = 2 [Plaisted, 1977b]. The analogous problem in which the instance also includes a positive integer K, and we are asked if the least common multiple of the given polynomials has degree less than K, is NP-hard under the same restrictions. Both problems can be solved in pseudo-polynomial time using standard algorithms.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Expand the sparse polynomials to dense form and apply the Euclidean GCD algorithm. This is pseudo-polynomial in the maximum exponent. For small instances, direct computation is feasible.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Polynomials (m = 3):
- A_1 = <(1, 3), (-1, 0)> representing f_1(z) = z^3 - 1
- A_2 = <(1, 4), (-1, 1)> representing f_2(z) = z^4 - z = z(z^3 - 1)
- A_3 = <(1, 6), (-1, 3)> representing f_3(z) = z^6 - z^3 = z^3(z^3 - 1)

**Computation:**
- f_1(z) = z^3 - 1 = (z - 1)(z^2 + z + 1)
- f_2(z) = z(z^3 - 1) = z(z - 1)(z^2 + z + 1)
- f_3(z) = z^3(z^3 - 1) = z^3(z - 1)(z^2 + z + 1)

gcd(f_1, f_2, f_3) = z^3 - 1 = (z - 1)(z^2 + z + 1)

Degree of GCD = 3 > 0.

**Answer:** YES -- the polynomials have a non-trivial GCD.

**Counter-example (NO answer):**
- A_1 = <(1, 2), (-1, 0)> representing f_1(z) = z^2 - 1 = (z-1)(z+1)
- A_2 = <(1, 2), (1, 0)> representing f_2(z) = z^2 + 1

gcd(f_1, f_2) = gcd(z^2 - 1, z^2 + 1). Since z^2 - 1 and z^2 + 1 differ by 2, and share no common complex roots (roots of z^2-1 are +/-1; roots of z^2+1 are +/-i), gcd = 1.

Degree of GCD = 0. **Answer:** NO.
