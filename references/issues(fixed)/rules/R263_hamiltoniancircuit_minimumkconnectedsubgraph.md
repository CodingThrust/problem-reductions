---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to MINIMUM K-CONNECTED SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** MINIMUM K-CONNECTED SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT31

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integers K ≤ |V| and B ≤ |E|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≤ B such that G' = (V,E') is K-connected, i.e., cannot be disconnected by removing fewer than K vertices?
>
> Reference: [Chung and Graham, 1977]. Transformation from HAMILTONIAN CIRCUIT.
>
> Comment: Corresponding edge-connectivity problem is also NP-complete. Both problems remain NP-complete for any fixed K ≥ 2 and can be solved trivially in polynomial time for K = 1.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Chung and Graham, 1977]**: [`Chung1977`] F. R. K. Chung and R. L. Graham (1977). "".