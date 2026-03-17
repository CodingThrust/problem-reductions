---
name: Problem
about: Propose a new problem type
title: "[Model] CosineProductIntegration"
labels: model
assignees: ''
---

## Motivation

COSINE PRODUCT INTEGRATION (P233) from Garey & Johnson, A7 AN14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN14

**Mathematical definition:**

INSTANCE: Sequence (a_1, a_2, . . . , a_n) of integers.
QUESTION: Does ∫_0^{2π} (∏_{i=1}^{n} cos(a_i·θ)) dθ = 0?

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

INSTANCE: Sequence (a_1, a_2, . . . , a_n) of integers.
QUESTION: Does ∫_0^{2π} (∏_{i=1}^{n} cos(a_i·θ)) dθ = 0?

Reference: [Plaisted, 1976]. Transformation from PARTITION.
Comment: Solvable in pseudo-polynomial time. See reference for related complexity results concerning integration.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
