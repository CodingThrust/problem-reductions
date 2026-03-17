---
name: Problem
about: Propose a new problem type
title: "[Model] KthLargestMTuple"
labels: model
assignees: ''
canonical_name: 'K-th Largest m-Tuple'
milestone: 'Garey & Johnson'
---

## Motivation

K-th LARGEST m-TUPLE (P148) from Garey & Johnson, A3 SP21. This problem generalizes the K-th LARGEST SUBSET problem from subsets of a single set to m-tuples drawn from a Cartesian product of m sets. It asks whether at least K distinct m-tuples have total size at least B. Like K-th LARGEST SUBSET, it is **not known to be in NP** and was shown to be PP-complete under polynomial-time Turing reductions (Haase & Kiefer, 2016). It is solvable in polynomial time for fixed m and in pseudo-polynomial time in general.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R86 (PARTITION -> K-th LARGEST m-TUPLE)

## Definition

**Name:** `KthLargestMTuple`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP21

**Mathematical definition:**

INSTANCE: Sets X_1, X_2, ..., X_m ⊆ Z^+, a size s(x) ∈ Z^+ for each x ∈ X_i, 1 ≤ i ≤ m, and positive integers K and B.
QUESTION: Are there K or more distinct m-tuples (x_1, x_2, ..., x_m) in X_1 × X_2 × ... × X_m for which Σ_{i=1}^{m} s(x_i) ≥ B?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** m (one variable per coordinate of the tuple)
- **Per-variable domain:** variable i ranges over {0, 1, ..., |X_i| - 1}, indexing elements in X_i
- **Meaning:** Each configuration (j_1, j_2, ..., j_m) selects elements x_{j_1} ∈ X_1, x_{j_2} ∈ X_2, ..., x_{j_m} ∈ X_m, defining one m-tuple. The problem asks whether at least K such m-tuples satisfy Σ_{i=1}^{m} s(x_{j_i}) ≥ B. This is a satisfaction (decision) problem.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `KthLargestMTuple`
**Variants:** none (no type parameters; sizes are plain positive integers)

| Field    | Type              | Description                                                      |
|----------|-------------------|------------------------------------------------------------------|
| `sets`   | `Vec<Vec<u64>>`   | The m sets X_1, ..., X_m, each containing positive integer sizes |
| `k`      | `u64`             | Threshold K — number of distinct m-tuples required               |
| `bound`  | `u64`             | Lower bound B on the sum of sizes in each m-tuple                |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The problem is PP-complete under polynomial-time Turing reductions (Haase & Kiefer, 2016). For fixed m, the number of m-tuples is |X_1| · |X_2| · ... · |X_m|, and brute-force enumeration runs in O(Π|X_i| · m) time. The pseudo-polynomial algorithm of Johnson & Mizoguchi (1978) runs in time polynomial in K, Σ|X_i|, and log Σ s(x). For the binary case (m = 2, "selecting the K-th element in X + Y"), the problem is solvable in O(n log n) time (Johnson & Mizoguchi, 1978).

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization note -->

**Not known to be in NP.** The K-th LARGEST m-TUPLE problem is not a standard NP decision problem because a "yes" certificate would need to exhibit K m-tuples, and K can be exponentially large in the input size. The problem is PP-complete (Haase & Kiefer, 2016), placing it strictly above NP in the complexity hierarchy (under standard assumptions). If it were in NP, the polynomial hierarchy would collapse to P^NP.

Note: The problem is solvable in polynomial time for fixed m, because the total number of m-tuples is polynomial when m is constant. The NP-hardness arises when m is part of the input.

## Extra Remark

**Full book text:**

INSTANCE: Sets X_1,X_2,...,X_m ⊆ Z^+, a size s(x) ∈ Z^+ for each x ∈ X_i, 1 ≤ i ≤ m, and positive integers K and B.
QUESTION: Are there K or more distinct m-tuples (x_1,x_2,...,x_m) in X_1×X_2×···×X_m for which Σ_{i=1}^{m} s(x_i) ≥ B?
Reference: [Johnson and Mizoguchi, 1978]. Transformation from PARTITION.
Comment: Not known to be in NP. Solvable in polynomial time for fixed m, and in pseudo-polynomial time in general (polynomial in K, Σ|X_i|, and log Σs(x)). The corresponding enumeration problem is #P-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all m-tuples in X_1 × ... × X_m; count those with sum ≥ B; check if count ≥ K.)
- [x] It can be solved by reducing to integer programming. (Use integer variables to index each set, with a constraint on the sum.)
- [ ] Other: For fixed m, enumerate all Π|X_i| tuples in polynomial time. For general m, use dynamic programming on partial sums across the m sets.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 3 sets:
- X_1 = {2, 5, 8} (3 elements)
- X_2 = {3, 6} (2 elements)
- X_3 = {1, 4, 7} (3 elements)

B = 12, K = 8

**Total m-tuples:** 3 × 2 × 3 = 18

**Enumeration of all m-tuples with sum ≥ 12:**

| x_1 | x_2 | x_3 | Sum | ≥ 12? |
|-----|-----|-----|-----|-------|
| 2   | 3   | 1   | 6   | No    |
| 2   | 3   | 4   | 9   | No    |
| 2   | 3   | 7   | 12  | Yes   |
| 2   | 6   | 1   | 9   | No    |
| 2   | 6   | 4   | 12  | Yes   |
| 2   | 6   | 7   | 15  | Yes   |
| 5   | 3   | 1   | 9   | No    |
| 5   | 3   | 4   | 12  | Yes   |
| 5   | 3   | 7   | 15  | Yes   |
| 5   | 6   | 1   | 12  | Yes   |
| 5   | 6   | 4   | 15  | Yes   |
| 5   | 6   | 7   | 18  | Yes   |
| 8   | 3   | 1   | 12  | Yes   |
| 8   | 3   | 4   | 15  | Yes   |
| 8   | 3   | 7   | 18  | Yes   |
| 8   | 6   | 1   | 15  | Yes   |
| 8   | 6   | 4   | 18  | Yes   |
| 8   | 6   | 7   | 21  | Yes   |

Feasible m-tuples (sum ≥ 12): 14 tuples.
Since 14 ≥ K = 8, the answer is **YES**.

## References

- **[Johnson and Mizoguchi, 1978]**: [`Johnson1978a`] David B. Johnson and Takumi Mizoguchi (1978). "Selecting the $K$th element in $X+Y$ and $X_1+X_2+\cdots+X_m$". *SIAM Journal on Computing* 7, pp. 147-153.
- **[Haase and Kiefer, 2016]**: [`Haase2016`] Christoph Haase and Stefan Kiefer (2016). "The complexity of the Kth largest subset problem and related problems". *Information Processing Letters* 116(2), pp. 111-115.
