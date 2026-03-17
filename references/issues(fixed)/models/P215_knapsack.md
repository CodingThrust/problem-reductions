---
name: Problem
about: Propose a new problem type
title: "[Model] Knapsack"
labels: model
assignees: ''
---

## Motivation

KNAPSACK (P215) from Garey & Johnson, A6 MP9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP9

**Mathematical definition:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, and positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that Σᵤ∈U' s(u) ≤ B and such that Σᵤ∈U' v(u) ≥ K?

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
QUESTION: Is there a subset U' ⊆ U such that Σᵤ∈U' s(u) ≤ B and such that Σᵤ∈U' v(u) ≥ K?

Reference: [Karp, 1972]. Transformation from PARTITION.
Comment: Remains NP-complete if s(u) = v(u) for all u ∈ U (SUBSET SUM). Can be solved in pseudo-polynomial time by dynamic programming (e.g., see [Dantzig, 1957] or [Lawler, 1976a]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
