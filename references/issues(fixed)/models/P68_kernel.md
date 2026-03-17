---
name: Problem
about: Propose a new problem type
title: "[Model] Kernel"
labels: model
assignees: ''
---

## Motivation

KERNEL (P68) from Garey & Johnson, A1.5 GT57. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT57

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A).
QUESTION: Does G have a kernel, i.e., a subset V' ⊆ V such that no two vertices in V' are joined by an arc in A and such that for every vertex v ∈ V−V' there is a vertex u ∈ V' for which (u,v) ∈ A?

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

INSTANCE: Directed graph G = (V,A).
QUESTION: Does G have a kernel, i.e., a subset V' ⊆ V such that no two vertices in V' are joined by an arc in A and such that for every vertex v ∈ V−V' there is a vertex u ∈ V' for which (u,v) ∈ A?

Reference: [Chvátal, 1973]. Transformation from 3SAT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
