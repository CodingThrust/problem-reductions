---
name: Problem
about: Propose a new problem type
title: "[Model] OpenShopScheduling"
labels: model
assignees: ''
---

## Motivation

OPEN-SHOP SCHEDULING (P198) from Garey & Johnson, A5 SS14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS14

**Mathematical definition:**

INSTANCE: Number m ∈ Z+ of processors, set J of jobs, each job j ∈ J consisting of m tasks t1[j],t2[j], ..., tm[j] (with ti[j] to be executed by processor i), a length l(t) ∈ Z0+ for each such task t, and an overall deadline D ∈ Z+.
QUESTION: Is there an open-shop schedule for J that meets the deadline, i.e., a collection of one-processor schedules σi: J→Z0+, 1 ≤ i ≤ m, such that σi(j) > σi(k) implies σi(j) ≥ σi(k) + l(ti[k]), such that for each j ∈ J the intervals [σi(j), σi(j) + l(ti[j])) are all disjoint, and such that σi(j) + l(ti[j]) ≤ D for 1 ≤ i ≤ m, 1 ≤ j ≤ |J|?

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

INSTANCE: Number m ∈ Z+ of processors, set J of jobs, each job j ∈ J consisting of m tasks t1[j],t2[j], ..., tm[j] (with ti[j] to be executed by processor i), a length l(t) ∈ Z0+ for each such task t, and an overall deadline D ∈ Z+.
QUESTION: Is there an open-shop schedule for J that meets the deadline, i.e., a collection of one-processor schedules σi: J→Z0+, 1 ≤ i ≤ m, such that σi(j) > σi(k) implies σi(j) ≥ σi(k) + l(ti[k]), such that for each j ∈ J the intervals [σi(j), σi(j) + l(ti[j])) are all disjoint, and such that σi(j) + l(ti[j]) ≤ D for 1 ≤ i ≤ m, 1 ≤ j ≤ |J|?

Reference: [Gonzalez and Sahni, 1976]. Transformation from PARTITION.

Comment: Remains NP-complete if m = 3, but can be solved in polynomial time if m = 2. NP-complete in the strong sense for m arbitrary [Lenstra, 1977]. The general problem is solvable in polynomial time if "preemptive" schedules are allowed [Gonzalez and Sahni, 1976], even if two distinct release times are allowed [Cho and Sahni, 1978]. The m = 2 preemptive case can be solved in polynomial time even if arbitrary release times are allowed, and the general preemptive case with arbitrary release times and deadlines can be solved by linear programming [Cho and Sahni, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
