---
name: Problem
about: Propose a new problem type
title: "[Model] SubgraphIsomorphism"
labels: model
assignees: ''
---

## Motivation

SUBGRAPH ISOMORPHISM (P59) from Garey & Johnson, A1.4 GT48. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT48

**Mathematical definition:**

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2).
QUESTION: Does G contain a subgraph isomorphic to H, i.e., a subset V ⊆ V_1 and a subset E ⊆ E_1 such that |V| = |V_2|, |E| = |E_2|, and there exists a one-to-one function f: V_2 → V satisfying {u,v} ∈ E_2 if and only if {f(u),f(v)} ∈ E?

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

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2).
QUESTION: Does G contain a subgraph isomorphic to H, i.e., a subset V ⊆ V_1 and a subset E ⊆ E_1 such that |V| = |V_2|, |E| = |E_2|, and there exists a one-to-one function f: V_2 → V satisfying {u,v} ∈ E_2 if and only if {f(u),f(v)} ∈ E?

Reference: [Cook, 1971a]. Transformation from CLIQUE.
Comment: Contains CLIQUE, COMPLETE BIPARTITE SUBGRAPH, HAMILTONIAN CIRCUIT, etc., as special cases. Can be solved in polynomial time if G is a forest and H is a tree [Edmonds and Matula, 1975] (see also [Reyner, 1977]), but remains NP-complete if G is a tree and H is a forest (see Chapter 4) or if G is a graph and H is a tree (HAMILTONIAN PATH). Variant for directed graphs is also NP-complete, even if G is acyclic and H is a directed tree [Aho and Sethi, 1977], but can be solved in polynomial time if G is a directed forest and H is a directed tree [Reyner, 1977]. If |V_1| = |V_2| and |E_1| = |E_2| we have the GRAPH ISOMORPHISM problem, which is open for both directed and undirected graphs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
