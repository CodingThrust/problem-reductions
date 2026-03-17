---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumEquivalentDigraph"
labels: model
assignees: ''
---

## Motivation

MINIMUM EQUIVALENT DIGRAPH (P4) from Garey & Johnson, Chapter 3, Section 3.2.1, p.65. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.1, p.65

**Mathematical definition:**

INSTANCE: A directed graph G = (V,A) and a positive integer K ≤ |A|.
QUESTION: Is there a directed graph G' = (V,A') such that A' ⊆ A, |A'| ≤ K, and such that, for every pair of vertices u and v in V, G' contains a directed path from u to v if and only if G contains a directed path from u to v?

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

INSTANCE: A directed graph G = (V,A) and a positive integer K ≤ |A|.
QUESTION: Is there a directed graph G' = (V,A') such that A' ⊆ A, |A'| ≤ K, and such that, for every pair of vertices u and v in V, G' contains a directed path from u to v if and only if G contains a directed path from u to v?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
