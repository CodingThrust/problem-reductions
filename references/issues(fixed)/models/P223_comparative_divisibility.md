---
name: Problem
about: Propose a new problem type
title: "[Model] ComparativeDivisibility"
labels: model
assignees: ''
---

## Motivation

COMPARATIVE DIVISIBILITY (P223) from Garey & Johnson, A7 AN4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN4

**Mathematical definition:**

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers.
QUESTION: Is there a positive integer c such that the number of i for which c divides a_i is more than the number of j for which c divides b_j?

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

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers.
QUESTION: Is there a positive integer c such that the number of i for which c divides a_i is more than the number of j for which c divides b_j?

Reference: [Plaisted, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if all a_i are different and all b_j are different [Garey and Johnson, ——].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
