---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SIMPLE MAX CUT to MINIMUM CUT LINEAR ARRANGEMENT"
labels: rule
assignees: ''
---

**Source:** SIMPLE MAX CUT
**Target:** MINIMUM CUT LINEAR ARRANGEMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT44

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K.
> QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that for all i, 1 < i < |V|,
>
> |{{u,v} ∈ E: f(u) ≤ i < f(v)}| ≤ K ?
>
> Reference: [Stockmeyer, 1974b], [Gavril, 1977a]. Transformation from SIMPLE MAX CUT.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Stockmeyer, 1974b]**: [`Stockmeyer1974b`] Larry J. Stockmeyer (1974). "private communication".
- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.