---
name: Problem
about: Propose a new problem type
title: "[Model] ResourceConstrainedScheduling"
labels: model
assignees: ''
---

## Motivation

RESOURCE CONSTRAINED SCHEDULING (P194) from Garey & Johnson, A5 SS10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS10

**Mathematical definition:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, number r ∈ Z+ of resources, resource bounds Bi, 1 ≤ i ≤ r, resource requirement Ri(t), 0 ≤ Ri(t) ≤ Bi, for each task t and resource i, and an overall deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the resource constraints, i.e., such that for all u ≥ 0, if S(u) is the set of all t ∈ T for which σ(t) ≤ u < σ(t) + l(t), then for each resource i the sum of Ri(t) over all t ∈ S(u) is at most Bi?

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

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, number r ∈ Z+ of resources, resource bounds Bi, 1 ≤ i ≤ r, resource requirement Ri(t), 0 ≤ Ri(t) ≤ Bi, for each task t and resource i, and an overall deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the resource constraints, i.e., such that for all u ≥ 0, if S(u) is the set of all t ∈ T for which σ(t) ≤ u < σ(t) + l(t), then for each resource i the sum of Ri(t) over all t ∈ S(u) is at most Bi?

Reference: [Garey and Johnson, 1975]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense, even if r = 1 and m = 3. Can be solved in polynomial time by matching for m = 2 and r arbitrary. If a partial order < is added, the problem becomes NP-complete in the strong sense for r = 1, m = 2, and < a "forest." If each resource requirement is restricted to be either 0 or Bi, the problem is NP-complete for m = 2, r = 1, and < arbitrary [Ullman, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
