---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeTardyTaskWeight"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE TARDY TASK WEIGHT (P187) from Garey & Johnson, A5 SS3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS3

**Mathematical definition:**

INSTANCE: Set T of tasks, for each task t ∈ T a length l(t) ∈ Z+, a weight w(t) ∈ Z+, and a deadline d(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T such that the sum of w(t), taken over all t ∈ T for which σ(t) + l(t) > d(t), does not exceed K?

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

INSTANCE: Set T of tasks, for each task t ∈ T a length l(t) ∈ Z+, a weight w(t) ∈ Z+, and a deadline d(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T such that the sum of w(t), taken over all t ∈ T for which σ(t) + l(t) > d(t), does not exceed K?

Reference: [Karp, 1972]. Transformation from PARTITION.

Comment: Can be solved in pseudo-polynomial time (time polynomial in |T|, ∑l(t), and log ∑w(t)) [Lawler and Moore, 1969]. Can be solved in polynomial time if weights are "agreeable" (i.e., w(t) < w(t') implies l(t) ≥ l(t')) [Lawler, 1976c].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
