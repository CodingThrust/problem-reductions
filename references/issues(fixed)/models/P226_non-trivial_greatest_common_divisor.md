---
name: Problem
about: Propose a new problem type
title: "[Model] NonTrivialGreatestCommonDivisor"
labels: model
assignees: ''
---

## Motivation

NON-TRIVIAL GREATEST COMMON DIVISOR (P226) from Garey & Johnson, A7 AN7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN7

**Mathematical definition:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0.
QUESTION: Does the greatest common divisor of the polynomials Σ_{j=1}^{k} a_i[j]·z^{b_i[j]}, 1 ≤ i ≤ m, have degree greater than zero?

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

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0.
QUESTION: Does the greatest common divisor of the polynomials Σ_{j=1}^{k} a_i[j]·z^{b_i[j]}, 1 ≤ i ≤ m, have degree greater than zero?

Reference: [Plaisted, 1977a]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1 [Plaisted, 1976] or if m = 2 [Plaisted, 1977b]. The analogous problem in which the instance also includes a positive integer K, and we are asked if the least common multiple of the given polynomials has degree less than K, is NP-hard under the same restrictions. Both problems can be solved in pseudo-polynomial time using standard algorithms.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
