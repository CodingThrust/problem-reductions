---
name: Problem
about: Propose a new problem type
title: "[Model] PrecedenceConstrainedScheduling"
labels: model
assignees: ''
---

## Motivation

PRECEDENCE CONSTRAINED SCHEDULING (P193) from Garey & Johnson, A5 SS9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS9

**Mathematical definition:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, partial order < on T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the precedence constraints, i.e., such that t < t' implies σ(t') ≥ σ(t) + l(t) = σ(t) + 1?

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

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, partial order < on T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the precedence constraints, i.e., such that t < t' implies σ(t') ≥ σ(t) + l(t) = σ(t) + 1?

Reference: [Ullman, 1975]. Transformation from 3SAT.

Comment: Remains NP-complete for D = 3 [Lenstra and Rinnooy Kan, 1978a]. Can be solved in polynomial time if m = 2 (e.g., see [Coffman and Graham, 1972]) or if m is arbitrary and < is a "forest" [Hu, 1961] or has a chordal graph as complement [Papadimitriou and Yannakakis, 1978b]. Complexity remains open for all fixed m ≥ 3 when < is arbitrary. The m = 2 case becomes NP-complete if both task lengths 1 and 2 are allowed [Ullman, 1975]. If each task t can only be executed by a specified processor p(t), the problem is NP-complete for m = 2 and < arbitrary, and for m arbitrary and < a forest, but can be solved in polynomial time for m arbitrary if < is a "cyclic forest" [Goyal, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
