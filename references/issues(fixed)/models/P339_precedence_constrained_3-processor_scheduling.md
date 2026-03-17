---
name: Problem
about: Propose a new problem type
title: "[Model] PrecedenceConstrained3ProcessorScheduling"
labels: model
assignees: ''
---

## Motivation

PRECEDENCE CONSTRAINED 3-PROCESSOR SCHEDULING (P339) from Garey & Johnson, A13 OPEN8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN8

**Mathematical definition:**

INSTANCE: Set T of unit length tasks, partial order < on T, and a deadline D ∈ Z+.
QUESTION: Can T be scheduled on 3 processors so as to satisfy the precedence constraints and meet the overall deadline D, i.e., is there a schedule σ: T → {0, 1, . . . , D-1} such that t < t' implies σ(t) < σ(t') and such that for each integer i, 0 ≤ i ≤ D-1, there are at most 3 tasks t ∈ T for which σ(t) = i?

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

[OPEN8] PRECEDENCE CONSTRAINED 3-PROCESSOR SCHEDULING
INSTANCE: Set T of unit length tasks, partial order < on T, and a deadline D ∈ Z+.
QUESTION: Can T be scheduled on 3 processors so as to satisfy the precedence constraints and meet the overall deadline D, i.e., is there a schedule σ: T → {0, 1, . . . , D-1} such that t < t' implies σ(t) < σ(t') and such that for each integer i, 0 ≤ i ≤ D-1, there are at most 3 tasks t ∈ T for which σ(t) = i?
Comment: The corresponding problem for 2 processors is solvable in polynomial time [Fujii, Kasami, and Ninomiya, 1969], [Coffman and Graham, 1972], even with individual task deadlines and release times [Garey and Johnson, 1977b]. If the number of processors is allowed to vary as part of the instance, the problem is NP-complete [Ullman, 1975]. See PRECEDENCE CONSTRAINED SCHEDULING for more details. Is there any fixed value of K for which the K-processor version of the above problem is NP-complete?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
