---
name: Problem
about: Propose a new problem type
title: "[Model] PartiallyOrderedKnapsack"
labels: model
assignees: ''
---

## Motivation

PARTIALLY ORDERED KNAPSACK (P218) from Garey & Johnson, A6 MP12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP12

**Mathematical definition:**

INSTANCE: Finite set U, partial order < on U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that if u ∈ U' and u' < u, then u' ∈ U', and such that Σᵤ∈U' s(u) ≤ B and Σᵤ∈U' v(u) ≥ K?

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

INSTANCE: Finite set U, partial order < on U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that if u ∈ U' and u' < u, then u' ∈ U', and such that Σᵤ∈U' s(u) ≤ B and Σᵤ∈U' v(u) ≥ K?

Reference: [Garey and Johnson, ——]. Transformation from CLIQUE. Problem is discussed in [Ibarra and Kim, 1975b].
Comment: NP-complete in the strong sense, even if s(u) = v(u) for all u ∈ U. General problem is solvable in pseudo-polynomial time if < is a "tree" partial order [Garey and Johnson, ——].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
