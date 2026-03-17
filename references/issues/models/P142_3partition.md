---
name: Problem
about: Propose a new problem type
title: "[Model] 3Partition"
labels: model
assignees: ''
---

## Motivation

3-PARTITION (P142) from Garey & Johnson, A3 SP15. A classical strongly NP-complete problem: given 3m positive integers with each between B/4 and B/2 and total sum mB, can they be partitioned into m triples each summing to B? Unlike the standard PARTITION problem (which is only weakly NP-hard), 3-PARTITION has no pseudo-polynomial-time algorithm unless P = NP. This makes it the canonical source for strong NP-completeness reductions, especially to scheduling and packing problems.

**Associated rules (as source):**
- R66: 3-Partition -> Intersection Graph for Segments on a Grid
- R67: 3-Partition -> Edge Embedding on a Grid
- R80: 3DM -> 3-Partition (3-Partition as target)
- R88: 3-Partition -> Dynamic Storage Allocation
- R98: Partition / 3-Partition -> Expected Retrieval Cost
- R131: 3-Partition -> Sequencing with Release Times and Deadlines
- R135: 3-Partition -> Sequencing to Minimize Weighted Tardiness
- R139: 3-Partition -> Resource Constrained Scheduling
- R144: 3-Partition -> Flow-Shop Scheduling
- R147: 3-Partition -> Job-Shop Scheduling
- R232: 3-Partition -> Bandwidth
- R233: 3-Partition -> Directed Bandwidth
- R234: 3-Partition -> Weighted Diameter

<!-- ⚠️ Unverified: AI-generated motivation and associated rules list -->

## Definition

**Name:** `ThreePartition`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP15

**Mathematical definition:**

INSTANCE: Set A of 3m elements, a bound B in Z^+, and a size s(a) in Z^+ for each a in A such that B/4 < s(a) < B/2 and such that sum_{a in A} s(a) = mB.
QUESTION: Can A be partitioned into m disjoint sets A_1,A_2,...,A_m such that, for 1 <= i <= m, sum_{a in A_i} s(a) = B (note that each A_i must therefore contain exactly three elements from A)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 3m (one variable per element, assigning it to a group)
- **Per-variable domain:** {1, 2, ..., m} -- the group index to which the element is assigned
- **Meaning:** g_i in {1,...,m} is the group for element a_i. The assignment is feasible iff each group contains exactly 3 elements and sum_{j: g_j = k} s(a_j) = B for all k in {1,...,m}.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ThreePartition`
**Variants:** none (no type parameters; sizes and bound are plain positive integers)

| Field   | Type       | Description                                              |
|---------|------------|----------------------------------------------------------|
| `sizes` | `Vec<u64>` | Positive integer size s(a) for each element a in A       |
| `bound` | `u64`      | Target sum B for each triple                             |

Note: The constraint B/4 < s(a) < B/2 and sum = mB are invariants that should be validated on construction. The number of groups m = sizes.len() / 3.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** 3-PARTITION is strongly NP-complete, meaning no pseudo-polynomial-time algorithm exists unless P = NP. The naive brute-force approach enumerates all ways to partition 3m elements into m triples, which is O((3m)! / (3!)^m / m!) -- exponential. A dynamic programming approach over subset sums has time complexity O(n * B^(m-1)), which is pseudo-polynomial (and thus not polynomial for strongly NP-hard problems). The meet-in-the-middle technique can reduce the base of the exponential. For practical purposes, the complexity is dominated by the strong NP-completeness: no algorithm with running time polynomial in the input size (even when numbers are encoded in unary) is known. [Garey & Johnson, 1975; Garey & Johnson, *Computers and Intractability*, 1979.]

## Extra Remark

**Full book text:**

INSTANCE: Set A of 3m elements, a bound B in Z^+, and a size s(a) in Z^+ for each a in A such that B/4 < s(a) < B/2 and such that sum_{a in A} s(a) = mB.
QUESTION: Can A be partitioned into m disjoint sets A_1,A_2,...,A_m such that, for 1 <= i <= m, sum_{a in A_i} s(a) = B (note that each A_i must therefore contain exactly three elements from A)?
Reference: [Garey and Johnson, 1975]. Transformation from 3DM (see Section 4.2).
Comment: NP-complete in the strong sense.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all partitions of 3m elements into m triples; check if each triple sums to B.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{i,k} in {0,1}, sum_k x_{i,k} = 1 for each i, sum_i x_{i,k} = 3 for each k, sum_i x_{i,k} * s(a_i) = B for each k.)
- [ ] Other: Dynamic programming over subset sums (pseudo-polynomial, O(n * B^(m-1))).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
A = {a_1, ..., a_6} (3m = 6 elements, so m = 2)
B = 12
Sizes: s(a_1) = 4, s(a_2) = 5, s(a_3) = 3, s(a_4) = 4, s(a_5) = 4, s(a_6) = 4
- All sizes satisfy B/4 = 3 < s(a_i) < B/2 = 6
- Total sum = 4 + 5 + 3 + 4 + 4 + 4 = 24 = 2 * 12 = mB

**Feasible assignment:**
A_1 = {a_1, a_2, a_3} = {4, 5, 3} (sum = 12 = B)
A_2 = {a_4, a_5, a_6} = {4, 4, 4} (sum = 12 = B)

Answer: YES -- a valid 3-partition exists.
