---
name: Problem
about: Propose a new problem type
title: "[Model] CapacityAssignment"
labels: model
assignees: ''
---

## Motivation

CAPACITY ASSIGNMENT (P155) from Garey & Johnson, A4 SR7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR7

**Mathematical definition:**

INSTANCE: Set C of communication links, set M ⊆ Z+ of capacities, cost function g: C×M → Z+, delay penalty function d: C×M → Z+ such that, for all c ∈ C and i < j ∈ M, g(c,i) ≤ g(c,j) and d(c,i) ≥ d(c,j), and positive integers K and J.
QUESTION: Is there an assignment σ: C → M such that the total cost ∑c ∈ C g(c,σ(c)) does not exceed K and such that the total delay penalty ∑c ∈ C d(c,σ(c)) does not exceed J?

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

INSTANCE: Set C of communication links, set M ⊆ Z+ of capacities, cost function g: C×M → Z+, delay penalty function d: C×M → Z+ such that, for all c ∈ C and i < j ∈ M, g(c,i) ≤ g(c,j) and d(c,i) ≥ d(c,j), and positive integers K and J.
QUESTION: Is there an assignment σ: C → M such that the total cost ∑c ∈ C g(c,σ(c)) does not exceed K and such that the total delay penalty ∑c ∈ C d(c,σ(c)) does not exceed J?
Reference: [Van Sickle and Chandy, 1977]. Transformation from SUBSET SUM.
Comment: Solvable in pseudo-polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
