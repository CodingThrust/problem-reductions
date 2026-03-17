---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to STRONG CONNECTIVITY AUGMENTATION"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** STRONG CONNECTIVITY AUGMENTATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND19, p.211

## GJ Source Entry

> [ND19] STRONG CONNECTIVITY AUGMENTATION
> INSTANCE: Directed graph G=(V,A), weight w(u,v)∈Z^+ for each ordered pair (u,v)∈V×V, positive integer B.
> QUESTION: Is there a set A' of ordered pairs of vertices from V such that ∑_{a∈A'} w(a)≤B and such that the graph G'=(V,A∪A') is strongly connected?
> Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete if all weights are either 1 or 2 and A is empty. Can be solved in polynomial time if all weights are equal.

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