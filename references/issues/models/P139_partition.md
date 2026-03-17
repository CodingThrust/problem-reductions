---
name: Problem
about: Propose a new problem type
title: "[Model] Partition"
labels: model
assignees: ''
---

## Motivation

PARTITION (P139) from Garey & Johnson, A3 SP12. A classical NP-complete problem useful for reductions to KNAPSACK, MULTIPROCESSOR SCHEDULING, SEQUENCING WITHIN INTERVALS, and many other problems. Though NP-complete, PARTITION is only weakly NP-hard: it admits a pseudo-polynomial dynamic-programming algorithm running in O(n · B_total) time, making it tractable when element sizes are small.

## Definition

**Name:** `Partition`
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP12

**Mathematical definition:**

INSTANCE: Finite set A and a size s(a) ∈ Z^+ for each a ∈ A.
QUESTION: Is there a subset A' ⊆ A such that Σ_{a ∈ A'} s(a) = Σ_{a ∈ A−A'} s(a)?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |A| (one binary variable per element)
- **Per-variable domain:** {0, 1} — 0 means element goes to the first part, 1 means it goes to the second part
- **Meaning:** x_i = 0 if a_i ∈ A', x_i = 1 if a_i ∈ A \ A'. The problem is feasible iff Σ_{i: x_i=0} s(a_i) = Σ_{i: x_i=1} s(a_i) = B_total / 2.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `Partition`
**Variants:** none (no type parameters; sizes are plain positive integers)

| Field    | Type        | Description                                           |
|----------|-------------|-------------------------------------------------------|
| `sizes`  | `Vec<u64>`  | Positive integer size s(a) for each element a ∈ A     |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The Schroeppel–Shamir meet-in-the-middle algorithm (1981) solves PARTITION (via SUBSET SUM) in time O*(2^(n/2)) and space O*(2^(n/4)). The naive brute-force approach is O(2^n). [Schroeppel & Shamir, SIAM J. Comput. 10(4):456–464, 1981.]

## Extra Remark

**Full book text:**

INSTANCE: Finite set A and a size s(a) ∈ Z^+ for each a ∈ A.
QUESTION: Is there a subset A' ⊆ A such that Σ_{a ∈ A'} s(a) = Σ_{a ∈ A−A'} s(a)?
Reference: [Karp, 1972]. Transformation from 3DM (see Section 3.1.5).
Comment: Remains NP-complete even if we require that |A'| = |A|/2, or if the elements in A are ordered as a_1,a_2,…,a_{2n} and we require that A' contain exactly one of a_{2i−1},a_{2i} for 1 ≤ i ≤ n. However, all these problems can be solved in pseudo-polynomial time by dynamic programming (see Section 4.2).

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2^n subsets; check if any half sums to B_total / 2.)
- [x] It can be solved by reducing to integer programming. (Binary ILP with constraint Σ x_i · s(a_i) = B_total / 2.)
- [ ] Other: Pseudo-polynomial DP in O(n · B_total) time and O(B_total) space (standard subset-sum DP table).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
A = {3, 1, 1, 2, 2, 1} (n = 6 elements)
s(a_1) = 3, s(a_2) = 1, s(a_3) = 1, s(a_4) = 2, s(a_5) = 2, s(a_6) = 1
Total sum = 10; target half-sum = 5.

**Feasible assignment:**
A' = {a_1, a_4} = {3, 2} (sum = 5)
A \ A' = {a_2, a_3, a_5, a_6} = {1, 1, 2, 1} (sum = 5)

Answer: YES — a balanced partition exists.
