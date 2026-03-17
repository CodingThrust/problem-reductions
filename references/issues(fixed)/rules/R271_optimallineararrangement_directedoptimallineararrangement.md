---
name: Rule
about: Propose a new reduction rule
title: "[Rule] OPTIMAL LINEAR ARRANGEMENT to DIRECTED OPTIMAL LINEAR ARRANGEMENT"
labels: rule
assignees: ''
---

**Source:** OPTIMAL LINEAR ARRANGEMENT
**Target:** DIRECTED OPTIMAL LINEAR ARRANGEMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT43

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A), positive integer K.
> QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that f(u) < f(v) whenever (u,v) ∈ A and such that ∑_{(u,v)∈A} (f(v)-f(u)) ≤ K?
>
> Reference: [Even and Shiloach, 1975]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
>
> Comment: Solvable in polynomial time if G is a tree, even if each edge has a given integer weight and the cost function is a weighted sum [Adolphson and Hu, 1973].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Even and Shiloach, 1975]**: [`Even1975`] S. Even and Y. Shiloach (1975). "{NP}-completeness of several arrangement problems". Dept. of Computer Science, Technion.
- **[Adolphson and Hu, 1973]**: [`Adolphson1973`] D. Adolphson and T. C. Hu (1973). "Optimal linear ordering". *SIAM Journal on Applied Mathematics* 25, pp. 403–423.