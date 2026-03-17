---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingWithReleaseTimesAndDeadlines"
labels: model
assignees: ''
---

## Motivation

SEQUENCING WITH RELEASE TIMES AND DEADLINES (P185) from Garey & Johnson, A5 SS1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS1

**Mathematical definition:**

INSTANCE: Set T of tasks and, for each task t ∈ T, a length l(t) ∈ Z+, a release time r(t) ∈ Z0+, and a deadline d(t) ∈ Z+.
QUESTION: Is there a one-processor schedule for T that satisfies the release time constraints and meets all the deadlines, i.e., a one-to-one function σ:T→Z0+, with σ(t) > σ(t') implying σ(t) ≥ σ(t') + l(t'), such that, for all t ∈ T, σ(t) ≥ r(t) and σ(t) + l(t) ≤ d(t)?

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

INSTANCE: Set T of tasks and, for each task t ∈ T, a length l(t) ∈ Z+, a release time r(t) ∈ Z0+, and a deadline d(t) ∈ Z+.
QUESTION: Is there a one-processor schedule for T that satisfies the release time constraints and meets all the deadlines, i.e., a one-to-one function σ:T→Z0+, with σ(t) > σ(t') implying σ(t) ≥ σ(t') + l(t'), such that, for all t ∈ T, σ(t) ≥ r(t) and σ(t) + l(t) ≤ d(t)?

Reference: [Garey and Johnson, 1977b]. Transformation from 3-PARTITION (see Section 4.2).

Comment: NP-complete in the strong sense. Solvable in pseudo-polynomial time if the number of allowed values for r(t) and d(t) is bounded by a constant, but remains NP-complete (in the ordinary sense) even when each can take on only two values. If all task lengths are 1, or "preemptions" are allowed, or all release times are 0, the general problem can be solved in polynomial time, even under "precedence constraints" [Lawler, 1973], [Lageweg, Lenstra, and Rinnooy Kan, 1976]. Can also be solved in polynomial time even if release times and deadlines are allowed to be arbitrary rationals and there are precedence constraints, so long as all tasks have equal length [Carlier, 1978], [Simons, 1978], [Garey, Johnson, Simons, and Tarjan, 1978], or preemptions are allowed [Blazewicz, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
