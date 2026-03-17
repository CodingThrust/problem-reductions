---
name: Problem
about: Propose a new problem type
title: "[Model] TimetableDesign"
labels: model
assignees: ''
---

## Motivation

TIMETABLE DESIGN (P203) from Garey & Johnson, A5 SS19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS19

**Mathematical definition:**

INSTANCE: Set H of "work periods," set C of "craftsmen," set T of "tasks," a subset A(c) ⊆ H of "available hours" for each craftsman c ∈ C, a subset A(t) ⊆ H of "available hours" for each task t ∈ T, and, for each pair (c,t) ∈ C×T, a number R(c,t) ∈ Z0+ of "required work periods."
QUESTION: Is there a timetable for completing all the tasks, i.e., a function f: C×T×H → {0,1} (where f(c,t,h) = 1 means that craftsman c works on task t during period h) such that (1) f(c,t,h) = 1 only if h ∈ A(c)∩A(t), (2) for each h ∈ H and c ∈ C there is at most one t ∈ T for which f(c,t,h) = 1, (3) for each h ∈ H and t ∈ T there is at most one c ∈ C for which f(c,t,h) = 1, and (4) for each pair (c,t) ∈ C×T there are exactly R(c,t) values of h for which f(c,t,h) = 1?

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

INSTANCE: Set H of "work periods," set C of "craftsmen," set T of "tasks," a subset A(c) ⊆ H of "available hours" for each craftsman c ∈ C, a subset A(t) ⊆ H of "available hours" for each task t ∈ T, and, for each pair (c,t) ∈ C×T, a number R(c,t) ∈ Z0+ of "required work periods."
QUESTION: Is there a timetable for completing all the tasks, i.e., a function f: C×T×H → {0,1} (where f(c,t,h) = 1 means that craftsman c works on task t during period h) such that (1) f(c,t,h) = 1 only if h ∈ A(c)∩A(t), (2) for each h ∈ H and c ∈ C there is at most one t ∈ T for which f(c,t,h) = 1, (3) for each h ∈ H and t ∈ T there is at most one c ∈ C for which f(c,t,h) = 1, and (4) for each pair (c,t) ∈ C×T there are exactly R(c,t) values of h for which f(c,t,h) = 1?

Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.

Comment: Remains NP-complete even if |H| = 3, A(t) = H for all t ∈ T, and each R(c,t) ∈ {0,1}. The general problem can be solved in polynomial time if |A(c)| ≤ 2 for all c ∈ C or if A(c) = A(t) = H for all c ∈ C and t ∈ T.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
