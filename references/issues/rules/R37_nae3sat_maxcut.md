---
name: Rule
about: Propose a new reduction rule
title: "[Rule] NAE3SAT to MAX CUT"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'NAE 3SAT is a specialization of 3SAT. Implement general version first.'
---

# [Rule] NAE3SAT → MAX CUT

**Status:** SKIP_SPECIALIZATION

NAE 3SAT (Not-All-Equal 3-Satisfiability) is a known specialization of 3SAT (no clause is allowed to have all literals evaluate to true). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** NAE 3SAT (Not-All-Equal 3-Satisfiability)
- **General version:** 3SAT (3-Satisfiability)
- **Restriction:** No clause has all literals true; each clause must have at least one true and one false literal

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND16, p.210

> [ND16] MAX CUT
> INSTANCE: Graph G=(V,E), weight w(e)∈Z^+ for each e∈E, positive integer W.
> QUESTION: Is there a partition of V into sets V' and V-V' such that the total weight of edges from E that have one endpoint in V' and one endpoint in V-V' is at least W?
> Reference: [Karp, 1972]. Transformation from NAE3SAT (for the unweighted case).
> Comment: NP-complete even for unweighted graphs [Garey, Johnson, and Stockmeyer, 1976]. Approximable to within a factor of .878 [Goemans and Williamson, 1995]. Can be solved in polynomial time for planar graphs [Hadlock, 1975].

## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Goemans and Williamson, 1995]**: [`Goemans and Williamson1995`] Michel X. Goemans and David P. Williamson (1995). "Improved approximation algorithms for maximum cut and satisfiability problems using semidefinite programming". *Journal of the Association for Computing Machinery* 42(6), pp. 1115–1145.
- **[Hadlock, 1975]**: [`Hadlock1975`] F. O. Hadlock (1975). "Finding a maximum cut of a planar graph in polynomial time". *SIAM Journal on Computing* 4, pp. 221–225.
