---
name: Problem
about: Propose a new problem type
title: "[Model] ComparativeVectorInequalities"
labels: model
assignees: ''
---

## Motivation

COMPARATIVE VECTOR INEQUALITIES (P219) from Garey & Johnson, A6 MP13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP13

**Mathematical definition:**

INSTANCE: Sets X = {x̄₁,x̄₂,...,x̄ₖ} and Y = {ȳ₁,ȳ₂,...,ȳₗ} of m-tuples of integers.
QUESTION: Is there an m-tuple z̄ of integers such that the number of m-tuples x̄ᵢ satisfying x̄ᵢ ≥ z̄ is at least as large as the number of m-tuples ȳⱼ satisfying ȳⱼ ≥ z̄, where two m-tuples ū and v̄ satisfy ū ≥ v̄ if and only if no component of ū is less than the corresponding component of v̄?

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

INSTANCE: Sets X = {x̄₁,x̄₂,...,x̄ₖ} and Y = {ȳ₁,ȳ₂,...,ȳₗ} of m-tuples of integers.
QUESTION: Is there an m-tuple z̄ of integers such that the number of m-tuples x̄ᵢ satisfying x̄ᵢ ≥ z̄ is at least as large as the number of m-tuples ȳⱼ satisfying ȳⱼ ≥ z̄, where two m-tuples ū and v̄ satisfy ū ≥ v̄ if and only if no component of ū is less than the corresponding component of v̄?

Reference: [Plaisted, 1976]. Transformation from COMPARATIVE CONTAINMENT (with equal weights).
Comment: Remains NP-complete even if all components of the x̄ᵢ and ȳⱼ are required to belong to {0,1}.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
