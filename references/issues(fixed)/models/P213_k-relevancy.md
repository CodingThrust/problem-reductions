---
name: Problem
about: Propose a new problem type
title: "[Model] KRelevancy"
labels: model
assignees: ''
---

## Motivation

K-RELEVANCY (P213) from Garey & Johnson, A6 MP7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP7

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, and a positive integer K ≤ |X|.
QUESTION: Is there a subset X' ⊆ X with |X'| ≤ K such that, for all m-tuples ȳ of rational numbers, if x̄·ȳ ≤ b for all (x̄,b) ∈ X', then x̄·ȳ ≤ b for all (x̄,b) ∈ X?

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

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, and a positive integer K ≤ |X|.
QUESTION: Is there a subset X' ⊆ X with |X'| ≤ K such that, for all m-tuples ȳ of rational numbers, if x̄·ȳ ≤ b for all (x̄,b) ∈ X', then x̄·ȳ ≤ b for all (x̄,b) ∈ X?

Reference: [Reiss and Dobkin, 1976]. Transformation from X3C.
Comment: NP-complete in the strong sense. Equivalent to linear programming if K = |X| − 1 [Reiss and Dobkin, 1976]. Other NP-complete problems of this form, where a standard linear programming problem is modified by asking that the desired property hold for some subset of K constraints, can be found in the reference.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
