---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to ALTERNATING MAXIMUM WEIGHTED MATCHING"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** ALTERNATING MAXIMUM WEIGHTED MATCHING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.256

## GJ Source Entry

> [GP8] ALTERNATING MAXIMUM WEIGHTED MATCHING (*)
> INSTANCE: Graph G = (V,E), a weight w(e) E Z+ for each e E E, and a bound B E Z+.
> QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new edge from E, subject to the constraint that no edge can share an endpoint with any of the already chosen edges. If the sum of the weights of the edges chosen ever exceeds B, player 1 wins.
> Reference: [Dobkin and Ladner, 1978]. Transformation from QBF.
> Comment: PSPACE-complete, even though the corresponding weighted matching problem can be solved in polynomial time (e.g., see [Lawler, 1976a]).

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

- **[Dobkin and Ladner, 1978]**: [`Dobkin1978`] D. Dobkin and R. E. Ladner (1978). "Private communication".
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.