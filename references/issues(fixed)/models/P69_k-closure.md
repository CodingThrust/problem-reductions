---
name: Problem
about: Propose a new problem type
title: "[Model] KClosure"
labels: model
assignees: ''
---

## Motivation

K-CLOSURE (P69) from Garey & Johnson, A1.5 GT58. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT58

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≤ K such that for all (u,v) ∈ A either u ∈ V' or v ∉ V'?

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

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≤ K such that for all (u,v) ∈ A either u ∈ V' or v ∉ V'?

Reference: [Queyranne, 1976]. Transformation from CLIQUE.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
