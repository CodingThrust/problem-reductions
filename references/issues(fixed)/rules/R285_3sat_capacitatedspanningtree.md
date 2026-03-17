---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CAPACITATED SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CAPACITATED SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2.1 ND5

## GJ Source Entry

> [ND5] CAPACITATED SPANNING TREE
> INSTANCE: Graph G = (V,E), specified vertex v_0 ∈ V, capacity c(e) ∈ Z_0+ and length l(e) ∈ Z_0+ for each e ∈ E, requirement r(v) ∈ Z_0+ for each v ∈ V − {v_0}, and a bound B ∈ Z_0+.
> QUESTION: Is there a spanning tree T for G such that the sum of the lengths of the edges in T does not exceed B and such that for each edge e in T, if U(e) is the set of vertices whose path to v_0 in T contains e, then ∑_{u ∈ U(e)} r(u) ≤ c(e)?
> Reference: [Papadimitriou, 1976c]. Transformation from 3SAT.
> Comment: NP-complete in the strong sense, even if all requirements are 1 and all capacities are equal to 3. Solvable in polynomial time by weighted matching techniques if all requirements are 1 and all capacities 2. Can also be solved in polynomial time (by minimum cost network flow algorithms, e.g., see [Edmonds and Karp, 1972]) if all capacities are 1 and all requirements are either 0 or 1, but remains NP-complete if all capacities are 2, all requirements 0 or 1, and all edge lengths 0 or 1 [Even and Johnson, 1977].

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

- **[Papadimitriou, 1976c]**: [`Papadimitriou1976c`] Christos H. Papadimitriou (1976). "The complexity of the capacitated tree problem". Center for Research in Computing Technology, Harvard University.
- **[Edmonds and Karp, 1972]**: [`Edmonds1972`] J. Edmonds and R. M. Karp (1972). "Theoretical improvements in algorithmic efficiency for network flow problems". *Journal of the Association for Computing Machinery* 19, pp. 248–264.
- **[Even and Johnson, 1977]**: [`Even1977a`] S. Even and D. S. Johnson (1977). "Unpublished results".