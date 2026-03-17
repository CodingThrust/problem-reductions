---
name: Problem
about: Propose a new problem type
title: "[Model] SubsetProduct"
labels: model
assignees: ''
---

## Motivation

SUBSET PRODUCT (P141) from Garey & Johnson, A3 SP14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP14

**Mathematical definition:**

INSTANCE: Finite set A, a size s(a) ∈ Z^+ for each a ∈ A, and a positive integer B.
QUESTION: Is there a subset A' ⊆ A such that the product of the sizes of the elements in A' is exactly B?

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

INSTANCE: Finite set A, a size s(a) ∈ Z^+ for each a ∈ A, and a positive integer B.
QUESTION: Is there a subset A' ⊆ A such that the product of the sizes of the elements in A' is exactly B?
Reference: [Yao, 1978b]. Transformation from X3C.
Comment: NP-complete in the strong sense.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
