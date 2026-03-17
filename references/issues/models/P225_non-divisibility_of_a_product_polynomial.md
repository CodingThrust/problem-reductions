---
name: Problem
about: Propose a new problem type
title: "[Model] NonDivisibilityOfAProductPolynomial"
labels: model
assignees: ''
---

## Motivation

NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL (P225) from Garey & Johnson, A7 AN6. An NP-complete problem in sparse polynomial arithmetic, established by Plaisted (1977). Asks whether a product of sparse polynomials is NOT divisible by z^N - 1. This problem is significant because membership in NP is itself non-trivial (proven in Plaisted's second 1977 paper), and the problem provides a bridge between Boolean satisfiability and algebraic divisibility.

**Associated rules:**
<!-- ⚠️ Unverified: AI-collected rule associations -->
- R169: 3SAT -> Non-Divisibility of a Product Polynomial (source: Plaisted, 1977a/1977b)

## Definition

**Name:** `NonDivisibilityProductPolynomial`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN6

**Mathematical definition:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0, and an integer N.
QUESTION: Is product_{i=1}^{m} (sum_{j=1}^{k} a_i[j] * z^{b_i[j]}) not divisible by z^N - 1?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 0 (this is a pure decision problem; the answer is determined by the input polynomials and modulus N)
- **Per-variable domain:** N/A
- **Meaning:** The problem asks a divisibility question about a fixed product of polynomials. There are no configuration variables to search over in the standard problem formulation. However, the NP membership proof (Plaisted, 1977b) uses a witness: a root of unity at which the product polynomial does not vanish, certifying non-divisibility.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `NonDivisibilityProductPolynomial`
**Variants:** none (no type parameters)

| Field | Type | Description |
|-------|------|-------------|
| `polynomials` | `Vec<Vec<(i64, u64)>>` | Sequences A_1, ..., A_m; each A_i is a list of (coefficient, exponent) pairs representing a sparse polynomial |
| `modulus_n` | `u64` | The integer N; the question is about divisibility by z^N - 1 |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete (Plaisted, 1977a/1977b). The naive approach is to evaluate the product polynomial at all N-th roots of unity (the polynomial z^N - 1 divides the product iff the product vanishes at all N-th roots of unity). This takes O(N * m * k) arithmetic operations on complex numbers, which is pseudo-polynomial in N. For the decision problem, a brute-force approach checks all N roots of unity, each evaluation taking O(m * k) time. No strongly polynomial algorithm is known. The NP certificate is a specific N-th root of unity at which the product does not vanish. [Plaisted, JCSS 14:210-221, 1977; Plaisted, FOCS 1977, pp. 241-253]

## Extra Remark

**Full book text:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0, and an integer N.
QUESTION: Is product_{i=1}^{m} (sum_{j=1}^{k} a_i[j] * z^{b_i[j]}) not divisible by z^N - 1?

Reference: [Plaisted, 1977a], [Plaisted, 1977b]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
Comment: The related problem in which we are given two sequences <a_1, a_2, . . . , a_m> and <b_1, b_2, . . . , b_n> of positive integers and are asked whether product_{i=1}^{m} (z^{a_i} - 1) does not divide product_{j=1}^{n} (z^{b_j} - 1) is also NP-complete [Plaisted, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Evaluate the product polynomial at each N-th root of unity (pseudo-polynomial in N). Non-divisibility holds iff at least one root yields a nonzero value.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
- Polynomials (m = 2):
  - A_1 = <(1, 2), (-1, 0)> representing f_1(z) = z^2 - 1
  - A_2 = <(1, 1), (1, 0)> representing f_2(z) = z + 1
- N = 4

**Computation:**
Product polynomial: P(z) = (z^2 - 1)(z + 1) = z^3 + z^2 - z - 1

The 4th roots of unity are: {1, i, -1, -i}.
- P(1) = 1 + 1 - 1 - 1 = 0
- P(i) = -i - 1 - i - 1 = -2 - 2i != 0
- P(-1) = -1 + 1 + 1 - 1 = 0
- P(-i) = i - 1 + i - 1 = -2 + 2i != 0

Since P(z) does NOT vanish at all 4th roots of unity (specifically P(i) != 0), z^4 - 1 does NOT divide P(z).

**Answer:** YES -- the product polynomial is NOT divisible by z^4 - 1.

**Verification:** z^4 - 1 = (z-1)(z+1)(z^2+1). P(z) = (z^2-1)(z+1) = (z-1)(z+1)^2. Since z^2+1 does not divide (z-1)(z+1)^2, z^4 - 1 does not divide P(z). Confirmed.

**Counter-example (NO answer):**
- A_1 = <(1, 2), (-1, 0)> representing z^2 - 1
- A_2 = <(1, 2), (1, 0)> representing z^2 + 1
- N = 4

Product: (z^2 - 1)(z^2 + 1) = z^4 - 1. This IS divisible by z^4 - 1, so the answer to "is it NOT divisible?" is NO.
