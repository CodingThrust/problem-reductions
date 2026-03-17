---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeMaximumCumulativeCost"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST (P191) from Garey & Johnson, A5 SS7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS7

**Mathematical definition:**

INSTANCE: Set T of tasks, partial order < on T, a "cost" c(t) ∈ Z for each t ∈ T (if c(t) < 0, it can be viewed as a "profit"), and a constant K ∈ Z.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and which has the property that, for every task t ∈ T, the sum of the costs for all tasks t' with σ(t') ≤ σ(t) is at most K?

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

INSTANCE: Set T of tasks, partial order < on T, a "cost" c(t) ∈ Z for each t ∈ T (if c(t) < 0, it can be viewed as a "profit"), and a constant K ∈ Z.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and which has the property that, for every task t ∈ T, the sum of the costs for all tasks t' with σ(t') ≤ σ(t) is at most K?

Reference: [Abdel-Wahab, 1976]. Transformation from REGISTER SUFFICIENCY.

Comment: Remains NP-complete even if c(t) ∈ {-1,0,1} for all t ∈ T. Can be solved in polynomial time if < is series-parallel [Abdel-Wahab and Kameda, 1978], [Monma and Sidney, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
