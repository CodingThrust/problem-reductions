---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to BOUNDED COMPONENT SPANNING FOREST"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** BOUNDED COMPONENT SPANNING FOREST
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND10, p.208

## GJ Source Entry

> [ND10] BOUNDED COMPONENT SPANNING FOREST
> INSTANCE: Graph G=(V,E), positive integers K and J.
> QUESTION: Does G have a spanning forest with at most K edges and at most J connected components, each of which is a path?
> Reference: [Garey and Johnson, 1979]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: NP-complete even for K=|V|-1 (i.e., spanning trees). Related to the DEGREE-CONSTRAINED SPANNING SUBGRAPH problem (ND14 in the original).

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

- **[Garey and Johnson, 1979]**: [`Garey19xx`] M. R. Garey and D. S. Johnson (1979). "Unpublished results".