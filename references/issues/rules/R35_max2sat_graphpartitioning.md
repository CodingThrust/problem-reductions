---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MAX 2SAT to GRAPH PARTITIONING"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'MAX 2-SAT is a specialization of SAT. Implement general version first.'
---

# [Rule] MAX 2SAT → GRAPH PARTITIONING

**Status:** SKIP_SPECIALIZATION

MAX 2-SAT (Maximum 2-Satisfiability) is a known specialization of SAT (maximize the number of satisfied clauses, where each clause has at most 2 literals). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** MAX 2-SAT (Maximum 2-Satisfiability)
- **General version:** SAT (Satisfiability)
- **Restriction:** Maximize satisfied clauses; each clause has at most 2 literals

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND14, p.209-210

> [ND14] GRAPH PARTITIONING
> INSTANCE: Graph G=(V,E), positive integers K, J, and B.
> QUESTION: Can V be partitioned into K disjoint sets V_1,...,V_K such that |V_i|≤J for 1≤i≤K and such that the number of edges with both endpoints in the same V_i is at least B?
> Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from MAX 2SAT.
> Comment: NP-complete even if K=2 [Garey, Johnson, and Stockmeyer, 1976]. Related to MAX CUT (ND16).

## References

- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
