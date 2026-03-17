---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to DIRECTED TWO-COMMODITY INTEGRAL FLOW"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** DIRECTED TWO-COMMODITY INTEGRAL FLOW
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND38, p.216

## GJ Source Entry

> [ND38] DIRECTED TWO-COMMODITY INTEGRAL FLOW
> INSTANCE: Directed graph G=(V,A), specified vertices s_1, s_2, t_1, and t_2, capacity c(a)∈Z^+ for each a∈A, requirements R_1,R_2∈Z^+.
> QUESTION: Are there two flow functions f_1,f_2: A→Z_0^+ such that
> (1) for each a∈A, f_1(a)+f_2(a)≤c(a),
> (2) for each v∈V−{s,t} and i∈{1,2}, flow f_i is conserved at v, and
> (3) for i∈{1,2}, the net flow into t_i under flow f_i is at least R_i?
> Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if c(a)=1 for all a∈A and R_1=1. Variant in which s_1=s_2, t_1=t_2, and arcs can be restricted to carry only one specified commodity is also NP-complete (follows from [Even, Itai, and Shamir, 1976]). Corresponding M-commodity problem with non-integral flows allowed is polynomially equivalent to LINEAR PROGRAMMING for all M≥2 [Itai, 1977].

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

- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691–703.
- **[Itai, 1977]**: [`Itai1977a`] Alon Itai (1977). "Two commodity flow". Dept. of Computer Science, Technion.