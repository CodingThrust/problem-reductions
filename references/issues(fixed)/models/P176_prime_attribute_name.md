---
name: Problem
about: Propose a new problem type
title: "[Model] PrimeAttributeName"
labels: model
assignees: ''
---

## Motivation

PRIME ATTRIBUTE NAME (P176) from Garey & Johnson, A4 SR28. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR28

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a specified name x ∈ A.
QUESTION: Is x a "prime attribute name" for <A,F>, i.e., is there a key K for <A,F> such that x ∈ K?

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

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a specified name x ∈ A.
QUESTION: Is x a "prime attribute name" for <A,F>, i.e., is there a key K for <A,F> such that x ∈ K?
Reference: [Lucchesi and Osborne, 1977]. Transformation from MINIMUM CARDINALITY KEY.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
