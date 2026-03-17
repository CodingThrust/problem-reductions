---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeWeightedCompletionTime"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME (P188) from Garey & Johnson, A5 SS4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS4

**Mathematical definition:**

INSTANCE: Set T of tasks, partial order < on T, for each task t ∈ T a length l(t) ∈ Z+ and a weight w(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and for which the sum, over all t ∈ T, of (σ(t) + l(t))·w(t) is K or less?

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

INSTANCE: Set T of tasks, partial order < on T, for each task t ∈ T a length l(t) ∈ Z+ and a weight w(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and for which the sum, over all t ∈ T, of (σ(t) + l(t))·w(t) is K or less?

Reference: [Lawler, 1978]. Transformation from OPTIMAL LINEAR ARRANGEMENT.

Comment: NP-complete in the strong sense and remains so even if all task lengths are 1 or all task weights are 1. Can be solved in polynomial time for < a "forest" [Horn, 1972], [Adolphson and Hu, 1973], [Garey, 1973], [Sidney, 1975] or if < is "series-parallel" or "generalized series-parallel" [Knuth, 1973], [Lawler, 1978], [Adolphson, 1977], [Monma and Sidney, 1977]. If the partial order < is replaced by individual task deadlines, the resulting problem in NP-complete in the strong sense [Lenstra, 1977], but can be solved in polynomial time if all task weights are equal [Smith, 1956]. If there are individual task release times instead of deadline, the resulting problem is NP-complete in the strong sense, even if all task weights are 1 [Lenstra, Rinnooy Kan, and Brucker, 1977]. The "preemptive" version of this latter problem is NP-complete in the strong sense [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1978], but is solvable in polynomial time if all weights are equal [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
