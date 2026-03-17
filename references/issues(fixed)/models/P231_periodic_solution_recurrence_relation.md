---
name: Problem
about: Propose a new problem type
title: "[Model] PeriodicSolutionRecurrenceRelation"
labels: model
assignees: ''
---

## Motivation

PERIODIC SOLUTION RECURRENCE RELATION (P231) from Garey & Johnson, A7 AN12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN12

**Mathematical definition:**

INSTANCE: Ordered pairs (c_i, b_i), 1 ≤ i ≤ m, of integers, with all b_i positive.
QUESTION: Is there a sequence a_0, a_1, . . . , a_{n-1} of integers, with n ≥ max{b_i}, such that the infinite sequence a_0, a_1, . . . defined by the recurrence relation
a_i = Σ_{j=1}^{m} c_j·a_{(i-b_j)}
satisfies a_i ≡ a_{i(mod n)}, for all i ≥ n?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: Ordered pairs (c_i, b_i), 1 ≤ i ≤ m, of integers, with all b_i positive.
QUESTION: Is there a sequence a_0, a_1, . . . , a_{n-1} of integers, with n ≥ max{b_i}, such that the infinite sequence a_0, a_1, . . . defined by the recurrence relation

    a_i = Σ_{j=1}^{m} c_j·a_{(i-b_j)}

satisfies a_i ≡ a_{i(mod n)}, for all i ≥ n?

Reference: [Plaisted, 1977b]. Tranformation from 3SAT
Comment: Not known to be in NP or co-NP. See reference for related results.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
