---
name: Problem
about: Propose a new problem type
title: "[Model] GraphIsomorphism"
labels: model
assignees: ''
---

## Motivation

GRAPH ISOMORPHISM (P332) from Garey & Johnson, A13 OPEN1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN1

**Mathematical definition:**

INSTANCE: Two graphs G1 = (V1, E1) and G2 = (V2, E2).
QUESTION: Are G1 and G2 isomorphic, i.e., is there a one-to-one onto function f: V1 → V2 such that {u, v} ∈ E1 if and only if {f(u), f(v)} ∈ E2?

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

[OPEN1] GRAPH ISOMORPHISM
INSTANCE: Two graphs G1 = (V1, E1) and G2 = (V2, E2).
QUESTION: Are G1 and G2 isomorphic, i.e., is there a one-to-one onto function f: V1 → V2 such that {u, v} ∈ E1 if and only if {f(u), f(v)} ∈ E2?
Comment: The problem remains open even if G1 and G2 are restricted to regular graphs, bipartite graphs, line graphs, comparability graphs, chordal graphs, or undirected path graphs (i.e., intersection graphs for the set of paths in an undirected tree), [Hirschberg and Edelberg, 1973], [Babai, 1976], [Booth, 1978], [Miller, 1977]. Solvable in polynomial time for planar graphs (e.g., see [Hopcroft and Wong, 1974]) and for interval graphs [Booth and Lueker, 1975]. The problem is in NP ∩ co-NP for "arc transitive" cubic graphs [Miller, 1977]. Problems polynomially equivalent to GRAPH ISOMORPHISM include directed graph isomorphism, context-free grammar isomorphism [Hunt and Rosenkrantz, 1977], finitely presented algebra isomorphism [Kozen, 1977a], semi-group isomorphism [Booth, 1978], conjunctive query isomorphism [Chandra and Merlin, 1977], the problem of determining whether a graph is isomorphic to its complement [Colbourne and Colbourne, 1978], and the problem of counting the number of distinct isomorphisms between G1 and G2 [Babai, 1977], [Mathon, 1978]. A special case of CLIQUE that is polynomially equivalent to GRAPH ISOMORPHISM is described in [Kozen, 1978]. Isomorphism problems that are perhaps easier than GRAPH ISOMORPHISM include group isomorphism and Latin square isomorphism, both of which can be solved in time O(n^(log n)) [Miller, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
