---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to BICONNECTIVITY AUGMENTATION"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** BICONNECTIVITY AUGMENTATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND18, p.210

## GJ Source Entry

> [ND18] BICONNECTIVITY AUGMENTATION
> INSTANCE: Graph G=(V,E), weight w({u,v})∈Z^+ for each unordered pair {u,v} of vertices from V, positive integer B.
> QUESTION: Is there a set E' of unordered pairs of vertices from V such that ∑_{e∈E'} w(e)≤B and such that the graph G'=(V,E∪E') is biconnected, i.e., cannot be disconnected by removing a single vertex?
> Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: The related problem in which G' must be bridge-connected, i.e., cannot be disconnected by removing a single edge, is also NP-complete. Both problems remain NP-complete if all weights are either 1 or 2 and E is empty. Both can be solved in polynomial time if all weights are equal.

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

- **[Eswaran and Tarjan, 1976]**: [`Eswaran and Tarjan1976`] K. P. Eswaran and R. E. Tarjan (1976). "Augmentation problems". *SIAM Journal on Computing* 5, pp. 653–665.