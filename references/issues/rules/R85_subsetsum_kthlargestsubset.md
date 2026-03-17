---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SUBSET SUM to K-th LARGEST SUBSET"
labels: rule
assignees: ''
canonical_source_name: 'Subset Sum'
canonical_target_name: 'K-th Largest Subset'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** SUBSET SUM
**Target:** K-th LARGEST SUBSET
**Motivation:** Establishes NP-hardness of K-th LARGEST SUBSET via polynomial-time reduction from SUBSET SUM. The K-th LARGEST SUBSET problem generalizes SUBSET SUM from asking "does a subset with sum exactly B exist?" to "how many subsets have sum at most B?" This reduction, due to Johnson and Kashdan (1976), shows that even the threshold-counting version of subset feasibility is computationally hard. The target problem is notable for being PP-complete (not merely NP-complete) -- it is not known to be in NP, as a certificate may require exhibiting exponentially many subsets.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SP20, p.225

## GJ Source Entry

> [SP20] K^th LARGEST SUBSET (*)
> INSTANCE: Finite set A, size s(a)∈Z^+ for each a∈A, positive integers K and B.
> QUESTION: Are there K or more distinct subsets A'⊆A for which the sum of the sizes of the elements in A' does not exceed B?
> Reference: [Johnson and Kashdan, 1976]. Transformation from SUBSET SUM.
> Comment: Not known to be in NP. Solvable in pseudo-polynomial time (polynomial in K, |A|, and log Σ s(a)) [Lawler, 1972]. The corresponding enumeration problem is #P-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a SUBSET SUM instance (A, s, B_ss) where A = {a_1, ..., a_n} with sizes s(a_i) ∈ Z^+ and target B_ss, construct a K-th LARGEST SUBSET instance as follows:

1. **Set and sizes:** Keep the same set A with the same sizes s(a) for each a ∈ A.
2. **Bound:** Set B = B_ss (the subset sum target becomes the upper bound on subset sums).
3. **Threshold K:** Set K = (number of subsets of A with sum ≤ B_ss - 1) + 1. Equivalently, let C(A, t) denote the number of subsets of A with sum ≤ t. Then set K = C(A, B_ss - 1) + 1.

**Correctness:**
- If SUBSET SUM has a solution (some A' with Σ s(a) = B_ss), then the number of subsets with sum ≤ B_ss is at least C(A, B_ss - 1) + 1 = K (the solution subset is a new feasible subset not counted in C(A, B_ss - 1)). So the K-th LARGEST SUBSET answer is YES.
- If SUBSET SUM has no solution (no subset sums to exactly B_ss), then every subset with sum ≤ B_ss actually has sum ≤ B_ss - 1 (since all sizes are positive integers). So the count of subsets with sum ≤ B_ss equals C(A, B_ss - 1) < K. The K-th LARGEST SUBSET answer is NO.

**Note:** Computing K = C(A, B_ss - 1) + 1 requires counting subsets, which is itself a #P problem. This is why the reduction is a polynomial-time Turing reduction (using an oracle for subset counting) rather than a many-one reduction. Garey & Johnson mark the problem with (*) indicating it is "not known to be in NP."

**Alternative (direct) formulation for implementation:** For a simpler (but not polynomial-time computable) construction, one can use dynamic programming in pseudo-polynomial time O(n · B) to compute C(A, B_ss - 1), then set K accordingly. This suffices for small test instances.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = number of elements (`num_elements` of source SUBSET SUM instance)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_elements`             | `num_elements` (= n)             |

**Derivation:** The set A and sizes are copied unchanged. The bound B is a scalar parameter. The threshold K is computed from the source instance but does not affect the structural size. Construction is O(n) for the set copy, plus O(n · B_ss) for computing K via DP.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a SUBSET SUM instance, reduce to K-th LARGEST SUBSET, solve the target with BruteForce (enumerate all 2^n subsets, count those with sum ≤ B), extract the count, verify it agrees with the source SUBSET SUM answer.
- Compare with known results from literature: verify that K is correctly computed via DP counting, and that the YES/NO answer matches.
- Edge cases: test with no feasible subset (B < min element), all subsets feasible (B ≥ Σ s(a)), and instances where exactly one subset sums to B_ss.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (SUBSET SUM):**
A = {3, 5, 7, 1, 2, 4} (n = 6 elements)
s(a_1) = 3, s(a_2) = 5, s(a_3) = 7, s(a_4) = 1, s(a_5) = 2, s(a_6) = 4
Target B_ss = 10

SUBSET SUM asks: Is there A' ⊆ A with Σ s(a) = 10?
Answer: YES — e.g., A' = {3, 7} (sum = 10), or A' = {3, 5, 2} (sum = 10), or A' = {5, 1, 4} (sum = 10).

**Constructed K-th LARGEST SUBSET instance:**

Step 1: Set and sizes unchanged: A = {3, 5, 7, 1, 2, 4}, same sizes.
Step 2: Bound B = 10.
Step 3: Count subsets with sum ≤ 9 (= B_ss - 1).

Subsets with sum ≤ 9:
- {} : 0 ✓
- {1} : 1 ✓, {2} : 2 ✓, {3} : 3 ✓, {4} : 4 ✓, {5} : 5 ✓, {7} : 7 ✓ (6 singletons)
- {1,2} : 3 ✓, {1,3} : 4 ✓, {1,4} : 5 ✓, {1,5} : 6 ✓, {1,7} : 8 ✓
- {2,3} : 5 ✓, {2,4} : 6 ✓, {2,5} : 7 ✓, {2,7} : 9 ✓
- {3,4} : 7 ✓, {3,5} : 8 ✓
- {4,5} : 9 ✓, {4,7} : 11 ✗, {5,7} : 12 ✗ (11 pairs)
- {1,2,3} : 6 ✓, {1,2,4} : 7 ✓, {1,2,5} : 8 ✓, {1,2,7} : 10 ✗
- {1,3,4} : 8 ✓, {1,3,5} : 9 ✓, {1,4,5} : 10 ✗
- {2,3,4} : 9 ✓, {2,3,5} : 10 ✗, {2,4,5} : 11 ✗
- {3,4,5} : 12 ✗ (7 triples ≤ 9)
- {1,2,3,4} : 10 ✗ (all 4+ element subsets have sum ≥ 10 or close)

C(A, 9) = 1 + 6 + 11 + 7 = 25 subsets with sum ≤ 9.
Set K = 25 + 1 = 26.

**K-th LARGEST SUBSET instance:** A, B = 10, K = 26.
Subsets with sum ≤ 10: includes all 25 above, plus subsets summing to exactly 10:
{3,7}, {3,5,2}, {5,1,4}, {1,2,7} = 4 additional subsets.
Total = 25 + 4 = 29 ≥ 26 = K → **YES**

This confirms the SUBSET SUM answer: a subset summing to exactly 10 exists.

**If instead B_ss = 22 (no subset sums to 22, since total = 22 but that uses all elements):**
Actually Σ = 22, and {3,5,7,1,2,4} sums to 22. So B_ss = 23 (impossible):
C(A, 22) = 2^6 = 64 (all subsets have sum ≤ 22). C(A, 23 - 1) = C(A, 22) = 64. K = 65.
Subsets with sum ≤ 23 = 64 < 65 → **NO** (no subset sums to exactly 23).


## References

- **[Johnson and Kashdan, 1976]**: [`Johnson1976a`] David B. Johnson and S. D. Kashdan (1976). "Lower bounds for selection in $X+Y$ and other multisets". Computer Science Department, Pennsylvania State University.
- **[Lawler, 1972]**: [`Lawler1972`] Eugene L. Lawler (1972). "A procedure for computing the {$K$} best solutions to discrete optimization problems and its application to the shortest path problem". *Management Science* 18, pp. 401–405.
- **[Haase and Kiefer, 2016]**: [`Haase2016`] Christoph Haase and Stefan Kiefer (2016). "The complexity of the Kth largest subset problem and related problems". *Information Processing Letters* 116(2), pp. 111–115.
