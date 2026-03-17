---
name: Problem
about: Propose a new problem type
title: "[Model] ComparativeDivisibility"
labels: model
assignees: ''
---

## Motivation

COMPARATIVE DIVISIBILITY (P223) from Garey & Johnson, A7 AN4. An NP-complete number-theoretic problem: given two sequences of positive integers, determine whether there exists a positive integer c that divides more elements of the first sequence than the second. This problem, shown NP-complete by Plaisted (1976), is notable because the nondeterminism is "hidden" -- the problem statement does not explicitly involve choosing from alternatives, yet it is NP-complete. The problem remains NP-complete even when all elements in each sequence are distinct.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** (none known in GJ appendix)
- **As target:** R167 (3SAT -> COMPARATIVE DIVISIBILITY)

## Definition

**Name:** `ComparativeDivisibility`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN4

**Mathematical definition:**

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers.
QUESTION: Is there a positive integer c such that the number of i for which c divides a_i is more than the number of j for which c divides b_j?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** 1 (the unknown positive integer c)
- **Per-variable domain:** positive integers -- in practice, c must be a divisor of at least one a_i, so the search space is bounded by max(a_1, ..., a_n)
- **Meaning:** c is a positive integer "divisor candidate." We count how many elements of the a-sequence it divides versus the b-sequence, and ask whether there exists c with a strictly favorable comparison.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ComparativeDivisibility`
**Variants:** none

| Field | Type | Description |
|-------|------|-------------|
| `a_sequence` | `Vec<u64>` | First sequence of positive integers a_1, ..., a_n |
| `b_sequence` | `Vec<u64>` | Second sequence of positive integers b_1, ..., b_m |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Brute-force: enumerate all candidate divisors c. Since c must divide at least one a_i to achieve a positive count, the candidates are all divisors of elements in the a-sequence. For each candidate c, count divisibilities in both sequences. Total time: O(sum of divisor counts of a_i * (n + m)). In the worst case, the elements can have O(sqrt(max_val)) divisors each. The problem is NP-complete because the encoded integers can have exponentially many divisors when they are products of many small primes.

## Extra Remark

**Full book text:**

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers.
QUESTION: Is there a positive integer c such that the number of i for which c divides a_i is more than the number of j for which c divides b_j?

Reference: [Plaisted, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if all a_i are different and all b_j are different [Garey and Johnson, ----].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all divisors of all a_i as candidates for c; for each candidate, count how many a's and b's it divides; check if any candidate yields a strict majority.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: Factoring-based approaches to enumerate divisors.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
a-sequence: [12, 18, 30]   (n = 3)
b-sequence: [6, 10]        (m = 2)

**Question:** Is there a positive integer c dividing more a_i's than b_j's?

**Solution search (candidate divisors of a-elements):**
- Divisors of 12: {1, 2, 3, 4, 6, 12}
- Divisors of 18: {1, 2, 3, 6, 9, 18}
- Divisors of 30: {1, 2, 3, 5, 6, 10, 15, 30}
- All candidates: {1, 2, 3, 4, 5, 6, 9, 10, 12, 15, 18, 30}

For each candidate c:
- c = 1: divides 3 a's, 2 b's. 3 > 2? YES!
- c = 2: divides 3 a's (12,18,30), 2 b's (6,10). 3 > 2? YES!
- c = 3: divides 3 a's (12,18,30), 1 b (6). 3 > 1? YES!
- c = 6: divides 3 a's (12,18,30), 1 b (6). 3 > 1? YES!
- c = 9: divides 1 a (18), 0 b's. 1 > 0? YES!

**Answer:** YES, many valid c's exist. For instance, c = 3: divides 12 (yes, 12/3=4), 18 (yes, 18/3=6), 30 (yes, 30/3=10), 6 (yes, 6/3=2), 10 (no, 10/3 is not integer). So c=3 divides 3 of 3 a's and 1 of 2 b's. 3 > 1.

**More interesting example:**
a-sequence: [6, 10, 15]  (n = 3)
b-sequence: [30, 30, 30] (m = 3)

For each candidate c:
- c = 1: 3 a's, 3 b's. 3 > 3? NO.
- c = 2: divides 6,10 (2 a's), divides 30 (3 b's). 2 > 3? NO.
- c = 3: divides 6,15 (2 a's), divides 30 (3 b's). 2 > 3? NO.
- c = 5: divides 10,15 (2 a's), divides 30 (3 b's). 2 > 3? NO.
- c = 6: divides 6 (1 a), divides 30 (1 b). 1 > 1? NO.
- c = 10: divides 10 (1 a), divides 30 (1 b). 1 > 1? NO.
- c = 15: divides 15 (1 a), divides 30 (1 b). 1 > 1? NO.
- c = 30: divides 0 a's, 3 b's. NO.

**Answer:** NO. No c can divide more a's than b's. (Here every divisor of any a_i also divides 30, which appears 3 times in the b-sequence.)
