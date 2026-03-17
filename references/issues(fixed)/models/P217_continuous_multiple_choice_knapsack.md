---
name: Problem
about: Propose a new problem type
title: "[Model] ContinuousMultipleChoiceKnapsack"
labels: model
assignees: ''
---

## Motivation

CONTINUOUS MULTIPLE CHOICE KNAPSACK (P217) from Garey & Johnson, A6 MP11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP11

**Mathematical definition:**

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, a partition of U into disjoint sets U₁,U₂,...,Uₘ, and positive integers B and K.
QUESTION: Is there a choice of a unique element uᵢ ∈ Uᵢ, 1 ≤ i ≤ m, and an assignment of rational numbers rᵢ, 0 ≤ rᵢ ≤ 1, to these elements, such that Σᵢ₌₁ᵐ rᵢ·s(uᵢ) ≤ B and Σᵢ₌₁ᵐ rᵢ·v(uᵢ) ≥ K?

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

INSTANCE: Finite set U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, a partition of U into disjoint sets U₁,U₂,...,Uₘ, and positive integers B and K.
QUESTION: Is there a choice of a unique element uᵢ ∈ Uᵢ, 1 ≤ i ≤ m, and an assignment of rational numbers rᵢ, 0 ≤ rᵢ ≤ 1, to these elements, such that Σᵢ₌₁ᵐ rᵢ·s(uᵢ) ≤ B and Σᵢ₌₁ᵐ rᵢ·v(uᵢ) ≥ K?

Reference: [Ibaraki, 1978]. Transformation from PARTITION.
Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if |Uᵢ| ≤ 2, 1 ≤ i ≤ m. Solvable in polynomial time by "greedy" algorithms if |Uᵢ| = 1, 1 ≤ i ≤ m, or if we only require that the rᵢ ≥ 0 but place no upper bound on them. [Ibaraki, Hasegawa, Teranaka, and Iwase, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
