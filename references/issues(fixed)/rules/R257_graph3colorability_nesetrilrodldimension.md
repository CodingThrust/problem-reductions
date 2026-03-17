---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH 3-COLORABILITY to NESETRIL-RÖDL DIMENSION"
labels: rule
assignees: ''
---

**Source:** GRAPH 3-COLORABILITY
**Target:** NESETRIL-RÖDL DIMENSION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT62

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Is there a one-to-one function f: V → {(a1,a2, . . . ,aK): 1 ≤ ai ≤ |V| for 1 ≤ i ≤ K} such that, for all u,v ∈ V, {u,v} ∈ E if and only if f(u) and f(v) disagree in all K components?
>
> Reference: [Nesetril and Pultr, 1977]. Transformation from GRAPH 3-COLORABILITY. The definition appears in [Nesetril and Rödl, 1977].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Nesetril and Pultr, 1977]**: [`Nesetril1977`] J. Nesetril and A. Pultr (1977). "The complexity of a dimension of a graph". In: *Proceedings of the Wroclaw Conference on Foundations of Computer Science*.
- **[Nesetril and Rödl, 1977]**: [`Nesetril1977b`] J. Nesetril and V. R{\"o}dl (1977). "A simple proof of Galvin-Ramsey properties of finite graphs and a dimension of a graph".