---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SIMPLE MAX CUT to OPTIMAL LINEAR ARRANGEMENT"
labels: rule
assignees: ''
---

**Source:** SIMPLE MAX CUT
**Target:** OPTIMAL LINEAR ARRANGEMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT42

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K.
> QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that ∑_{u,v}∈E |f(u)-f(v)| ≤ K?
>
> Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from SIMPLE MAX CUT.
>
> Comment: Remains NP-complete if G is bipartite [Even and Shiloach, 1975]. Solvable in polynomial time if G is a tree [Shiloach, 1976], [Gol'dberg and Klipker, 1976].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Even and Shiloach, 1975]**: [`Even1975`] S. Even and Y. Shiloach (1975). "{NP}-completeness of several arrangement problems". Dept. of Computer Science, Technion.
- **[Shiloach, 1976]**: [`Shiloach1976`] Yossi Shiloach (1976). "A minimum linear arrangement algorithm for undirected trees". Dept. of Applied Mathematics, Weizmann Institute.
- **[Gol'dberg and Klipker, 1976]**: [`Goldberg1976`] M. K. Gol'dberg and I. A. Klipker (1976). "Minimal placing of trees on a line". Physico-Technical Institute of Low Temperatures, Academy of Sciences of Ukrainian SSR.