---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH 3-COLORABILITY to INTERSECTION PATTERN"
labels: rule
assignees: ''
---

**Source:** GRAPH 3-COLORABILITY
**Target:** INTERSECTION PATTERN
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP9, p.222

## GJ Source Entry

> [SP9] INTERSECTION PATTERN
> INSTANCE: An n×n matrix A=(a_{ij}) with entries in Z_0^+.
> QUESTION: Is there a collection C={C_1,C_2,…,C_n} of sets such that for all i,j, 1≤i,j≤n, a_{ij}=|C_i∩C_j|?
> Reference: [Chvátal, 1978]. Transformation from GRAPH 3-COLORABILITY.
> Comment: Remains NP-complete even if all a_{ii}=3, 1≤i≤m (and hence all C_i must have cardinality 3). If all a_{ii}=2, it is equivalent to edge graph recognition and hence can be solved in polynomial time (e.g., see [Harary, 1969]).

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

- **[Chvátal, 1978]**: [`Chvatal1978`] V. Chv{\'a}tal (1978). "".
- **[Harary, 1969]**: [`Harary1969`] F. Harary (1969). "Graph Theory". Addison-Wesley, Reading, MA.