---
name: Problem
about: Propose a new problem type
title: "[Model] SchedulingToMinimizeWeightedCompletionTime"
labels: model
assignees: ''
---

## Motivation

SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME (P197) from Garey & Johnson, A5 SS13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS13

**Mathematical definition:**

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, for each task t ∈ T a length l(t) ∈ Z+ and a weight w(t) ∈ Z+, and a positive integer K.
QUESTION: Is there an m-processor schedule σ for T such that the sum, over all t ∈ T, of (σ(t) + l(t))·w(t) is no more than K?

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

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, for each task t ∈ T a length l(t) ∈ Z+ and a weight w(t) ∈ Z+, and a positive integer K.
QUESTION: Is there an m-processor schedule σ for T such that the sum, over all t ∈ T, of (σ(t) + l(t))·w(t) is no more than K?

Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from PARTITION.

Comment: Remains NP-complete for m = 2, and is NP-complete in the strong sense for m arbitrary [Lageweg and Lenstra, 1977]. The problem is solvable in pseudo-polynomial time for fixed m. These results continue to hold if "preemptive" schedules are allowed [McNaughton, 1959]. Can be solved in polynomial time if all lengths are equal (by matching techniques). If instead all weights are equal, it can be solved in polynomial time even for "different speed" processors [Conway, Maxwell, and Miller, 1967] and for "unrelated" processors [Horn, 1973], [Bruno, Coffman, and Sethi, 1974]. The "preemptive" case for different speed processors also can be solved in polynomial time [Gonzalez, 1977]. If precedence constraints are allowed, the original problem is NP-complete in the strong sense even if all weights are equal, m = 2, and the partial order is either an "in-tree" or an "out-tree" [Sethi, 1977a]. If resources are allowed, the same subcases mentioned under RESOURCE CONSTRAINED SCHEDULING are NP-complete, even for equal weights [Blazewicz, 1977a].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
