---
name: Problem
about: Propose a new problem type
title: "[Model] InducedSubgraphWithPropertyΠ(*)"
labels: model
assignees: ''
---

## Motivation

INDUCED SUBGRAPH WITH PROPERTY Π (*) (P32) from Garey & Johnson, A1.2 GT21. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT21

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' has property Π (see comments for possible choices for Π)?

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
QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' has property Π (see comments for possible choices for Π)?
Reference: [Yannakakis, 1978a], [Yannakakis, 1978b], [Lewis, 1978]. Transformation from 3SAT.
Comment: NP-hard for any property Π that holds for arbitrarily large graphs, does not hold for all graphs, and is "hereditary," i.e., holds for all induced subgraphs of G whenever it holds for G. If in addition one can determine in polynomial time whether Π holds for a graph, then the problem is NP-complete. Examples of such properties Π include "G is a clique," "G is an independent set," "G is planar," "G is bipartite," "G is outerplanar," "G is an edge graph," "G is chordal," "G is a comparability graph," and "G is a forest." The same general results hold if G is restricted to planar graphs and Π satisfies the above constraints for planar graphs, or if G is restricted to acyclic directed graphs and Π satisfies the above constraints for such graphs. A weaker result holds when G is restricted to bipartite graphs [Yannakakis, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
