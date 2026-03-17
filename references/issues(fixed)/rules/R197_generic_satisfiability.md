---
name: Rule
about: Propose a new reduction rule
title: "[Rule] (generic transformation) to SATISFIABILITY"
labels: rule
assignees: ''
---

**Source:** (generic transformation)
**Target:** SATISFIABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.1, p.259

## GJ Source Entry

> [LO1] SATISFIABILITY
> INSTANCE: Set U of variables, collection C of clauses over U (see Section 2.6 for definitions).
> QUESTION: Is there a satisfying truth assignment for C?
> Reference: [Cook, 1971a]. Generic transformation.
> Comment: Remains NP-complete even if each c E C satisfies |c| = 3 (3SAT), or if each c E C satisfies |c| ≤ 3 and, for each u E U, there are at most 3 clauses in C that contain either u or ū. Also remains NP-complete if each c E C has |c| ≤ 3 and the bipartite graph G = (V,E), where V = U ∪ C and E contains exactly those pairs {u,c} such that either u or ū belongs to the clause c, is planar (PLANAR 3SAT) [Lichtenstein, 1977]. The general problem is solvable in polynomial time if each c E C has |c| ≤ 2 (e.g., see [Even, Itai, and Shamir, 1976]).

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

- **[Cook, 1971a]**: [`Cook1971a`] S. A. Cook (1971). "The complexity of theorem-proving procedures". In: *Proceedings of the 3rd Annual ACM Symposium on Theory of Computing*, pp. 151–158. Association for Computing Machinery.
- **[Lichtenstein, 1977]**: [`Lichtenstein1977`] David Lichtenstein (1977). "Planar satisfiability and its uses". *SIAM Journal on Computing*.
- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691–703.