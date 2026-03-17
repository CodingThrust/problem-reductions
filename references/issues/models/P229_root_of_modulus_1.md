---
name: Problem
about: Propose a new problem type
title: "[Model] RootOfModulus1"
labels: model
assignees: ''
---

## Motivation

ROOT OF MODULUS 1 (P229) from Garey & Johnson, A7 AN10. An NP-hard problem (not known to be in NP or co-NP) concerning whether a sparse polynomial, given as a list of integer coefficient-exponent pairs, has a root on the complex unit circle. This bridges Boolean satisfiability with analytic number theory and complex analysis. Plaisted (1977) established NP-hardness via reduction from 3SAT using cyclotomic polynomial techniques.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R173: 3SAT -> ROOT OF MODULUS 1 (establishes NP-hardness via Plaisted's cyclotomic encoding)

## Definition

**Name:** `RootOfModulus1`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN10

**Mathematical definition:**

INSTANCE: Ordered pairs (a[i], b[i]), 1 <= i <= n, of integers, with each b[i] >= 0.
QUESTION: Does the polynomial P(z) = Sigma_{i=1}^{n} a[i] * z^{b[i]} have a root on the complex unit circle, i.e., is there a complex number q with |q| = 1 such that Sigma_{i=1}^{n} a[i] * q^{b[i]} = 0?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** This is a decision (satisfaction) problem; there is no natural finite configuration space to enumerate. The "variable" is a continuous complex number q on the unit circle.
- **Per-variable domain:** The unit circle {q in C : |q| = 1}, a continuous domain. For computational purposes, one might discretize by sampling N-th roots of unity (q = e^{2 pi i k / N} for k = 0, ..., N-1, where N = lcm of relevant periods).
- **Meaning:** q is the candidate root. The question is whether P(q) = 0 for some q on the unit circle.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `RootOfModulus1`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `terms` | `Vec<(i64, u64)>` | Ordered pairs (a[i], b[i]) where a[i] is the coefficient and b[i] is the exponent |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** No polynomial-time algorithm is known. The problem is NP-hard but not known to be in NP or co-NP (Plaisted, 1977). For the special case where exponents are bounded, one can evaluate the polynomial at all N-th roots of unity in O(N * n) time (where N = max exponent + 1), but N can be exponentially large. Numerical root-finding methods (e.g., Durand-Kerner) can find roots of the dense form of the polynomial in O(d^2) arithmetic operations where d = max{b[i]}, but d may be exponential in the input size. The problem's hardness stems from the sparse representation: the degree can be exponentially larger than the input description.

## Extra Remark

**Full book text:**

INSTANCE: Ordered pairs (a[i], b[i]), 1 <= i <= n, of integers, with each b[i] >= 0.
QUESTION: Does the polynomial Sigma_{i=1}^{n} a[i] * z^{b[i]} have a root on the complex unit circle, i.e., is there a complex number q with |q| = 1 such that Sigma_{i=1}^{n} a[i] * q^{b[i]} = 0?

Reference: [Plaisted, 1977b]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: For special cases with bounded exponents, evaluate at all N-th roots of unity. In general, numerical root-finding on the dense expansion; or symbolic methods using cyclotomic polynomial factorization. No known polynomial-time algorithm for the general sparse case.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Terms: [(1, 0), (1, 2), (-2, 1)]
This represents the polynomial P(z) = 1 + z^2 - 2z = z^2 - 2z + 1 = (z - 1)^2.

**Root on the unit circle:** z = 1, and |1| = 1.
Check: P(1) = 1 + 1 - 2 = 0. Correct.
Answer: YES -- the polynomial has a root on the unit circle.

**Negative example:**
Terms: [(2, 0), (1, 1)]
This represents P(z) = 2 + z.
Root: z = -2, but |-2| = 2 != 1.
No root on the unit circle.
Answer: NO.

**Nontrivial example:**
Terms: [(1, 0), (1, 3)]
This represents P(z) = 1 + z^3.
Roots: z^3 = -1, so z = e^{i pi (2k+1)/3} for k = 0, 1, 2.
All roots have |z| = 1 (they lie on the unit circle).
Answer: YES -- the polynomial has roots on the unit circle (specifically at z = e^{i pi/3}, e^{i pi}, e^{5 i pi/3}).
