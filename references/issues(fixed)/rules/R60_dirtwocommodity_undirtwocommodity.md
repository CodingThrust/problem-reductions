---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DIRECTED TWO-COMMODITY INTEGRAL FLOW to UNDIRECTED TWO-COMMODITY INTEGRAL FLOW"
labels: rule
assignees: ''
---

**Source:** DIRECTED TWO-COMMODITY INTEGRAL FLOW
**Target:** UNDIRECTED TWO-COMMODITY INTEGRAL FLOW
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND39, p.217

## GJ Source Entry

> [ND39] UNDIRECTED TWO-COMMODITY INTEGRAL FLOW
> INSTANCE: Graph G=(V,E), specified vertices s_1, s_2, t_1, and t_2, a capacity c(e)∈Z^+ for each e∈E, requirements R_1,R_2∈Z^+.
> QUESTION: Are there two flow functions f_1,f_2: {(u,v),(v,u): {u,v}∈E}→Z_0^+ such that
> (1) for all {u,v}∈E and i∈{1,2}, either f_i((u,v))=0 or f_i((v,u))=0,
> (2) for each {u,v}∈E,
>  max{f_1((u,v)),f_1((v,u))}+max{f_2((u,v)),f_2((v,u))}≤c({u,v}),
> (3) for each v∈V−{s,t} and i∈{1,2}, flow f_i is conserved at v, and
> (4) for i∈{1,2}, the net flow into t_i under flow f_i is at least R_i?
> Reference: [Even, Itai, and Shamir, 1976]. Transformation from DIRECTED TWO-COMMODITY INTEGRAL FLOW.
> Comment: Remains NP-complete even if c(e)=1 for all e∈E. Solvable in polynomial time if c(e) is even for all e∈E. Corresponding problem with non-integral flows allowed can be solved in polynomial time.

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