---
name: Problem
about: Propose a new problem type
title: "[Model] EliminationDegreeSequence"
labels: model
assignees: ''
---

## Motivation

ELIMINATION DEGREE SEQUENCE (P58) from Garey & Johnson, A1.3 GT47. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT47

**Mathematical definition:**

INSTANCE: Graph G = (V,E), sequence <d_1,d_2,...,d_{|V|}> of non-negative integers not exceeding |V|−1.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that, for 1 ≤ i ≤ |V|, if f(v) = i then there are exactly d_i vertices u such that f(u) > i and {u,v} ∈ E?

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

INSTANCE: Graph G = (V,E), sequence <d_1,d_2,...,d_{|V|}> of non-negative integers not exceeding |V|−1.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that, for 1 ≤ i ≤ |V|, if f(v) = i then there are exactly d_i vertices u such that f(u) > i and {u,v} ∈ E?

Reference: [Garey, Johnson, and Papadimitriou, 1977]. Transformation from EXACT COVER BY 3-SETS.
Comment: The variant in which it is required that f be such that, for 1 ≤ i ≤ |V|, if f(v) = i then there are exactly d_i vertices u such that {u,v} ∈ E, is trivially solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
