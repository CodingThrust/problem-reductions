---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SIMPLE MAX CUT to MINIMUM CUT INTO BOUNDED SETS"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'Simple MAX CUT is a specialization of MAX CUT. Implement general version first.'
---

# [Rule] SIMPLE MAX CUT → MINIMUM CUT INTO BOUNDED SETS

**Status:** SKIP_SPECIALIZATION

Simple MAX CUT is a known specialization of MAX CUT (the unweighted variant where all edge weights are 1). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** SIMPLE MAX CUT
- **General version:** MAX CUT
- **Restriction:** All edge weights are equal to 1 (unweighted variant)

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND17, p.210

> [ND17] MINIMUM CUT INTO BOUNDED SETS
> INSTANCE: Graph G=(V,E), weight w(e)∈Z^+ for each e∈E, specified vertices s,t∈V, positive integer B≤|V|, positive integer K.
> QUESTION: Is there a partition of V into disjoint sets V_1 and V_2 such that s∈V_1, t∈V_2, |V_1|≤B, |V_2|≤B, and such that the sum of the weights of the edges from E that have one endpoint in V_1 and one endpoint in V_2 is no more than K?
> Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from SIMPLE MAX CUT.
> Comment: Remains NP-complete for B=|V|/2 and w(e)=1 for all e∈E. Can be solved in polynomial time for B=|V| by standard network flow techniques.

## References

- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
