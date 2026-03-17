---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingWithDeadlinesAndSetUpTimes"
labels: model
assignees: ''
---

## Motivation

SEQUENCING WITH DEADLINES AND SET-UP TIMES (P190) from Garey & Johnson, A5 SS6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS6

**Mathematical definition:**

INSTANCE: Set C of "compilers," set T of tasks, for each t ∈ T a length l(t) ∈ Z+, a deadline d(t) ∈ Z+, and a compiler k(t) ∈ C, and for each c ∈ C a "set-up time" l(c) ∈ Z0+.
QUESTION: Is there a one-processor schedule σ for T that meets all the task deadlines and that satisfies the additional constraint that, whenever two tasks t and t' with σ(t) < σ(t') are scheduled "consecutively" (i.e., no other task t'' has σ(t) < σ(t'') < σ(t')) and have different compilers (i.e., k(t) ≠ k(t')), then σ(t') ≥ σ(t) + l(t) + l(k(t'))?

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

INSTANCE: Set C of "compilers," set T of tasks, for each t ∈ T a length l(t) ∈ Z+, a deadline d(t) ∈ Z+, and a compiler k(t) ∈ C, and for each c ∈ C a "set-up time" l(c) ∈ Z0+.
QUESTION: Is there a one-processor schedule σ for T that meets all the task deadlines and that satisfies the additional constraint that, whenever two tasks t and t' with σ(t) < σ(t') are scheduled "consecutively" (i.e., no other task t'' has σ(t) < σ(t'') < σ(t')) and have different compilers (i.e., k(t) ≠ k(t')), then σ(t') ≥ σ(t) + l(t) + l(k(t'))?

Reference: [Bruno and Downey, 1978]. Transformation from PARTITION.

Comment: Remains NP-complete even if all set-up times are equal. The related problem in which set-up times are replaced by "changeover costs," and we want to know if there is a schedule that meets all the deadlines and has total changeover cost at most K, is NP-complete even if all changeover costs are equal. Both problems can be solved in pseudo-polynomial time when the number of distinct deadlines is bounded by a constant. If the number of deadlines is unbounded, it is open whether these problems are NP-complete in the strong sense.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
