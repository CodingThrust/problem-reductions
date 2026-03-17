---
name: Problem
about: Propose a new problem type
title: "[Model] SetBasis"
labels: model
assignees: ''
---

## Motivation

SET BASIS (P134) from Garey & Johnson, A3 SP7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP7

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Is there a collection B of subsets of S with |B| = K such that, for each c ∈ C, there is a subcollection of B whose union is exactly c?

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
QUESTION: Is there a collection B of subsets of S with |B| = K such that, for each c ∈ C, there is a subcollection of B whose union is exactly c?
Reference: [Stockmeyer, 1975]. Transformation from VERTEX COVER.
Comment: Remains NP-complete if all c ∈ C have |c| ≤ 3, but is trivial if all c ∈ C have |c| ≤ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
