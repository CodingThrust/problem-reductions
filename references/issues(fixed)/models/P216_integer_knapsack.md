---
name: Problem
about: Propose a new problem type
title: "[Model] IntegerKnapsack"
labels: model
assignees: ''
---

## Motivation

INTEGER KNAPSACK (P216) from Garey & Johnson, A6 MP10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP10

**Mathematical definition:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there an assignment of a non-negative integer c(u) to each u ∈ U such that Σᵤ∈U c(u)·s(u) ≤ B and such that Σᵤ∈U c(u)·v(u) ≥ K?

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

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there an assignment of a non-negative integer c(u) to each u ∈ U such that Σᵤ∈U c(u)·s(u) ≤ B and such that Σᵤ∈U c(u)·v(u) ≥ K?

Reference: [Lueker, 1975]. Transformation from SUBSET SUM.
Comment: Remains NP-complete if s(u) = v(u) for all u ∈ U. Solvable in pseudo-polynomial time by dynamic programming. Solvable in polynomial time if |U| = 2 [Hirschberg and Wong, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
