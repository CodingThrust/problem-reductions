---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MAX 2-SATISFIABILITY to MAX CUT"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'MAX 2-SAT is a specialization of SAT. Implement general version first.'
---

# [Rule] MAX 2-SATISFIABILITY → MAX CUT

**Status:** SKIP_SPECIALIZATION

MAX 2-SAT (Maximum 2-Satisfiability) is a known specialization of SAT (maximize the number of satisfied clauses, where each clause has at most 2 literals). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** MAX 2-SAT (Maximum 2-Satisfiability)
- **General version:** SAT (Satisfiability)
- **Restriction:** Maximize satisfied clauses; each clause has at most 2 literals

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND16, p.210

> [ND16] MAX CUT
> INSTANCE: Graph G=(V,E), weight w(e)∈Z^+ for each e∈E, positive integer K.
> QUESTION: Is there a partition of V into disjoint sets V_1 and V_2 such that the sum of the weights of the edges from E that have one endpoint in V_1 and one endpoint in V_2 is at least K?
> Reference: [Karp, 1972]. Transformation from MAXIMUM 2-SATISFIABILITY.
> Comment: Remains NP-complete if w(e)=1 for all e∈E (the SIMPLE MAX CUT problem) [Garey, Johnson, and Stockmeyer, 1976]. Can be solved in polynomial time if G is planar [Hadlock, 1975], [Orlova and Dorfman, 1972].

## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.
- **[Hadlock, 1975]**: [`Hadlock1975`] F. O. Hadlock (1975). "Finding a maximum cut of a planar graph in polynomial time". *SIAM Journal on Computing* 4, pp. 221–225.
- **[Orlova and Dorfman, 1972]**: [`Orlova1972`] G. I. Orlova and Y. G. Dorfman (1972). "Finding the maximum cut in a graph". *Engineering Cybernetics* 10, pp. 502–506.
