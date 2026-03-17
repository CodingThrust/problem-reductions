---
name: Problem
about: Propose a new problem type
title: "[Model] MultiprocessorScheduling"
labels: model
assignees: ''
---

## Motivation

MULTIPROCESSOR SCHEDULING (P192) from Garey & Johnson, A5 SS8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS8

**Mathematical definition:**

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, length l(t) ∈ Z+ for each t ∈ T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule for T that meets the overall deadline D, i.e., a function σ:T→Z0+ such that, for all u ≥ 0, the number of tasks t ∈ T for which σ(t) ≤ u < σ(t) + l(t) is no more than m and such that, for all t ∈ T, σ(t) + l(t) ≤ D?

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

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, length l(t) ∈ Z+ for each t ∈ T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule for T that meets the overall deadline D, i.e., a function σ:T→Z0+ such that, for all u ≥ 0, the number of tasks t ∈ T for which σ(t) ≤ u < σ(t) + l(t) is no more than m and such that, for all t ∈ T, σ(t) + l(t) ≤ D?

Reference: Transformation from PARTITION (see Section 3.2.1).

Comment: Remains NP-complete for m = 2, but can be solved in pseudo-polynomial time for any fixed m. NP-complete in the strong sense for m arbitrary (3-PARTITION is a special case). If all tasks have the same length, then this problem is trivial to solve in polynomial time, even for "different speed" processors.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
