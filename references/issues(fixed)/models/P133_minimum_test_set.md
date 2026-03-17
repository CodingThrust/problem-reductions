---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumTestSet"
labels: model
assignees: ''
---

## Motivation

MINIMUM TEST SET (P133) from Garey & Johnson, A3 SP6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP6

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Is there a subcollection C' ⊆ C with |C'| ≤ K such that for each pair of distinct elements u,v ∈ S, there is some set c ∈ C' that contains exactly one of u and v?

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

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Is there a subcollection C' ⊆ C with |C'| ≤ K such that for each pair of distinct elements u,v ∈ S, there is some set c ∈ C' that contains exactly one of u and v?
Reference: [Garey and Johnson, ——]. Transformation from 3DM.
Comment: Remains NP-complete if all c ∈ C have |c| ≤ 3, but is solvable in polynomial time if all c ∈ C have |c| ≤ 2. Variant in which C' can contain unions of subsets in C as well as subsets in C is also NP-complete [Ibaraki, Kameda, and Toida, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
