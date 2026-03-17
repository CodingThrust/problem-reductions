---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeWeightedTardiness"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE WEIGHTED TARDINESS (P189) from Garey & Johnson, A5 SS5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS5

**Mathematical definition:**

INSTANCE: Set T of tasks, for each task t ∈ T a length l(t) ∈ Z+, a weight w(t) ∈ Z+, and a deadline d(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T such that the sum, taken over all t ∈ T satisfying σ(t) + l(t) > d(t), of (σ(t) + l(t) - d(t))·w(t) is K or less?

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
QUESTION: Is there a one-processor schedule σ for T such that the sum, taken over all t ∈ T satisfying σ(t) + l(t) > d(t), of (σ(t) + l(t) - d(t))·w(t) is K or less?

Reference: [Lawler, 1977a]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense. If all weights are equal, the problem can be solved in pseudo-polynomial time [Lawler, 1977a] and is open as to ordinary NP-completeness. If all lengths are equal (with weights arbitrary), it can be solved in polynomial time by bipartite matching. If precedence constraints are added, the problem is NP-complete even with equal lengths and equal weights [Lenstra and Rinnooy Kan, 1978a]. If release times are added instead, the problem is NP-complete in the strong sense for equal task weights (see SEQUENCING WITH RELEASE TIMES AND DEADLINES), but can be solved by bipartite matching for equal lengths and arbitrary weights [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
