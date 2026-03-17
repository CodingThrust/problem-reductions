---
name: Problem
about: Propose a new problem type
title: "[Model] NumberOfRootsForAProductPolynomial"
labels: model
assignees: ''
---

## Motivation

NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL (P230) from Garey & Johnson, A7 AN11. An NP-hard problem (not known to be in NP or co-NP) that asks whether a product of sparse polynomials has fewer than K distinct complex roots. Plaisted (1977) established NP-hardness via a reduction from 3SAT that maps Boolean satisfiability to polynomial root-counting through a homomorphism from Boolean expressions onto divisors of z^N - 1. The problem remains NP-hard even when all coefficients are restricted to {-1, +1}.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R174: 3SAT -> NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL (establishes NP-hardness via Plaisted's cyclotomic polynomial encoding)

## Definition

**Name:** `NumberOfRootsProductPolynomial`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN11

**Mathematical definition:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0, and a positive integer K.
QUESTION: Does the polynomial Pi_{i=1}^{m} (Sigma_{j=1}^{k} a_i[j] * z^{b_i[j]}) have fewer than K distinct complex roots?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** This is a decision problem about a property of a polynomial product; there is no natural finite configuration space to enumerate. The "answer" is a count of distinct complex roots.
- **Per-variable domain:** The complex plane C (roots can be anywhere in C, not just the unit circle).
- **Meaning:** The question asks whether the total number of distinct points z in C where the product polynomial vanishes is fewer than K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `NumberOfRootsProductPolynomial`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `factors` | `Vec<Vec<(i64, u64)>>` | m sequences, each a list of (coefficient, exponent) pairs defining a sparse polynomial factor |
| `threshold` | `u64` | The positive integer K; we ask if the product has fewer than K distinct complex roots |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** No polynomial-time algorithm is known. The problem is NP-hard but not known to be in NP or co-NP (Plaisted, 1977). To solve it exactly, one could in principle:
  1. Expand the product polynomial into dense form (degree can be exponential in the input size).
  2. Find all roots using numerical methods (e.g., companion matrix eigenvalue approach, O(d^3) for degree d).
  3. Count distinct roots up to numerical precision.
  However, the degree of the product polynomial can be exponential in the sparse input size, making this approach super-polynomial. The fundamental hardness arises from the exponential gap between sparse representation size and polynomial degree.

## Extra Remark

**Full book text:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 <= i <= m, of pairs of integers, with each b_i[j] >= 0, and a positive integer K.
QUESTION: Does the polynomial Pi_{i=1}^{m} (Sigma_{j=1}^{k} a_i[j] * z^{b_i[j]}) have fewer than K distinct complex roots?

Reference: [Plaisted, 1977a]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1, as does the variant in which the instance also includes an integer M and we are asked whether the product polynomial has fewer than K complex roots of multiplicity M [Plaisted, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: For small degree instances, expand the product polynomial and use numerical root-finding (e.g., Aberth-Ehrlich method, companion matrix eigenvalues). For symbolic approaches, compute GCDs of factors to find common roots. No known polynomial-time algorithm for the general sparse case.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Factors (m = 3):
- A_1 = [(1, 0), (-1, 1)] represents z^0 - z^1 = 1 - z (root at z = 1)
- A_2 = [(1, 0), (1, 1)] represents z^0 + z^1 = 1 + z (root at z = -1)
- A_3 = [(1, 0), (0, 1), (1, 2)] represents 1 + z^2 (roots at z = i and z = -i)

Threshold: K = 5.

**Product polynomial:** P(z) = (1 - z)(1 + z)(1 + z^2) = (1 - z^2)(1 + z^2) = 1 - z^4.

**Distinct complex roots of P(z) = 1 - z^4:**
z^4 = 1, so z in {1, -1, i, -i}. That is 4 distinct complex roots.

**Question:** Does P(z) have fewer than K = 5 distinct complex roots?
Since 4 < 5, the answer is YES.

**Modified threshold:** If K = 3, then does P(z) have fewer than 3 distinct roots? No, since 4 >= 3. Answer: NO.
