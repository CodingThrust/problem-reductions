---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeTardyTasks"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE TARDY TASKS (P186) from Garey & Johnson, A5 SS2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS2

**Mathematical definition:**

INSTANCE: Set T of tasks, partial order < on T, for each task t ∈ T a length l(t) ∈ Z+ and a deadline d(t) ∈ Z+, and a positive integer K ≤ |T|.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints, i.e., such that t < t' implies σ(t) + l(t) ≤ σ(t'), and such that there are at most K tasks t ∈ T for which σ(t) + l(t) > d(t)?

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

INSTANCE: Set T of tasks, partial order < on T, for each task t ∈ T a length l(t) ∈ Z+ and a deadline d(t) ∈ Z+, and a positive integer K ≤ |T|.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints, i.e., such that t < t' implies σ(t) + l(t) ≤ σ(t'), and such that there are at most K tasks t ∈ T for which σ(t) + l(t) > d(t)?

Reference: [Garey and Johnson, 1976c]. Transformation from CLIQUE (see Section 3.2.3).

Comment: Remains NP-complete even if all task lengths are 1 and < consists only of "chains" (each task has at most one immediate predecessor and at most one immediate successor) [Lenstra, 1977]. The general problem can be solved in polynomial time if K = 0 [Lawler, 1973], or if < is empty [Moore, 1968] [Sidney, 1973]. The < empty case remains polynomially solvable if "agreeable" release times (i.e., r(t) < r(t') implies d(t) ≤ d(t')) are added [Kise, Ibaraki, and Mine, 1978], but is NP-complete for arbitrary release times (see previous problem).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
