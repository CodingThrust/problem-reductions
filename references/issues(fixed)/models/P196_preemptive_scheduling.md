---
name: Problem
about: Propose a new problem type
title: "[Model] PreemptiveScheduling"
labels: model
assignees: ''
---

## Motivation

PREEMPTIVE SCHEDULING (P196) from Garey & Johnson, A5 SS12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS12

**Mathematical definition:**

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, partial order < on T, length l(t) ∈ Z+ for each t ∈ T, and an overall deadline D ∈ Z+.
QUESTION: Is there an m-processor "preemptive" schedule for T that obeys the precedence constraints and meets the overall deadline? (Such a schedule σ is identical to an ordinary m-processor schedule, except that we are allowed to subdivide each task t ∈ T into any number of subtasks t1, t2, ..., tk such that ∑k_{i=1} l(ti) = l(t) and it is required that σ(ti + 1) ≥ σ(ti)+l(ti) for 1 ≤ i < k. The precedence constraints are extended to subtasks by requiring that every subtask of t precede every subtask of t' whenever t < t'.)

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

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, partial order < on T, length l(t) ∈ Z+ for each t ∈ T, and an overall deadline D ∈ Z+.
QUESTION: Is there an m-processor "preemptive" schedule for T that obeys the precedence constraints and meets the overall deadline? (Such a schedule σ is identical to an ordinary m-processor schedule, except that we are allowed to subdivide each task t ∈ T into any number of subtasks t1, t2, ..., tk such that ∑k_{i=1} l(ti) = l(t) and it is required that σ(ti + 1) ≥ σ(ti)+l(ti) for 1 ≤ i < k. The precedence constraints are extended to subtasks by requiring that every subtask of t precede every subtask of t' whenever t < t'.)

Reference: [Ullman, 1975]. Transformation from 3SAT.

Comment: Can be solved in polynomial time if m = 2 [Muntz and Coffman, 1969], if < is a "forest" [Muntz and Coffman, 1970], or if < is empty and individual task deadlines are allowed [Horn, 1974]. If "(uniform) different speed" processors are allowed, the problem can be solved in polynomial time if m = 2 or if < is empty [Horvath, Lam, and Sethi, 1977], [Gonzalez and Sahni, 1978b] in the latter case even if individual task deadlines are allowed [Sahni and Cho, 1977a]; if both m = 2 and < is empty, it can be solved in polynomial time, even if both integer release times and deadlines are allowed [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1977]. For "unrelated" processors, the case with m fixed and < empty can be solved in polynomial time [Gonzalez, Lawler, and Sahni, 1978], and the case with m arbitrary and < empty can be solved by linear programming [Lawler and Labetoulle, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
