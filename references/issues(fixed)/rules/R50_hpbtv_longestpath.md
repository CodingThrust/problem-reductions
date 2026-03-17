---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH BETWEEN TWO VERTICES to LONGEST PATH"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH BETWEEN TWO VERTICES
**Target:** LONGEST PATH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND29, p.213

## GJ Source Entry

> [ND29] LONGEST PATH
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+ for each e∈E, positive integer K, specified vertices s,t∈V.
> QUESTION: Is there a simple path in G from s to t of length K or more, i.e., whose edge lengths sum to at least K?
> Reference: Transformation from HAMILTONIAN PATH BETWEEN TWO VERTICES.
> Comment: Remains NP-complete if l(e)=1 for all e∈E, as does the corresponding problem for directed paths in directed graphs. The general problem can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a]. The analogous directed and undirected "shortest path" problems can be solved for arbitrary graphs in polynomial time (e.g., see [Lawler, 1976a]), but are NP-complete if negative lengths are allowed.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.