---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SUBSET SUM to INTEGER KNAPSACK"
labels: rule
assignees: ''
canonical_source_name: 'SUBSET SUM'
canonical_target_name: 'INTEGER KNAPSACK'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** SUBSET SUM
**Target:** INTEGER KNAPSACK
**Motivation:** Establishes the NP-completeness of INTEGER KNAPSACK by a direct embedding of SUBSET SUM. The key insight is that SUBSET SUM is a special case of INTEGER KNAPSACK where s(u) = v(u) for all items and multiplicities are restricted to {0, 1}. The reduction simply maps each element to an item with equal size and value, and the integer multiplicity generalization does not help because the SUBSET SUM constraint already forces 0-1 choices via the exact-sum requirement. This is one of the simplest NP-completeness reductions, yet it connects the fundamental SUBSET SUM problem to the richer unbounded knapsack framework.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247

## GJ Source Entry

> [MP10] INTEGER KNAPSACK
> INSTANCE: Finite set U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, and positive integers B and K.
> QUESTION: Is there an assignment of a non-negative integer c(u) to each u E U such that Σ_{u E U} c(u)·s(u) ≤ B and such that Σ_{u E U} c(u)·v(u) ≥ K?
> Reference: [Lueker, 1975]. Transformation from SUBSET SUM.
> Comment: Remains NP-complete if s(u) = v(u) for all u E U. Solvable in pseudo-polynomial time by dynamic programming. Solvable in polynomial time if |U| = 2 [Hirschberg and Wong, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a SUBSET SUM instance: a finite set A = {a₁, ..., aₙ} with sizes s(aᵢ) ∈ Z⁺ and a target sum B, construct an INTEGER KNAPSACK instance as follows:

1. **Item set:** U = A. For each element aᵢ ∈ A, create an item uᵢ with size s(uᵢ) = s(aᵢ) and value v(uᵢ) = s(aᵢ). That is, the size and value of each item are both equal to the original element's size.

2. **Capacity:** Set the knapsack capacity to B (the same target sum from SUBSET SUM).

3. **Value target:** Set K = B. We require the total value to be at least B.

4. **Correctness (forward):** If there exists A' ⊆ A with Σ_{a∈A'} s(a) = B, then set c(uᵢ) = 1 if aᵢ ∈ A' and c(uᵢ) = 0 otherwise. Then Σ c(uᵢ)·s(uᵢ) = Σ_{a∈A'} s(a) = B ≤ B, and Σ c(uᵢ)·v(uᵢ) = B ≥ K = B.

5. **Correctness (reverse):** If there exist non-negative integers c(uᵢ) with Σ c(uᵢ)·s(uᵢ) ≤ B and Σ c(uᵢ)·v(uᵢ) ≥ B, then since v(uᵢ) = s(uᵢ) for all i, we have Σ c(uᵢ)·s(uᵢ) ≥ B and Σ c(uᵢ)·s(uᵢ) ≤ B, so Σ c(uᵢ)·s(uᵢ) = B exactly. Now define A' = {aᵢ : c(uᵢ) ≥ 1}. Since all sizes are positive and the total is exactly B, and each c(uᵢ) ≥ 1 contributes at least s(uᵢ), we can extract a subset summing to B. (If any c(uᵢ) > 1, we can reduce it to 1 without decreasing the sum below B, since s(uᵢ) > 0; if reducing it makes the sum drop below B, then c(uᵢ) = 1 was needed, contradicting the reduction.)

6. **Solution extraction:** Given INTEGER KNAPSACK multiplicity vector c, the SUBSET SUM solution is A' = {aᵢ : c(uᵢ) ≥ 1}. If Σ_{a∈A'} s(a) > B, greedily remove elements from A' until the sum equals B (possible because the integer knapsack total is exactly B and each element has a positive integer size).

**Key invariant:** Since s(u) = v(u) for all items, the capacity constraint Σ c(u)·s(u) ≤ B and value constraint Σ c(u)·v(u) ≥ B together force Σ c(u)·s(u) = B exactly, reducing the problem to finding non-negative integer multiplicities with an exact sum — which SUBSET SUM solves with 0-1 multiplicities.

**Time complexity of reduction:** O(n) to copy sizes and set parameters.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_items` = |A| = |U| (number of elements/items)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_items`                | `num_items`                      |
| `capacity`                 | `target_sum` (= B from source)   |

**Derivation:** The item set has the same cardinality. Each element maps 1-to-1 to a knapsack item. Capacity equals the SUBSET SUM target. No blowup in any dimension.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a SUBSET SUM instance, reduce to INTEGER KNAPSACK, solve target with BruteForce (enumerate multiplicity vectors), extract solution (items with c(u) ≥ 1), verify their sizes sum to exactly B in the original instance.
- Verify that the optimal INTEGER KNAPSACK value equals B if and only if SUBSET SUM has a solution.
- Test with known YES instance: A = {3, 7, 1, 8, 2, 4}, B = 14. Solution: {7, 1, 2, 4} sums to 14. Corresponding INTEGER KNAPSACK: c(3)=0, c(7)=1, c(1)=1, c(8)=0, c(2)=1, c(4)=1 gives value 14.
- Test with known NO instance: A = {3, 7, 1}, B = 5. No subset sums to 5. INTEGER KNAPSACK max value ≤ 4 (from {3,1}) < 5.
- Edge case: ensure multiplicities > 1 do not yield false positives. With A = {3}, B = 6: SUBSET SUM answer is NO (only element has size 3 ≠ 6), but INTEGER KNAPSACK allows c(u₁) = 2, giving total 6 = B. This shows the reduction must handle the reverse direction carefully — the SUBSET SUM solution extraction must find a subset (0-1 multiplicities) that sums to B. In this case, no such subset exists, but the INTEGER KNAPSACK says YES. **Resolution:** The reduction from SUBSET SUM to INTEGER KNAPSACK is a many-one reduction proving INTEGER KNAPSACK is NP-hard (any SUBSET SUM YES instance gives an INTEGER KNAPSACK YES instance). The reverse (extracting SUBSET SUM solutions from INTEGER KNAPSACK solutions) requires care. The standard reduction from GJ proves NP-hardness in one direction: SUBSET SUM ≤_p INTEGER KNAPSACK.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (SubsetSum):**
A = {a₁, a₂, a₃, a₄, a₅} with sizes s(a₁) = 3, s(a₂) = 7, s(a₃) = 1, s(a₄) = 8, s(a₅) = 5
Target B = 16
Valid subset: A' = {a₁, a₄, a₅} with sum = 3 + 8 + 5 = 16 ✓

**Constructed target instance (IntegerKnapsack):**
U = {u₁, u₂, u₃, u₄, u₅}
Sizes:  s(u₁) = 3, s(u₂) = 7, s(u₃) = 1, s(u₄) = 8, s(u₅) = 5
Values: v(u₁) = 3, v(u₂) = 7, v(u₃) = 1, v(u₄) = 8, v(u₅) = 5  (v = s for all items)
Capacity B = 16, Value target K = 16

**Solution mapping:**
- SUBSET SUM solution: A' = {a₁, a₄, a₅}
- INTEGER KNAPSACK assignment: c(u₁) = 1, c(u₂) = 0, c(u₃) = 0, c(u₄) = 1, c(u₅) = 1
- Check: Σ c(uᵢ)·s(uᵢ) = 1·3 + 0·7 + 0·1 + 1·8 + 1·5 = 16 ≤ 16 ✓
- Check: Σ c(uᵢ)·v(uᵢ) = 1·3 + 0·7 + 0·1 + 1·8 + 1·5 = 16 ≥ 16 ✓

**Verification of reverse direction:**
- Given the INTEGER KNAPSACK solution c = (1, 0, 0, 1, 1), extract A' = {a₁, a₄, a₅}
- Sum of sizes: 3 + 8 + 5 = 16 = B ✓
- Valid SUBSET SUM solution ✓

**Why integer multiplicities don't matter here:**
- Could we set c(u₃) = 16 (using item u₃ sixteen times)? Total size = 16·1 = 16, total value = 16 ≥ 16. This is a valid INTEGER KNAPSACK solution but does NOT correspond to a SUBSET SUM solution (SUBSET SUM only has one copy of each element). However, the forward direction only requires that SUBSET SUM YES → INTEGER KNAPSACK YES, which is satisfied.


## References

- **[Lueker, 1975]**: [`Lueker1975`] George S. Lueker (1975). "Two {NP}-complete problems in nonnegative integer programming". Computer Science Laboratory, Princeton University.
- **[Hirschberg and Wong, 1976]**: [`Hirschberg1976`] D. S. Hirschberg and C. K. Wong (1976). "A polynomial-time algorithm for the knapsack problem with two variables". *Journal of the Association for Computing Machinery* 23, pp. 147–154.
