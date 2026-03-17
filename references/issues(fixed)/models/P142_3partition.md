---
name: Problem
about: Propose a new problem type
title: "[Model] 3Partition"
labels: model
assignees: ''
---

## Motivation

3-PARTITION (P142) from Garey & Johnson, A3 SP15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP15

**Mathematical definition:**

INSTANCE: Set A of 3m elements, a bound B ∈ Z^+, and a size s(a) ∈ Z^+ for each a ∈ A such that B/4 < s(a) < B/2 and such that Σ_{a ∈ A} s(a) = mB.
QUESTION: Can A be partitioned into m disjoint sets A_1,A_2,…,A_m such that, for 1 ≤ i ≤ m, Σ_{a ∈ A_i} s(a) = B (note that each A_i must therefore contain exactly three elements from A)?

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

INSTANCE: Set A of 3m elements, a bound B ∈ Z^+, and a size s(a) ∈ Z^+ for each a ∈ A such that B/4 < s(a) < B/2 and such that Σ_{a ∈ A} s(a) = mB.
QUESTION: Can A be partitioned into m disjoint sets A_1,A_2,…,A_m such that, for 1 ≤ i ≤ m, Σ_{a ∈ A_i} s(a) = B (note that each A_i must therefore contain exactly three elements from A)?
Reference: [Garey and Johnson, 1975]. Transformation from 3DM (see Section 4.2).
Comment: NP-complete in the strong sense.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
