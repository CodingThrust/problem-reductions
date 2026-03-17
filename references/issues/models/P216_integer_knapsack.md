---
name: Problem
about: Propose a new problem type
title: "[Model] IntegerKnapsack"
labels: model
assignees: ''
---

## Motivation

INTEGER KNAPSACK (P216) from Garey & Johnson, A6 MP10. A classical NP-complete problem that generalizes the standard 0-1 Knapsack by allowing each item to be used with any non-negative integer multiplicity c(u), rather than just 0 or 1. This is also known as the "unbounded knapsack problem" in much of the literature. It remains NP-complete even in the special case where s(u) = v(u) for all items, which connects it directly to SUBSET SUM. Solvable in pseudo-polynomial time by dynamic programming, and polynomial time when |U| = 2 (Hirschberg and Wong, 1976).

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R160: SUBSET SUM -> INTEGER KNAPSACK (establishes NP-completeness via Lueker 1975)

## Definition

**Name:** `IntegerKnapsack`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP10

**Mathematical definition:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there an assignment of a non-negative integer c(u) to each u ∈ U such that Σᵤ∈U c(u)·s(u) ≤ B and such that Σᵤ∈U c(u)·v(u) ≥ K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |U| (one variable per item in the set U)
- **Per-variable domain:** {0, 1, 2, ..., floor(B / s(u))} — non-negative integers, bounded above by floor(B / min_size) in the worst case. For the codebase representation, we use a discretized domain where each variable c(u) ranges from 0 to floor(B / s(u)).
- **Meaning:** c(u) is the non-negative integer multiplicity assigned to item u. The assignment is feasible if the total size Σ c(u)·s(u) ≤ B, and the objective is to maximize total value Σ c(u)·v(u). The decision version asks whether total value ≥ K is achievable.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `IntegerKnapsack`
**Variants:** none (operates on a generic item set with sizes and values)

| Field      | Type          | Description                                          |
|------------|---------------|------------------------------------------------------|
| `sizes`    | `Vec<i64>`    | Size s(u) for each item u ∈ U                       |
| `values`   | `Vec<i64>`    | Value v(u) for each item u ∈ U                      |
| `capacity` | `i64`         | Knapsack capacity B (total size budget)              |

**Notes:**
- This is a satisfaction problem in the GJ decision form (`Metric = bool`), or can be modeled as an optimization problem maximizing total value subject to capacity, with `Metric = SolutionSize<i64>`.
- Key difference from the existing `Knapsack` (0-1 Knapsack): each item can be used with integer multiplicity c(u) ≥ 0, not just 0 or 1. The configuration space per variable is larger: dims() returns `[floor(B/s(0))+1, floor(B/s(1))+1, ...]` instead of `[2, 2, ..., 2]`.
- Key getter methods needed: `num_items()` (= |U|), `capacity()` (= B).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Lueker, 1975; transformation from SUBSET SUM). Remains NP-complete even when s(u) = v(u) for all u. NP-complete in the strong sense.
- **Best known exact algorithm:** Dynamic programming in O(n · B) time and O(B) space, where n = |U| and B is the capacity. This is pseudo-polynomial time. The DP recurrence is: dp[w] = max over all u of (dp[w - s(u)] + v(u)) for w = 1, ..., B, with dp[0] = 0. Each item can be used multiple times naturally in this formulation.
- **Special case |U| = 2:** Solvable in polynomial time (Hirschberg and Wong, 1976) via a number-theoretic algorithm exploiting the structure of linear Diophantine equations.
- **Approximation:** Admits an FPTAS. For any ε > 0, a (1-ε)-approximation can be computed in O(n / ε) time using LP relaxation and rounding techniques.
- **References:**
  - G. S. Lueker (1975). "Two NP-complete problems in nonnegative integer programming." Computer Science Laboratory, Princeton University. Original NP-completeness proof.
  - D. S. Hirschberg and C. K. Wong (1976). "A polynomial-time algorithm for the knapsack problem with two variables." *JACM* 23, pp. 147–154.

## Extra Remark

**Full book text:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there an assignment of a non-negative integer c(u) to each u ∈ U such that Σᵤ∈U c(u)·s(u) ≤ B and such that Σᵤ∈U c(u)·v(u) ≥ K?

Reference: [Lueker, 1975]. Transformation from SUBSET SUM.
Comment: Remains NP-complete if s(u) = v(u) for all u ∈ U. Solvable in pseudo-polynomial time by dynamic programming. Solvable in polynomial time if |U| = 2 [Hirschberg and Wong, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all multiplicity vectors (c(u_1), ..., c(u_n)) with 0 ≤ c(u_i) ≤ floor(B/s(u_i)); check feasibility and compute total value.)
- [x] It can be solved by reducing to integer programming. (ILP with integer variables c_i ≥ 0 for each item; constraint Σ c_i · s_i ≤ B; objective maximize Σ c_i · v_i.)
- [ ] Other: Dynamic programming in O(n·B) pseudo-polynomial time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
U = {u₁, u₂, u₃, u₄, u₅} (n = 5 items)
Sizes:  s(u₁) = 3, s(u₂) = 4, s(u₃) = 5, s(u₄) = 2, s(u₅) = 7
Values: v(u₁) = 4, v(u₂) = 5, v(u₃) = 7, v(u₄) = 3, v(u₅) = 9
Capacity B = 15, Target K = 20

**Solution:** c(u₁) = 0, c(u₂) = 0, c(u₃) = 3, c(u₄) = 0, c(u₅) = 0
- Total size: 0·3 + 0·4 + 3·5 + 0·2 + 0·7 = 15 ≤ 15 ✓
- Total value: 0·4 + 0·5 + 3·7 + 0·3 + 0·9 = 21 ≥ 20 ✓

**Alternative solution:** c(u₁) = 0, c(u₂) = 0, c(u₃) = 1, c(u₄) = 5, c(u₅) = 0
- Total size: 0·3 + 0·4 + 1·5 + 5·2 + 0·7 = 15 ≤ 15 ✓
- Total value: 0·4 + 0·5 + 1·7 + 5·3 + 0·9 = 22 ≥ 20 ✓

Note how the same item u₃ can be used 3 times — this is impossible in 0-1 Knapsack.

**Negative instance:**
Same items but B = 5 and K = 20.
Best: c(u₃) = 1 gives value 7; c(u₄) = 2, c(u₁) = 0 gives size 4, value 6... No combination with total size ≤ 5 can achieve value ≥ 20. Answer: NO.
