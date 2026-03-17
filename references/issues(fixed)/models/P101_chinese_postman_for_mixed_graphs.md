---
name: Problem
about: Propose a new problem type
title: "[Model] ChinesePostmanForMixedGraphs"
labels: model
assignees: ''
---

## Motivation

CHINESE POSTMAN FOR MIXED GRAPHS (P101) from Garey & Johnson, A2 ND25. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND25

**Mathematical definition:**

INSTANCE: Mixed graph G = (V,A,E), where A is a set of directed edges and E is a set of undirected edges on V, length l(e) ∈ Z0+ for each e ∈ A∪E, bound B ∈ Z+.
QUESTION: Is there a cycle in G that includes each directed and undirected edge at least once, traversing directed edges only in the specified direction, and that has total length no more than B?

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

INSTANCE: Mixed graph G = (V,A,E), where A is a set of directed edges and E is a set of undirected edges on V, length l(e) ∈ Z0+ for each e ∈ A∪E, bound B ∈ Z+.
QUESTION: Is there a cycle in G that includes each directed and undirected edge at least once, traversing directed edges only in the specified direction, and that has total length no more than B?

Reference: [Papadimitriou, 1976b]. Transformation from 3SAT.
Comment: Remains NP-complete even if all edge lengths are equal, G is planar, and the maximum vertex degree is 3. Can be solved in polynomial time if either A or E is empty (i.e., if G is either a directed or an undirected graph) [Edmonds and Johnson, 1973].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
