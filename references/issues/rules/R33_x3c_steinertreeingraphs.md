---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to STEINER TREE IN GRAPHS"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'X3C is a specialization of Set Covering. Implement general version first.'
---

# [Rule] X3C → STEINER TREE IN GRAPHS

**Status:** SKIP_SPECIALIZATION

X3C (Exact Cover by 3-Sets) is a known specialization of Set Covering (each set has exactly 3 elements, and an exact cover is required). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** X3C (Exact Cover by 3-Sets)
- **General version:** Set Covering
- **Restriction:** Each set has exactly 3 elements; an exact cover (every element covered exactly once) is required

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND12, p.209

> [ND12] STEINER TREE IN GRAPHS
> INSTANCE: Graph G=(V,E), weight w(e)∈Z^+ for each e∈E, a subset R⊆V of required vertices, positive integer B.
> QUESTION: Is there a subtree of G that includes all vertices in R and has total weight no more than B?
> Reference: [Karp, 1972]. Transformation from X3C.
> Comment: NP-complete even for unit weights [Garey and Johnson, 1977]. Approximable to within a factor of 2-2/|R| [Takahashi and Matsuyama, 1980]. The problem is solvable in polynomial time when R=V (minimum spanning tree) or |R| is fixed.

## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Garey and Johnson, 1977]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Takahashi and Matsuyama, 1980]**: [`Takahashi and Matsuyama1980`] Hiromitsu Takahashi and Akira Matsuyama (1980). "An approximate solution for the {Steiner} problem in graphs". *Mathematica Japonica* 24, pp. 573–577.
