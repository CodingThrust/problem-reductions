---
name: Problem
about: Propose a new problem type
title: "[Model] FlowShopScheduling"
labels: model
assignees: ''
---

## Motivation

FLOW-SHOP SCHEDULING (P199) from Garey & Johnson, A5 SS15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS15

**Mathematical definition:**

INSTANCE: Number m ∈ Z+ of processors, set J of jobs, each job j ∈ J consisting of m tasks t1[j],t2[j], ..., tm[j], a length l(t) ∈ Z0+ for each such task t, and an overall deadline D ∈ Z+.
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline, where such a schedule is identical to an open-shop schedule with the additional constraint that, for each j ∈ J and 1 ≤ i < m, σi+1(j) ≥ σi(j) + l(ti[j])?

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

INSTANCE: Number m ∈ Z+ of processors, set J of jobs, each job j ∈ J consisting of m tasks t1[j],t2[j], ..., tm[j], a length l(t) ∈ Z0+ for each such task t, and an overall deadline D ∈ Z+.
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline, where such a schedule is identical to an open-shop schedule with the additional constraint that, for each j ∈ J and 1 ≤ i < m, σi+1(j) ≥ σi(j) + l(ti[j])?

Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense for m = 3. Solvable in polynomial time for m = 2 [Johnson, 1954]. The same results hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a], although if release times are added in this case, the problem is NP-complete in the strong sense, even for m = 2 [Cho and Sahni, 1978]. If the goal is to meet a bound K on the sum, over all j ∈ J, of σm(j) + l(tm[j]), then the non-preemptive problem is NP-complete in the strong sense even if m = 2 [Garey, Johnson, and Sethi, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
