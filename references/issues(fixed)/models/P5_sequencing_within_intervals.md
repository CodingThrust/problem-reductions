---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingWithinIntervals"
labels: model
assignees: ''
---

## Motivation

SEQUENCING WITHIN INTERVALS (P5) from Garey & Johnson, Chapter 3, Section 3.2.2, p.70. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.2, p.70

**Mathematical definition:**

INSTANCE: A finite set T of "tasks" and, for each t ∈ T, an integer "release time" r(t) ≥ 0, a "deadline" d(t) ∈ Z+, and a "length" l(t) ∈ Z+.
QUESTION: Does there exist a feasible schedule for T, that is, a function σ: T → Z+ such that, for each t ∈ T, σ(t) ≥ r(t), σ(t)+l(t) ≤ d(t), and, if t' ∈ T−{t}, then either σ(t')+l(t') ≤ σ(t) or σ(t') ≥ σ(t)+l(t)? (The task t is "executed" from time σ(t) to time σ(t)+l(t), cannot start executing until time r(t), must be completed by time d(t), and its execution cannot overlap the execution of any other task t'.)

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

INSTANCE: A finite set T of "tasks" and, for each t ∈ T, an integer "release time" r(t) ≥ 0, a "deadline" d(t) ∈ Z+, and a "length" l(t) ∈ Z+.
QUESTION: Does there exist a feasible schedule for T, that is, a function σ: T → Z+ such that, for each t ∈ T, σ(t) ≥ r(t), σ(t)+l(t) ≤ d(t), and, if t' ∈ T−{t}, then either σ(t')+l(t') ≤ σ(t) or σ(t') ≥ σ(t)+l(t)? (The task t is "executed" from time σ(t) to time σ(t)+l(t), cannot start executing until time r(t), must be completed by time d(t), and its execution cannot overlap the execution of any other task t'.)

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
