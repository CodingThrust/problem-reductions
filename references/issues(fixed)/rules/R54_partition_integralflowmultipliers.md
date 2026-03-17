---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to INTEGRAL FLOW WITH MULTIPLIERS"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** INTEGRAL FLOW WITH MULTIPLIERS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND33, p.215

## GJ Source Entry

> [ND33] INTEGRAL FLOW WITH MULTIPLIERS
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, multiplier h(v)∈Z^+ for each v∈V−{s,t}, capacity c(a)∈Z^+ for each a∈A, requirement R∈Z^+.
> QUESTION: Is there a flow function f: A→Z_0^+ such that
> (1) f(a)≤c(a) for all a∈A,
> (2) for each v∈V−{s,t}, Σ_{(u,v)∈A} h(v)·f((u,v)) = Σ_{(v,u)∈A} f((v,u)), and
> (3) the net flow into t is at least R?
> Reference: [Sahni, 1974]. Transformation from PARTITION.
> Comment: Can be solved in polynomial time by standard network flow techniques if h(v)=1 for all v∈V−{s,t}. Corresponding problem with non-integral flows allowed can be solved by linear programming.

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