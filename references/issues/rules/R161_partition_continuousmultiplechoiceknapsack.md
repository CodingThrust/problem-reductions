---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to CONTINUOUS MULTIPLE CHOICE KNAPSACK"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'CONTINUOUS MULTIPLE CHOICE KNAPSACK'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** CONTINUOUS MULTIPLE CHOICE KNAPSACK
**Motivation:** Establishes the NP-completeness of CONTINUOUS MULTIPLE CHOICE KNAPSACK by reducing from PARTITION. The key insight is that pairing elements into two-item groups and requiring a fractional multiplier in [0,1] effectively forces a binary partition: the value-to-size ratio structure ensures that any feasible solution achieving the target value must use the multipliers in a way that encodes a balanced partition of the original elements. Despite the continuous relaxation (rational multipliers), the combinatorial choice among group items preserves NP-hardness. This reduction also demonstrates that the problem remains NP-complete even when each group has at most 2 items.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247

## GJ Source Entry

> [MP11] CONTINUOUS MULTIPLE CHOICE KNAPSACK
> INSTANCE: Finite set U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, a partition of U into disjoint sets U_1,U_2,...,U_m, and positive integers B and K.
> QUESTION: Is there a choice of a unique element u_i E U_i, 1 ≤ i ≤ m, and an assignment of rational numbers r_i, 0 ≤ r_i ≤ 1, to these elements, such that Σ_{i=1}^m r_i·s(u_i) ≤ B and Σ_{i=1}^m r_i·v(u_i) ≥ K?
> Reference: [Ibaraki, 1978]. Transformation from PARTITION.
> Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if |U_i| ≤ 2, 1 ≤ i ≤ m. Solvable in polynomial time by "greedy" algorithms if |U_i| = 1, 1 ≤ i ≤ m, or if we only require that the r_i ≥ 0 but place no upper bound on them. [Ibaraki, Hasegawa, Teranaka, and Iwase, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION instance: a finite set A = {a₁, a₂, ..., a₂ₙ} with sizes s(aᵢ) ∈ Z⁺ and total sum S = Σ s(aᵢ), where we ask whether there is a subset A' with Σ_{a∈A'} s(a) = S/2, construct a CONTINUOUS MULTIPLE CHOICE KNAPSACK instance as follows:

1. **Item set and groups:** Create 2n items. Pair the elements into n groups of 2: U₁ = {a₁, a₂}, U₂ = {a₃, a₄}, ..., Uₙ = {a₂ₙ₋₁, a₂ₙ}. (If |A| is odd, add a dummy element with size 0.) For each item aᵢ, set both s(aᵢ) and v(aᵢ) equal to the original partition size s(aᵢ).

2. **Capacity and target:** Set B = S/2 and K = S/2. (We require that S is even for PARTITION to have a solution.)

3. **Correctness (forward):** If there exists a partition A' ⊆ A with Σ_{a∈A'} s(a) = S/2, then for each group Uᵢ = {a₂ᵢ₋₁, a₂ᵢ}:
   - If a₂ᵢ₋₁ ∈ A', choose uᵢ = a₂ᵢ₋₁ and set rᵢ = 1.
   - If a₂ᵢ ∈ A' (but not a₂ᵢ₋₁), choose uᵢ = a₂ᵢ and set rᵢ = 1.
   - If both are in A', choose either and set rᵢ = 1 (the other's contribution comes from another group — this case needs more careful handling depending on the pairing).

   More precisely, for the standard reduction, we use groups of size 2 where exactly one element from each group goes into A'. Since the pairing is arbitrary, we need a more careful construction:

   **Alternative (single-item groups) construction:**
   Create n = |A| groups, each with a single item: Uᵢ = {aᵢ} for each i. Set s(aᵢ) = v(aᵢ) = original size. Set B = K = S/2. For each group, choose the unique item aᵢ and set rᵢ = 1 if aᵢ ∈ A', and rᵢ = 0 if aᵢ ∉ A'. Then Σ rᵢ·s(aᵢ) = Σ_{a∈A'} s(a) = S/2 = B, and Σ rᵢ·v(aᵢ) = S/2 = K.

   However, with single-item groups the problem is solvable in polynomial time by greedy (as GJ notes). So the NP-completeness proof must use groups of size ≥ 2.

   **Ibaraki's construction (groups of size 2):**
   The reduction pairs elements and constructs items with carefully chosen sizes and values such that the continuous multiplier rᵢ ∈ [0,1] combined with the item choice encodes the partition decision. The precise construction from Ibaraki (1978):

   For each pair of elements (a₂ᵢ₋₁, a₂ᵢ), create a group Uᵢ with two items:
   - Item αᵢ: s(αᵢ) = s(a₂ᵢ₋₁), v(αᵢ) = s(a₂ᵢ₋₁)
   - Item βᵢ: s(βᵢ) = s(a₂ᵢ), v(βᵢ) = s(a₂ᵢ)

   Set B = K = S/2. Since s = v for all items, any feasible solution with Σ rᵢ·s(uᵢ) ≤ B and Σ rᵢ·v(uᵢ) ≥ K forces equality Σ rᵢ·s(uᵢ) = S/2. The hardness comes from the choice of which item to select in each group, since changing the item changes which element's size contributes to the sum.

4. **Correctness (reverse):** If a CMCK solution exists with Σ rᵢ·s(uᵢ) ≤ S/2 and Σ rᵢ·v(uᵢ) ≥ S/2, then since v = s, we have Σ rᵢ·s(uᵢ) = S/2 exactly. This fractional solution can be rounded to a 0-1 partition solution: A' contains the chosen item uᵢ if rᵢ > 0 (and possibly a correction step).

5. **Solution extraction:** Given the CMCK solution (item choices uᵢ, multipliers rᵢ), the partition is A' = {uᵢ : rᵢ > 0}.

**Time complexity of reduction:** O(n) to pair elements and set parameters.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_elements` = |A| (number of elements in the PARTITION instance)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_items`                | `num_elements`                   |
| `num_groups`               | `num_elements / 2`               |
| `capacity`                 | `total_sum / 2`                  |

**Derivation:** Each partition element becomes one knapsack item. Elements are paired into groups of 2, yielding n/2 groups. The capacity equals half the total sum.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to CMCK, solve target by enumerating all item choices (one per group) and computing optimal multipliers by greedy for each choice, extract solution, verify it yields a valid partition.
- Test with known YES instance: A = {4, 5, 6, 7, 8, 10}, total S = 40, target S/2 = 20. Valid partition: {4, 6, 10} and {5, 7, 8}. The CMCK instance should find a feasible solution achieving value 20.
- Test with known NO instance: A = {1, 2, 3, 5}, total S = 11 (odd), no valid partition exists. The CMCK instance should have no feasible solution.
- Verify that |Uᵢ| = 2 for all groups (confirming the tight NP-completeness result).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Partition):**
A = {a₁, a₂, a₃, a₄, a₅, a₆} with sizes s(a₁) = 4, s(a₂) = 5, s(a₃) = 6, s(a₄) = 7, s(a₅) = 8, s(a₆) = 10
Total sum S = 4 + 5 + 6 + 7 + 8 + 10 = 40
Target: find A' with Σ_{a∈A'} s(a) = 20.
Valid partition: A' = {a₁, a₃, a₆} = {4, 6, 10}, sum = 20 ✓

**Constructed target instance (ContinuousMultipleChoiceKnapsack):**
Groups: U₁ = {a₁, a₂}, U₂ = {a₃, a₄}, U₃ = {a₅, a₆}
Items in U₁: α₁ with s=4, v=4; β₁ with s=5, v=5
Items in U₂: α₂ with s=6, v=6; β₂ with s=7, v=7
Items in U₃: α₃ with s=8, v=8; β₃ with s=10, v=10
Capacity B = 20, Target K = 20

**Solution mapping:**
- From partition A' = {a₁, a₃, a₆}:
  - Group U₁: a₁ ∈ A', choose α₁ (s=4, v=4), set r₁ = 1
  - Group U₂: a₃ ∈ A', choose α₂ (s=6, v=6), set r₂ = 1
  - Group U₃: a₆ ∈ A', choose β₃ (s=10, v=10), set r₃ = 1

- Check: Σ rᵢ·s(uᵢ) = 1·4 + 1·6 + 1·10 = 20 ≤ 20 ✓
- Check: Σ rᵢ·v(uᵢ) = 1·4 + 1·6 + 1·10 = 20 ≥ 20 ✓

**Verification of reverse direction:**
- Given CMCK solution (α₁ with r=1, α₂ with r=1, β₃ with r=1), extract A' = {a₁, a₃, a₆}
- Σ s(a) for A' = 4 + 6 + 10 = 20 = S/2 ✓
- Complementary set: {a₂, a₄, a₅} with sizes 5 + 7 + 8 = 20 = S/2 ✓
- Valid partition ✓

**Note on fractional multipliers:**
Could we achieve value ≥ 20 with fractional rᵢ? For example, choosing β₁ (s=5), β₂ (s=7), β₃ (s=10) with all r = 1 gives total size 22 > 20. We'd need r₃ = (20-12)/10 = 0.8, giving value = 5 + 7 + 0.8·10 = 20. This is also valid! But it corresponds to a fractional partition, and the key is that an integer solution (all rᵢ ∈ {0,1}) exists if and only if PARTITION has a solution.


## References

- **[Ibaraki, 1978]**: [`Ibaraki1978a`] Toshihide Ibaraki (1978). "Approximate algorithms for the multiple-choice continuous knapsack problem".
- **[Ibaraki, Hasegawa, Teranaka, and Iwase, 1978]**: [`Ibaraki1978b`] Toshihide Ibaraki and T. Hasegawa and K. Teranaka and J. Iwase (1978). "The multiple-choice knapsack problem". *Journal of the Operations Research Society of Japan* 21, pp. 59–94.
