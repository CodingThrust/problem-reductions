---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INTEGRAL FLOW WITH HOMOLOGOUS ARCS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INTEGRAL FLOW WITH HOMOLOGOUS ARCS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND35, p.215

## GJ Source Entry

> [ND35] INTEGRAL FLOW WITH HOMOLOGOUS ARCS
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, capacity c(a)∈Z^+ for each a∈A, requirement R∈Z^+, set H⊆A×A of "homologous" pairs of arcs.
> QUESTION: Is there a flow function f: A→Z_0^+ such that
> (1) f(a)≤c(a) for all a∈A,
> (2) for each v∈V−{s,t}, flow is conserved at v,
> (3) for all pairs <a,a'>∈H, f(a)=f(a'), and
> (4) the net flow into t is at least R?
> Reference: [Sahni, 1974]. Transformation from 3SAT.
> Comment: Remains NP-complete if c(a)=1 for all a∈A (by modifying the construction in [Even, Itai, and Shamir, 1976]). Corresponding problem with non-integral flows is polynomially equivalent to LINEAR PROGRAMMING [Itai, 1977].

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
- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691–703.
- **[Itai, 1977]**: [`Itai1977a`] Alon Itai (1977). "Two commodity flow". Dept. of Computer Science, Technion.