---
name: Problem
about: Propose a new problem type
title: "[Model] SimultaneousIncongruences"
labels: model
assignees: ''
---

## Motivation

SIMULTANEOUS INCONGRUENCES (P221) from Garey & Johnson, A7 AN2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN2

**Mathematical definition:**

INSTANCE: Collection {(a_1,b_1), . . . , (a_n,b_n)} of ordered pairs of positive integers, with a_i ≤ b_i for 1 ≤ i ≤ n.
QUESTION: Is there an integer x such that, for 1 ≤ i ≤ n, x ≢ a_i (mod b_i)?

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

INSTANCE: Collection {(a_1,b_1), . . . , (a_n,b_n)} of ordered pairs of positive integers, with a_i ≤ b_i for 1 ≤ i ≤ n.
QUESTION: Is there an integer x such that, for 1 ≤ i ≤ n, x ≢ a_i (mod b_i)?

Reference: [Stockmeyer and Meyer, 1973]. Transformation from 3SAT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
