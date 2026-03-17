---
name: Problem
about: Propose a new problem type
title: "[Model] 2DimensionalConsecutiveSets"
labels: model
assignees: ''
---

## Motivation

2-DIMENSIONAL CONSECUTIVE SETS (P167) from Garey & Johnson, A4 SR19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR19

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, collection C = {Σ1,Σ2,...,Σn} of subsets of Σ.
QUESTION: Is there a partition of Σ into disjoint sets X1,X2,...,Xk such that each Xi has at most one element in common with each Σj and such that, for each Σj ∈ C, there is an index l(j) such that Σj is contained in
Xl(j) ∪ Xl(j)+1 ∪ · · · ∪ Xl(j)+|Σj|−1 ?

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

INSTANCE: Finite alphabet Σ, collection C = {Σ1,Σ2,...,Σn} of subsets of Σ.
QUESTION: Is there a partition of Σ into disjoint sets X1,X2,...,Xk such that each Xi has at most one element in common with each Σj and such that, for each Σj ∈ C, there is an index l(j) such that Σj is contained in
Xl(j) ∪ Xl(j)+1 ∪ · · · ∪ Xl(j)+|Σj|−1 ?
Reference: [Lipsky, 1977b]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete if all Σj ∈ C have |Σj| ≤ 5, but is solvable in polynomial time if all Σj ∈ C have |Σj| ≤ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
