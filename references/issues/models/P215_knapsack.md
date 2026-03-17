---
name: Problem
about: Propose a new problem type
title: "[Model] Knapsack"
labels: model
assignees: ''
---

## Motivation

KNAPSACK (P215) from Garey & Johnson, A6 MP9. One of Karp's original 21 NP-complete problems, established by reduction from PARTITION. The decision problem asks whether items can be selected within a weight budget to exceed a value threshold. Though NP-complete, KNAPSACK is only weakly NP-hard and admits a pseudo-polynomial DP algorithm. It is also one of the best-known problems with an FPTAS.

## Definition

**Name:** `Knapsack`
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP9

**Mathematical definition:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that Σᵤ∈U' s(u) ≤ B and such that Σᵤ∈U' v(u) ≥ K?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |U| (one binary variable per item)
- **Per-variable domain:** {0, 1} — 0 means item is excluded, 1 means item is included in the knapsack
- **Meaning:** x_u = 1 if u ∈ U', 0 otherwise. Feasibility requires Σ x_u · s(u) ≤ B and Σ x_u · v(u) ≥ K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `Knapsack`
**Variants:** none (no type parameters; sizes and values are plain positive integers)

| Field          | Type        | Description                                               |
|----------------|-------------|-----------------------------------------------------------|
| `sizes`        | `Vec<u64>`  | Size s(u) of each item u ∈ U                              |
| `values`       | `Vec<u64>`  | Value v(u) of each item u ∈ U                             |
| `budget`       | `u64`       | Weight budget B (total size of selected items must be ≤ B)|
| `target_value` | `u64`       | Value threshold K (total value must be ≥ K)               |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The Horowitz–Sahni meet-in-the-middle algorithm (1974) solves KNAPSACK in time O*(2^(n/2)) and space O(2^(n/2)). The Schroeppel–Shamir improvement (1981) achieves O*(2^(n/2)) time with only O*(2^(n/4)) space. The naive brute-force approach is O(2^n). [Horowitz & Sahni, J. ACM 21(2):277–292, 1974; Schroeppel & Shamir, SIAM J. Comput. 10(4):456–464, 1981.]

## Extra Remark

**Full book text:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that Σᵤ∈U' s(u) ≤ B and such that Σᵤ∈U' v(u) ≥ K?

Reference: [Karp, 1972]. Transformation from PARTITION.
Comment: Remains NP-complete if s(u) = v(u) for all u ∈ U (SUBSET SUM). Can be solved in pseudo-polynomial time by dynamic programming (e.g., see [Dantzig, 1957] or [Lawler, 1976a]).

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2^n subsets; check size ≤ B and value ≥ K.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: maximize Σ v(u) x_u subject to Σ s(u) x_u ≤ B, x_u ∈ {0,1}.)
- [ ] Other: Pseudo-polynomial DP in O(n · B) time and O(B) space (standard 0-1 knapsack DP table).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
Items U = {u_1, u_2, u_3, u_4, u_5, u_6} (n = 6)

| Item | Size s(u) | Value v(u) |
|------|-----------|------------|
| u_1  | 3         | 4          |
| u_2  | 5         | 5          |
| u_3  | 2         | 3          |
| u_4  | 7         | 7          |
| u_5  | 1         | 2          |
| u_6  | 4         | 4          |

Budget B = 10, Target value K = 12.

**Feasible assignment:**
U' = {u_1, u_3, u_5, u_6}: sizes 3+2+1+4 = 10 ≤ B ✓, values 4+3+2+4 = 13 ≥ K ✓

Answer: YES — a valid selection exists.
