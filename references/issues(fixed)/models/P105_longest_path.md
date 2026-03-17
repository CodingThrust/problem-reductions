---
name: Problem
about: Propose a new problem type
title: "[Model] LongestPath"
labels: model
assignees: ''
---

## Motivation

LONGEST PATH (P105) from Garey & Johnson, A2 ND29. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND29

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K, specified vertices s,t ∈ V.
QUESTION: Is there a simple path in G from s to t of length K or more, i.e., whose edge lengths sum to at least K?

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

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K, specified vertices s,t ∈ V.
QUESTION: Is there a simple path in G from s to t of length K or more, i.e., whose edge lengths sum to at least K?
Reference: Transformation from HAMILTONIAN PATH BETWEEN TWO VERTICES.
Comment: Remains NP-complete if l(e) = 1 for all e ∈ E, as does the corresponding problem for directed paths in directed graphs. The general problem can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a]. The analogous directed and undirected "shortest path" problems can be solved for arbitrary graphs in polynomial time (e.g., see [Lawler, 1976a]), but are NP-complete if negative lengths are allowed.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
