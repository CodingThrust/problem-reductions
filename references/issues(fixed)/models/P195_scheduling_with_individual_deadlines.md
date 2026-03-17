---
name: Problem
about: Propose a new problem type
title: "[Model] SchedulingWithIndividualDeadlines"
labels: model
assignees: ''
---

## Motivation

SCHEDULING WITH INDIVIDUAL DEADLINES (P195) from Garey & Johnson, A5 SS11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS11

**Mathematical definition:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, partial order < on T, and for each task t ∈ T a deadline d(t) ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that obeys the precedence constraints and meets all the deadlines, i.e., σ(t) + l(t) ≤ d(t) for all t ∈ T?

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

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, partial order < on T, and for each task t ∈ T a deadline d(t) ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that obeys the precedence constraints and meets all the deadlines, i.e., σ(t) + l(t) ≤ d(t) for all t ∈ T?

Reference: [Brucker, Garey, and Johnson, 1977]. Transformation from VERTEX COVER.

Comment: Remains NP-complete even if < is an "out-tree" partial order (no task has more than one immediate predecessor), but can be solved in polynomial time if < is an "in-tree" partial order (no task has more than one immediate successor). Solvable in polynomial time if m = 2 and < is arbitrary [Garey and Johnson, 1976c], even if individual release times are included [Garey and Johnson, 1977b]. For < empty, can be solved in polynomial time by matching for m arbitrary, even with release times and with a single resource having 0-1 valued requirements [Blazewicz, 1977b], [Blazewicz, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
