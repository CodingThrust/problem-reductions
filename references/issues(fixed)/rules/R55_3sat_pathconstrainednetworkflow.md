---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PATH CONSTRAINED NETWORK FLOW"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** PATH CONSTRAINED NETWORK FLOW
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND34, p.215

## GJ Source Entry

> [ND34] PATH CONSTRAINED NETWORK FLOW
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, a capacity c(a)∈Z^+ for each a∈A, a collection P of directed paths in G, and a requirement R∈Z^+.
> QUESTION: Is there a function g: P→Z_0^+ such that if f: A→Z_0^+ is the flow function defined by f(a)=Σ_{p∈P(a)} g(p), where P(a)⊆P is the set of all paths in P containing the arc a, then f is such that
> (1) f(a)≤c(a) for all a∈A,
> (2) for each v∈V−{s,t}, flow is conserved at v, and
> (3) the net flow into t is at least R?
> Reference: [Prömel, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if all c(a)=1. The corresponding problem with non-integral flows is equivalent to LINEAR PROGRAMMING, but the question of whether the best rational flow fails to exceed the best integral flow is NP-complete.

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

- **[Prömel, 1978]**: [`Promel1978`] H. J. Pr{\"o}mel (1978). "".