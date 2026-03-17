---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Timetable Design"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Timetable Design
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.243

## GJ Source Entry

> [SS19] TIMETABLE DESIGN
> INSTANCE: Set H of "work periods," set C of "craftsmen," set T of "tasks," a subset A(c) ⊆ H of "available hours" for each craftsman c E C, a subset A(t) ⊆ H of "available hours" for each task t E T, and, for each pair (c,t) E C×T, a number R(c,t) E Z_0+ of "required work periods."
> QUESTION: Is there a timetable for completing all the tasks, i.e., a function f: C×T×H → {0,1} (where f(c,t,h) = 1 means that craftsman c works on task t during period h) such that (1) f(c,t,h) = 1 only if h E A(c) ∩ A(t), (2) for each h E H and c E C there is at most one t E T for which f(c,t,h) = 1, (3) for each h E H and t E T there is at most one c E C for which f(c,t,h) = 1, and (4) for each pair (c,t) E C×T there are exactly R(c,t) values of h for which f(c,t,h) = 1?
> Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if |H| = 3, A(t) = H for all t E T, and each R(c,t) E {0,1}. The general problem can be solved in polynomial time if |A(c)| <= 2 for all c E C or if A(c) = A(t) = H for all c E C and t E T.

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