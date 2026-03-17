---
name: Problem
about: Propose a new problem type
title: "[Model] UniconnectedSubgraph"
labels: model
assignees: ''
---

## Motivation

UNICONNECTED SUBGRAPH (P41) from Garey & Johnson, A1.2 GT30. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT30

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≥ K such that G' = (V,A') has at most one directed path between any pair of vertices?

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

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≥ K such that G' = (V,A') has at most one directed path between any pair of vertices?
Reference: [Maheshwari, 1976]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for acyclic directed graphs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
