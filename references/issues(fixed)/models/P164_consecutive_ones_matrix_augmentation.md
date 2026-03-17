---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveOnesMatrixAugmentation"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE ONES MATRIX AUGMENTATION (P164) from Garey & Johnson, A4 SR16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR16

**Mathematical definition:**

INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a matrix A', obtained from A by changing K or fewer 0 entries to 1's, such that A' has the consecutive ones property?

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

INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a matrix A', obtained from A by changing K or fewer 0 entries to 1's, such that A' has the consecutive ones property?
Reference: [Booth, 1975], [Papadimitriou, 1976a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
Comment: Variant in which we ask instead that A' have the circular ones property is also NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
