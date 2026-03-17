---
name: Problem
about: Propose a new problem type
title: "[Model] InducedConnectedSubgraphWithPropertyΠ(*)"
labels: model
assignees: ''
---

## Motivation

INDUCED CONNECTED SUBGRAPH WITH PROPERTY Π (*) (P33) from Garey & Johnson, A1.2 GT22. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT22

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' is connected and has property Π (see comments for possible choices for Π)?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' is connected and has property Π (see comments for possible choices for Π)?
Reference: [Yannakakis, 1978b]. Transformation from 3SAT.
Comment: NP-hard for any hereditary property that holds for arbitrarily large connected graphs but not for all connected graphs. If, in addition, one can determine in polynomial time whether Π holds for a graph, then the problem is NP-complete. Examples include all the properties mentioned for the preceding problem except "G is an independent set". The related question "Is the maximum induced subgraph of G having property Π also connected?" is not in NP or co-NP unless NP = co-NP [Yannakakis, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
