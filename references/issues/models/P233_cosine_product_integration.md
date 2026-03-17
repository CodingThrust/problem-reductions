---
name: Problem
about: Propose a new problem type
title: "[Model] CosineProductIntegration"
labels: model
assignees: ''
---

## Motivation

COSINE PRODUCT INTEGRATION (P233) from Garey & Johnson, A7 AN14. Given a sequence of integers, the problem asks whether the integral of the product of cosines (with those integers as frequency coefficients) over [0, 2pi] equals zero. This is equivalent via Fourier analysis to asking whether there is NO partition of the integers into two equal-sum subsets, making it the complement of the PARTITION problem. NP-complete by reduction from PARTITION (Plaisted, 1976). Despite being phrased as a continuous integral, the problem encodes purely discrete combinatorial structure. Solvable in pseudo-polynomial time via dynamic programming on achievable partial sums.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R177 (PARTITION to COSINE PRODUCT INTEGRATION)
- As source: (none known)

## Definition

**Name:** `CosineProductIntegration`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Cosine Product Integration
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN14

**Mathematical definition:**

INSTANCE: Sequence (a_1, a_2, . . . , a_n) of integers.
QUESTION: Does ∫_0^{2π} (∏_{i=1}^{n} cos(a_i·θ)) dθ = 0?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n (one binary variable per element, representing the sign assignment in the product-to-sum expansion)
- **Per-variable domain:** {0, 1} -- conceptually, 0 maps to sign +1, and 1 maps to sign -1 in the expansion cos(a_i θ) = (e^{i a_i θ} + e^{-i a_i θ})/2
- **Meaning:** The integral equals (2π / 2^n) times the number of sign assignments ε ∈ {-1,+1}^n such that Σ ε_i a_i = 0. The integral is zero iff no such sign assignment exists, i.e., iff PARTITION has no solution. Thus the "variables" correspond to the partition choices, and the problem is a decision problem with no explicit configuration space.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `CosineProductIntegration`
**Variants:** none (no type parameters; coefficients are plain integers)

| Field          | Type        | Description                                          |
|----------------|-------------|------------------------------------------------------|
| `coefficients` | `Vec<i64>`  | Integer sequence (a_1, a_2, ..., a_n) of cosine frequencies |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The problem reduces to PARTITION (complement): the integral is nonzero iff a balanced sign assignment exists. The Schroeppel-Shamir meet-in-the-middle algorithm (1981) solves the underlying PARTITION/SUBSET SUM problem in O*(2^(n/2)) time and O*(2^(n/4)) space. Additionally, the problem is solvable in pseudo-polynomial time O(n · S) where S = Σ|a_i|, since one can track achievable partial sums via dynamic programming. [Plaisted, 1976; Schroeppel & Shamir, SIAM J. Comput. 10(4):456-464, 1981.]

## Extra Remark

**Full book text:**

INSTANCE: Sequence (a_1, a_2, . . . , a_n) of integers.
QUESTION: Does ∫_0^{2π} (∏_{i=1}^{n} cos(a_i·θ)) dθ = 0?

Reference: [Plaisted, 1976]. Transformation from PARTITION.
Comment: Solvable in pseudo-polynomial time. See reference for related complexity results concerning integration.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2^n sign assignments ε ∈ {-1,+1}^n; check if any yields Σ ε_i a_i = 0. The integral is zero iff no such assignment exists.)
- [ ] It can be solved by reducing to integer programming. (Binary ILP: variables x_i ∈ {0,1}, constraint Σ (2x_i - 1) · a_i = 0. Feasible iff integral ≠ 0.)
- [ ] Other: Pseudo-polynomial DP in O(n · S) time tracking achievable signed sums, where S = Σ|a_i|.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Sequence: (2, 3, 5) (n = 3)

**Analysis:**
Enumerate all 2^3 = 8 sign assignments:
- (+2, +3, +5) = 10
- (+2, +3, -5) = 0 ✓
- (+2, -3, +5) = 4
- (+2, -3, -5) = -6
- (-2, +3, +5) = 6
- (-2, +3, -5) = -4
- (-2, -3, +5) = 0 ✓
- (-2, -3, -5) = -10

Two sign assignments yield zero, so the integral = (2π/8) · 2 = π/2 ≠ 0.

Answer: **NO** -- the integral does NOT equal zero (because a partition 2+3 = 5 exists).

**Another example (integral = 0):**
Sequence: (1, 2, 6) (n = 3)
Sign assignments: sums are 9, -3, 5, -7, 7, -5, 3, -9. None equal zero.
Answer: **YES** -- the integral equals zero (no balanced partition exists since 1+2+6 = 9 is odd... wait, with signs: we need Σ ε_i a_i = 0, i.e., the positive and negative groups have equal sum. Total = 9, so each group would need sum 4.5, impossible with integers.)
