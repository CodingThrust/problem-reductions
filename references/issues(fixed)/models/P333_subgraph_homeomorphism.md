---
name: Problem
about: Propose a new problem type
title: "[Model] SubgraphHomeomorphism(forAFixedGraphH)"
labels: model
assignees: ''
---

## Motivation

SUBGRAPH HOMEOMORPHISM (FOR A FIXED GRAPH H) (P333) from Garey & Johnson, A13 OPEN2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN2

**Mathematical definition:**

INSTANCE: Graph G = (V, E).
QUESTION: Does G contain a subgraph homeomorphic to H, i.e., a subgraph G' = (V', E') that can be converted to a graph isomorphic to H by repeatedly removing any vertex of degree 2 and adding the edge joining its two neighbors?

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

INSTANCE: Graph G = (V, E).
QUESTION: Does G contain a subgraph homeomorphic to H, i.e., a subgraph G' = (V', E') that can be converted to a graph isomorphic to H by repeatedly removing any vertex of degree 2 and adding the edge joining its two neighbors?

Comment: If H is allowed to vary as part of the instance, the problem is NP-complete, since it contains HAMILTONIAN CIRCUIT as a special case. Solvable in polynomial time for certain fixed graphs H, such as a triangle. Is there any fixed graph H for which this problem is NP-complete? If not, is there any fixed graph H = (U, F) for which the following related problem is NP-complete: Given a graph G = (V, E) and a one-to-one function f: U → V, is there a subgraph G' = (V', E') that can be converted to a graph isomorphic to H as above and such that f provides the required isomorphism? This latter problem is also known to be NP-complete if H is allowed to vary as part of the instance, since it contains DISJOINT CONNECTING PATHS as a special case. Several complicated polynomial time algorithms have been found for particular values of H, such as a triangle [LaPaugh and Rivest, 1978] and two disjoint edges [Shiloach, 1978]. Is there any fixed integer K such that the problem is NP-complete for H consisting of K disjoint edges?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
