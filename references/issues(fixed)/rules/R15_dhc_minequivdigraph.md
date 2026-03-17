---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Directed Hamiltonian Circuit to Minimum Equivalent Digraph"
labels: rule
assignees: ''
---

**Source:** Directed Hamiltonian Circuit
**Target:** Minimum Equivalent Digraph
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1 item (5), p.65

## Reduction Algorithm

> (5) MINIMUM EQUIVALENT DIGRAPH
> INSTANCE: A directed graph G = (V,A) and a positive integer K ≤ |A|.
> QUESTION: Is there a directed graph G' = (V,A') such that A' ⊆ A, |A'| ≤ K, and such that, for every pair of vertices u and v in V, G' contains a directed path from u to v if and only if G contains a directed path from u to v.
> Proof: Restrict to DIRECTED HAMILTONIAN CIRCUIT by allowing only instances in which G is strongly connected, that is, contains a path from every vertex u to every vertex v, and K = |V|. Note that this is actually a restriction to DIRECTED HAMILTONIAN CIRCUIT FOR STRONGLY CONNECTED DIGRAPHS, but the NP-completeness of that problem follows immediately from the constructions we gave for HC and DIRECTED HC.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
