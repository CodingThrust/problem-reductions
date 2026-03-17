---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH GRUNDY NUMBERING to DIGRAPH D-MORPHISM"
labels: rule
assignees: ''
---

**Source:** GRAPH GRUNDY NUMBERING
**Target:** DIGRAPH D-MORPHISM
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT53

## Reduction Algorithm

> INSTANCE: Directed graphs G = (V1,A1), H = (V2,A2).
> QUESTION: Is there a D-morphism from G to H, i.e., a function f: V1 → V2 such that for all (u,v) ∈ A1 either (f(u),f(v)) ∈ A2 or (f(v),f(u)) ∈ A2 and such that for all u ∈ V1 and v' ∈ V2 if (f(u),v') ∈ A2 then there exists a v ∈ f^{-1}(v') for which (u,v) ∈ A1?
>
> Reference: [Fraenkel and Yesha, 1977]. Transformation from GRAPH GRUNDY NUMBERING.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Fraenkel and Yesha, 1977]**: [`Fraenkel1977`] A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations".