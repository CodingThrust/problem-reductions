---
name: Problem
about: Propose a new problem type
title: "[Model] ContinuousMultipleChoiceKnapsack"
labels: model
assignees: ''
---

## Motivation

CONTINUOUS MULTIPLE CHOICE KNAPSACK (P217) from Garey & Johnson, A6 MP11. An NP-complete knapsack variant that combines two generalizations: (1) items are partitioned into groups U₁, ..., Uₘ from which exactly one item per group must be chosen, and (2) the chosen item can be used with a fractional (rational) multiplier rᵢ ∈ [0, 1], blending discrete choice with continuous allocation. This problem arises in resource allocation where one must select among alternative resources and determine what fraction to deploy. Despite the continuous relaxation, the combinatorial choice among groups keeps the problem NP-complete. Solvable in pseudo-polynomial time, and polynomial time if each group has exactly one item (reduces to a continuous knapsack, solvable by greedy) or if the rᵢ are unbounded.

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R161: PARTITION -> CONTINUOUS MULTIPLE CHOICE KNAPSACK (establishes NP-completeness via Ibaraki 1978)

## Definition

**Name:** `ContinuousMultipleChoiceKnapsack`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP11

**Mathematical definition:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, a partition of U into disjoint sets U₁, U₂, ..., Uₘ, and positive integers B and K.
QUESTION: Is there a choice of a unique element uᵢ ∈ Uᵢ, 1 ≤ i ≤ m, and an assignment of rational numbers rᵢ, 0 ≤ rᵢ ≤ 1, to these elements, such that Σᵢ₌₁ᵐ rᵢ·s(uᵢ) ≤ B and Σᵢ₌₁ᵐ rᵢ·v(uᵢ) ≥ K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The decision has two components: (a) m discrete choice variables (one per group, selecting which item to use), and (b) m continuous multiplier variables rᵢ ∈ [0, 1]. For a codebase model with discretized domains, the choice variables dominate.
- **Per-variable domain (discrete part):** For group i, the choice variable selects from {0, 1, ..., |Uᵢ|-1}, indexing the items in group Uᵢ.
- **Per-variable domain (continuous part):** rᵢ ∈ [0, 1] (rational). For brute-force or enumeration, this can be discretized to {0, 1/D, 2/D, ..., 1} for some resolution D, or handled analytically: given a fixed set of item choices, the optimal rᵢ values can be computed by a greedy/LP approach.
- **Meaning:** For each group Uᵢ, choose exactly one item uᵢ and a fractional amount rᵢ ∈ [0,1]. The total size Σ rᵢ·s(uᵢ) must not exceed B, and total value Σ rᵢ·v(uᵢ) must be at least K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ContinuousMultipleChoiceKnapsack`
**Variants:** none

| Field      | Type              | Description                                                   |
|------------|-------------------|---------------------------------------------------------------|
| `sizes`    | `Vec<i64>`        | Size s(u) for each item u ∈ U                                |
| `values`   | `Vec<i64>`        | Value v(u) for each item u ∈ U                               |
| `groups`   | `Vec<Vec<usize>>` | Partition of items into groups: groups[i] lists item indices in Uᵢ |
| `capacity` | `i64`             | Knapsack capacity B                                           |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`.
- The continuous multipliers rᵢ are not part of the combinatorial configuration but are determined analytically once the discrete item choices are fixed. Given a fixed selection of items (one per group), the optimal multipliers can be found by a simple greedy algorithm (sort by value-to-size ratio, fill greedily).
- Key getter methods needed: `num_items()` (= |U|), `num_groups()` (= m), `capacity()` (= B).
- The problem remains NP-complete even if |Uᵢ| ≤ 2 for all groups.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Ibaraki, 1978; transformation from PARTITION). Remains NP-complete even if |Uᵢ| ≤ 2 for all groups.
- **Best known exact algorithm:** Pseudo-polynomial time dynamic programming. For a fixed set of item choices (one per group), the continuous multipliers can be optimized in O(m log m) time by greedy. The full problem requires enumerating over the Πᵢ|Uᵢ| possible item selections; DP approaches can reduce this to pseudo-polynomial time in the capacity B.
- **Special cases solved in polynomial time:**
  - |Uᵢ| = 1 for all i: reduces to continuous knapsack, solvable by greedy in O(m log m) time.
  - rᵢ ≥ 0 with no upper bound: solvable in polynomial time (Ibaraki, Hasegawa, Teranaka, and Iwase, 1978).
- **References:**
  - T. Ibaraki (1978). "Approximate algorithms for the multiple-choice continuous knapsack problem."
  - T. Ibaraki, T. Hasegawa, K. Teranaka, J. Iwase (1978). "The multiple-choice knapsack problem." *J. Operations Research Soc. Japan* 21, pp. 59–94.

## Extra Remark

**Full book text:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, a partition of U into disjoint sets U₁,U₂,...,Uₘ, and positive integers B and K.
QUESTION: Is there a choice of a unique element uᵢ ∈ Uᵢ, 1 ≤ i ≤ m, and an assignment of rational numbers rᵢ, 0 ≤ rᵢ ≤ 1, to these elements, such that Σᵢ₌₁ᵐ rᵢ·s(uᵢ) ≤ B and Σᵢ₌₁ᵐ rᵢ·v(uᵢ) ≥ K?

Reference: [Ibaraki, 1978]. Transformation from PARTITION.
Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if |Uᵢ| ≤ 2, 1 ≤ i ≤ m. Solvable in polynomial time by "greedy" algorithms if |Uᵢ| = 1, 1 ≤ i ≤ m, or if we only require that the rᵢ ≥ 0 but place no upper bound on them. [Ibaraki, Hasegawa, Teranaka, and Iwase, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all combinations of item choices (one per group); for each combination, solve the continuous allocation by greedy; check if total value ≥ K.)
- [x] It can be solved by reducing to integer programming. (Mixed-integer program: binary variables y_{i,j} = 1 if item j chosen in group i, with Σⱼ y_{i,j} = 1 for each group; continuous variables 0 ≤ rᵢ ≤ 1; linearize products rᵢ·y_{i,j} via big-M or McCormick envelopes.)
- [ ] Other: Pseudo-polynomial DP; for fixed item choices, greedy in O(m log m).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
U = {a, b, c, d, e, f} (n = 6 items)
Sizes:  s(a) = 4, s(b) = 6, s(c) = 3, s(d) = 8, s(e) = 5, s(f) = 7
Values: v(a) = 5, v(b) = 9, v(c) = 4, v(d) = 10, v(e) = 6, v(f) = 8
Groups: U₁ = {a, b} (indices 0,1), U₂ = {c, d} (indices 2,3), U₃ = {e, f} (indices 4,5)
m = 3 groups
Capacity B = 10, Target K = 15

**Solution:** Choose u₁ = b from U₁, u₂ = d from U₂, u₃ = e from U₃.
Multipliers: r₁ = 1.0, r₂ = 0.5, r₃ = 0.0
- Total size: 1.0·6 + 0.5·8 + 0.0·5 = 6 + 4 + 0 = 10 ≤ 10 ✓
- Total value: 1.0·9 + 0.5·10 + 0.0·6 = 9 + 5 + 0 = 14 < 15 ✗

Adjust: r₁ = 0.5, r₂ = 0.5, r₃ = 1.0
- Total size: 0.5·6 + 0.5·8 + 1.0·5 = 3 + 4 + 5 = 12 > 10 ✗

Adjust: Choose u₁ = b, u₂ = d, u₃ = f. Multipliers: r₁ = 1.0, r₂ = 0.5, r₃ = 0.0
- Same as above, value = 14.

Better: Choose u₁ = b, u₂ = d, u₃ = f. Ratios: v/s = 9/6=1.5, 10/8=1.25, 8/7≈1.14.
Greedy by ratio: fill b fully (size 6, value 9), remaining capacity 4. Fill d partially: r₂ = 4/8 = 0.5, value = 0.5·10 = 5. Total value = 9 + 5 = 14. Still < 15.

Choose u₁ = a, u₂ = d, u₃ = f. Ratios: 5/4=1.25, 10/8=1.25, 8/7≈1.14.
Fill a fully (size 4, value 5), fill d with r₂ = 6/8 = 0.75 (size 6, value 7.5). Total size = 10, total value = 12.5 < 15.

**Revised instance (achievable):**
Same items, B = 12, K = 15.
Choose u₁ = b (ratio 1.5), u₂ = d (ratio 1.25), u₃ = f (ratio 1.14).
Greedy: fill b fully (size 6, value 9), remaining 6. Fill d: r₂ = 6/8 = 0.75, value = 7.5. Total value = 16.5 ≥ 15 ✓
Total size = 6 + 0.75·8 = 6 + 6 = 12 ≤ 12 ✓

Answer: YES, with choices (b, d, f) and multipliers (1.0, 0.75, 0.0 for f is unused — actually f is not needed).
Formally: u₁ = b with r₁ = 1.0, u₂ = d with r₂ = 0.75, u₃ = f with r₃ = 0.
Check: Σ rᵢ·s(uᵢ) = 1·6 + 0.75·8 + 0·7 = 12 ≤ 12 ✓
       Σ rᵢ·v(uᵢ) = 1·9 + 0.75·10 + 0·8 = 16.5 ≥ 15 ✓
