---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to GRAPH GRUNDY NUMBERING"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** GRAPH GRUNDY NUMBERING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT56

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A).
> QUESTION: Is there a function f: V → Z+ such that, for each v ∈ V, f(v) is the least non-negative integer not contained in the set {f(u): u ∈ V,(v,u) ∈ A}?
>
> Reference: [van Leeuwen, 1976a]. Transformation from 3SAT.
> Comment: Remains NP-complete when restricted to planar graphs in which no vertex has in- or out-degree exceeding 5 [Fraenkel and Yesha, 1977].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[van Leeuwen, 1976a]**: [`van Leeuwen1976a`] Jan van Leeuwen (1976). "Having a {Grundy}-numbering is {NP}-complete". Computer Science Dept., Pennsylvania State University.
- **[Fraenkel and Yesha, 1977]**: [`Fraenkel1977`] A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations".