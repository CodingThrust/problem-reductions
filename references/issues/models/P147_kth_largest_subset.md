---
name: Problem
about: Propose a new problem type
title: "[Model] KthLargestSubset"
labels: model
assignees: ''
canonical_name: 'K-th Largest Subset'
milestone: 'Garey & Johnson'
---

## Motivation

K-th LARGEST SUBSET (P147) from Garey & Johnson, A3 SP20. This problem asks whether at least K distinct subsets of a finite set A have total size not exceeding a bound B. It is a natural generalization of SUBSET SUM from a single feasibility question to a counting threshold. The problem is notable for being NP-hard but **not known to be in NP** -- it was shown to be PP-complete under polynomial-time Turing reductions by Haase and Kiefer (2016). The corresponding enumeration problem is #P-complete.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R85 (SUBSET SUM -> K-th LARGEST SUBSET)

## Definition

**Name:** `KthLargestSubset`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP20

**Mathematical definition:**

INSTANCE: Finite set A, size s(a) ∈ Z^+ for each a ∈ A, positive integers K and B.
QUESTION: Are there K or more distinct subsets A' ⊆ A for which the sum of the sizes of the elements in A' does not exceed B?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |A| (one binary variable per element)
- **Per-variable domain:** {0, 1} — 0 means element is excluded from the subset, 1 means included
- **Meaning:** Each binary vector x ∈ {0,1}^n defines a subset A' = {a_i : x_i = 1}. The problem asks whether at least K distinct subsets satisfy Σ_{i: x_i=1} s(a_i) ≤ B. This is a satisfaction (decision) problem, not an optimization problem. The answer is YES or NO.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `KthLargestSubset`
**Variants:** none (no type parameters; sizes are plain positive integers)

| Field    | Type        | Description                                                |
|----------|-------------|------------------------------------------------------------|
| `sizes`  | `Vec<u64>`  | Positive integer size s(a) for each element a ∈ A          |
| `bound`  | `u64`       | Upper bound B on the subset sum                            |
| `k`      | `u64`       | Threshold K — number of distinct feasible subsets required  |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The problem is PP-complete under polynomial-time Turing reductions (Haase & Kiefer, Information Processing Letters 116(2):111-115, 2016). A brute-force approach enumerates all 2^n subsets and counts those with sum ≤ B, taking O(2^n · n) time. The pseudo-polynomial algorithm of Lawler (1972) solves it in time polynomial in K, |A|, and log Σ s(a). No sub-exponential exact algorithm is known for the general case.

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization note -->

**Not known to be in NP.** The K-th LARGEST SUBSET problem is not a standard NP decision problem because a "yes" certificate would need to exhibit K subsets, and K can be exponentially large (up to 2^n). The problem is PP-complete (Haase & Kiefer, 2016), meaning it sits strictly above NP in the polynomial hierarchy (assuming standard complexity-theoretic conjectures). If it were in NP, the polynomial hierarchy would collapse to P^NP.

## Extra Remark

**Full book text:**

INSTANCE: Finite set A, size s(a) ∈ Z^+ for each a ∈ A, positive integers K and B.
QUESTION: Are there K or more distinct subsets A' ⊆ A for which the sum of the sizes of the elements in A' does not exceed B?
Reference: [Johnson and Kashdan, 1976]. Transformation from SUBSET SUM.
Comment: Not known to be in NP. Solvable in pseudo-polynomial time (polynomial in K, |A|, and log Σs(a)) [Lawler, 1972]. The corresponding enumeration problem is #P-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2^n subsets; count those with sum ≤ B; check if count ≥ K.)
- [x] It can be solved by reducing to integer programming. (Enumerate subsets via ILP with indicator constraints, though this is impractical for large K.)
- [ ] Other: Pseudo-polynomial DP approach: build a table counting the number of subsets with each possible sum from 0 to B, then check if the total count ≥ K. Runs in O(n · B) time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
A = {2, 3, 5, 7, 1, 4} (n = 6 elements)
s(a_1) = 2, s(a_2) = 3, s(a_3) = 5, s(a_4) = 7, s(a_5) = 1, s(a_6) = 4
B = 6, K = 12

**Enumeration of subsets with sum ≤ 6:**
- {} : sum = 0 ✓
- {2} : 2 ✓
- {3} : 3 ✓
- {5} : 5 ✓
- {1} : 1 ✓
- {4} : 4 ✓
- {2,3} : 5 ✓
- {2,5} : 7 ✗
- {2,1} : 3 ✓
- {2,4} : 6 ✓
- {3,1} : 4 ✓
- {3,4} : 7 ✗
- {5,1} : 6 ✓
- {1,4} : 5 ✓
- {2,3,1} : 6 ✓
- {2,1,4} : 7 ✗
- {3,1,4} : 8 ✗
- ... (other multi-element subsets exceed B)

Feasible subsets (sum ≤ 6): {}, {2}, {3}, {5}, {1}, {4}, {2,3}, {2,1}, {2,4}, {3,1}, {5,1}, {1,4}, {2,3,1} = 13 subsets.
Since 13 ≥ K = 12, the answer is **YES**.

## References

- **[Johnson and Kashdan, 1976]**: [`Johnson1976a`] David B. Johnson and S. D. Kashdan (1976). "Lower bounds for selection in $X+Y$ and other multisets". Computer Science Department, Pennsylvania State University.
- **[Lawler, 1972]**: [`Lawler1972`] Eugene L. Lawler (1972). "A procedure for computing the $K$ best solutions to discrete optimization problems and its application to the shortest path problem". *Management Science* 18, pp. 401-405.
- **[Haase and Kiefer, 2016]**: [`Haase2016`] Christoph Haase and Stefan Kiefer (2016). "The complexity of the Kth largest subset problem and related problems". *Information Processing Letters* 116(2), pp. 111-115.
