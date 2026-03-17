---
name: Problem
about: Propose a new problem type
title: "[Model] EquilibriumPoint"
labels: model
assignees: ''
---

## Motivation

EQUILIBRIUM POINT (P234) from Garey & Johnson, A7 AN15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN15

**Mathematical definition:**

INSTANCE: Set x = {x_1, x_2, . . . , x_n} of variables, collection {F_i: 1 ≤ i ≤ n} of product polynomials over X and the integers, and a finite "range-set" M_i ⊆ Z for 1 ≤ i ≤ n.
QUESTION: Does there exist a sequence y_1, y_2, . . . , y_n of integers, with y_i ∈ M_i, such that for 1 ≤ i ≤ n and all y ∈ M_i,
F_i(y_1, y_2, . . . , y_{i-1}, y_i, y_{i+1}, . . . , y_n) ≥ F_i(y_1, y_2, . . . , y_{i-1}, y, y_{i+1}, . . . , y_n)?

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

INSTANCE: Set x = {x_1, x_2, . . . , x_n} of variables, collection {F_i: 1 ≤ i ≤ n} of product polynomials over X and the integers, and a finite "range-set" M_i ⊆ Z for 1 ≤ i ≤ n.
QUESTION: Does there exist a sequence y_1, y_2, . . . , y_n of integers, with y_i ∈ M_i, such that for 1 ≤ i ≤ n and all y ∈ M_i,

    F_i(y_1, y_2, . . . , y_{i-1}, y_i, y_{i+1}, . . . , y_n) ≥ F_i(y_1, y_2, . . . , y_{i-1}, y, y_{i+1}, . . . , y_n)?

Reference: [Sahni, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete even if M_i = {0,1} for 1 ≤ i ≤ n.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
