---
name: Rule
about: Propose a new reduction rule
title: "[Rule] INDEPENDENT SET to INTEGRAL FLOW WITH BUNDLES"
labels: rule
assignees: ''
---

**Source:** INDEPENDENT SET
**Target:** INTEGRAL FLOW WITH BUNDLES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND36, p.216

## GJ Source Entry

> [ND36] INTEGRAL FLOW WITH BUNDLES
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, "bundles" I_1,I_2,···,I_k⊆A such that ⋃_{1≤j≤k} I_j=A, bundle capacities c_1,c_2,···,c_k∈Z^+, requirement R∈Z^+.
> QUESTION: Is there a flow function f: A→Z_0^+ such that
> (1) for 1≤j≤k, Σ_{a∈I_j} f(a)≤c_j,
> (2) for each v∈V−{s,t}, flow is conserved at v, and
> (3) the net flow into t is at least R?
> Reference: [Sahni, 1974]. Transformation from INDEPENDENT SET.
> Comment: Remains NP-complete if all capacities are 1 and all bundles have two arcs. Corresponding problem with non-integral flows allowed can be solved by linear programming.

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

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.