---
name: Problem
about: Propose a new problem type
title: "[Model] SubsetSum"
labels: model
assignees: ''
---

## Motivation

SUBSET SUM (P140) from Garey & Johnson, A3 SP13. One of Karp's original 21 NP-complete problems (1972). Given a set of positive integers and a target value B, the problem asks whether any subset sums to exactly B. PARTITION is the special case where B equals half the total sum. SUBSET SUM is a fundamental building block for cryptographic schemes (e.g., Merkle-Hellman knapsack) and serves as a source for many further reductions. Though NP-complete, it is solvable in pseudo-polynomial time O(nB) by dynamic programming.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R78 (PARTITION to SUBSET SUM)
- As source: R85 (SUBSET SUM to K-th LARGEST SUBSET), R101 (SUBSET SUM to CAPACITY ASSIGNMENT), R160 (SUBSET SUM to INTEGER KNAPSACK), R181 (SUBSET SUM to INTEGER EXPRESSION MEMBERSHIP)

## Definition

**Name:** `SubsetSum`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Subset Sum
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP13

**Mathematical definition:**

INSTANCE: Finite set A, size s(a) in Z^+ for each a in A, positive integer B.
QUESTION: Is there a subset A' <= A such that the sum of the sizes of the elements in A' is exactly B?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |A| (one binary variable per element)
- **Per-variable domain:** {0, 1} -- 0 means element is not in A', 1 means element is in A'
- **Meaning:** x_i = 1 if element a_i is included in the selected subset A'. The problem is feasible iff Sum_{i: x_i=1} s(a_i) = B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `SubsetSum`
**Variants:** none (sizes are plain positive integers)

| Field    | Type        | Description                                           |
|----------|-------------|-------------------------------------------------------|
| `sizes`  | `Vec<u64>`  | Positive integer size s(a) for each element a in A    |
| `target` | `u64`       | Target sum B                                          |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The Horowitz-Sahni meet-in-the-middle algorithm (1974) solves SUBSET SUM in O*(2^(n/2)) time and O*(2^(n/2)) space. Schroeppel and Shamir (1981) improved the space to O*(2^(n/4)) while maintaining the same O*(2^(n/2)) time bound. The O*(2^(n/2)) time complexity remains the best known worst-case bound for the general problem and is a major open question in exact algorithms. [Horowitz & Sahni, JACM 21(1):73-90, 1974; Schroeppel & Shamir, SIAM J. Comput. 10(3):456-464, 1981.]

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization -->

- PARTITION is the special case where B = (Sum s(a)) / 2.
- KNAPSACK generalizes SUBSET SUM by allowing separate weights and values and using an inequality constraint (Sum w(a) <= C) with an objective (maximize Sum v(a)).
- The 0-1 KNAPSACK problem with w_i = v_i for all items is equivalent to SUBSET SUM.
- Solvable in pseudo-polynomial time O(nB) by dynamic programming.

## Extra Remark

**Full book text:**

INSTANCE: Finite set A, size s(a) in Z^+ for each a in A, positive integer B.
QUESTION: Is there a subset A' <= A such that the sum of the sizes of the elements in A' is exactly B?
Reference: [Karp, 1972]. Transformation from PARTITION.
Comment: Solvable in pseudo-polynomial time (see Section 4.2).

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2^n subsets; check if any has sum exactly B.)
- [x] It can be solved by reducing to integer programming. (Binary ILP with constraint Sum x_i * s(a_i) = B.)
- [ ] Other: Pseudo-polynomial DP in O(nB) time and O(B) space; meet-in-the-middle in O*(2^(n/2)) time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
A = {a_1, a_2, a_3, a_4, a_5, a_6} with sizes s = {3, 7, 1, 8, 2, 4} (n = 6 elements)
Target B = 11.

**Feasible assignment:**
A' = {a_1, a_4} = {3, 8} (sum = 3 + 8 = 11 = B)

Another solution: A' = {a_2, a_6} = {7, 4} (sum = 7 + 4 = 11 = B)

Answer: YES -- a subset summing to exactly B = 11 exists.
