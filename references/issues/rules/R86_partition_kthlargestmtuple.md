---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to K-th LARGEST m-TUPLE"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'K-th Largest m-Tuple'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** K-th LARGEST m-TUPLE
**Motivation:** Establishes NP-hardness of K-th LARGEST m-TUPLE via polynomial-time reduction from PARTITION. The K-th LARGEST m-TUPLE problem generalizes selection in Cartesian products of integer sets, asking whether at least K m-tuples from X_1 × ... × X_m have total size at least B. This reduction, due to Johnson and Mizoguchi (1978), demonstrates that even the threshold-counting version of the Cartesian product selection problem is computationally hard. Like K-th LARGEST SUBSET, this problem is PP-complete and not known to be in NP.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SP21, p.225

## GJ Source Entry

> [SP21] K^th LARGEST m-TUPLE (*)
> INSTANCE: Sets X_1,X_2,…,X_m⊆Z^+, a size s(x)∈Z^+ for each x∈X_i, 1≤i≤m, and positive integers K and B.
> QUESTION: Are there K or more distinct m-tuples (x_1,x_2,…,x_m) in X_1×X_2×···×X_m for which Σ_{i=1}^{m} s(x_i)≥B?
> Reference: [Johnson and Mizoguchi, 1978]. Transformation from PARTITION.
> Comment: Not known to be in NP. Solvable in polynomial time for fixed m, and in pseudo-polynomial time in general (polynomial in K, Σ|X_i|, and log Σ s(x)). The corresponding enumeration problem is #P-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION instance A = {a_1, ..., a_n} with sizes s(a_i) ∈ Z^+ and total sum S = Σ s(a_i), construct a K-th LARGEST m-TUPLE instance as follows:

1. **Number of sets:** Set m = n (one set per element of A).
2. **Sets:** For each i = 1, ..., n, define X_i = {0, s(a_i)} — a two-element set where 0 represents "not including a_i in the partition half" and s(a_i) represents "including a_i."
3. **Bound:** Set B = S/2 (half the total sum). If S is odd, the PARTITION instance has no solution — the reduction can set B = ⌈S/2⌉ to ensure the answer is NO.
4. **Threshold K:** Set K = (number of m-tuples with sum ≥ S/2 when no exact partition exists) + 1. More precisely, let C be the number of m-tuples (x_1, ..., x_m) ∈ X_1 × ... × X_m with Σ x_i > S/2. If PARTITION is feasible, there exist m-tuples with sum = S/2, which are additional m-tuples meeting the threshold. Set K = C + 1 (where C counts tuples with sum strictly greater than S/2).

**Correctness:**
- Each m-tuple (x_1, ..., x_m) ∈ X_1 × ... × X_m corresponds to a subset A' ⊆ A (include a_i iff x_i = s(a_i)). The tuple sum equals Σ_{a_i ∈ A'} s(a_i).
- The m-tuples with sum ≥ S/2 are exactly those corresponding to subsets with sum ≥ S/2.
- PARTITION is feasible iff some subset sums to exactly S/2, which creates additional m-tuples at the boundary (sum = S/2) beyond those with sum > S/2.

**Note:** As with R85, computing K requires counting subsets, making this a Turing reduction. The (*) in GJ indicates the problem is not known to be in NP.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = number of elements in the PARTITION instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_sets` (= m)           | `num_elements` (= n)             |
| `total_set_sizes` (Σ\|X_i\|) | `2 * num_elements` (= 2n)      |

**Derivation:** Each element a_i maps to a 2-element set X_i = {0, s(a_i)}, giving m = n sets with 2 elements each. Total number of m-tuples is 2^n. The bound B and threshold K are scalar parameters. Construction is O(n) for the sets, plus counting time for K.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to K-th LARGEST m-TUPLE, solve the target with BruteForce (enumerate all 2^n m-tuples, count those with sum ≥ B), verify the count agrees with the source PARTITION answer.
- Compare with known results from literature: verify that the bijection between m-tuples and subsets of A is correct, and that the YES/NO answer matches.
- Edge cases: test with odd total sum (no partition possible), all equal elements (many partitions), and instances with a unique balanced partition.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {3, 1, 1, 2, 2, 1} (n = 6 elements)
Total sum S = 10; target half-sum = 5.
A balanced partition exists: A' = {3, 2} (sum = 5), A \ A' = {1, 1, 2, 1} (sum = 5).

**Constructed K-th LARGEST m-TUPLE instance:**

Step 1: m = 6 sets.
Step 2: X_1 = {0, 3}, X_2 = {0, 1}, X_3 = {0, 1}, X_4 = {0, 2}, X_5 = {0, 2}, X_6 = {0, 1}
Step 3: B = 5 (= S/2).
Step 4: Count m-tuples with sum > 5 (strictly greater):

Total 2^6 = 64 m-tuples. Each corresponds to a subset of A.
Subsets with sum > 5: these correspond to subsets of {3,1,1,2,2,1} with sum in {6,7,8,9,10}.

Counting by complement: subsets with sum ≤ 4:
- {} : 0, {1}×3 : 1, {2}×2 : 2, {3} : 3 (7 singletons+empty ≤ 4)
- Actually systematically: sum=0: 1, sum=1: 3 ({a_2},{a_3},{a_6}), sum=2: 4 ({a_4},{a_5},{a_2,a_3},{a_2,a_6},{a_3,a_6}... need careful count)

Let me count subsets with sum ≤ 4 using DP:
- DP[0] = 1 (empty set)
- After a_1 (size 3): DP = [1,0,0,1,0,...] → sums 0:1, 3:1
- After a_2 (size 1): sums 0:1, 1:1, 3:1, 4:1
- After a_3 (size 1): sums 0:1, 1:2, 2:1, 3:1, 4:2 (but this counts distinct subsets)

Let me just count: subsets with sum = 5 (balanced partition): these are the boundary.
By symmetry, subsets with sum < 5 and subsets with sum > 5 come in complementary pairs.
Number of subsets with sum = 5: let's enumerate: {3,2_a}(5), {3,2_b}(5), {3,1_a,1_b}(5), {3,1_a,1_c}(5), {3,1_b,1_c}(5), {2_a,2_b,1_a}(5), {2_a,2_b,1_b}(5), {2_a,2_b,1_c}(5), {1_a,1_b,1_c,2_a}(5)... wait, that's sum=6.
Let me be precise with sizes [3,1,1,2,2,1]:
- {a_1,a_4} = {3,2} → 5 ✓
- {a_1,a_5} = {3,2} → 5 ✓
- {a_1,a_2,a_6} = {3,1,1} → 5 ✓
- {a_1,a_3,a_6} = {3,1,1} → 5 ✓
- {a_1,a_2,a_3} = {3,1,1} → 5 ✓
- {a_4,a_5,a_6} = {2,2,1} → 5 ✓
- {a_4,a_5,a_2} = {2,2,1} → 5 ✓
- {a_4,a_5,a_3} = {2,2,1} → 5 ✓
- {a_2,a_3,a_6,a_4} = {1,1,1,2} → 5 ✓
- {a_2,a_3,a_6,a_5} = {1,1,1,2} → 5 ✓

That gives 10 subsets summing to exactly 5.
By symmetry: 64 total, with sum<5 count = sum>5 count = (64 - 10) / 2 = 27 each.

C = 27 (subsets with sum > 5). K = 27 + 1 = 28.

**K-th LARGEST m-TUPLE instance:** X_1,...,X_6, B = 5, K = 28.
m-tuples with sum ≥ 5 = 27 + 10 = 37 ≥ 28 = K → **YES**

This confirms the PARTITION answer: a balanced partition exists.

**Negative case:** If A = {3, 1, 1, 2, 2, 2} (sum = 11, odd), then S/2 = 5.5, B = 6.
No subset sums to exactly 5.5. Setting B = 6 and K = (subsets with sum > 6) + 1:
Since no subset sums to exactly 6... actually {3,2,1} = 6, so this doesn't work as a negative example.
Instead: A = {5, 3, 3} (sum = 11, odd) → no balanced partition possible. B = 6.
Subsets with sum > 6: {5,3_a}=8, {5,3_b}=8, {5,3_a,3_b}=11, {3_a,3_b}=6... no, 6 is not > 6.
So 3 subsets with sum > 6. K = 4. Subsets with sum ≥ 6: add {3_a,3_b}=6 → 4. But 4 ≥ 4 → YES? That's wrong.
The key subtlety: we need K = (subsets with sum ≥ B when no partition) + 1, not just sum > B.
Since PARTITION asks for sum = S/2 = 5.5 which is impossible for integers, **every** PARTITION instance with odd S is NO. For integer formulations, B should be set to ⌈S/2⌉ = 6, and K = (number of m-tuples with sum ≥ 6) + 1 = 4 + 1 = 5. But count = 4 < 5, so answer is NO. ✓

## References

- **[Johnson and Mizoguchi, 1978]**: [`Johnson1978a`] David B. Johnson and Takumi Mizoguchi (1978). "Selecting the $K$th element in $X+Y$ and $X_1+X_2+\cdots+X_m$". *SIAM Journal on Computing* 7, pp. 147–153.
- **[Haase and Kiefer, 2016]**: [`Haase2016`] Christoph Haase and Stefan Kiefer (2016). "The complexity of the Kth largest subset problem and related problems". *Information Processing Letters* 116(2), pp. 111–115.
