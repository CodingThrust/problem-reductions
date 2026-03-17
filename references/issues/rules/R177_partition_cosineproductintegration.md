---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to COSINE PRODUCT INTEGRATION"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'Cosine Product Integration'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** COSINE PRODUCT INTEGRATION
**Motivation:** Establishes NP-completeness of COSINE PRODUCT INTEGRATION via polynomial-time reduction from PARTITION. The reduction exploits the identity that the integral of a product of cosines over [0, 2pi] is nonzero if and only if the integer coefficients can be partitioned into two subsets of equal sum, linking a classical combinatorial problem to a question about trigonometric integrals. This result, due to Plaisted (1976), demonstrates that even seemingly continuous analytic problems can encode discrete NP-complete structure.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN14] COSINE PRODUCT INTEGRATION
> INSTANCE: Sequence (a_1,a_2,...,a_n) of integers.
> QUESTION: Does ∫_0^{2π} (Π_{i=1}^n cos(a_i θ)) dθ = 0?
> Reference: [Plaisted, 1976]. Transformation from PARTITION.
> Comment: Solvable in pseudo-polynomial time. See reference for related complexity results concerning integration.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

The reduction uses the trigonometric product-to-sum identity. The key mathematical fact is:

∫_0^{2π} ∏_{i=1}^n cos(a_i θ) dθ = (2π / 2^n) · |{ε ∈ {-1,+1}^n : ε_1 a_1 + ε_2 a_2 + ... + ε_n a_n = 0}|

This follows from expanding the product of cosines using cos(x) = (e^{ix} + e^{-ix})/2, which yields 2^n terms of the form exp(i(±a_1 ± a_2 ± ... ± a_n)θ). Integrating each term over [0, 2π] gives 2π if and only if the exponent is zero, and 0 otherwise. Thus the integral is nonzero if and only if there exists a sign assignment ε ∈ {-1,+1}^n such that Σ ε_i a_i = 0, which is exactly the PARTITION problem (assign each a_i to the "+" or "-" subset).

Let A = {a_1, ..., a_n} with a_i ∈ Z be an arbitrary PARTITION instance.

1. **Construct the sequence:** Output the sequence (a_1, a_2, ..., a_n) directly as the COSINE PRODUCT INTEGRATION instance.
2. **Correctness:** PARTITION has a solution A' ⊆ A with Σ_{a ∈ A'} a = Σ_{a ∈ A\A'} a if and only if there exists ε ∈ {-1,+1}^n with Σ ε_i a_i = 0, if and only if ∫_0^{2π} ∏ cos(a_i θ) dθ ≠ 0. Note: The original PARTITION problem asks whether the integral is NOT zero; the GJ formulation asks whether it IS zero, so the answer to COSINE PRODUCT INTEGRATION is YES (integral = 0) iff PARTITION has NO solution.
3. **Solution extraction:** If the integral is nonzero, the sign assignment ε giving Σ ε_i a_i = 0 directly yields the partition: A' = {a_i : ε_i = +1}, A\A' = {a_i : ε_i = -1}.

**Note on the GJ formulation:** The GJ entry asks "Does the integral equal 0?", which is the complement of PARTITION feasibility. This still establishes NP-hardness (co-NP-hardness of the "= 0" question implies NP-hardness of the "≠ 0" question, and vice versa). The problem is solvable in pseudo-polynomial time by dynamic programming on the set of achievable partial sums.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in PARTITION instance

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `sequence_length`         | n                                |
| `max_coefficient`         | max(a_i) (unchanged)             |

**Derivation:** The reduction is an identity transformation: the PARTITION elements become the cosine coefficients directly. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to COSINE PRODUCT INTEGRATION, numerically evaluate the integral (or equivalently enumerate all 2^n sign assignments), verify that the integral is nonzero iff a balanced partition exists.
- Check that the constructed instance has exactly n cosine coefficients matching the original element sizes.
- Edge cases: test with n = 1 (integral = 0 since no balanced partition exists), n = 2 with a_1 = a_2 (integral ≠ 0, partition exists), all elements equal (partition exists iff n is even).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {1, 2, 3} (n = 3 elements)
Total sum = 6, target half-sum = 3.
A balanced partition exists: A' = {3} (sum = 3) and A \ A' = {1, 2} (sum = 3).

**Constructed COSINE PRODUCT INTEGRATION instance:**
Sequence: (1, 2, 3)
Question: Does ∫_0^{2π} cos(θ) · cos(2θ) · cos(3θ) dθ = 0?

**Verification:**
Expand using the product-to-sum identity. There are 2^3 = 8 sign assignments (ε_1, ε_2, ε_3) ∈ {-1,+1}^3:
- (+1, +1, +1): 1+2+3 = 6 ≠ 0
- (+1, +1, -1): 1+2-3 = 0 ✓
- (+1, -1, +1): 1-2+3 = 2 ≠ 0
- (+1, -1, -1): 1-2-3 = -4 ≠ 0
- (-1, +1, +1): -1+2+3 = 4 ≠ 0
- (-1, +1, -1): -1+2-3 = -2 ≠ 0
- (-1, -1, +1): -1-2+3 = 0 ✓
- (-1, -1, -1): -1-2-3 = -6 ≠ 0

Two sign assignments give zero, so the integral = (2π/8) · 2 = π/2 ≠ 0.
Answer to "Does the integral = 0?": **NO** (integral = π/2).
This correctly reflects that PARTITION has a solution.

**Solution extraction:**
Sign assignment (+1, +1, -1) gives: A' = {1, 2} (sum = 3), A\A' = {3} (sum = 3). Balanced ✓


## References

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264–267. IEEE Computer Society.
